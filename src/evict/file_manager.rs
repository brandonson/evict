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
use issue::{Issue, IssueTimelineEvent, IssueJsonParseError};
use file_util;
use std::io::Error as IoError;
use serde_json::Result as SerdeResult;
use serde_json;
use std::fs;
use std::fs::File;

use std::path::{Path, PathBuf};


#[cfg(not(test))]
pub static EVICT_DIRECTORY:&'static str = ".evict";
#[cfg(test)]
pub static EVICT_DIRECTORY:&'static str = ".evict-test";

error_type! {
  #[derive(Debug)]
  pub enum DataReadError {
    ParseError(IssueJsonParseError) {
      cause;
    },
    IoReadError(IoError) {
      cause;
    }
  }
}

static ISSUE_DIRECTORY:&'static str = "issue-dirs";

static BODY_FILENAME:&'static str = "body";

pub fn issue_directory() -> String {format!("{}/{}",
                                          EVICT_DIRECTORY,
                                          ISSUE_DIRECTORY)}

pub fn issue_directory_path() -> PathBuf {PathBuf::from(issue_directory())}

pub fn single_issue_filename(issue:&Issue) -> String {
  format!("{}/{}/{}", EVICT_DIRECTORY, ISSUE_DIRECTORY, issue.id())
}

pub fn write_issues(issues:&[Issue]) -> SerdeResult<()> {
  if !issue_directory_path().is_dir() {
    fs::create_dir(issue_directory_path());
  }
  write_issues_to_file(issues)
}

pub fn write_issues_to_file(issues:&[Issue]) -> SerdeResult<()> {
  let mut result = Ok(());
  for i in issues.iter() {
    if let e@Err(_) = write_single_issue(i) {
      result = e;
    }
  }
  result
}

fn write_single_issue(issue:&Issue) -> SerdeResult<()> {
  file_util::create_directory(single_issue_filename(issue).as_str());
  let mut result = write_issue_body(issue);
  for event in issue.events.iter() {
    if let e@Err(_) = write_issue_event(issue.id(), event) {
      result = e;
    }
  }
  result
}

fn write_issue_body(issue:&Issue) -> SerdeResult<()> {
  let filename = issue_body_filename(issue);
  let mut file_out = try!(File::create(filename));
  serde_json::to_writer_pretty(&mut file_out, &issue.no_comment_json())
}

fn issue_body_filename(issue:&Issue) -> String {
  format!("{}/{}/{}/{}", EVICT_DIRECTORY, ISSUE_DIRECTORY, issue.id(), BODY_FILENAME)
}

fn write_issue_event(issueId:&str, event:&IssueTimelineEvent) -> SerdeResult<()>{
  let filename = issue_event_filename(issueId, event);
  let mut output_file = try!(File::create(filename.as_str()));
  serde_json::to_writer_pretty(&mut output_file, event)
}

fn issue_event_filename(issueId:&str, event:&IssueTimelineEvent) -> String {
  format!("{}/{}/{}/{}", EVICT_DIRECTORY, ISSUE_DIRECTORY, issueId, event.id())
}

pub fn read_issues() -> Vec<Issue> {
  read_issues_from_folders()
}

fn read_issues_from_folders() -> Vec<Issue> {
  /*! Reads all issues from the folders located in the
   *  folder returned by full_issue_directory.
   *  If a folder/file in the issue directory does not parse
   *  as an issue, it is ignored.
   */
  let dirPath = issue_directory_path();
  let issueDirResult = fs::read_dir(&dirPath);

  //There aren't any issue directories to deal with
  //so just return an empty list
  if issueDirResult.is_err() {
    return vec![];
  }

  let issueDirs = issueDirResult.ok().unwrap();
  
  issueDirs.into_iter().filter_map (
    |path| read_issue_from_dir(path.unwrap().path()).ok()
  ).collect()
}


fn read_issue_from_dir(basePath:PathBuf) -> Result<Issue, DataReadError> {
  let files = fs::read_dir(&basePath);
  let bodyPath = Path::new(BODY_FILENAME);
  let issueBodyPath = basePath.join(bodyPath);
  let noBodyFiles:Vec<PathBuf> = files.ok().unwrap()
                                 .into_iter()
                                 .map(|dir_entry| dir_entry.unwrap().path())
                                 .filter(|path| *path != issueBodyPath)
                                 .collect();
  let bodyIssue = read_issue_body(issueBodyPath);
  bodyIssue.map (|mut bIssue| {
    let events = read_issue_events(noBodyFiles.as_slice());
    bIssue.events = events;
    bIssue
  })
}

fn read_issue_body(bodyPath:PathBuf) -> Result<Issue, DataReadError> {
  /*! Reads an issue from a file, except for the comments, which are stored
   *  separately from other data.
   */
  let data = try!(file_util::read_string_from_path(&bodyPath));
  Issue::from_str(&data).map_err(Into::into)
}

fn read_issue_events(bodyFiles:&[PathBuf]) -> Vec<IssueTimelineEvent> {
  bodyFiles.iter().filter_map(|pbuf| read_comment(pbuf).ok()).collect()
}

fn read_comment(commentFile:&PathBuf) -> Result<IssueTimelineEvent, serde_json::error::Error> {
  let data = try!(file_util::read_string_from_path(commentFile));
  serde_json::from_str(&data)
}

#[test]
pub fn write_read_issue_file(){
  use std::error::Error;

  file_util::create_directory_path(&Path::new(EVICT_DIRECTORY));
  file_util::create_directory_path(&issue_directory_path());
  let issues = vec!(Issue::new("A".to_string(), "B".to_string(), "C".to_string()));
  let write_res = write_issues(issues.as_slice());
  assert!(
    write_res.is_ok(),
    "Assert failed - result not ok, {}: {:?}",
    write_res.as_ref().unwrap_err().description(),
    write_res);
  let read = read_issues();
  println!("{:?}", read);
  assert!(issues == read);
  let _ = fs::remove_dir_all(&Path::new(EVICT_DIRECTORY));
}
