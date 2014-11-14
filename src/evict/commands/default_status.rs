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

use status_storage;

pub fn default_status(mut args:Vec<String>) -> int {
  if args.len() > 1 {
    println! ("default-status usage: evict default-status [new-status]");
    1
  }else{
    if args.len() == 0 {
      let default = status_storage::read_default_status();
      println!("Current default status is: {}", default.name);
      2
    }else{
      // w.r.t the swap_remove - there has got to be a better way to take an
      // element out of a vector. Would make it immut, too.
      let status = status_storage::StatusOption{name:args.swap_remove(0).unwrap()};
      
      match status_storage::write_default_status(&status) {
        Ok(true) => {0}
        Ok(false) => {println!("Could not write to file"); 3}
        Err(s) => {println!("{}", s); 4}
      }
    }
  }
}
