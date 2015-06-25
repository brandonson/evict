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
use serialize::json;
use serialize::json::ToJson;
use serde;
use serde::json::value::Value as JsonValue;
use serde::json::Error as JsonError;
use serde::json::value::from_value as from_json_value;

use time;
use evict;
use vcs_status;
use status_storage::DEFAULT_STATUS_NAME;
use self::IssueTimelineEvent::{TimelineComment, TimelineTag};

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

use std::error::Error;

type JsonObjectMap = BTreeMap<String, JsonValue>;

pub use serdetime::TIME_FORMAT;
pub use serdetime::SerdeTime;

pub static BODY_KEY:&'static str = "bodyText";
pub static TIME_KEY:&'static str = "time";
pub static AUTHOR_KEY:&'static str = "author";
pub static TITLE_KEY:&'static str = "title";
pub static ID_KEY:&'static str = "id";
pub static VERSION_KEY:&'static str = "evict-version";
pub static I_EVENT_KEY:&'static str = "events";
pub static BRANCH_KEY:&'static str = "branch";
pub static STATE_KEY:&'static str = "status";
pub static NAME_KEY:&'static str = "name";
pub static ENABLED_KEY:&'static str = "enabled";
pub static TIMELINE_EVT_KEY:&'static str = "t-evt-type";

#[derive(Debug)]
pub enum IssueJsonParseError {
  SerdeInternalError(JsonError),
  KeyNotFound(String),
  UnexpectedJsonValue
}

impl Display for IssueJsonParseError {
  fn fmt(&self, fmt:&mut Formatter) -> Result<(), FmtError> {
    write!(fmt, "{}", self.description())
  }
}

impl ::std::error::Error for IssueJsonParseError {
  fn description(&self) -> &str {
    use self::IssueJsonParseError::*;
    match *self {
      SerdeInternalError(_) => "json parser error",
      KeyNotFound(_) => "missing key/value",
      UnexpectedJsonValue => "json value of wrong type"
    }
  }

  fn cause(&self) -> Option<&::std::error::Error> {
    match *self {
      IssueJsonParseError::SerdeInternalError(ref e) => Some(e),
      _ => None
    }
  }
}

