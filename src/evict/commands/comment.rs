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

fn std_handler(flags:Flags, arg:~str) -> fsm::NextState<Flags, ~str> {
  match arg {
    idPart => fsm::Continue(Flags{issueIdPart:Some(idPart), .. flags})
  }
}

pub fn new_comment(args:~[~str], _:config::Config) -> int{
  let mut stateMachine = fsm::StateMachine::new(std_handler, Flags{issueIdPart:None});
  for a in args.move_iter(){
    stateMachine.process(a);
  }

  let finalFlags = stateMachine.move_state();
  if(finalFlags.issueIdPart.is_none()){
    io::println("The id for the issue, or an end section of it must be provided.");
    1
  }else{
    let branchOpt = vcs_status::current_branch();
    if(branchOpt.is_none()){
      io::println("Could not resolve current branch.");
      2
    }else{
      let branch = branchOpt.unwrap();
      let issues = file_manager::read_committable_issues(branch);

      let matching = selection::find_matching_issues(finalFlags.issueIdPart.unwrap(), 
                                                   issues);
      match comment_on_matching(matching){
        Ok(issue) => process_new_issue(issues, issue, branch),
	Err(exitcode) => exitcode
      }
    }
  }
}

fn comment_on_matching(matching:~[~Issue]) -> Result<~Issue,int> {
  if(matching.len() == 0){
    io::println("No issue matching the given id found.");
    Err(3)
  }else if(matching.len() == 1){
    let author = commands::get_author();
    let filename = fmt!("COMMENT_ON_%s",matching[0].id);
    let edited = commands::edit_file(filename);
    if(!edited){
      io::println("No comment body provided");
      Err(4)
    }else{
      let text = file_util::read_string_from_file(filename);
      file_util::delete_file(filename);
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

fn process_new_issue(allIssues:~[~Issue], newIssue:~Issue, branch:~str) -> int {
  let allIssuesLen = allIssues.len();
  let mut newIssues:~[~Issue] = allIssues.move_iter().filter(
                                                     |issue| {issue.id != newIssue.id})
                                           .collect();
  assert!(newIssues.len() == allIssuesLen - 1);
  
  newIssues.push(newIssue);
  
  let success = file_manager::write_committable_issues(branch, newIssues);
  if(success){
    0
  }else{
    7
  }
}
