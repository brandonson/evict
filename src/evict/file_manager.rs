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
use issue::{Issue, IssueTimelineEvent};
use file_util;
use std::io;
use serialize::json;
use serialize::json::ToJson;

#[cfg(not(test))]
pub static EVICT_DIRECTORY:&'static str = ".evict";
#[cfg(test)]
pub static EVICT_DIRECTORY:&'static str = ".evict-test";

static ISSUE_DIRECTORY:&'static str = "issue-dirs";

static BODY_FILENAME:&'static str = "body";

pub fn issue_directory() -> String {format!("{}/{}",
                                          EVICT_DIRECTORY,
                                          ISSUE_DIRECTORY)}

pub fn issue_directory_path() -> Path {Path::new(issue_directory())}

pub fn single_issue_filename(issue:&Issue) -> String {
  format!("{}/{}/{}", EVICT_DIRECTORY, ISSUE_DIRECTORY, issue.id)
}

pub fn write_issues(issues:&[Issue]) -> bool {
  write_issues_to_file(issues)
}

pub fn write_issues_to_file(issues:&[Issue]) -> bool {
  let mut allSuccess = true;
  for i in issues.iter() {
    allSuccess = allSuccess && write_single_issue(i);
  }
  allSuccess
}

fn write_single_issue(issue:&Issue) -> bool {
  file_util::create_directory(single_issue_filename(issue).as_slice());
  let mut allSuccess = write_issue_body(issue);
  for event in issue.events.iter() {
    allSuccess = allSuccess && write_issue_event(issue.id.as_slice(), event);
  }
  allSuccess
}

fn write_issue_body(issue:&Issue) -> bool {
  let filename = issue_body_filename(issue);
  let output = issue.no_comment_json().to_pretty_str();
  file_util::write_string_to_file(output.as_slice(), filename.as_slice(), true)
}

fn issue_body_filename(issue:&Issue) -> String {
  format!("{}/{}/{}/{}", EVICT_DIRECTORY, ISSUE_DIRECTORY, issue.id, BODY_FILENAME)
}

fn write_issue_event(issueId:&str, event:&IssueTimelineEvent) -> bool{
  let filename = issue_event_filename(issueId, event);
  let jsonStr = event.to_json().to_pretty_str();
  file_util::write_string_to_file(jsonStr.as_slice(), filename.as_slice(), true)
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
   *  into an issue, it is ignored.
   */
  let dirPath = issue_directory_path();
  let issueDirResult = io::fs::readdir(&dirPath);
  let issueDirs = issueDirResult.ok().unwrap_or(vec!());
  
  issueDirs.into_iter().filter_map (
    |path| read_issue_from_dir(path)
  ).collect()
}


fn read_issue_from_dir(basePath:Path) -> Option<Issue> {
  let files = io::fs::readdir(&basePath);
  let bodyPath = Path::new(BODY_FILENAME);
  let issueBodyPath = basePath.join(bodyPath);
  let noBodyFiles:Vec<Path> = files.ok().unwrap_or(vec!())
                                 .into_iter()
                                 .filter(|x| x != &issueBodyPath)
                                 .collect();
  let bodyIssue = read_issue_body(issueBodyPath);
  bodyIssue.map (|mut bIssue| {
    let events = read_issue_events(noBodyFiles.as_slice());
    bIssue.events = events;
    bIssue
  })
}

fn read_issue_body(bodyPath:Path) -> Option<Issue> {
  /*! Reads an issue from a file, except for the comments, which are stored
   *  separately from other data.
   */
  let dataStrOpt = file_util::read_string_from_path(&bodyPath);
  dataStrOpt.and_then(|dataStr| {
     json::from_str(dataStr.as_slice()).ok()
  }).and_then(|jsonVal| {
    Issue::from_json(&jsonVal)
  })
}

fn read_issue_events(bodyFiles:&[Path]) -> Vec<IssueTimelineEvent> {
  bodyFiles.iter().filter_map(read_comment).collect()
}

fn read_comment(commentFile:&Path) -> Option<IssueTimelineEvent> {
  let dataStrOpt = file_util::read_string_from_path(commentFile);
  dataStrOpt.and_then(|dataStr| {
    json::from_str(dataStr.as_slice()).ok()
  }).and_then(|jsonVal| {
    IssueTimelineEvent::from_json(&jsonVal)
  })
}

#[test]
pub fn write_read_issue_file(){
  file_util::create_directory_path(&Path::new(EVICT_DIRECTORY));
  file_util::create_directory_path(&issue_directory_path());
  let issues = vec!(Issue::new("A".into_string(), "B".into_string(), "C".into_string()));
  write_issues(issues.as_slice());
  let read = read_issues();
  assert!(issues == read);
  let _ = io::fs::rmdir_recursive(&Path::new(EVICT_DIRECTORY));
}
