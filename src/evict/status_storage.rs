/*
 *   Copyright 2013 Brandon Sanderson
 *
 *   This file is part of Evict-BT.
 *
 *   Evict-BT is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   Evict-BT is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *   along with Evict-BT.  If not, see <http://www.gnu.org/licenses/>.
 */
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
                             .unwrap_or(~"");
  fullString.any_line_iter().map(|x| StatusOption{name:x.to_owned()}).collect()
}

pub fn writeStatusOptions(statuses:~[StatusOption]) -> bool {
  let stringVec:~[~str] = statuses.move_iter().map(|x| x.name).collect();
  let fullString = stringVec.connect("\n");
  file_util::writeStringToFile(fullString, fullStatusFilename(), true)
}

pub fn readDefaultStatus() -> StatusOption {
  let fullFile = file_util::readStringFromFile(fullDefaultStatusFilename())
                           .unwrap_or(DEFAULT_STATUS_NAME.to_owned());
  let lineVec:~[&str] = fullFile.any_line_iter().collect();
  let firstLine = lineVec.head_opt().unwrap_or(&DEFAULT_STATUS_NAME);
  
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
