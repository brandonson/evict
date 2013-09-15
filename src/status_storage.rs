use file_util;
use file_manager;
use std::str::StrVector;
use issue::IssueStatus;

static STATUS_FILE:&'static str = "status_types";
static DEF_STATUS_FILE:&'static str = "default_status";
pub static DEFAULT_STATUS_NAME:&'static str = "<unknown>";

#[deriving(Eq)]
pub struct StatusOption{
  name:~str
}

impl StatusOption{
  pub fn makeStatus(&self) -> ~IssueStatus {
    ~IssueStatus::new(self.name.to_owned())
  }
}

fn fullStatusFilename() -> ~str {
  fmt!("%s%s", file_manager::EVICT_DIRECTORY, STATUS_FILE)
}

fn fullDefaultStatusFilename() -> ~str {
  fmt!("%s%s", file_manager::EVICT_DIRECTORY, DEF_STATUS_FILE)
}

pub fn readStatusOptions() -> ~[StatusOption] {
  let fullString = file_util::readStringFromFile(fullStatusFilename())
                             .unwrap_or_default(~"");
  fullString.any_line_iter().map(|x| StatusOption{name:x.to_owned()}).collect()
}

pub fn writeStatusOptions(statuses:~[StatusOption]) -> bool {
  let stringVec:~[~str] = statuses.move_iter().map(|x| x.name).collect();
  let fullString = stringVec.connect("\n");
  file_util::writeStringToFile(fullString, fullStatusFilename(), true)
}

pub fn readDefaultStatus() -> StatusOption {
  let fullFile = file_util::readStringFromFile(fullDefaultStatusFilename())
                           .unwrap_or_default(DEFAULT_STATUS_NAME.to_owned());
  let lineVec:~[&str] = fullFile.any_line_iter().collect();
  let firstLine = lineVec.head_opt().unwrap_or_default(&DEFAULT_STATUS_NAME);
  
  let statusOption = StatusOption{name:firstLine.to_owned()};
  if(!readStatusOptions().contains(&statusOption)){
    StatusOption{name:DEFAULT_STATUS_NAME.to_owned()}
  }else{
    statusOption
  }
}

pub fn writeDefaultStatus(status:&StatusOption) -> Result<bool, ~str> {
  let isOption = readStatusOptions().contains(status);
  if(!isOption){
    Err(fmt!("%s is not a status option", status.name))
  }else{
    Ok(file_util::writeStringToFile(status.name, fullDefaultStatusFilename(), true))
  }
}
