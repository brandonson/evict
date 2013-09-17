use file_manager;
use config;
use status_storage::{StatusOption, 
                     write_status_options, 
                     write_default_status};
use std;

pub fn initialize(_:~[~str], _:config::Config) -> int {
  let res = std::os::make_dir(&Path(file_manager::EVICT_DIRECTORY), 
                                    0400 | 0200 | 0040 | 0020 | 0004);
  if(res){
    let defaultStatus = StatusOption{name:~"open"};
    let statusOpts = ~[defaultStatus.clone(), StatusOption{name:~"closed"}];
    let optionSuccess = write_status_options(statusOpts);
    if(optionSuccess){
      let defaultResult = write_default_status(&defaultStatus);
      if(defaultResult.is_ok()){
        0
      }else{
        1
      }
    }else{
      2
    }
  }else{3}
}
