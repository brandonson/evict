use file_util;
use file_manager;
use std::str::StrVector;
use issue::IssueStatus;

static STATUS_FILE:&'static str = "status_types";

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

pub fn readStatusOptions() -> ~[~StatusOption] {
  let fullString = file_util::readStringFromFile(fullStatusFilename())
                             .unwrap_or_default(~"");
  fullString.any_line_iter().map(|x| ~StatusOption{name:x.to_owned()}).collect()
}

pub fn writeStatusOptions(statuses:~[~StatusOption]) -> bool {
  let stringVec:~[~str] = statuses.move_iter().map(|x| x.name).collect();
  let fullString = stringVec.connect("\n");
  file_util::writeStringToFile(fullString, fullStatusFilename(), true)
}

