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
use issue::{Issue,IssueComment,TimelineComment};
use file_manager;
use file_util;
use commands;
use selection;


#[deriving(Clone)]
struct Flags{
  issueIdPart:Option<~str>
}

fn std_handler(flags:Flags, arg:~str) -> fsm::NextState<Flags, ~str> {
  match arg {
    idPart => fsm::Continue(Flags{issueIdPart:Some(idPart), .. flags})
  }
}

pub fn new_comment(args:~[~str]) -> int{
  let mut stateMachine = fsm::StateMachine::new(std_handler, Flags{issueIdPart:None});
  for a in args.move_iter(){
    stateMachine.process(a);
  }

  let finalFlags = stateMachine.move_state();
  if(finalFlags.issueIdPart.is_none()){
    println("The id for the issue, or an end section of it must be provided.");
    1
  }else{
    let issues = file_manager::read_issues();

    let updated = selection::update_issue(finalFlags.issueIdPart.unwrap(), 
                                          issues,
                                          comment_on_matching);
    if(file_manager::write_issues(updated)){
      0
    }else{
      1
    }
  }
}

fn comment_on_matching(matching:Issue) -> Issue {
  let author = commands::get_author();
  let filename = format!("COMMENT_ON_{}",matching.id);
  let edited = commands::edit_file(filename);
  if(!edited){
    println("No comment body provided");
    matching 
  }else{
    let text = file_util::read_string_from_file(filename);
    file_util::delete_file(filename);
    if(text.is_none()){
      println("Could not read comment body from file");
      matching
    }else{
      let newComment = TimelineComment(IssueComment::new(author, text.unwrap()));
      let mut newEvents = matching.events.clone();
      newEvents.push(newComment);
      let newIssue = Issue{events:newEvents,
                            .. matching};
      newIssue
    }
  }
}

