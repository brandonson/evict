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

#[deriving(Clone, PartialEq)]
pub struct StatusOption{
  pub name:String
}

impl StatusOption{
  pub fn make_status(&self) -> IssueStatus {
    IssueStatus::new(self.name.to_string())
  }
}

fn full_status_filename() -> String {
  format!("{}/{}", file_manager::EVICT_DIRECTORY, STATUS_FILE)
}

fn full_default_status_filename() -> String {
  format!("{}/{}", file_manager::EVICT_DIRECTORY, DEF_STATUS_FILE)
}

pub fn read_status_options() -> Vec<StatusOption> {
  let fullString = file_util::read_string_from_file(full_status_filename().as_slice())
                             .unwrap_or("".into_string());
  fullString.as_slice().lines_any().map(
    |x| StatusOption{name:x.into_string()}
  ).collect()
}

pub fn write_status_options(statuses:Vec<StatusOption>) -> bool {
  let stringVec:Vec<String> = statuses.into_iter().map(|x| x.name).collect();
  let fullString = stringVec.connect("\n");
  file_util::write_string_to_file(fullString.as_slice(),
                                  full_status_filename().as_slice(),
                                  true)
}

pub fn read_default_status() -> StatusOption {
  let fullFile = file_util::read_string_from_file(full_default_status_filename().as_slice())
                           .unwrap_or(DEFAULT_STATUS_NAME.into_string());
  let lineVec:Vec<&str> = fullFile.as_slice().lines_any().collect();
  let firstLine = lineVec.as_slice().head().unwrap_or(&DEFAULT_STATUS_NAME);
  
  let statusOption = StatusOption{name:firstLine.into_string()};
  if !read_status_options().contains(&statusOption) {
    StatusOption{name:DEFAULT_STATUS_NAME.into_string()}
  }else{
    statusOption
  }
}

pub fn write_default_status(status:&StatusOption) -> Result<bool, String> {
  let isOption = read_status_options().contains(status);
  if !isOption {
    Err(format!("{} is not a status option", status.name))
  }else{
    Ok(file_util::write_string_to_file(status.name.as_slice(),
                                       full_default_status_filename().as_slice(),
                                       true))
  }
}
