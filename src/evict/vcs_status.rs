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
use std::process;

enum VCS{
  Git
}

impl VCS {
  fn current_branch_cmd_output(&self) -> Option<String>{
    match self {
      &VCS::Git => {
        let mut gitcmd = process::Command::new("git");
        gitcmd.arg("rev-parse").arg("--abbrev-ref").arg("HEAD");
        let output = gitcmd.output();
        match output {
          Ok(out) => String::from_utf8(out.stdout).ok(),
          Err(_) => None
        }
      }
    }
  }

  fn current() -> VCS {
    VCS::Git  //TODO actually detect a VCS
  }
}

pub fn current_branch() -> Option<String> {
  let output = VCS::current().current_branch_cmd_output(); 
  output.and_then(grab_first_line).map(|x| x.to_string())
}

fn grab_first_line(grab_from:String) -> Option<String> {
  //'loop' through the lines but just return
  //the first line we get
  for first in grab_from.as_str().lines_any() {
    return Some(first.to_string());
  }
  //there were no lines, return None
  None
}

