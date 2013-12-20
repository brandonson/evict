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
use std::str;
use std::io;
use std::io::Reader;
use std::io::Writer;

pub fn write_string_to_file(content:&str, filename:&str, overwrite:bool) -> bool {
  if(!overwrite && file_exists(filename)){
    false
  }else{
    //successful by default, then set to false
    //if an error is encountered
    let mut success = true;
    io::io_error::cond.trap(|_| {
      success = false;
    }).inside ( ||io::File::create(&Path::new(filename))
                           .write(content.to_owned().into_bytes())
    );
    success
  }
}
pub fn read_string_from_file(filename:&str) -> Option<~str> {
  read_string_from_path(&Path::new(filename)) 
}

pub fn read_string_from_path(path:&Path) -> Option<~str> {
  let mut error:bool = false;
  let u8bytes = io::io_error::cond.trap(|_| {
    error = true;
  }).inside (||io::File::open(path).read_to_end());
  if(error){
    None
  } else {
    Some(str::from_utf8_owned(u8bytes))
  } 
}

pub fn file_exists(name:&str) -> bool {
  Path::new(name).exists()
}

pub fn create_empty(name:&str) -> bool{
  write_string_to_file("", name, false)
}

pub fn create_directory(name:&str) -> bool {
  create_directory_path(&Path::new(name))
}

pub fn create_directory_path(p:&Path) -> bool {
  io_to_success (
    || io::fs::mkdir(p, 0400 | 0200 | 0040 | 0020 | 0004)
  )
}

pub fn delete_file(name:&str) -> bool{
  io_to_success( ||
    io::fs::unlink(&Path::new(name))
  )
}

pub fn io_to_success(ioCall:| |) -> bool {
  let mut success = true;
  io::io_error::cond.trap(|_| success = false).inside(ioCall);
  success
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
