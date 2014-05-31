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

use std::vec::Vec;
use std::io::BufferedReader;
use std::io::IoResult;

use issue::{IssueTag, Issue};
use issue::{IssueStatus};

use status_storage;

use fsm;

#[deriving(Clone)]
pub struct CommentFormat{
  issue_start:String,
  body_line_start:String,
  body_end_line_start:Option<String>
}

#[deriving(Clone)]
pub struct SourceSearcher{
  comment_fmts:Vec<CommentFormat>,
  issue_id_comment_start:String,
  tag_start_delim:char,
  tag_end_delim:char,
  tag_split_delim:char,
  issue_author_name:String,
  issue_status:IssueStatus
}

pub struct ParseResult{
  pub new_issues:Vec<Issue>,
  pub new_file_contents:String
}

struct PartialParseResult<'a>{
  new_issues:Vec<Issue>,
  issue_in_progress:Option<Issue>,
  body_in_progress:Option<String>,
  current_comment_format:Option<&'a CommentFormat>,
  new_contents:String,
  searcher:&'a SourceSearcher
}

struct ParseLineInfo{
  content:String,
  filename:String,
  line_number:uint
}

impl<'a> PartialParseResult<'a> {
  fn to_final_result(mut self) -> ParseResult {
    if self.issue_in_progress.is_some() {
      let mut issue = self.issue_in_progress.take_unwrap();
      if self.body_in_progress.is_some() {
        issue.body_text = self.body_in_progress.take_unwrap();
      }
      self.new_issues.push(issue);
    }
    let PartialParseResult{new_issues, new_contents, ..} = self;
    ParseResult{new_issues:new_issues, new_file_contents:new_contents}
  }
}

impl SourceSearcher {
  pub fn new_default_searcher(auth:String) -> SourceSearcher {
    let double_slash_format = CommentFormat{issue_start:"//".into_string(),
                                            body_line_start:"//".into_string(),
                                            body_end_line_start:None};
    let mline_comment_format = CommentFormat{issue_start:"/*".into_string(),
                                             body_line_start:"*".into_string(),
                                             body_end_line_start:Some("*/".into_string())};
    let status = status_storage::read_default_status().make_status();
    SourceSearcher{comment_fmts:vec!(double_slash_format,
                                     mline_comment_format),
                   issue_id_comment_start:"// EVICT-BT-ID: ".into_string(),
                   tag_start_delim: '[',
                   tag_end_delim: ']',
                   tag_split_delim: ',',
                   issue_author_name:auth,
                   issue_status:status}
  }

  pub fn parse_file<R:Reader>(&self, reader:&mut BufferedReader<R>, filename:String)
          -> IoResult<ParseResult>{
    self.parse_file_lines(reader.lines(), filename)
  }

  pub fn parse_file_lines<'a, ITER:Iterator<IoResult<String>>>
          (&self, mut iter: ITER, filename:String) -> IoResult<ParseResult> {
    let partial_result = PartialParseResult{new_issues:vec!(),
                           issue_in_progress:None, body_in_progress:None,
                           current_comment_format:None,
                           new_contents:"".into_string(),
                           searcher:self};
    let mut state_machine = fsm::StateMachine::new(main_parse_handler,
                                                   partial_result);

    let mut linenum = 1u;
                                          
    for lineRes in iter {
      match lineRes {
        Ok(line) =>{
          let info = ParseLineInfo{content:line,
                                   filename:filename.clone(),
                                   line_number:linenum};
          state_machine.process(info);
        }
        Err(message) => return Err(message)
      }
      linenum = linenum + 1;
    }
    return Ok(state_machine.move_state().to_final_result())
  }
}

