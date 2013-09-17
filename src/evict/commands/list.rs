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
use file_manager;
use vcs_status;
use issue;
use issue::Issue;
use config;
use file_util;
use std::run;
use fsm;
use selection;
use date_sort;

static TMP_OUTPUT_FILE:&'static str = ".evict/LIST_TEMP_FILE";

#[deriving(Clone, Eq)]
struct TimeSortedIssue(~Issue);

impl Ord for TimeSortedIssue{
  fn lt(&self, other:&TimeSortedIssue) -> bool{
    (*self).creationTime.to_timespec().lt(&(*other).creationTime.to_timespec())
  }
}
trait LinePushingString{
  fn push_strln(&mut self, rhs:&str);
}

impl LinePushingString for ~str{
  fn push_strln(&mut self, rhs:&str){
    self.push_str(rhs);
    self.push_str("\n");
  }
}

pub fn list_issues(args:~[~str], _:config::Config) -> int{
  let cBranch = vcs_status::current_branch();
  if(cBranch.is_none()){
    return 1;
  }
  
  let mut stateMachine = fsm::StateMachine::new(std_handler,
                                                Flags{short:false,
                                                      committed:false,
                                                      statuses:~[],
                                                      noComments:false,
                                                      id:None});

  for argVal in args.move_iter(){
    stateMachine.process(argVal);
  }
  let finalFlags = stateMachine.move_state();
  
  let mut issues = if (finalFlags.committed){
    file_manager::read_committed_issues()
  }else{
    file_manager::read_committable_issues(cBranch.unwrap())
  };

  for id in finalFlags.id.iter() {
    issues = selection::find_matching_issues(id.as_slice(), issues);
  }

  let resultStr = print_issue_vec(issues, &finalFlags);

  file_util::write_string_to_file(resultStr, TMP_OUTPUT_FILE, true);
  run::process_status("less", &[~"-RXF", TMP_OUTPUT_FILE.to_owned()]);
  file_util::delete_file(TMP_OUTPUT_FILE);
  0
}

struct Flags{
  short:bool,
  committed: bool,
  statuses: ~[~str],
  noComments: bool,
  id:Option<~str>
}

fn std_handler(flags:Flags, input:~str) -> fsm::NextState<Flags,~str> {
  match input {
    ~"--short" => fsm::Continue(Flags{short:true, .. flags}),
    ~"-s" => fsm::Continue(Flags{short:true, .. flags}),
    ~"--committed" => fsm::Continue(Flags{committed:true, .. flags}),
    ~"--status" => fsm::ChangeState(get_status, flags),
    ~"--nocomment" => fsm::Continue(Flags{noComments:true, .. flags}),
    ~"--id" => fsm::ChangeState(get_id, flags),
    _ => fsm::Continue(flags)
  }
}

fn get_status(mut flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  flags.statuses.push(input);
  fsm::ChangeState(std_handler, flags)
}

fn get_id(mut flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  flags.id = Some(input);
  fsm::ChangeState(std_handler, flags)
}

fn print_issue_vec(issues:~[~Issue], flags:&Flags) -> ~str{
  let dateSorted = date_sort::sort_by_time(issues);
  let mut resultStr = ~"";
  //reverse because they're sorted in ascending order
  //and we want descending
  for issue in dateSorted.rev_iter() {
    if (flags.statuses.len() == 0 ||
        flags.statuses.contains(&issue.status.name)){
      resultStr = print_issue(*issue, flags, resultStr);
    }
  }
  resultStr
}

fn print_issue(issue:&Issue, flags:&Flags, mut resultStr:~str) -> ~str {
  resultStr.push_strln("");
  resultStr.push_strln(fmt!("\x1b[33m%s (Issue ID: %s)\x1b[0m", issue.title, issue.id));
  if(!flags.short){
    resultStr.push_strln(fmt!("Current status: %s", issue.status.name));
    resultStr.push_strln(fmt!("\x1b[34mReported by %s on %s\x1b[0m",
                       issue.author, 
                       issue.creationTime.strftime(issue::TIME_FORMAT)));
    resultStr.push_strln(fmt!("Originated on branch %s\n", issue.branch)); 
    if(issue.bodyText.len() > 0){
      resultStr.push_strln(issue.bodyText);
    }
    if(!flags.noComments){
      if(issue.comments.len() == 0){
        resultStr.push_strln("    No comments on this issue.");
      }else{
        for comment in issue.comments.iter() {
          resultStr.push_strln(fmt!("  \x1b[32m%s on %s\x1b[0m",
                           comment.author, 
                           comment.creationTime.strftime(issue::TIME_FORMAT)));
	  resultStr.push_strln(fmt!("  For branch %s", comment.branch));
          for line in comment.bodyText.line_iter() {
            resultStr.push_strln(~"    " + line);
	  }
          resultStr.push_strln("");
        }
      }
    }
  }
  resultStr
}

