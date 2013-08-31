use std;
use commands;
use file_manager;
use config;

pub fn clearData(_:~[~str],_:config::Config) -> int {
  let evict_path = &std::path::Path(file_manager::EVICT_DIRECTORY);
  let res = commands::prompt(
             fmt!("Really clear everything from %s? [y/n]", 
                  std::os::make_absolute(evict_path).to_str()));
  if(res == ~"y"){
    let delResult = std::os::remove_dir_recursive(evict_path);
    if(delResult){0}else{1}
  }else{
    0
  }
}
