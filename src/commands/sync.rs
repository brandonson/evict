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
use file_manager::*;
use merge::mergeIssues;
use vcs_status;
use config;

pub fn syncIssues(_:~[~str], _:config::Config) -> int {
  let branchOpt = vcs_status::currentBranch();
  do branchOpt.map_move_default(2) |branch| {
    let incoming = readCommittedIssues();
    let mergeInto = readCommittableIssues(branch);
    
    let merged = mergeIssues(incoming, mergeInto);

    let success1 = writeCommittableIssues(branch, merged);
    let success2 = commitIssues(merged);
    if(success1 && success2){0}else{1}
  }
}
