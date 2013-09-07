use selection;
use vcs_status;
use config;
use file_manager;
use status_storage;
use std::uint;
use issue::IssueStatus;

pub fn setStatus(args:~[~str], _:config::Config) -> int {
  if(args.len() != 2){
    println("set-status usage: evict set-status <issue-id> <status>");
    println("    Where <status> is either the full name of a status");
    println("    or the index of a status");
    1
  }else{
    match vcs_status::currentBranch() {
      Some(branch) => {
        match resolveNewStatus(args[1]) {
          Some(newStatus) => {
            let issues = file_manager::readCommittableIssues(branch);
            let edited = do selection::updateIssue(args[0], issues) |mut oldIssue| {
              oldIssue.status = newStatus.clone();
              oldIssue
            };
            file_manager::writeCommittableIssues(branch, edited);
            0
          }
          None => {println("Could not read current branch"); 2}
        }
      }
      None => 3
    }
  }
}

fn resolveNewStatus(statusIdent:&str) -> Option<~IssueStatus> {
  let search = status_storage::readStatusOptions();
  match uint::from_str(statusIdent) {
    Some(index) => if(search.len() > index) {Some(search[index])} else {None},
    None => search.move_iter().find(|x| x.name.as_slice() == statusIdent)
  }.map(|x| x.makeStatus())
}
