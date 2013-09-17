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

pub fn active_issue_filename() -> ~str {fmt!("%s%s%s",EVICT_DIRECTORY, 
                                                ACTIVE_ISSUE_FILENAME_PART, 
                                                EXTENSION)}

pub fn committable_issue_filename(branchname:&str) -> ~str {
  fmt!("%s%s%s", EVICT_DIRECTORY, branchname, LOCAL_EXT)
}

pub fn write_committable_issues(branchname:&str, issues:&[~Issue]) -> bool {
  write_issues_to_file(issues, committable_issue_filename(branchname), true)
}

pub fn commit_issues(issues:&[~Issue]) -> bool {
  write_issues_to_file(issues, active_issue_filename(), true)
}

pub fn write_issues_to_file(issues:&[~Issue], filename:&str, overwrite:bool) -> bool {
  let sorted_issues = date_sort::sort_by_time(issues);
  let jsonList = do sorted_issues.map |issue| {issue.to_json()};
  let strval = json::List(jsonList).to_pretty_str();
  file_util::write_string_to_file(strval, filename, overwrite)
}

pub fn read_committable_issues(branchname:&str) -> ~[~Issue] {
  read_issues_from_file(committable_issue_filename(branchname))
}

pub fn read_committed_issues() -> ~[~Issue] {
  read_issues_from_file(active_issue_filename())
}

pub fn read_issues_from_file(filename:&str) -> ~[~Issue] {
  let strvalOpt = file_util::read_string_from_file(filename);
  match strvalOpt{
    Some(strval) => read_issues_from_string(strval),
    None => ~[]
  }
}

fn read_issues_from_string(strval:&str) -> ~[~Issue] {
  let json = extra::json::from_str(strval);
  match json {
    Ok(jsonVal) => read_issues_from_json(jsonVal),
    Err(_) => ~[]
  }
}

fn read_issues_from_json(json:extra::json::Json) -> ~[~Issue] {
  match json {
    extra::json::List(ref jsonVals) => 
      do jsonVals.iter().filter_map |jsval| {
        Issue::from_json(jsval)
      }.collect(),
    _ => ~[]
  }
}

#[test]
pub fn write_read_issue_file(){
  let testName = ~"writeReadIssueFileTest";
  let issues = ~[Issue::new(~"A", ~"B", ~"C", ~"D")];
  write_issues_to_file(issues, testName, false);
  let read = read_issues_from_file(testName);
  file_util::delete_file(testName);
  assert!(issues == read);
}