fn main_parse_handler<'a>(partial_result:PartialParseResult<'a>, lineinfo:ParseLineInfo)
    -> fsm::NextState<PartialParseResult<'a>, ParseLineInfo> {
  let ParseLineInfo{content:input, filename:file, line_number:linenum} = lineinfo;
  let trimmed = input.as_slice().trim();

  if trimmed.starts_with(partial_result.searcher.issue_id_comment_start.as_slice()) {
    let with_line = add_line(partial_result, input.as_slice());
    fsm::ChangeState(read_to_issue_end, with_line)
  }else{
    let comment_formats = &partial_result.searcher.comment_fmts;
    for cformat in comment_formats.iter() {
      if trimmed.starts_with(cformat.issue_start.as_slice()) &&
        trimmed.len() > cformat.issue_start.len() {
        let rem_text = trimmed.slice_from(cformat.issue_start.len());
        let (tags, title_text) = read_tags(rem_text, partial_result.searcher);
        
        if tags.is_some() {
        
          let mut new_issue = Issue::new(title_text.into_string(), "".into_string(),
                                         partial_result.searcher
                                                       .issue_author_name.clone());
          new_issue.status = partial_result.searcher.issue_status.clone(); 
          let tags = tags.unwrap();
          for t in tags.move_iter(){
            new_issue.add_tag(t);
          }

          //whitespace ends when the issue starter starts
          let whitespace_end = cformat.issue_start.as_slice().char_at(0);
        
          let split_vec:Vec<&str> = input.as_slice().splitn(whitespace_end, 1).collect();

          //get the whitespace so we keep indentation
          let whitespace = split_vec.get(0);

          //convenience - the id line starter
          let id_start = partial_result.searcher.issue_id_comment_start.as_slice();

          //create the id line
          let id_line = whitespace.into_string()
                                  .append(id_start)
                                  .append(new_issue.id.as_slice())
                                  .append("\n");

          //add onto file contents
          let with_issue_line = add_line(partial_result, id_line.as_slice());
          let issue_and_input = add_line(with_issue_line, input.as_slice());

          let bodyStart = format!("Parsed from {} line {}\n\n", file, linenum);
          
          let new_presult = PartialParseResult{
                              issue_in_progress:Some(new_issue),
                              current_comment_format:Some(cformat),
                              body_in_progress:Some(bodyStart),
                              .. issue_and_input};
          return fsm::ChangeState(parse_body, new_presult);
        }
      }
    }
    let with_line = add_line(partial_result, input.as_slice());
    fsm::Continue(with_line)
  }
}

fn read_tags<'a>(line_text:&'a str, format:&SourceSearcher)
    -> (Option<Vec<IssueTag>>, &'a str) {
  let trimmed = line_text.trim();
  if trimmed.char_at(0) == format.tag_start_delim {
    let with_tags = trimmed.slice_from(1);
    let split:Vec<&str> =  with_tags.split(format.tag_end_delim).collect();
    if split.len() < 2 {
      (None, line_text)
    }else{
      let tag_part = split.get(0);
      let title_part = split.get(1).trim();
      let split_tags = tag_part.split(format.tag_split_delim);
      let mut tag_vec = split_tags.filter_map(|tag| {
        tag_from_nonempty_str(tag, format.issue_author_name.as_slice())
      });
      (Some(tag_vec.collect()), title_part)
    }
  }else{
    (None, line_text)
  }
}

fn tag_from_nonempty_str(tag_name:&str, author:&str) -> Option<IssueTag> {
  let trimmed_str = tag_name.trim();
  if trimmed_str.len() > 0 {
    Some(IssueTag::new(tag_name.into_string(), author.into_string(), true))
  }else{
    None
  }
}

fn read_to_issue_end<'a>(partial_result:PartialParseResult<'a>, line_info:ParseLineInfo)
    -> fsm::NextState<PartialParseResult<'a>, ParseLineInfo>{
  let input = line_info.content;
  let trimmed = input.as_slice().trim();
  let result_w_line = add_line(partial_result, input.as_slice());

  let formats = &result_w_line.searcher.comment_fmts;
  for cformat in formats.iter() {
    if trimmed.starts_with(cformat.issue_start.as_slice()) {
      let new_presult = PartialParseResult{
                          current_comment_format:Some(cformat),
                          .. result_w_line};
      return fsm::ChangeState(read_to_issue_end_formatted, new_presult);
    }
  }
  fsm::ChangeState(main_parse_handler, result_w_line)
}

