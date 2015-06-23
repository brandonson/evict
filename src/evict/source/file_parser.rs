
use std::path::Path;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::fs::File;
use source::parse::{ParseResult, SourceSearcher};
use issue::Issue;

pub fn parse_and_rewrite_file(searcher:&SourceSearcher, file_path:&Path)
  -> IoResult<Vec<Issue>>{
  let mut file_reader = BufReader::new(File::open(file_path));
  
  let filename = file_path.as_str().unwrap_or("").to_string();

  match searcher.parse_file(&mut file_reader, filename) {
    Ok(parse_result) => handle_parse_result(parse_result, file_path),
    Err(msg) => Err(msg)
  }
}

fn handle_parse_result(result:ParseResult, file_path:&Path) -> IoResult<Vec<Issue>>{
  let ParseResult{new_issues, new_file_contents} = result; //split result
  let result = File::create(file_path).write(new_file_contents.into_bytes().as_str());
  match result{
    Ok(_) => Ok(new_issues),
    Err(msg) => Err(msg)
  }
}
