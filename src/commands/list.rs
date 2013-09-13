use file_manager;
use vcs_status;
use issue;
use issue::Issue;
use extra::sort::Sort;
use config;
use file_util;
use std::run;
use fsm;
use selection;

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
  
  let mut stateMachine = fsm::StateMachine::new(stdHandler,
                                                Flags{short:false,
                                                      committed:false,
                                                      statuses:~[],
                                                      noComments:false,
                                                      id:None});

  for argVal in args.move_iter(){
    stateMachine.process(argVal);
  }
  let finalFlags = stateMachine.consumeToState();
  
  let mut issues = if (finalFlags.committed){
    file_manager::readCommittedIssues()
  }else{
    file_manager::readCommittableIssues(cBranch.unwrap())
  };

  for id in finalFlags.id.iter() {
    issues = selection::findMatchingIssues(id.as_slice(), issues);
  }

  let resultStr = printIssueVec(issues, &finalFlags);

  file_util::writeStringToFile(resultStr, TMP_OUTPUT_FILE, true);
  run::process_status("less", &[~"-RXF", TMP_OUTPUT_FILE.to_owned()]);
  file_util::deleteFile(TMP_OUTPUT_FILE);
  0
}

struct Flags{
  short:bool,
  committed: bool,
  statuses: ~[~str],
  noComments: bool,
  id:Option<~str>
}

fn stdHandler(flags:Flags, input:~str) -> fsm::NextState<Flags,~str> {
  match input {
    ~"--short" => fsm::Continue(Flags{short:true, .. flags}),
    ~"-s" => fsm::Continue(Flags{short:true, .. flags}),
    ~"--committed" => fsm::Continue(Flags{committed:true, .. flags}),
    ~"--status" => fsm::ChangeState(getStatus, flags),
    ~"--nocomment" => fsm::Continue(Flags{noComments:true, .. flags}),
    ~"--id" => fsm::ChangeState(getId, flags),
    _ => fsm::Continue(flags)
  }
}

fn getStatus(mut flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  flags.statuses.push(input);
  fsm::ChangeState(stdHandler, flags)
}

fn getId(mut flags:Flags, input:~str) -> fsm::NextState<Flags, ~str> {
  flags.id = Some(input);
  fsm::ChangeState(stdHandler, flags)
}

fn printIssueVec(issues:~[~Issue], flags:&Flags) -> ~str{
  let mut wrapped:~[TimeSortedIssue] = issues.move_iter()
                                              .map(|x| TimeSortedIssue(x))
                                              .collect();
  wrapped.qsort();
  let unwrapped:~[~Issue] = wrapped.move_iter().map(|x| *x).collect();
  let mut resultStr = ~"";
  for issue in unwrapped.rev_iter() {
    if (flags.statuses.len() == 0 ||
        flags.statuses.contains(&issue.status.name)){
      resultStr = printIssue(*issue, flags, resultStr);
    }
  }
  resultStr
}

fn printIssue(issue:&Issue, flags:&Flags, mut resultStr:~str) -> ~str {
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

