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
use file_manager;
use file_manager::*;
use file_util;
use merge;
use vcs_status;
use config;

pub fn mergeBranches(args:~[~str], _:config::Config) -> int {
  if(args.len() == 0 || args.len() > 2){
    println("Usage: evict merge <from-branch> [<to-branch>]");
    1
  }else{
    let fromFile = committableIssueFilename(args[0]);
    let toFile = if(args.len() == 2) { 
                   committableIssueFilename(args[1])
                 } else {
                   let branchName = vcs_status::currentBranch();
                   if(branchName.is_none()){
                     println("Could not determine current branch");
                     return 2;
                   }
                   committableIssueFilename(branchName.unwrap())
                 };

    if(!file_util::fileExists(fromFile)){
      println(fmt!("There are no issues for %s", args[0]));
      3
    }else{
      let fromIssues = readIssuesFromFile(fromFile);
      let toIssues = readIssuesFromFile(toFile);
      let merged = merge::mergeIssues(fromIssues,toIssues);
      let success = file_manager::writeIssuesToFile(merged, toFile, true);
      if(success) {0} else {println("Could not write issues to file"); 4}
    }
  }
}
