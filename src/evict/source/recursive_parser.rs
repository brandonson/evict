use std::io::{IoResult, IoError};
use std::io::fs::PathExtensions;
use std::io::fs;
use std::task;
use std::comm;
use issue::Issue;
use source::parse::SourceSearcher;
use source::file_parser;

pub struct RecursiveParseResult{
  pub new_issues:Vec<Issue>,
  pub failures:Vec<IoError>
}

pub fn parse_directory(searcher:&SourceSearcher, file_path:Path)
  -> RecursiveParseResult {
  
  let mut dirs = vec!(file_path.clone());
  let mut files = vec!();
  
  let result = find_files_in_tree(&mut dirs, &mut files);
  
  if result.is_err() {
    return RecursiveParseResult{new_issues:vec!(),
                                failures:vec!(result.unwrap_err())};
  }


  let mut file_count = files.len();

  let (sender, recvr) = comm::channel();

  for to_parse in files.into_iter() {
    let task_searcher = searcher.clone();
    let task_sender = sender.clone();
    task::spawn(proc(){
      task_sender.send(file_parser::parse_and_rewrite_file(&task_searcher, &to_parse));
    })
  }

  let mut result = RecursiveParseResult{new_issues:vec!(), failures:vec!()};

  while file_count > 0 {
    let file_result = recvr.recv();
    match file_result {
      Ok(issues) => {
        result.new_issues.push_all(issues.as_slice());
      }
      Err(msg) => result.failures.push(msg)
    }
    file_count = file_count - 1;
  }

  result
}

///Finds all files in a directory tree
fn find_files_in_tree(directories:&mut Vec<Path>,
                      files:&mut Vec<Path>) -> IoResult<()>{
  if directories.len() == 0 {
    return Ok(());
  }  

  let subpaths = try!(fs::readdir(&directories.pop().unwrap()));

  for path in subpaths.iter() {
    if path.is_dir() {
      directories.push((*path).clone());
    }else{
      files.push((*path).clone());
    }
  }

  find_files_in_tree(directories, files)
}
