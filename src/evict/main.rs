extern mod extra;
extern mod fsm;

pub mod issue;
pub mod file_manager;
#[path="commands/mod.rs"]
pub mod commands;
pub mod file_util;
pub mod vcs_status;
pub mod merge;
pub mod selection;
pub mod config;
pub mod status_storage;
pub mod date_sort;

pub mod evict{
  pub static CURRENT_VERSION:uint = 1;
}

fn main(){
  let args = std::os::args();
  if(args.len() < 2){
    // < 2 because the first arg is the name of the binary
    std::io::println("No command given");
  }else{
    let cmdArgs = args.tailn(2).to_owned();
    
     
    let cmd = args[1];
    commands::execute_command(&cmd, &commands::standard_commands(), cmdArgs);
  }
}
