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

#[deriving(Clone, Eq)]
pub struct StatusOption{
  name:~str
}

impl StatusOption{
  pub fn make_status(&self) -> IssueStatus {
    IssueStatus::new(self.name.to_owned())
  }
}

fn full_status_filename() -> ~str {
  format!("{}/{}", file_manager::EVICT_DIRECTORY, STATUS_FILE)
}

fn full_default_status_filename() -> ~str {
  format!("{}/{}", file_manager::EVICT_DIRECTORY, DEF_STATUS_FILE)
}

pub fn read_status_options() -> ~[StatusOption] {
  let fullString = file_util::read_string_from_file(full_status_filename())
                             .unwrap_or(~"");
  fullString.lines_any().map(|x| StatusOption{name:x.to_owned()}).collect()
}

pub fn write_status_options(statuses:~[StatusOption]) -> bool {
  let stringVec:~[~str] = statuses.move_iter().map(|x| x.name).collect();
  let fullString = stringVec.connect("\n");
  file_util::write_string_to_file(fullString, full_status_filename(), true)
}

pub fn read_default_status() -> StatusOption {
  let fullFile = file_util::read_string_from_file(full_default_status_filename())
                           .unwrap_or(DEFAULT_STATUS_NAME.to_owned());
  let lineVec:~[&str] = fullFile.lines_any().collect();
  let firstLine = lineVec.head().unwrap_or(&DEFAULT_STATUS_NAME);
  
  let statusOption = StatusOption{name:firstLine.to_owned()};
  if !read_status_options().contains(&statusOption) {
    StatusOption{name:DEFAULT_STATUS_NAME.to_owned()}
  }else{
    statusOption
  }
}

pub fn write_default_status(status:&StatusOption) -> Result<bool, ~str> {
  let isOption = read_status_options().contains(status);
  if !isOption {
    Err(format!("{} is not a status option", status.name))
  }else{
    Ok(file_util::write_string_to_file(status.name, full_default_status_filename(), true))
  }
}
