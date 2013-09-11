use std::io::println;
use issue::Issue;
use file_manager;
use vcs_status;
use fsm;
use selection;
use config;

#[deriving(Clone)]
struct Flags{
  issue:Option<~str>
}

fn stdHandler(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  match input {
    ident => fsm::Continue(Flags{issue:Some(ident), .. flags})
  }
}

pub fn deleteIssue(args:~[~str], _:config::Config) -> int {
  let mut stateMachine = fsm::StateMachine::new(stdHandler, Flags{issue:None});
  for arg in args.move_iter() {
    stateMachine.process(arg);
  }
  let finalFlags = stateMachine.consumeToState();

  if(finalFlags.issue.is_none()){
    println("The id of the issue to delete or an end segment of the id must be provided.");
    1
  }else {
    let cBranch = vcs_status::currentBranch();
    if(cBranch.is_none()){
      2
    }else{
      let issueIdPart = finalFlags.issue.unwrap();
      let committed = checkCommitted(issueIdPart);
      if(committed){
        3
      }else{
	execDelete(cBranch.unwrap(), issueIdPart)
      }
    }
  }
}
fn checkCommitted(idPart:&str) -> bool {
  let committed = file_manager::readCommittedIssues();
  let mut result = false;
  for issue in committed.iter(){
    if(issue.id.ends_with(idPart)){
      println(fmt!("Issue %s (%s) has already been committed, cannot delete.", 
                   issue.id, issue.title));
      result = true;
    } 
  }
  return result;
}
fn execDelete(branch:~str, idPart:~str) -> int{
  let issues = file_manager::readCommittableIssues(branch);
  let matching = selection::findMatchingIssues(idPart, issues);
  if(matching.len() == 0){
    println(fmt!("No issue matching %s found.", idPart));
    4
  }else if(matching.len() == 1){
    let issueCount = issues.len();

    let mut remaining:~[~Issue] = ~[];
    for issue in issues.move_iter() {
       if(issue != matching[0]){
         remaining.push(issue);
       }
    }
    //We really, REALLY don't want to be deleting issues we don't expect to be
    assert!(issueCount - 1 == remaining.len());
    file_manager::writeCommittableIssues(branch, remaining);
    println(fmt!("Issue %s (%s) deleted.", matching[0].id, matching[0].title));
    0
  }else{
    println("Multiple matching issues found:");
    for issue in matching.iter() {
      println(fmt!("%s (%s)", issue.id, issue.title));
    }
    5
  }
}

