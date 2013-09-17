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
use config;

pub fn clear_data(_:~[~str],_:config::Config) -> int {
  let evictPath = &std::path::Path(file_manager::EVICT_DIRECTORY);
  let res = commands::prompt(
             fmt!("Really clear everything from %s? [y/n]", 
                  std::os::make_absolute(evictPath).to_str()));
  if(res == ~"y"){
    let delResult = std::os::remove_dir_recursive(evictPath);
    if(delResult){0}else{1}
  }else{
    0
  }
}
