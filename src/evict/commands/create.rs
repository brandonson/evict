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
use issue::Issue;
use file_manager;
use file_util;
use commands;

use status_storage;

static DEFAULT_ISSUE_BODY_FILE:&'static str = "ISSUE_MSG";
#[deriving(Clone)]
struct Flags{
  hasBody:bool,
  bodyFile:Option<~str>,
  title:Option<~str>,
  author:Option<~str>,
}

fn std_handler(flags:Flags, input:~str) -> fsm::NextState<Flags,~str> {
  match input.as_slice() {
    "--no-body" => fsm::Continue(Flags{hasBody:false, 
                                         .. flags}),
    "--body-file" => fsm::ChangeState(get_body_file, flags),
    "--title" => fsm::ChangeState(get_title, flags),
    "--author" => fsm::ChangeState(get_author, flags),
    _ => fsm::Continue(flags)
  }
}
fn get_body_file(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(std_handler, Flags{bodyFile:Some(input), .. flags})
}
fn get_title(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(std_handler, Flags{title:Some(input), .. flags})
}
fn get_author(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  fsm::ChangeState(std_handler, Flags{author:Some(input), .. flags})
}

pub fn create_issue(args:~[~str]) -> int {
  let mut stateMachine = fsm::StateMachine::new(std_handler, 
                                           Flags{hasBody:true, 
                                                 bodyFile:None, 
                                                 title:None,
						 author:None});
  for argVal in args.move_iter() {
    stateMachine.process(argVal);
  };
  let finalFlags = stateMachine.move_state();
  let title = match finalFlags.title {
    Some(ref titleVal) => titleVal.to_owned(),
    None => commands::prompt("Title: ")
  };
  let author = match finalFlags.author {
    Some(ref authorVal) => authorVal.to_owned(),
    None => commands::get_author()
  };
  let mut editedBodyFile = false;
  let bodyFile = if finalFlags.hasBody && finalFlags.bodyFile.is_none() {
    editedBodyFile =  commands::edit_file(DEFAULT_ISSUE_BODY_FILE);
    if !editedBodyFile {
      return 2;
    }
    Some(DEFAULT_ISSUE_BODY_FILE.to_owned())
  }else if !finalFlags.hasBody {
    None
  }else{
    finalFlags.bodyFile
  };
  let created = do_issue_creation(title, author, bodyFile);
  if editedBodyFile { file_util::delete_file(DEFAULT_ISSUE_BODY_FILE); };
  if created.is_some() {
    println!("Issue {} created.", created.unwrap().id); 
    0
  }else{
    1
  }
}

fn do_issue_creation(title:~str, author:~str, bodyFile:Option<~str>) -> Option<Issue>{
  let issueOpt = if bodyFile.is_none() {
                   Some(Issue::new(title, ~"", author))
                 }else{
                   let bodyTextOpt = file_util::read_string_from_file(bodyFile.unwrap());
                   bodyTextOpt.map(
                     |text| Issue::new(title.clone(), text, author.clone())
                   )
                 };
  if issueOpt.is_none() {
    println!("Could not open body file.");
    None
  }else{
    let mut issue = issueOpt.unwrap();
    issue.status = status_storage::read_default_status().make_status();
    if write_issue(issue.clone()) {
      Some(issue)
    }else{
      println!("Could not write issue to file.");
      None
    }
  }
}

fn write_issue(issue:Issue) -> bool{
  let mut committable = file_manager::read_issues();
  committable.push(issue);
  file_manager::write_issues(committable)
}

