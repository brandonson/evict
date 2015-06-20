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
use file_manager;
use status_storage::{StatusOption, 
                     write_status_options, 
                     write_default_status};
use file_util;

pub fn initialize(_:Vec<String>) -> isize {
  let createSuccess = file_util::create_directory(file_manager::EVICT_DIRECTORY);
  if createSuccess {
    let defaultStatus = StatusOption{name:"open".into_string()};
    let statusOpts = vec!(defaultStatus.clone(), StatusOption{name:"closed".into_string()});
    let optionSuccess = write_status_options(statusOpts);
    if optionSuccess {
      let defaultResult = write_default_status(&defaultStatus);
      if defaultResult.is_ok() {
        let idirSuccess = file_util::create_directory(
                                          file_manager::issue_directory().as_slice());
        if idirSuccess {0}else{1}
      }else{
        2
      }
    }else{
      3
    }
  }else{4}
}
