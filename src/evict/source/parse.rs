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
  issue_start:~str,
  body_line_start:~str,
  body_end_line_start:Option<~str>
}

#[deriving(Clone)]
pub struct SourceSearcher{
  comment_fmts:Vec<CommentFormat>,
  issue_id_comment_start:~str,
  tag_start_delim:char,
  tag_end_delim:char,
  tag_split_delim:char,
  issue_author_name:~str,
  issue_status:IssueStatus
}

pub struct ParseResult{
  pub new_issues:Vec<Issue>,
  pub new_file_contents:~str
}

struct PartialParseResult<'a>{
  new_issues:Vec<Issue>,
  issue_in_progress:Option<Issue>,
  body_in_progress:Option<~str>,
  current_comment_format:Option<&'a CommentFormat>,
  new_contents:~str,
  searcher:&'a SourceSearcher
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
  pub fn new_default_searcher(auth:~str) -> SourceSearcher {
    let double_slash_format = CommentFormat{issue_start:"//".to_owned(),
                                            body_line_start:"//".to_owned(),
                                            body_end_line_start:None};
    let mline_comment_format = CommentFormat{issue_start:"/*".to_owned(),
                                             body_line_start:"*".to_owned(),
                                             body_end_line_start:Some("*/".to_owned())};
    let status = status_storage::read_default_status().make_status();
    SourceSearcher{comment_fmts:vec!(double_slash_format,
                                     mline_comment_format),
                   issue_id_comment_start:"// EVICT-BT-ID: ".to_owned(),
                   tag_start_delim: '[',
                   tag_end_delim: ']',
                   tag_split_delim: ',',
                   issue_author_name:auth,
                   issue_status:status}
  }

  pub fn parse_file<R:Reader>(&self, reader:&mut BufferedReader<R>)
          -> IoResult<ParseResult>{
    self.parse_file_lines(reader.lines())
  }

  pub fn parse_file_lines<'a, ITER:Iterator<IoResult<~str>>>
          (&self, mut iter: ITER) -> IoResult<ParseResult> {
    let partial_result = PartialParseResult{new_issues:vec!(),
                           issue_in_progress:None, body_in_progress:None,
                           current_comment_format:None,
                           new_contents:"".to_owned(),
                           searcher:self};
    let mut state_machine = fsm::StateMachine::new(main_parse_handler,
                                                   partial_result);
                                          
    for lineRes in iter {
      match lineRes {
        Ok(line) =>{
          state_machine.process(line);
        }
        Err(message) => return Err(message)
      }
    }
    return Ok(state_machine.move_state().to_final_result())
  }
}

fn main_parse_handler<'a>(partial_result:PartialParseResult<'a>, input:~str)
    -> fsm::NextState<PartialParseResult<'a>, ~str> {
  let trimmed = input.trim();

  if trimmed.starts_with(partial_result.searcher.issue_id_comment_start) {
    let with_line = add_line(partial_result, input);
    fsm::ChangeState(read_to_issue_end, with_line)
  }else{
    let comment_formats = &partial_result.searcher.comment_fmts;
    for cformat in comment_formats.iter() {
      if trimmed.starts_with(cformat.issue_start) &&
         trimmed.len() > cformat.issue_start.len() {
         let rem_text = trimmed.slice_from(cformat.issue_start.len());
         let (tags, title_text) = read_tags(rem_text, partial_result.searcher);
         
         if tags.is_some() {
         
           let mut new_issue = Issue::new(title_text.to_owned(), "".to_owned(),
                                          partial_result.searcher
                                                        .issue_author_name.clone());
           new_issue.status = partial_result.searcher.issue_status.clone(); 
           let tags = tags.unwrap();
           for t in tags.move_iter(){
             new_issue.add_tag(t);
           }

           let id_line = partial_result.searcher.issue_id_comment_start +
                         new_issue.id + "\n";
           let with_issue_line = add_line(partial_result, id_line);
           let issue_and_input = add_line(with_issue_line, input);
           let new_presult = PartialParseResult{
                               issue_in_progress:Some(new_issue),
                               current_comment_format:Some(cformat),
                               body_in_progress:None,
                               .. issue_and_input};
           return fsm::ChangeState(parse_body, new_presult);
         }
      }
    }
    let with_line = add_line(partial_result, input);
    fsm::Continue(with_line)
  }
}

