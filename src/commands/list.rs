use file_manager;
use vcs_status;
use issue;
use issue::Issue;
use extra::sort::Sort;
use config;
use file_util;
use std::run;
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
pub fn listIssues(args:~[~str], _:config::Config) -> int{
  let cBranch = vcs_status::currentBranch();
  if(cBranch.is_none()){
    return 1;
  }
  let short = args.contains(&~"--short") || args.contains(&~"-s");
  let resultStr = if (args.contains(&~"--committed")){
    printIssueVec(file_manager::readCommittedIssues(), short)
  }else{
    printIssueVec(file_manager::readCommittableIssues(cBranch.unwrap()), short)
  };
  println(resultStr);
  file_util::writeStringToFile(resultStr, TMP_OUTPUT_FILE, true);
  run::process_status("less", &[~"-rXF", TMP_OUTPUT_FILE.to_owned()]);
  file_util::deleteFile(TMP_OUTPUT_FILE);
  0
}

fn printIssueVec(issues:~[~Issue], short:bool) -> ~str{
  let mut wrapped:~[TimeSortedIssue] = issues.move_iter()
                                              .map(|x| TimeSortedIssue(x))
                                              .collect();
  wrapped.qsort();
  let unwrapped:~[~Issue] = wrapped.move_iter().map(|x| *x).collect();
  let mut resultStr = ~"";
  for issue in unwrapped.rev_iter() {
    resultStr = printIssue(*issue, short, resultStr);
  }
  resultStr
}

fn printIssue(issue:&Issue, short:bool, mut resultStr:~str) -> ~str {
  resultStr.push_strln("");
  resultStr.push_strln(fmt!("\x1b[33m%s (Issue ID: %s)\x1b[0m", issue.title, issue.id));
  if(!short){
    resultStr.push_strln(fmt!("Current status: %s", issue.status.name));
    resultStr.push_strln(fmt!("\x1b[34mReported by %s on %s\x1b[0m",
                       issue.author, 
                       issue.creationTime.strftime(issue::TIME_FORMAT)));
    resultStr.push_strln(fmt!("Originated on branch %s\n", issue.branch)); 
    if(issue.bodyText.len() > 0){
      resultStr.push_strln(issue.bodyText);
    }
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
  resultStr
}

