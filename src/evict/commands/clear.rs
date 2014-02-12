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
use std;
use commands;
use file_manager;


pub fn clear_data(_:~[~str]) -> int {
  let evictPath = &std::path::Path::new(file_manager::EVICT_DIRECTORY);
  let absolute = std::os::make_absolute(evictPath);
  let res = commands::prompt(
             format!("Really clear everything from {}? [y/n]", 
                     absolute.display()));
  if res == ~"y" {
    let mut success = true;
    //try to delete, if we fail the just set success to false
    //(no point in retries or anything else, user can just
    // rerun the command)
    match std::io::fs::rmdir_recursive(evictPath) {
        Err(_)  => success = false,
        Ok(_) => {}
    }
    if success {
      println!("All Evict-BT info has been cleared");
      0
    }else{
      println!("Could not clear info.");
      1
    }
  }else{
    println!("Aborting.");
    0
  }
}
