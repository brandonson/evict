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
use std::io::process;
use std::str;
enum VCS{
  Git
}

impl VCS {
  fn current_branch_cmd_output(&self) -> Option<~str>{
    match self {
      &Git => {
        let output = process::Process::output("git", ["rev-parse".to_owned(), 
                                                    "--abbrev-ref".to_owned(), 
                                                    "HEAD".to_owned()])
                         .map(|x| x.output);
        match output {
          Ok(out) => str::from_utf8_owned(out.as_slice().to_owned()),
          Err(_) => None
        }
      }
    }
  }

  fn current() -> VCS {
    Git  //TODO actually detect a VCS
  }
}

pub fn current_branch() -> Option<~str> {
  let output = VCS::current().current_branch_cmd_output(); 
  output.and_then(grab_first_line)
}

fn grab_first_line(grab_from:~str) -> Option<~str> {
  //'loop' through the lines but just return
  //the first line we get
  for first in grab_from.lines_any() {
    return Some(first.to_owned());
  }
  //there were no lines, return None
  None
}

