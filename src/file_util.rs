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
