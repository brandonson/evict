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
use vcs_status;
use config;
use file_manager;
use status_storage;
use issue::IssueStatus;

pub fn setStatus(args:~[~str], _:config::Config) -> int {
  if(args.len() != 2){
    println("set-status usage: evict set-status <issue-id> <status>");
    println("    Where <status> is either the full name of a status");
    println("    or the index of a status");
    1
  }else{
    match vcs_status::currentBranch() {
      Some(branch) => {
        match resolveNewStatus(args[1]) {
          Some(newStatus) => {
            let issues = file_manager::readCommittableIssues(branch);
            let edited = do selection::updateIssue(args[0], issues) |mut oldIssue| {
              oldIssue.status = newStatus.clone();
              oldIssue
            };
            file_manager::writeCommittableIssues(branch, edited);
            0
          }
          None => {println("Could not read current branch"); 2}
        }
      }
      None => 3
    }
  }
}

fn resolveNewStatus(statusIdent:&str) -> Option<~IssueStatus> {
  let search = status_storage::readStatusOptions();
  match from_str::<uint>(statusIdent) {
    Some(index) => if(search.len() > index) {Some(search[index])} else {None},
    None => search.move_iter().find(|x| x.name.as_slice() == statusIdent)
  }.map(|x| x.makeStatus())
}
