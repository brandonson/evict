/*
 *   Copyright 2013 Brandon Sanderson
 *
 *   This file is part of Evict-BT.
 *
 *   Evict-BT is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   Evict-BT is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *   along with Evict-BT.  If not, see <http://www.gnu.org/licenses/>.
 */
use fsm;
use std::io;
use issue::Issue;
use file_manager;
use file_util;
use vcs_status;
use commands;
use config;
use status_storage;

static DEFAULT_ISSUE_BODY_FILE:&'static str = "ISSUE_MSG";
#[deriving(Clone)]
struct Flags{
  hasBody:bool,
  bodyFile:Option<~str>,
  title:Option<~str>,
  author:Option<~str>,
}

fn stdHandler(flags:Flags, input:~str) -> fsm::NextState<Flags,~str> {
  match input {
    ~"--no-body" => fsm::Continue(Flags{hasBody:false, 
                                         .. flags}),
    ~"--body-file" => fsm::ChangeState(getBodyFile, flags),
    ~"--title" => fsm::ChangeState(getTitle, flags),
    ~"--author" => fsm::ChangeState(getAuthor, flags),
    _ => fsm::Continue(flags)
  }
}
fn getBodyFile(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(stdHandler, Flags{bodyFile:Some(input), .. flags})
}
fn getTitle(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(stdHandler, Flags{title:Some(input), .. flags})
}
fn getAuthor(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(stdHandler, Flags{author:Some(input), .. flags})
}

pub fn createIssue(args:~[~str], _:config::Config) -> int {
  let mut stateMachine = fsm::StateMachine::new(stdHandler, 
                                           Flags{hasBody:true, 
                                                 bodyFile:None, 
                                                 title:None,
						 author:None});
  for argVal in args.move_iter() {
    stateMachine.process(argVal);
  };
  let finalFlags = stateMachine.consumeToState();
  let title = match finalFlags.title {
    Some(ref titleVal) => titleVal.to_owned(),
    None => commands::prompt("Title: ")
  };
  let author = match finalFlags.author {
    Some(ref authorVal) => authorVal.to_owned(),
    None => commands::getAuthor()
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
  let created = doIssueCreation(title, author, bodyFile);
  if(editedBodyFile){ file_util::deleteFile(DEFAULT_ISSUE_BODY_FILE); };
  if(created.is_some()){
    io::println(fmt!("Issue %s created.", created.unwrap().id)); 
    0
  }else{
    1
  }
}

fn doIssueCreation(title:~str, author:~str, bodyFile:Option<~str>) -> Option<~Issue>{
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
    let mut issue = issueOpt.unwrap();
    issue.status = status_storage::readDefaultStatus().makeStatus();
    if(writeIssue(issue.clone())){
      Some(issue)
    }else{
      io::println("Could not write issue to file.");
      None
    }
  }
}

fn writeIssue(issue:~Issue) -> bool{
  let branchnameOpt = vcs_status::currentBranch();
  if(branchnameOpt.is_none()){
    io::println("Could determine current branch.  Is there an active VCS for this directory?");
    return false;
  }
  
  let branchname = branchnameOpt.unwrap();
  let mut committable = file_manager::readCommittableIssues(branchname);
  committable.push(issue);
  file_manager::writeCommittableIssues(branchname, committable)
}

