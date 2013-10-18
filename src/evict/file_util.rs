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
use std::io;
use std::path::Path;
use std::os;
use std::result;
pub fn write_string_to_file(content:&str, filename:&str, overwrite:bool) -> bool {
  if(!overwrite && file_exists(filename)){
    false
  }else{
    let path = Path::new(filename);
    let flags = &[io::Truncate, io::Create];
    match io::file_writer(&path, flags){
      result::Ok(writer) => {writer.write_str(content); true}
      result::Err(_) => {false}
    }
  }
 
}
pub fn read_string_from_file(filename:&str) -> Option<~str> {
  match io::read_whole_file_str(&Path::new(filename)){
    result::Ok(result) => Some(result),
    result::Err(_) => None
  }
}

pub fn file_exists(name:&str) -> bool {
  os::path_exists(&Path::new(name))
}

pub fn create_empty(name:&str) -> bool{
  write_string_to_file("", name, false)
}

pub fn create_directory(name:&str) -> bool {
  os::make_dir(&Path::new(name), 0400 | 0200 | 0040 | 0020 | 0004)
}

pub fn delete_file(name:&str) -> bool{
  os::remove_file(&Path::new(name))
}

#[test]
pub fn create_delete_and_existence(){
  let testname = ~"file_util_testCDAE";

  assert!(create_empty(testname));
  assert!(file_exists(testname));
  assert!(delete_file(testname));
  assert!(!file_exists(testname));
}

#[test]
pub fn create_empty_is_empty(){
  let testname = ~"file_util_testCEIE";
  
  assert!(create_empty(testname));
  assert!(read_string_from_file(testname) == Some(~""));
  assert!(delete_file(testname));
}

#[test]
pub fn write_read_str(){
  let testname = ~"file_util_testWRS";
  let testString = ~"This is a test string";

  assert!(write_string_to_file(testString, testname, false));
  assert!(read_string_from_file(testname) == Some(testString));
  assert!(delete_file(testname));
}
