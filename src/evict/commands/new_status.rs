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

pub fn new_status(mut args:Vec<String>) -> isize {
  if args.len() != 1 {
    println!("new-status usage: evict new-status <status-name>");
    1
  }else{
    let mut newStatuses = status_storage::read_status_options();

    //need a better way to move a value out of a vec
    newStatuses.push(status_storage::StatusOption{name:args.swap_remove(0)});
    status_storage::write_status_options(newStatuses);
    0
  }
}
