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
use extra;
use issue::Issue;
use file_util;
use extra::json;
use date_sort;

#[cfg(not(test))]
pub static EVICT_DIRECTORY:&'static str = ".evict/";
#[cfg(test)]
pub static EVICT_DIRECTORY:&'static str = ".evict-test/";

static EXTENSION:&'static str = ".ebtd";

static LOCAL_EXT:&'static str = ".ebtdlocal";

static ACTIVE_ISSUE_FILENAME_PART:&'static str = "issues";

pub fn activeIssueFilename() -> ~str {fmt!("%s%s%s",EVICT_DIRECTORY, 
                                                ACTIVE_ISSUE_FILENAME_PART, 
                                                EXTENSION)}

pub fn committableIssueFilename(branchname:&str) -> ~str {
  fmt!("%s%s%s", EVICT_DIRECTORY, branchname, LOCAL_EXT)
}

pub fn writeCommittableIssues(branchname:&str, issues:&[~Issue]) -> bool {
  writeIssuesToFile(issues, committableIssueFilename(branchname), true)
}

pub fn commitIssues(issues:&[~Issue]) -> bool {
  writeIssuesToFile(issues, activeIssueFilename(), true)
}

pub fn writeIssuesToFile(issues:&[~Issue], filename:&str, overwrite:bool) -> bool {
  let sortedIssues = date_sort::sortByTime(issues);
  let jsonList = do sortedIssues.map |issue| {issue.getJson()};
  let strval = json::List(jsonList).to_pretty_str();
  file_util::writeStringToFile(strval, filename, overwrite)
}

pub fn readCommittableIssues(branchname:&str) -> ~[~Issue] {
  readIssuesFromFile(committableIssueFilename(branchname))
}

pub fn readCommittedIssues() -> ~[~Issue] {
  readIssuesFromFile(activeIssueFilename())
}

pub fn readIssuesFromFile(filename:&str) -> ~[~Issue] {
  let strvalOpt = file_util::readStringFromFile(filename);
  match strvalOpt{
    Some(strval) => readIssuesFromString(strval),
    None => ~[]
  }
}

fn readIssuesFromString(strval:&str) -> ~[~Issue] {
  let json = extra::json::from_str(strval);
  match json {
    Ok(jsonVal) => readIssuesFromJson(jsonVal),
    Err(_) => ~[]
  }
}

fn readIssuesFromJson(json:extra::json::Json) -> ~[~Issue] {
  match json {
    extra::json::List(ref jsonVals) => 
      do jsonVals.iter().filter_map |jsval| {
        Issue::fromJson(jsval)
      }.collect(),
    _ => ~[]
  }
}

#[test]
pub fn writeReadIssueFile(){
  let testName = ~"writeReadIssueFileTest";
  let issues = ~[Issue::new(~"A", ~"B", ~"C", ~"D")];
  writeIssuesToFile(issues, testName, false);
  let read = readIssuesFromFile(testName);
  file_util::deleteFile(testName);
  assert!(issues == read);
}
