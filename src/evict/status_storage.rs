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
use issue::IssueStatus;
use std::io::Error as IoError;
use std::io::Result as IoResult;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

static STATUS_FILE:&'static str = "status_types";
static DEF_STATUS_FILE:&'static str = "default_status";
pub static DEFAULT_STATUS_NAME:&'static str = "<unknown>";

#[derive(Debug)]
pub enum StatusWriteError{
  IoWriteFailure(IoError),
  InvalidStatus(String)
}

impl Display for StatusWriteError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.description())
  }
}

impl ::std::error::Error for StatusWriteError {
  fn description(&self) -> &str {
    use self::StatusWriteError::*;
    match *self {
      IoWriteFailure(_) => "I/O to the file failed",
      InvalidStatus(ref strval) => strval.as_str()
    }
  }

  fn cause(&self) -> Option<&Error> {
    use self::StatusWriteError::*;
    match *self {
      IoWriteFailure(ref err) => Some(err),
      _ => None
    }
  }
}

impl From<IoError> for StatusWriteError {
  fn from(err:IoError) -> StatusWriteError {
    StatusWriteError::IoWriteFailure(err)
  }
}

#[derive(Clone, PartialEq)]
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
  let fullString = file_util::read_string_from_file(full_status_filename().as_str())
                             .unwrap_or("".to_string());
  fullString.as_str().lines().map(
    |x| StatusOption{name:x.to_string()}
  ).collect()
}

pub fn write_status_options(statuses:Vec<StatusOption>) -> IoResult<()> {
  let stringVec:Vec<String> = statuses.into_iter().map(|x| x.name).collect();
  let fullString = stringVec.join("\n");
  file_util::write_string_to_file(fullString.as_str(),
                                  full_status_filename().as_str(),
                                  true)
}

pub fn read_default_status() -> StatusOption {
  let fullFile = file_util::read_string_from_file(full_default_status_filename().as_str())
                           .unwrap_or(DEFAULT_STATUS_NAME.to_string());
  let lineVec:Vec<&str> = fullFile.as_str().lines().collect();
  let firstLine = lineVec.as_slice().get(0).unwrap_or(&DEFAULT_STATUS_NAME);
  
  let statusOption = StatusOption{name:firstLine.to_string()};
  if !read_status_options().contains(&statusOption) {
    StatusOption{name:DEFAULT_STATUS_NAME.to_string()}
  }else{
    statusOption
  }
}

pub fn write_default_status(status:&StatusOption) -> Result<(), StatusWriteError> {
  use self::StatusWriteError::*;
  let isOption = read_status_options().contains(status);
  if !isOption {
    Err(InvalidStatus(format!("{} is not a valid status option", status.name.to_string())))
  }else{
    file_util::write_string_to_file(
      status.name.as_str(),
      full_default_status_filename().as_str(),
      true).map_err(Into::into)
  }
}
