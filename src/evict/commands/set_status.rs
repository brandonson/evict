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

use file_manager;
use status_storage;
use issue::IssueStatus;

pub fn set_status(args:Vec<String>) -> isize {
  if args.len() != 2 {
    println!("set-status usage: evict set-status <issue-id> <status>");
    println!("    Where <status> is either the full name of a status");
    println!("    or the index of a status");
    1
  }else{
    match resolve_new_status(args[1].as_slice()) {
      Some(newStatus) => {
        let issues = file_manager::read_issues();
        let edited = selection::update_issue(args[0].as_slice(), issues, |mut oldIssue| {
          oldIssue.status = newStatus.clone();
          oldIssue
        });
        file_manager::write_issues(edited.as_slice());
        0
      }
      None => {println!("Given status does not exist"); 2}
    }
  }
}

fn resolve_new_status(statusIdent:&str) -> Option<IssueStatus> {
  let search = status_storage::read_status_options();
  match usize::from_str_radix(statusIdent, 10) {
    Ok(index) =>
      if search.len() > index {
        Some(search[index].clone())
      } else {
        None
      },
    _ => search.into_iter().find(|x| x.name.as_slice() == statusIdent)
  }.map(|x| x.make_status())
}

