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
use std::*;


pub fn writeStringToFile(content:&str, filename:&str, overwrite:bool) -> bool {
  if(!overwrite && fileExists(filename)){
    false
  }else{
    let path = Path(filename);
    let flags = &[io::Truncate, io::Create];
    match io::file_writer(&path, flags){
      result::Ok(writer) => {writer.write_str(content); true}
      result::Err(_) => {false}
    }
  }
 
}
pub fn readStringFromFile(filename:&str) -> Option<~str> {
  match io::read_whole_file_str(&Path(filename)){
    result::Ok(result) => Some(result),
    result::Err(_) => None
  }
}
pub fn fileExists(name:&str) -> bool {
  match io::file_reader(&Path(name)){
    result::Ok(_) => true,
    _ => false,
  }
}
pub fn createEmpty(name:&str) -> bool{
  writeStringToFile("", name, false)
}

pub fn deleteFile(name:&str) -> bool{
  os::remove_file(&Path(name))
}

#[test]
pub fn createDeleteAndExistence(){
  let testname = ~"file_util_testCDAE";

  assert!(createEmpty(testname));
  assert!(fileExists(testname));
  assert!(deleteFile(testname));
  assert!(!fileExists(testname));
}

#[test]
pub fn createEmptyIsEmpty(){
  let testname = ~"file_util_testCEIE";
  
  assert!(createEmpty(testname));
  assert!(readStringFromFile(testname) == Some(~""));
  assert!(deleteFile(testname));
}

#[test]
pub fn writeReadStr(){
  let testname = ~"file_util_testWRS";
  let testString = ~"This is a test string";

  assert!(writeStringToFile(testString, testname, false));
  assert!(readStringFromFile(testname) == Some(testString));
  assert!(deleteFile(testname));
}
