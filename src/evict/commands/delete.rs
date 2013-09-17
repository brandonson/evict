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

fn std_handler(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  match input {
    ident => fsm::Continue(Flags{issue:Some(ident), .. flags})
  }
}

pub fn delete_issue(args:~[~str], _:config::Config) -> int {
  let mut stateMachine = fsm::StateMachine::new(std_handler, Flags{issue:None});
  for arg in args.move_iter() {
    stateMachine.process(arg);
  }
  let finalFlags = stateMachine.move_state();

  if(finalFlags.issue.is_none()){
    println("The id of the issue to delete or an end segment of the id must be provided.");
    1
  }else {
    let cBranch = vcs_status::current_branch();
    if(cBranch.is_none()){
      2
    }else{
      let issueIdPart = finalFlags.issue.unwrap();
      let committed = check_committed(issueIdPart);
      if(committed){
        3
      }else{
	exec_delete(cBranch.unwrap(), issueIdPart)
      }
    }
  }
}
fn check_committed(idPart:&str) -> bool {
  let committed = file_manager::read_committed_issues();
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
fn exec_delete(branch:~str, idPart:~str) -> int{
  let issues = file_manager::read_committable_issues(branch);
  let matching = selection::find_matching_issues(idPart, issues);
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
    file_manager::write_committable_issues(branch, remaining);
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

