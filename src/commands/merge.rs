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
