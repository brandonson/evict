use config;
use status_storage;

pub fn defaultStatus(args:~[~str], _:config::Config) -> int {
  if(args.len() > 1){
    println ("default-status usage: evict default-status [new-status]");
    1
  }else{
    if(args.len() == 0){
      let default = status_storage::readDefaultStatus();
      println(fmt!("Current default status is: %s", default.name));
      2
    }else{
      let status = status_storage::StatusOption{name:args[0]};
      match status_storage::writeDefaultStatus(&status) {
        Ok(true) => {0}
        Ok(false) => {println("Could not write to file"); 3}
        Err(s) => {println(s); 4}
      }
    }
  }
}
