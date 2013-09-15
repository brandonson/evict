use config;
use status_storage;

pub fn newStatus(args:~[~str], _:config::Config) -> int {
  if(args.len() != 1){
    println("new-status usage: evict new-status <status-name>");
    1
  }else{
    let mut newStatuses = status_storage::readStatusOptions();
    newStatuses.push(status_storage::StatusOption{name:args[0]});
    status_storage::writeStatusOptions(newStatuses);
    0
  }
}
