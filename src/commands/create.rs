use fsm;
use std::io;
use issue::Issue;
use file_manager;
use file_util;
use vcs_status;
use commands;

static DEFAULT_ISSUE_BODY_FILE:&'static str = "ISSUE_MSG";
#[deriving(Clone)]
struct Flags{
  hasBody:bool,
  bodyFile:Option<~str>,
  title:Option<~str>,
  author:Option<~str>,
  local:bool
}

fn stdHandler(flags:&Flags, input:~str) -> fsm::NextState<Flags,~str> {
  match input {
    ~"--no-body" => fsm::Continue(~Flags{hasBody:false, 
                                         .. (*flags).clone()}),
    ~"--body-file" => fsm::ChangeState(getBodyFile, ~((*flags).clone())),
    ~"--title" => fsm::ChangeState(getTitle, ~((*flags).clone())),
    ~"--local" => fsm::Continue(~Flags{local:true, .. (*flags).clone()}),
    ~"--author" => fsm::ChangeState(getAuthor, ~((*flags).clone())),
    _ => fsm::Continue(~((*flags).clone()))
  }
}
fn getBodyFile(flags:&Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(stdHandler, ~Flags{bodyFile:Some(input), .. (*flags).clone()})
}
fn getTitle(flags:&Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(stdHandler, ~Flags{title:Some(input), .. (*flags).clone()})
}
fn getAuthor(flags:&Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(stdHandler, ~Flags{author:Some(input), .. (*flags).clone()})
}

pub fn createIssue(args:~[~str]) -> int {
  let mut stateMachine = fsm::StateMachine::new(stdHandler, 
                                           ~Flags{hasBody:true, 
                                                 bodyFile:None, 
                                                 title:None,
						 author:None,
                                                 local:false});
  for argVal in args.iter() {
    stateMachine.process(argVal.clone());
  };
  let finalFlags = stateMachine.consumeToState();
  let isLocal = finalFlags.local;
  let title = match finalFlags.title {
    Some(ref titleVal) => titleVal.to_owned(),
    None => commands::prompt("Title: ")
  };
  let author = match finalFlags.author {
    Some(ref authorVal) => authorVal.to_owned(),
    None => commands::prompt("Author: ")
  };
  let mut editedBodyFile = false;
  let bodyFile = if(finalFlags.hasBody && finalFlags.bodyFile.is_none()){
    editedBodyFile =  commands::editFile(DEFAULT_ISSUE_BODY_FILE);
    if(!editedBodyFile){
      return 2;
    }
    Some(DEFAULT_ISSUE_BODY_FILE.to_owned())
  }else if(!finalFlags.hasBody){
    None
  }else{
    finalFlags.bodyFile
  };
  let created = doIssueCreation(title, author, bodyFile, isLocal);
  if(editedBodyFile){ file_util::deleteFile(DEFAULT_ISSUE_BODY_FILE); };
  if(created.is_some()){
    io::println(fmt!("Issue %s created.", created.unwrap().id)); 
    0
  }else{
    1
  }
}

fn doIssueCreation(title:~str, author:~str, bodyFile:Option<~str>, local:bool) -> Option<~Issue>{
  let issueOpt = if(bodyFile.is_none()){
                   Some(Issue::new(title, ~"", author, Issue::generateId()))
                 }else{
                   let bodyTextOpt = file_util::readStringFromFile(bodyFile.unwrap());
                   do bodyTextOpt.map_move |text| {
                     Issue::new(title.clone(), text, author.clone(), Issue::generateId())
		   }
                 };
  if(issueOpt.is_none()){
    io::println(fmt!("Could not open body file."));
    None
  }else{
    let issue = issueOpt.unwrap();
    if(writeIssue(issue.clone(), local)){
      Some(issue)
    }else{
      io::println("Could not write issue to file.");
      None
    }
  }
}

fn writeIssue(issue:~Issue, local:bool) -> bool{
  let branchnameOpt = vcs_status::currentBranch();
  if(branchnameOpt.is_none()){
    io::println("Could determine current branch.  Is there an active VCS for this directory?");
    return false;
  }
  
  let branchname = branchnameOpt.unwrap();
  if(!local){
    let mut committable = file_manager::readCommittableIssues(branchname);
    committable.push(issue);
    file_manager::writeCommittableIssues(branchname, committable)
  }else{
    let mut local = file_manager::readLocalIssues(branchname);
    local.push(issue);
    file_manager::writeLocalIssues(branchname, local)
  }
}

