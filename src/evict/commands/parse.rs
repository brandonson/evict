use file_manager;
use source::recursive_parser::parse_directory;
use source::parse::SourceSearcher;
use std::path::Path;
use commands;

pub fn parse_issues(args:~[~str]) -> int{
  let author = commands::get_author();
  let result = parse_directory(&SourceSearcher::new_default_searcher(author),
                               Path::new(""));
  file_manager::write_issues(result.new_issues.as_slice());
  for errstr in result.failures.iter(){
    println!("Parser error: {}", errstr);
  }
  0
}
