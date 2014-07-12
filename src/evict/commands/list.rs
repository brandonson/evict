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
use issue;
use issue::{Issue,TimelineComment};

use std::io::process;

use file_util;
use libc;
use fsm;
use selection;
use date_sort;

static TMP_OUTPUT_FILE:&'static str = ".evict/LIST_TEMP_FILE";

trait LinePushingString{
  fn push_strln<S:Str>(&mut self, rhs:S);
}

impl LinePushingString for String{
  fn push_strln<S:Str>(&mut self, rhs:S){
    self.push_str(rhs.as_slice());
    self.push_str("\n");
  }
}


pub fn list_issues(args:Vec<String>) -> int{
  let mut stateMachine = fsm::StateMachine::new(std_handler,
                                                Flags{short:false,
                                                      statuses:vec!(),
                                                      noComments:false,
                                                      id:None,
                                                      tags:vec!()});

  for arg in args.move_iter(){
    stateMachine.process(arg);
  }
  let final_flags = stateMachine.move_state();
  
  let mut issues = file_manager::read_issues();

  for id in final_flags.id.iter() {
    issues = selection::find_matching_issues(id.as_slice(), issues.as_slice());
  }

  issues = issues.move_iter().filter(|check| {
    //if there are no tags, then we keep all issues
    let mut found = final_flags.tags.len() == 0;
    let all_tags = check.all_tags();

    for tag in final_flags.tags.iter() {
      found = found || all_tags.contains(tag)
    }
    found
  }).collect(); 

  let to_print = print_issue_vec(issues, &final_flags);

  let written = file_util::write_string_to_file(to_print.as_slice(), TMP_OUTPUT_FILE, true);
  if !written {
    println!("File write failure.");
  }
  let mut paginate_command = process::Command::new("less");
  paginate_command.arg("-RXF").arg(TMP_OUTPUT_FILE);
  paginate_command.stdout(process::InheritFd(libc::STDOUT_FILENO));
  paginate_command.stderr(process::InheritFd(libc::STDERR_FILENO));
  
  let paginate_proc = paginate_command.spawn();

  if paginate_proc.is_err() {
    println!("Couldn't paginate output.  Printing straight to terminal");
    println!("{}", to_print);
  }
  let exit_code = paginate_proc.ok().unwrap().wait();
  if !exit_code.is_ok() || !exit_code.ok().unwrap().success() {
    println!("Something went wrong in pagination.");
  }
  file_util::delete_file(TMP_OUTPUT_FILE);
  0
}

struct Flags{
  short:bool,
  statuses: Vec<String>,
  noComments: bool,
  id:Option<String>,
  tags:Vec<String>
}

fn std_handler(flags:Flags, input:String) -> fsm::NextState<Flags,String> {
  match input.as_slice() {
    "--short" => fsm::Continue(Flags{short:true, .. flags}),
    "-s" => fsm::Continue(Flags{short:true, .. flags}),
    "--status" => fsm::ChangeState(get_status, flags),
    "--nocomment" => fsm::Continue(Flags{noComments:true, .. flags}),
    "--id" => fsm::ChangeState(get_id, flags),
    "--tag" => fsm::ChangeState(get_tag, flags),
    _ => fsm::Continue(flags)
  }
}

fn get_status(mut flags:Flags, input:String) -> fsm::NextState<Flags, String> {
  flags.statuses.push(input);
  fsm::ChangeState(std_handler, flags)
}

fn get_id(mut flags:Flags, input:String) -> fsm::NextState<Flags, String> {
  flags.id = Some(input);
  fsm::ChangeState(std_handler, flags)
}

fn get_tag(mut flags:Flags, input:String) -> fsm::NextState<Flags, String> {
  flags.tags.push(input);
  fsm::ChangeState(std_handler, flags)
}

fn print_issue_vec(issues:Vec<Issue>, flags:&Flags) -> String{
  let date_sorted = date_sort::sort_by_time(issues);
  let mut to_print = String::new();
  //reverse because they're sorted in ascending order
  //and we want descending
  for issue in date_sorted.iter().rev() {
    if flags.statuses.len() == 0 ||
      flags.statuses.contains(&issue.status.name){ 
      to_print = print_issue(issue, flags, to_print);
    }
  }
  to_print
}

fn print_issue(issue:&Issue, flags:&Flags, mut to_print:String)
  -> String {
  to_print.push_strln("");
  to_print.push_strln(format!("\x1b[33m{} (Issue ID: {})\x1b[0m",
                              issue.title, issue.id));
  if !flags.short {
    to_print.push_strln(format!("Current status: {}", issue.status.name));
    to_print.push_strln(format!("\x1b[34mReported by {} on {}\x1b[0m",
                       issue.author, 
                       issue.creation_time.strftime(issue::TIME_FORMAT)));
    to_print.push_strln(format!("Originated on branch {}\n", issue.branch)); 
    if issue.body_text.len() > 0 {
      to_print.push_strln(issue.body_text.as_slice());
    }
    if !flags.noComments {
      if issue.events.len() == 0 {
        to_print.push_strln("    Nothing here for this issue.");
      }else{
        //the string for all comment info
        let mut comment_output = String::new();
        for evt in issue.events.iter() {
          match evt {
            &TimelineComment(ref comment) => {
              comment_output.push_strln(format!("  \x1b[32m{} on {}\x1b[0m",
                               comment.author, 
                               comment.creation_time.strftime(issue::TIME_FORMAT)));
              comment_output.push_strln(format!("  For branch {}", comment.branch));
              for line in comment.body_text.as_slice().lines() {
                comment_output.push_strln(format!("    {}", line));
              }
              comment_output.push_strln("");
            }
            _ => {}
          }
        }

        let tag_list = issue.all_tags();
        let mut tag_output = String::new();
        if tag_list.len() == 0 {
          tag_output.push_str("  No tags for this issue");
        }else {
          tag_output.push_str("  Tags: ");
          let mut isStart = true;
          for tagname in tag_list.iter() {
            if !isStart {
              tag_output.push_str(", "); 
            }
            isStart = false;
            tag_output.push_str(tagname.as_slice());
          }
        }

        to_print.push_strln(tag_output);
        to_print.push_strln(comment_output);
      }
    }
  }
  to_print
}