impl From<JsonError> for IssueJsonParseError {
  fn from(err:JsonError) -> IssueJsonParseError {
    IssueJsonParseError::SerdeInternalError(err)
  }
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct IssueComment{
  pub creation_time: SerdeTime,
  pub author:String,
  pub body_text:String,
  pub branch:String,
  pub id:String
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct IssueTag{
  pub time: SerdeTime,
  pub tag_name: String,
  pub enabled: bool,
  pub author: String,
  pub change_id: String
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum IssueTimelineEvent{
  TimelineComment(IssueComment),
  TimelineTag(IssueTag)
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct IssueStatus{
  pub name:String,
  pub last_change_time: SerdeTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IssueBase {
  pub title:String,
  pub creation_time: SerdeTime, 
  pub author: String,
  pub id: String,
  pub branch: String,
  pub body_text:String,
}

#[derive(Clone, Debug)]
pub struct Issue{
  pub events:Vec<IssueTimelineEvent>,
  pub status:IssueStatus,
  pub base: IssueBase,
}


impl PartialEq for Issue{
  fn eq(&self, other:&Issue) -> bool {
    return self.id() == other.id();
  }
}

impl IssueStatus{
  pub fn new(name:String) -> IssueStatus {
    IssueStatus{name:name, last_change_time:SerdeTime(time::now())}
  }
}

fn get_string_for_key(map:&json::Object, key:&str) -> Option<String>{
  let value_opt = map.get(&key.to_string());
  value_opt.and_then (|value| {
    match value {
      &json::Json::String(ref strVal) => Some(strVal.to_string()),
      _ => None
    }
  })
}

//Delegates
impl Issue{
  pub fn title(&self) -> &str {
    self.base.title.as_str()
  }

  pub fn creation_time(&self) -> time::Tm {
    self.base.creation_time.0
  }

  pub fn author(&self) -> &str {
    self.base.author.as_str()
  }

  pub fn id(&self) -> &str {
    self.base.id.as_str()
  }

  pub fn branch(&self) -> &str {
    self.base.branch.as_str()
  }

  pub fn body_text(&self) -> &str {
    self.base.body_text.as_str()
  }
}

impl Issue{

  pub fn add_comment(&mut self, comment:IssueComment) {
    self.events.push(TimelineComment(comment))
  }
  
  pub fn add_tag(&mut self, tag:IssueTag) {
    self.events.push(TimelineTag(tag))
  }

  pub fn most_recent_tag_for_name<'x>(&'x self, name:&str) -> Option<&'x IssueTag> {
    let mut recent:Option<&'x IssueTag> = None;
    for evt in self.events.iter(){
      match evt {
        &TimelineTag(ref tag) => {
          if tag.tag_name.as_str() == name {
            if recent.is_none() {
              recent = Some(tag);
            }else{
              let old_tag = recent.take().unwrap();
              if old_tag.time.0.to_timespec() < tag.time.0.to_timespec() {
                recent = Some(tag);
              }else{
                recent = Some(old_tag);
              }
            }
          }
        }
        _ => {}
      }
    }
    recent
  }

  ///Returns a vector of all tags currently enabled on this Issue.
  ///Assumes that the list of events is sorted by date.  Issue::from_json
  ///applies this sorting, so it rarely needs to be done by callers of
  ///this function.
  pub fn all_tags(&self) -> Vec<String> {
    let mut untagged:Vec<String> = vec!();
    let mut tag_list:Vec<String> = vec!();
    for evt in self.events.iter().rev() {
      match evt {
        &TimelineTag(ref tag) => {
          let is_untag = untagged.contains(&tag.tag_name);
          if !is_untag && tag.enabled {
            tag_list.push(tag.tag_name.clone());
          }else if !is_untag && !tag.enabled{
            untagged.push(tag.tag_name.clone());
          }
        }
        _ => {}
      }
    }
    tag_list
  }

  pub fn no_comment_json(&self) -> JsonValue {
    let mut map:JsonObjectMap = BTreeMap::new();
    let base_json = serde::json::value::to_value(&self.base);
    let state_json = serde::json::value::to_value(&self.status);

    map.insert(STATE_KEY.to_string(), state_json);
    map.insert("base".to_string(), base_json);
    JsonValue::Object(map)
  }

  pub fn from_str(string:&str) -> Result<Issue, IssueJsonParseError> {
    let json = try!(serde::json::from_str(string));
    Issue::from_json(json)
  }

  pub fn from_json(json:JsonValue) -> Result<Issue, IssueJsonParseError> {
    //reads issue. also sorts so that the events are in order by time
    //this time ordering is necessary for all_tags to work properly

    match json {
      JsonValue::Object(map) => Issue::read_from_map(map),
      _ => Err(IssueJsonParseError::UnexpectedJsonValue)
    }.map(|x| ::date_sort::sort_by_time(vec!(x)).pop().unwrap())
    // [ugly] Fix date sorting individual issue events
    // This will fix the line above.  Probably just means splitting out
    // part of date_sort::sort_by_time
  }

  fn read_from_map(mut map:JsonObjectMap) -> Result<Issue, IssueJsonParseError>{
    let state:IssueStatus = try!(
      match map.remove(STATE_KEY) {
        Some(val_json) => {
          from_json_value(val_json).map_err(Into::into)
        }
        None => Err(IssueJsonParseError::KeyNotFound(STATE_KEY.to_string()))
      });
    let base_opt = map.remove("base").map(from_json_value);

    match base_opt {
      Some(r) => 
        r.map(|base| Issue{ events: vec!(), status: state, base: base })
         .map_err(Into::into),
      None => Err(IssueJsonParseError::KeyNotFound("base".to_string()))
    }
  }

  pub fn new(title:String, body:String, author:String) -> Issue{
    let branch = vcs_status::current_branch().unwrap_or("<unknown>".to_string());
    Issue{
      base: IssueBase{
        title:title,
        author:author,
        id:generate_id(),
        creation_time:SerdeTime(time::now()),
        branch:branch,
        body_text: body
      },
      events:vec!(),
      status:IssueStatus::default()
    }
  }

}

impl IssueTag{
  pub fn new(name:String, author:String, enabled:bool) -> IssueTag{
    IssueTag{time:SerdeTime(time::now()), author:author, enabled:enabled,
             tag_name:name, change_id:generate_id()}
  }
}

impl IssueComment{
  pub fn new(author:String, body:String) -> IssueComment{
    let branch = vcs_status::current_branch().unwrap_or("<unknown>".to_string());
    IssueComment{author:author, body_text:body, creation_time:SerdeTime(time::now()),
                  branch: branch, id:generate_id()}
  }
}

impl IssueTimelineEvent{
  pub fn event_type(&self) -> String {
    match self {
      &TimelineComment(_) => "comment",
      &TimelineTag(_) => "tag"
    }.to_string()
  }

  pub fn time<'x>(&'x self) -> &'x time::Tm {
    match self {
      &TimelineComment(ref comment) => &comment.creation_time.0,
      &TimelineTag(ref tag) => &tag.time.0
    }
  }

  pub fn id<'x>(&'x self) -> &'x str {
    match self {
      &TimelineComment(ref comment) => comment.id.as_str(),
      &TimelineTag(ref tag) => tag.change_id.as_str()
    }
  }
}

impl IssueStatus{
  pub fn default() -> IssueStatus{
    IssueStatus{name:DEFAULT_STATUS_NAME.to_string(), last_change_time:SerdeTime(time::now())}
  }
}

pub fn generate_id() -> String {
  // [id, todo] Make this generate a proper unique id
  let ctime = time::get_time();
  format!("{}{}", ctime.sec, ctime.nsec)
}

fn json_time(time:&time::Tm) -> json::Json {
  json::Json::String(time::strftime(TIME_FORMAT, time).unwrap().to_string())
}

#[test]
pub fn issue_equality(){
  let i1 = Issue::new("A".to_string(), "B".to_string(), "C".to_string());
  let mut i2 = Issue::new("X".to_string(), "Y".to_string(), "Z".to_string());
  i2.base.id = i1.id().to_string();  //hackery because ids are generated by Issue::new
  let i3 = Issue::new("D".to_string(), "E".to_string(), "F".to_string());
  //identify by ids
  assert!(i1 == i2);
  assert!(i2 != i3);
}

#[test]
pub fn write_and_read_issue_json(){
  let title = "Foo".to_string();
  let body = "Body".to_string();
  let author = "Author".to_string();

  let issue = Issue::new(title.to_string(), 
                         body.to_string(),
                         author.to_string());

  let json = issue.no_comment_json();
  println!("{:?}", json);
  let read_result = Issue::from_json(json);

  println!("{:?}", read_result);
  assert!(read_result.is_ok());

  let read_issue = read_result.unwrap();

  assert!(read_issue == issue);
  assert!(read_issue.title() == title);
  assert!(read_issue.author() == author);
  assert!(read_issue.id() == issue.id());
  assert!(time::strftime(TIME_FORMAT, &read_issue.creation_time()) == 
          time::strftime(TIME_FORMAT, &issue.creation_time()));
}