fn read_tags<'a>(line_text:&'a str, format:&SourceSearcher)
    -> (Option<~[IssueTag]>, &'a str) {
  let trimmed = line_text.trim();
  if trimmed.char_at(0) == format.tag_start_delim {
    let with_tags = trimmed.slice_from(1);
    let split:~[&str] =  with_tags.split(format.tag_end_delim).collect();
    if split.len() < 2 {
      (None, line_text)
    }else{
      let tag_part = split[0];
      let title_part = split[1].trim();
      let split_tags = tag_part.split(format.tag_split_delim);
      let mut tag_vec = split_tags.filter_map(|tag| {
        tag_from_nonempty_str(tag, format.issue_author_name)
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
    Some(IssueTag::new(tag_name.to_owned(), author.to_owned(), true))
  }else{
    None
  }
}

fn read_to_issue_end<'a>(partial_result:PartialParseResult<'a>, input:~str)
    -> fsm::NextState<PartialParseResult<'a>, ~str>{
  let trimmed = input.trim();
  let result_w_line = add_line(partial_result, input);

  let formats = &result_w_line.searcher.comment_fmts;
  for cformat in formats.iter() {
    if trimmed.starts_with(cformat.issue_start) {
      let new_presult = PartialParseResult{
                          current_comment_format:Some(cformat),
                          .. result_w_line};
      return fsm::ChangeState(read_to_issue_end_formatted, new_presult);
    }
  }
  fsm::ChangeState(main_parse_handler, result_w_line)
}

fn read_to_issue_end_formatted<'a>(partial_result:PartialParseResult<'a>, input:~str)
    -> fsm::NextState<PartialParseResult<'a>, ~str>{
  let trimmed = input.trim();
  let npresult = add_line(partial_result, input);
  let format = npresult.current_comment_format.unwrap();
  let is_end = line_ends_format(input, format);

  if is_end || !trimmed.starts_with(format.body_line_start) {
    fsm::ChangeState(main_parse_handler, npresult)
  }else {
    fsm::Continue(npresult)
  }
}

fn parse_body<'a>(partial_result:PartialParseResult<'a>, input:~str)
    -> fsm::NextState<PartialParseResult<'a>, ~str> {
  let trimmed = input.trim();
  let mut with_line = add_line(partial_result, input);
  let format = with_line.current_comment_format.unwrap();
  let is_end = line_ends_format(trimmed, format);

  if is_end || !trimmed.starts_with(format.body_line_start) {
    let mut issue = with_line.issue_in_progress.take_unwrap();
    let nbody = with_line.body_in_progress.take().unwrap_or("".to_owned());
    issue.body_text = nbody;
    with_line.new_issues.push(issue);
    fsm::ChangeState(main_parse_handler, with_line)
  }else{
    let body_so_far = with_line.body_in_progress.take().unwrap_or("".to_owned());
    let stripped_body_line = trimmed.slice_from(format.body_line_start.len())
                                    .trim();
    let new_body = body_so_far + stripped_body_line + "\n";
    with_line.body_in_progress = Some(new_body);
    fsm::Continue(with_line)
  }
}

fn add_line<'a>(presult:PartialParseResult<'a>, line:&str) -> PartialParseResult<'a> {
  let contents = if presult.new_contents == "".to_owned() {
    line.to_owned()
  }else{
    presult.new_contents + line
  };
  PartialParseResult{new_contents:contents, .. presult}
}

fn line_ends_format(input:&str, format:&CommentFormat) -> bool {
  let result_opt = format.body_end_line_start.as_ref().map(|x| input.starts_with(*x));
  println!("{}", result_opt);
  result_opt.unwrap_or(false)
}

#[test]
fn basic_parse_test(){
  use issue::TimelineTag;
  use std::vec::MoveItems;

  let searcher = SourceSearcher::new_default_searcher(~"me");
  let lines:MoveItems<IoResult<~str>> = vec!(Ok(~"//[sometag] This is a title"))
                                            .move_iter();
  let result = searcher.parse_file_lines(lines);  
  assert!(result.is_ok());
  let result = result.unwrap();
  assert!(result.new_issues.len() == 1);

  println!("{}", result.new_file_contents.lines().len());
  println!("{}", result.new_file_contents);

  assert!(result.new_file_contents.lines().len() == 2);

  let issue = result.new_issues.get(0);

  assert!(issue.title == ~"This is a title");
  assert!(issue.author == ~"me");
  assert!(issue.body_text == ~"");
  assert!(issue.events.len() == 1);
  match issue.events.get(0) {
    &TimelineTag(IssueTag{ref tag_name, ..}) => assert!(tag_name == &~"sometag"),
    _ => fail!("Didn't get a tag")
  }

}
