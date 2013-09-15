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
use std::*;

enum VCS{
  Git
}

impl VCS {
  fn currentBranchCmdOutput(&self) -> ~str{
    match self {
      &Git =>
        str::from_utf8(run::process_output("git", [~"rev-parse", 
                                                    ~"--abbrev-ref", 
                                                    ~"HEAD"]).output)
    }
  }
}
fn currentVCS() -> VCS{
  Git //TODO actually detect a VCS
}

pub fn currentBranch() -> Option<~str> {
  let output = currentVCS().currentBranchCmdOutput(); 
  let mut line:~str = ~"";
  for branch in output.any_line_iter() {
    line = branch.to_owned();
    break;
  }
  if (line == ~"") {
    None
  }else{
    Some(line)
  }
}

