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
use std::io::{println};
use file_manager::*;
use file_util;
use merge;
use vcs_status;
use config;

pub fn merge_branches(args:~[~str], _:config::Config) -> int {
  if(args.len() == 0 || args.len() > 2){
    println("Usage: evict merge <from-branch> [<to-branch>]");
    1
  }else{
    let fromFile = committable_issue_filename(args[0]);
    let toFile = if(args.len() == 2) { 
                   committable_issue_filename(args[1])
                 } else {
                   let branchName = vcs_status::current_branch();
                   if(branchName.is_none()){
                     println("Could not determine current branch");
                     return 2;
                   }
                   committable_issue_filename(branchName.unwrap())
                 };

    if(!file_util::file_exists(fromFile)){
      println(fmt!("There are no issues for %s", args[0]));
      3
    }else{
      let fromIssues = read_issues_from_file(fromFile);
      let toIssues = read_issues_from_file(toFile);
      let merged = merge::merge_issues(fromIssues,toIssues);
      let success = write_issues_to_file(merged, toFile, true);
      if(success) {0} else {println("Could not write issues to file"); 4}
    }
  }
}
