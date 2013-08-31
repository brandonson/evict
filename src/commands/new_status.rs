use config;
use issue;
use status_storage;

pub fn newStatus(args:~[~str], _:config::Config) -> int {
  if(args.len() != 1){
    println("new-status usage: evict new-status <status-name>");
    1
  }else{
    let mut newStatuses = status_storage::readIssueStatuses();
    newStatuses.push(~issue::IssueStatus{name:args[0]});
    status_storage::writeIssueStatuses(newStatuses);
    0
  }
}
