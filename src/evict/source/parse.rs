use std::vec::Vec;
use std::io::BufferedReader;
use std::io::IoResult;

use issue::{IssueTag, Issue};

use fsm;

pub struct CommentFormat{
  issue_start:~str,
  body_line_start:~str,
  body_end_line_start:Option<~str>
}

pub struct SourceSearcher{
  comment_fmts:Vec<CommentFormat>,
  issue_id_comment_start:~str,
  tag_start_delim:char,
  tag_end_delim:char,
  tag_split_delim:char,
  issue_author_name:~str
}

pub struct ParseResult{
  new_issues:Vec<Issue>,
  new_file_contents:~str
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
  fn to_final_result(self) -> ParseResult {
    let PartialParseResult{new_issues, new_contents, ..} = self;
    ParseResult{new_issues:new_issues, new_file_contents:new_contents}
  }
}

impl SourceSearcher {
  pub fn new_default_searcher(auth:~str) -> SourceSearcher {
    let double_slash_format = CommentFormat{issue_start:~"//",
                                            body_line_start:~"//",
                                            body_end_line_start:None};
    let mline_comment_format = CommentFormat{issue_start:~"/*",
                                             body_line_start:~"*",
                                             body_end_line_start:Some(~"*/")};
    SourceSearcher{comment_fmts:vec!(double_slash_format,
                                     mline_comment_format),
                   issue_id_comment_start:~"//--evict-id",
                   tag_start_delim: '[',
                   tag_end_delim: ']',
                   tag_split_delim: ',',
                   issue_author_name:auth}
  }

  pub fn parse_file<R:Reader>(&self, reader:&mut BufferedReader<R>)
          -> IoResult<ParseResult>{
    let partial_result = PartialParseResult{new_issues:vec!(),
                           issue_in_progress:None, body_in_progress:None,
                           current_comment_format:None,
                           new_contents:~"",
                           searcher:self};
    let mut state_machine = fsm::StateMachine::new(main_parse_handler,
                                                   partial_result);
                                          
    for lineRes in reader.lines() {
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
         
           let mut new_issue = Issue::new(title_text.to_owned(), ~"",
                                          partial_result.searcher
                                                        .issue_author_name.clone());
         
           let tags = tags.unwrap();
           for t in tags.move_iter(){
             new_issue.add_tag(t);
           }

           let id_line = partial_result.searcher.issue_id_comment_start +
                         new_issue.id;
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
  let is_end = line_ends_format(input, format);

  if is_end || !trimmed.starts_with(format.body_line_start) {
    let mut issue = with_line.issue_in_progress.take_unwrap();
    let nbody = with_line.body_in_progress.take().unwrap_or(~"");
    issue.body_text = nbody;
    with_line.new_issues.push(issue);
    fsm::ChangeState(main_parse_handler, with_line)
  }else{
    let body_so_far = with_line.body_in_progress.take().unwrap_or(~"");
    let stripped_body_line = input.slice_from(format.body_line_start.len())
                                  .trim();
    let new_body = body_so_far + stripped_body_line;
    with_line.body_in_progress = Some(new_body);
    fsm::Continue(with_line)
  }
}

fn add_line<'a>(presult:PartialParseResult<'a>, line:&str) -> PartialParseResult<'a> {
  let contents = presult.new_contents + "\n" + line;
  PartialParseResult{new_contents:contents, .. presult}
}

fn line_ends_format(input:&str, format:&CommentFormat) -> bool {
  format.body_end_line_start.as_ref().map(|x| input.starts_with(*x)).unwrap_or(false)
}
