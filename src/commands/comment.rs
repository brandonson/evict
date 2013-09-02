use fsm;
use issue::{Issue,IssueComment};
use vcs_status;
use file_manager;
use std::io;
use file_util;
use commands;
use selection;
use config;

#[deriving(Clone)]
struct Flags{
  issueIdPart:Option<~str>
}

fn stdHandler(flags:&Flags, arg:~str) -> fsm::NextState<Flags, ~str> {
  match arg {
    idPart => fsm::Continue(~Flags{issueIdPart:Some(idPart), .. (*flags).clone()})
  }
}

pub fn newComment(args:~[~str], _:config::Config) -> int{
  let mut stateMachine = fsm::StateMachine::new(stdHandler, ~Flags{issueIdPart:None});
  for a in args.move_iter(){
    stateMachine.process(a);
  }

  let finalFlags = stateMachine.consumeToState();
  if(finalFlags.issueIdPart.is_none()){
    io::println("The id for the issue, or an end section of it must be provided.");
    1
  }else{
    let branchOpt = vcs_status::currentBranch();
    if(branchOpt.is_none()){
      io::println("Could not resolve current branch.");
      2
    }else{
      let branch = branchOpt.unwrap();
      let issues = file_manager::readCommittableIssues(branch);

      let matching = selection::findMatchingIssues(finalFlags.issueIdPart.unwrap(), 
                                                   issues);
      match commentOnMatching(matching){
        Ok(issue) => processNewIssue(issues, issue, branch),
	Err(exitcode) => exitcode
      }
    }
  }
}

fn commentOnMatching(matching:~[~Issue]) -> Result<~Issue,int> {
  if(matching.len() == 0){
    io::println("No issue matching the given id found.");
    Err(3)
  }else if(matching.len() == 1){
    let author = commands::getAuthor();
    let filename = fmt!("COMMENT_ON_%s",matching[0].id);
    let edited = commands::editFile(filename);
    if(!edited){
      io::println("No comment body provided");
      Err(4)
    }else{
      let text = file_util::readStringFromFile(filename);
      file_util::deleteFile(filename);
      if(text.is_none()){
        io::println("Could not read comment body from file");
	Err(5)
      }else{
        let newComment = IssueComment::new(author, text.unwrap());
        let mut newComments = matching[0].comments.clone();
        newComments.push(newComment);
        let newIssue = ~Issue{comments:newComments,
                              .. *matching[0]};
        Ok(newIssue)
      }
    }
  }else{
    io::println("Multiple matching issues");
    for issue in matching.iter() {
      io::println(fmt!("%s (%s)", issue.id, issue.title));
    }
    Err(6)
  }
}

fn processNewIssue(allIssues:~[~Issue], newIssue:~Issue, branch:~str) -> int {
  let allIssuesLen = allIssues.len();
  let mut newIssues:~[~Issue] = allIssues.move_iter().filter(
                                                     |issue| {issue.id != newIssue.id})
                                           .collect();
  assert!(newIssues.len() == allIssuesLen - 1);
  
  newIssues.push(newIssue);
  
  let success = file_manager::writeCommittableIssues(branch, newIssues);
  if(success){
    0
  }else{
    7
  }
}
