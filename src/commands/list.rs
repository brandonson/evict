use file_manager;
use vcs_status;
use issue;
use issue::Issue;
use std::io;
use extra::sort::Sort;
use config;

#[deriving(Clone, Eq)]
struct TimeSortedIssue(~Issue);

impl Ord for TimeSortedIssue{
  fn lt(&self, other:&TimeSortedIssue) -> bool{
    (*self).creationTime.to_timespec().lt(&(*other).creationTime.to_timespec())
  }
}

pub fn listIssues(args:~[~str], _:config::Config) -> int{
  let cBranch = vcs_status::currentBranch();
  if(cBranch.is_none()){
    return 1;
  }
  let short = args.contains(&~"--short") || args.contains(&~"-s");
  if (args.contains(&~"--committed")){
    printIssueVec(file_manager::readCommittedIssues(), short);
  }else{
    printIssueVec(file_manager::readCommittableIssues(cBranch.unwrap()), short);
  }
  0
}

fn printIssueVec(issues:~[~Issue], short:bool) {
  let mut wrapped:~[TimeSortedIssue] = issues.move_iter()
                                              .map(|x| TimeSortedIssue(x))
                                              .collect();
  wrapped.qsort();
  let unwrapped:~[~Issue] = wrapped.move_iter().map(|x| *x).collect();
  for issue in unwrapped.rev_iter() {
    printIssue(*issue, short);
  }
}

fn printIssue(issue:&Issue, short:bool) {
  io::println("");
  io::println(fmt!("\x1b[33m%s (Issue ID: %s)\x1b[0m", issue.title, issue.id));
  if(!short){
    io::println(fmt!("Current status: %s", issue.status.name));
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
        io::println(fmt!("  \x1b[32m%s on %s\x1b[0m",
                         comment.author, 
                         comment.creationTime.strftime(issue::TIME_FORMAT)));
	io::println(fmt!("  For branch %s", comment.branch));
        for line in comment.bodyText.line_iter() {
          io::println(~"    " + line);
	}
        io::println("");
      }
    }
  }
}

