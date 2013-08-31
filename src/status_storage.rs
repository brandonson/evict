use issue::IssueStatus;
use file_util;
use file_manager;
use std::str::StrVector;

static STATUS_FILE:&'static str = "status_types";

fn fullStatusFilename() -> ~str {
  fmt!("%s%s", file_manager::EVICT_DIRECTORY, STATUS_FILE)
}

pub fn readIssueStatuses() -> ~[~IssueStatus] {
  let fullString = file_util::readStringFromFile(fullStatusFilename()).unwrap_or_default(~"");
  fullString.any_line_iter().map(|x| ~IssueStatus{name:x.to_owned()}).collect()
}

pub fn writeIssueStatuses(statuses:~[~IssueStatus]) -> bool {
  let stringVec:~[~str] = statuses.move_iter().map(|x| x.name).collect();
  let fullString = stringVec.connect("\n");
  file_util::writeStringToFile(fullString, fullStatusFilename(), true)
}

