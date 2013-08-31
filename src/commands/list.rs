use file_manager;
use vcs_status;
use issue;
use issue::Issue;
use std::io;
use config;

pub fn listIssues(args:~[~str], _:config::Config) -> int{
  let cBranch = vcs_status::currentBranch();
  if(cBranch.is_none()){
    return 1;
  }
  let short = args.contains(&~"--short") || args.contains(&~"-s");
  if(args.contains(&~"--local")){
    printIssueVec(file_manager::readLocalIssues(cBranch.unwrap()), short);
  }else if (args.contains(&~"--committed")){
    printIssueVec(file_manager::readCommittedIssues(), short);
  }else{
    printIssueVec(file_manager::readCommittableIssues(cBranch.unwrap()), short);
  }
  0
}

fn printIssueVec(issues:~[~Issue], short:bool) {
  for issue in issues.iter() {
    printIssue(*issue, short);
  }
}

fn printIssue(issue:&Issue, short:bool) {
  io::println("");
  io::println(fmt!("\x1b[33m%s (Issue ID: %s)\x1b[0m", issue.title, issue.id));
  if(!short){
    io::println(fmt!("\x1b[34mReported by %s on %s\x1b[0m",
                       issue.author, 
                       issue.creationTime.strftime(issue::TIME_FORMAT)));
    io::println(fmt!("Originated on branch %s\n", issue.branch)); 
    if(issue.bodyText.len() > 0){
      io::println(issue.bodyText);
    }
    if(issue.comments.len() == 0){
      io::println("    No comments on this issue.");
    }else{
      for comment in issue.comments.iter() {
        io::println(fmt!("  \x1b[32m%s on %s\x1b[0m\n",
                         comment.author, 
                         comment.creationTime.strftime(issue::TIME_FORMAT)));
	io::println(fmt!("  For branch %s", comment.branch));
        for line in comment.bodyText.line_iter() {
          io::println(~"    " + line);
	}
      }
    }
  }
}

