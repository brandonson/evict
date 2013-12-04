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
use issue::Issue;
use file_manager;
use fsm;
use selection;


#[deriving(Clone)]
struct Flags{
  issue:Option<~str>
}

fn std_handler(flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  match input {
    ident => fsm::Continue(Flags{issue:Some(ident), .. flags})
  }
}

pub fn delete_issue(args:~[~str]) -> int {
  let mut stateMachine = fsm::StateMachine::new(std_handler, Flags{issue:None});
  for arg in args.move_iter() {
    stateMachine.process(arg);
  }
  let finalFlags = stateMachine.move_state();

  if(finalFlags.issue.is_none()){
    println("The id of the issue to delete or an end segment of the id must be provided.");
    1
  }else {
    let issueIdPart = finalFlags.issue.unwrap();
    exec_delete(issueIdPart)
  }
}
fn exec_delete(idPart:~str) -> int{
  let issues = file_manager::read_issues();
  let matching = selection::find_matching_issues(idPart, issues);
  if(matching.len() == 0){
    println(format!("No issue matching {} found.", idPart));
    4
  }else if(matching.len() == 1){
    let issueCount = issues.len();

    let mut remaining:~[Issue] = ~[];
    for issue in issues.move_iter() {
       if(issue != matching[0]){
         remaining.push(issue);
       }
    }
    //We really, REALLY don't want to be deleting issues we don't expect to be
    assert!(issueCount - 1 == remaining.len());
    file_manager::write_issues(remaining);
    println(format!("Issue {} ({}) deleted.", matching[0].id, matching[0].title));
    0
  }else{
    println("Multiple matching issues found:");
    for issue in matching.iter() {
      println(format!("{} ({})", issue.id, issue.title));
    }
    5
  }
}

