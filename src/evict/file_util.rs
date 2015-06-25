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
use std::fs::PathExt;
use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::fs;

pub fn write_string_to_file(content:&str, filename:&str, overwrite:bool) -> IoResult<()> {
  if !overwrite && file_exists(filename) {
    Err(IoError::new(ErrorKind::AlreadyExists, "File already exists and should not be overwritten"))
  }else{
    //successful by default, then set to false
    //if an error is encountered
    fs::File::create(&Path::new(filename)).and_then(
                    |mut f| f.write(content.to_string().into_bytes().as_slice()).map(|_| ()))
  }
}
pub fn read_string_from_file(filename:&str) -> IoResult<String> {
  read_string_from_path(&Path::new(filename)) 
}

pub fn read_string_from_path(path:&Path) -> IoResult<String> {
  let mut string = String::new();
  fs::File::open(path).and_then(|mut f| f.read_to_string(&mut string)).map(|_| string)
}

pub fn file_exists(name:&str) -> bool {
  Path::new(name).exists()
}

pub fn create_empty(name:&str) -> IoResult<()>{
  write_string_to_file("", name, false)
}

pub fn create_directory(name:&str) -> bool {
  create_directory_path(&Path::new(name))
}

pub fn create_directory_path(p:&Path) -> bool {
  io_to_success (
    || fs::create_dir(p)
  )
}

pub fn delete_file(name:&str) -> bool{
  io_to_success( ||
    fs::remove_file(&Path::new(name))
  )
}

pub fn io_to_success<IOC:Fn() -> IoResult<()>>(ioCall:IOC) -> bool {
  let mut success = true;
  match ioCall() {
    Err(_) => success = false,
    Ok(_) => {}
  }
  success
}

#[test]
pub fn create_delete_and_existence(){
  let testname = "file_util_testCDAE";

  assert!(create_empty(testname).is_ok());
  assert!(file_exists(testname));
  assert!(delete_file(testname));
  assert!(!file_exists(testname));
}

#[test]
pub fn create_empty_is_empty(){
  let testname = "file_util_testCEIE";
  
  assert!(create_empty(testname).is_ok());
  let file_string = read_string_from_file(testname);
  assert!(file_string.is_ok());
  assert_eq!(file_string.unwrap(), "".to_string());
  assert!(delete_file(testname));
}

#[test]
pub fn write_read_str(){
  let testname = "file_util_testWRS";
  let testString = "This is a test string".to_string();

  assert!(write_string_to_file(testString.as_str(), testname, false).is_ok());
  let file_string = read_string_from_file(testname);
  assert!(file_string.is_ok());
  assert_eq!(file_string.unwrap(), testString);
  assert!(delete_file(testname));
}
