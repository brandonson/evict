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
use issue::{Issue,TimelineComment, TimelineTag};

use file_util;
use std::io::process;
use libc;
use time;
use collections::treemap::TreeMap;
use fsm;
use selection;
use date_sort;

static TMP_OUTPUT_FILE:&'static str = ".evict/LIST_TEMP_FILE";

trait LinePushingString{
  fn push_strln(&mut self, rhs:&str);
}

impl LinePushingString for ~str{
  fn push_strln(&mut self, rhs:&str){
    self.push_str(rhs);
    self.push_str("\n");
  }
}

pub fn list_issues(args:~[~str]) -> int{
  let mut stateMachine = fsm::StateMachine::new(std_handler,
                                                Flags{short:false,
                                                      committed:false,
                                                      statuses:~[],
                                                      noComments:false,
                                                      id:None});

  for arg in args.move_iter(){
    stateMachine.process(arg);
  }
  let final_flags = stateMachine.move_state();
  
  let mut issues = file_manager::read_issues();

  for id in final_flags.id.iter() {
    issues = selection::find_matching_issues(id.as_slice(), issues.as_slice());
  }

  let to_print = print_issue_vec(issues, &final_flags);

  let written = file_util::write_string_to_file(to_print, TMP_OUTPUT_FILE, true);
  if !written {
    println!("File write failure.");
  }
  let paginate_proc = process::Process::configure(
                          process::ProcessConfig{
                            program:"less",
                            args:&[~"-RXF", TMP_OUTPUT_FILE.to_owned()],
                            stdout:process::InheritFd(libc::STDOUT_FILENO),
                            stderr:process::InheritFd(libc::STDERR_FILENO),
                            .. process::ProcessConfig::new()});
  if paginate_proc.is_err() {
    println!("Couldn't paginate output.  Printing straight to terminal");
    println!("{}", to_print);
  }
  let exit_code = paginate_proc.ok().unwrap().wait();
  if !exit_code.success() {
    println!("Something went wrong in pagination.");
  }
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
  match input.as_slice() {
    "--short" => fsm::Continue(Flags{short:true, .. flags}),
    "-s" => fsm::Continue(Flags{short:true, .. flags}),
    "--committed" => fsm::Continue(Flags{committed:true, .. flags}),
    "--status" => fsm::ChangeState(get_status, flags),
    "--nocomment" => fsm::Continue(Flags{noComments:true, .. flags}),
    "--id" => fsm::ChangeState(get_id, flags),
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

fn print_issue_vec(issues:Vec<Issue>, flags:&Flags) -> ~str{
  let date_sorted = date_sort::sort_by_time(issues);
  let mut to_print = ~"";
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

fn print_issue(issue:&Issue, flags:&Flags, mut to_print:~str) -> ~str {
  to_print.push_strln("");
  to_print.push_strln(format!("\x1b[33m{} (Issue ID: {})\x1b[0m", issue.title, issue.id));
  if !flags.short {
    to_print.push_strln(format!("Current status: {}", issue.status.name));
    to_print.push_strln(format!("\x1b[34mReported by {} on {}\x1b[0m",
                       issue.author, 
                       issue.creation_time.strftime(issue::TIME_FORMAT)));
    to_print.push_strln(format!("Originated on branch {}\n", issue.branch)); 
    if issue.body_text.len() > 0 {
      to_print.push_strln(issue.body_text);
    }
    if !flags.noComments {
      if issue.events.len() == 0 {
        to_print.push_strln("    Nothing here for this issue.");
      }else{
        //the string for all comment info
        let mut comment_output = ~"";
        //the tags for this comment
        let mut tag_map:TreeMap<~str, time::Tm> = TreeMap::new();
        for evt in issue.events.iter() {
          match evt {
            &TimelineComment(ref comment) => {
              comment_output.push_strln(format!("  \x1b[32m{} on {}\x1b[0m",
                               comment.author, 
                               comment.creation_time.strftime(issue::TIME_FORMAT)));
              comment_output.push_strln(format!("  For branch {}", comment.branch));
              for line in comment.body_text.lines() {
                comment_output.push_strln(~"    " + line);
              }
              comment_output.push_strln("");
            }
            &TimelineTag(ref tag) => {
              if tag.enabled {
                tag_map.insert(tag.tag_name.clone(), tag.time.clone());
              }else{
                tag_map.remove(&tag.tag_name);
              }
            }
          }
        }
        let mut tag_output = ~"";
        if tag_map.len() == 0 {
          tag_output.push_str("  No tags for this issue");
        }else {
          tag_output.push_str("  Tags: ");
          let mut isStart = true;
          let tag_map = tag_map; //freeze to allow iteration
          for (tagname,_) in tag_map.iter() {
            if !isStart {
              tag_output.push_str(", "); 
              isStart = true;
            }
            tag_output.push_str(*tagname);
          }
        }

        to_print.push_strln(tag_output);
        to_print.push_strln(comment_output);
      }
    }
  }
  to_print
}

