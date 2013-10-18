use file_manager;
use status_storage::{StatusOption, 
                     write_status_options, 
                     write_default_status};
use file_util;

pub fn initialize(_:~[~str]) -> int {
  let res = file_util::create_directory(file_manager::EVICT_DIRECTORY);
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
