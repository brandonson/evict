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
use fsm::NextState::*;
use fsm::*;
use issue::Issue;
use file_manager;
use file_util;
use commands;

use status_storage;

use std::io::Result as IoResult;

// EVICT-BT-ID: 1399720517980750949
// [conventions] bodyFile in Flags should be body_file

static DEFAULT_ISSUE_BODY_FILE:&'static str = "ISSUE_MSG";
struct Flags{
  hasBody:bool,
  bodyFile:Option<String>,
  title:Option<String>,
  author:Option<String>,
}

fn std_handler(flags:Flags, input:String) -> NextState<Flags,String> {
  match input.as_str() {
    "--no-body" => Continue(Flags{hasBody:false, 
                                         .. flags}),
    "--body-file" => ChangeState(get_body_file, flags),
    "--title" => ChangeState(get_title, flags),
    "--author" => ChangeState(get_author, flags),
    _ => Continue(flags)
  }
}
fn get_body_file(flags:Flags, input:String) -> NextState<Flags, String> {
  ChangeState(std_handler, Flags{bodyFile:Some(input), .. flags})
}
fn get_title(flags:Flags, input:String) -> NextState<Flags, String> {
  ChangeState(std_handler, Flags{title:Some(input), .. flags})
}
fn get_author(flags:Flags, input:String) -> NextState<Flags, String> {
  ChangeState(std_handler, Flags{author:Some(input), .. flags})
}

pub fn create_issue(args:Vec<String>) -> isize {
  let mut stateMachine = StateMachine::new(std_handler, 
                                           Flags{hasBody:true, 
                                                 bodyFile:None, 
                                                 title:None,
	                                         author:None});
  for argVal in args.into_iter() {
    stateMachine.process(argVal);
  };
  let finalFlags = stateMachine.extract_state();
  let title = match finalFlags.title {
    Some(ref titleVal) => titleVal.to_string(),
    None => commands::prompt("Title: ")
  };
  let author = match finalFlags.author {
    Some(ref authorVal) => authorVal.to_string(),
    None => commands::get_author()
  };
  let mut editedBodyFile = false;
  let bodyFile = if finalFlags.hasBody && finalFlags.bodyFile.is_none() {
    editedBodyFile =  commands::edit_file(DEFAULT_ISSUE_BODY_FILE);
    if !editedBodyFile {
      return 2;
    }
    Some(DEFAULT_ISSUE_BODY_FILE.to_string())
  }else if !finalFlags.hasBody {
    None
  }else{
    finalFlags.bodyFile
  };
  let created = do_issue_creation(title, author, bodyFile);
  if editedBodyFile { file_util::delete_file(DEFAULT_ISSUE_BODY_FILE); };
  if created.is_ok() {
    println!("Issue {} created.", created.unwrap().id()); 
    0
  }else{
    println!("Issue creation failed.");
    1
  }
}

fn do_issue_creation(title:String, author:String, bodyFile:Option<String>) -> IoResult<Issue>{
  let mut issue = try!(if bodyFile.is_none() {
                   Ok(Issue::new(title, "".to_string(), author))
                 }else{
                   let bodyTextOpt = file_util::read_string_from_file(bodyFile.unwrap().as_str());
                   bodyTextOpt.map(
                     |text| Issue::new(title.clone(), text, author.clone())
                   )
                 });
  issue.status = status_storage::read_default_status().make_status();
  write_issue(issue.clone()).map(|_| issue)
}

fn write_issue(issue:Issue) -> IoResult<()> {
  let mut committable = file_manager::read_issues();
  committable.push(issue);
  file_manager::write_issues(committable.as_slice())
}