fn read_to_issue_end_formatted<'a>(partial_result:PartialParseResult<'a>, line_info:ParseLineInfo)
    -> fsm::NextState<PartialParseResult<'a>, ParseLineInfo>{
  let input = line_info.content;
  let trimmed = input.as_slice().trim();
  let npresult = add_line(partial_result, input.as_slice());
  let format = npresult.current_comment_format.unwrap();
  let is_end = line_ends_format(input.as_slice(), format);

  if is_end || !trimmed.starts_with(format.body_line_start.as_slice()) {
    fsm::ChangeState(main_parse_handler, npresult)
  }else {
    fsm::Continue(npresult)
  }
}

fn parse_body<'a>(partial_result:PartialParseResult<'a>, line_info:ParseLineInfo)
    -> fsm::NextState<PartialParseResult<'a>, ParseLineInfo> {
  let input = line_info.content;

  let trimmed = input.as_slice().trim();
  let mut with_line = add_line(partial_result, input.as_slice());
  let format = with_line.current_comment_format.unwrap();
  let is_end = line_ends_format(trimmed, format);

  if is_end || !trimmed.starts_with(format.body_line_start.as_slice()) {
    let mut issue = with_line.issue_in_progress.take_unwrap();
    let nbody = with_line.body_in_progress.take().unwrap_or("".into_string());
    issue.body_text = nbody;
    with_line.new_issues.push(issue);
    fsm::ChangeState(main_parse_handler, with_line)
  }else{
    let body_so_far = with_line.body_in_progress.take().unwrap_or("".into_string());
    let stripped_body_line = trimmed.slice_from(format.body_line_start.len())
                                    .trim();
    let new_body = body_so_far.append(stripped_body_line).append("\n");
    with_line.body_in_progress = Some(new_body);
    fsm::Continue(with_line)
  }
}

fn add_line<'a>(presult:PartialParseResult<'a>, line:&str) -> PartialParseResult<'a> {
  let contents = if presult.new_contents == "".into_string() {
    line.into_string()
  }else{
    presult.new_contents.to_string().append(line)
  };
  PartialParseResult{new_contents:contents, .. presult}
}

fn line_ends_format(input:&str, format:&CommentFormat) -> bool {
  let result_opt = format.body_end_line_start.as_ref().map(|x| input.starts_with(x.as_slice()));
  result_opt.unwrap_or(false)
}

#[test]
fn basic_parse_test(){
  use issue::TimelineTag;
  use std::vec::MoveItems;

  let searcher = SourceSearcher::new_default_searcher("me".into_string());
  let lines:MoveItems<IoResult<String>> = vec!(Ok("  //[sometag] This is a title".into_string()))
                                            .move_iter();
  let result = searcher.parse_file_lines(lines, "foo".to_string());  
  assert!(result.is_ok());
  let result = result.unwrap();
  assert!(result.new_issues.len() == 1);

  println!("{}", result.new_file_contents.as_slice().lines().len());
  println!("{}", result.new_file_contents);

  assert!(result.new_file_contents.as_slice().lines().len() == 2);
  assert!(result.new_file_contents.as_slice()
                                  .lines()
                                  .collect::<Vec<&str>>()
                                  .get(0)
                                  .starts_with("  "));

  let issue = result.new_issues.get(0);

  assert!(issue.title == "This is a title".into_string());
  assert!(issue.author == "me".into_string());
  assert!(issue.body_text == "Parsed from foo line 1\n\n".into_string());
  assert!(issue.events.len() == 1);
  match issue.events.get(0) {
    &TimelineTag(IssueTag{ref tag_name, ..}) => assert!(tag_name == & "sometag".into_string()),
    _ => fail!("Didn't get a tag")
  }

}
