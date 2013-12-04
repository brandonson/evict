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
use selection;
use issue::{IssueTag, Issue};
use commands;
use file_manager;

pub fn tag(args:~[~str]) -> int {
  tag_cmd(args, "tag", true)
}

pub fn untag(args:~[~str]) -> int {
  tag_cmd(args, "untag", false)
}

pub fn tag_cmd(args:~[~str], cmdName:&str, enabledAfter:bool) -> int {
  if(args.len() != 2){
    println(format!("{} usage: evict {} <issue-id> <tag>", cmdName, cmdName));
    1
  }else{
    let issues = file_manager::read_issues();
    let updated = selection::update_issue(args[0],
                                          issues,
                                          |issue| modify_tag(issue, args[1], enabledAfter));
    if(file_manager::write_issues(updated)){
      0
    }else{
      2
    }
  }
}

fn modify_tag(mut issue:Issue, tag:&str, enabledAfter:bool) -> Issue {
  let author = commands::get_author();
  //Clone early here so we don't have a mut borrow at the same
  //time as the immut borrow that happens here
  let lastTag = {
    let optTag = issue.most_recent_tag_for_name(tag);
    optTag.map(|tag| tag.clone())
  };
  if(lastTag.is_none() || lastTag.unwrap().enabled != enabledAfter){
    issue.add_tag(IssueTag::new(tag.to_owned(), author, enabledAfter));
  }
  issue
}

