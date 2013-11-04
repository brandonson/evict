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
use std::path::Path;
use std::os;
use std::str;
use std::rt::io;
use std::rt::io::Reader;
use std::rt::io::Writer;

pub fn write_string_to_file(content:&str, filename:&str, overwrite:bool) -> bool {
  if(!overwrite && file_exists(filename)){
    false
  }else{
    //successful by default, then set to false
    //if an error is encountered
    let mut success = true;
    do io::io_error::cond.trap(|_| {
      success = false;
    }).inside {
      io::file::open(&Path::new(filename), io::CreateOrTruncate, io::Write)
                 .write(content.to_owned().into_bytes());
    };
    success
  }
}
pub fn read_string_from_file(filename:&str) -> Option<~str> {
  read_string_from_path(&Path::new(filename)) 
}

pub fn read_string_from_path(path:&Path) -> Option<~str> {
  let mut error:bool = false;
  let u8bytes = do io::io_error::cond.trap(|_| {
    error = true;
  }).inside {
    io::file::open(path, io::Open, io::Read)
               .read_to_end()
  };
  if(error){
    None
  } else {
    Some(str::from_utf8(u8bytes))
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
