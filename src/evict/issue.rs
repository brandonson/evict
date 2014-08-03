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

use time;
use collections::treemap;
use evict;
use vcs_status;
use status_storage::DEFAULT_STATUS_NAME;

pub static TIME_FORMAT:&'static str = "%F %Y at %T";

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

#[deriving(Clone, PartialEq)]
pub struct IssueComment{
  pub creation_time: time::Tm,
  pub author:String,
  pub body_text:String,
  pub branch:String,
  pub id:String
}

#[deriving(Clone, PartialEq)]
pub struct IssueTag{
  pub time: time::Tm,
  pub tag_name: String,
  pub enabled: bool,
  pub author: String,
  pub change_id: String
}

#[deriving(Clone, PartialEq)]
pub enum IssueTimelineEvent{
  TimelineComment(IssueComment),
  TimelineTag(IssueTag)
}

#[deriving(Clone, PartialEq)]
pub struct IssueStatus{
  pub name:String,
  pub last_change_time: time::Tm
}

#[deriving(Clone)]
pub struct Issue{
  pub title:String,
  pub creation_time: time::Tm,
  pub author:String,

  pub body_text:String,
  pub id:String,
  pub events:Vec<IssueTimelineEvent>,
  pub branch:String,
  pub status:IssueStatus
}


impl PartialEq for Issue{
  fn eq(&self, other:&Issue) -> bool {
    return self.id == other.id;
  }
}

impl IssueStatus{
  pub fn new(name:String) -> IssueStatus {
    IssueStatus{name:name, last_change_time:time::now()}
  }
}

fn get_string_for_key(map:&json::Object, key:&str) -> Option<String>{
  let value_opt = map.find(&key.to_string().into_string());
  value_opt.and_then (|value| {
    match value {
      &json::String(ref strVal) => Some(strVal.to_string()),
      _ => None
    }
  })
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
          if tag.tag_name.as_slice() == name {
            if recent.is_none() {
              recent = Some(tag);
            }else{
              let old_tag = recent.take_unwrap();
              if old_tag.time.to_timespec() < tag.time.to_timespec() {
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

  pub fn to_json(&self) -> json::Json {
    let mut map:json::Object = treemap::TreeMap::new();
    map.insert(VERSION_KEY.to_string().into_string(), json::String(evict::CURRENT_VERSION.to_string().into_string()));
    map.insert(TITLE_KEY.to_string().into_string(), json::String(self.title.to_string().into_string()));
    map.insert(BODY_KEY.to_string().into_string(), json::String(self.body_text.to_string().into_string()));
    map.insert(TIME_KEY.to_string().into_string(), 
               json::String(time::strftime(TIME_FORMAT, &self.creation_time).to_string().into_string()));
    map.insert(AUTHOR_KEY.to_string().into_string(), json::String(self.author.to_string().into_string()));
    map.insert(ID_KEY.to_string().into_string(), json::String(self.id.to_string().into_string()));


    let mut event_json = self.events.iter().map(|c| c.to_json());

    map.insert(I_EVENT_KEY.to_string().into_string(), json::List(event_json.collect()));
    map.insert(BRANCH_KEY.to_string().into_string(), json::String(self.branch.to_string().into_string()));
    map.insert(STATE_KEY.to_string().into_string(), self.status.to_json());
    json::Object(map)
  }

  pub fn no_comment_json(&self) -> json::Json {
    let mut map:json::Object = treemap::TreeMap::new();
    map.insert(VERSION_KEY.to_string().into_string(), json::String(evict::CURRENT_VERSION.to_string().into_string()));
    map.insert(TITLE_KEY.to_string().into_string(), json::String(self.title.to_string().into_string()));
    map.insert(BODY_KEY.to_string().into_string(), json::String(self.body_text.to_string().into_string()));
    map.insert(TIME_KEY.to_string().into_string(), 
               json::String(time::strftime(TIME_FORMAT, &self.creation_time).to_string().into_string()));
    map.insert(AUTHOR_KEY.to_string().into_string(), json::String(self.author.to_string().into_string()));
    map.insert(ID_KEY.to_string().into_string(), json::String(self.id.to_string().into_string()));
    
    let mut event_json = self.events.iter().map(|c| c.to_json());

    map.insert(I_EVENT_KEY.to_string().into_string(), json::List(event_json.collect()));
    
    map.insert(BRANCH_KEY.to_string().into_string(), json::String(self.branch.to_string().into_string()));
    map.insert(STATE_KEY.to_string().into_string(), self.status.to_json());
    json::Object(map)
  }

  pub fn from_json(json:&json::Json) -> Option<Issue> {
    //reads issue. also sorts so that the events are in order by time
    //this time ordering is necessary for all_tags to work properly

    match json {
      &json::Object(ref map) => Issue::read_from_map(map),
      _ => None
    }.map(|x| ::date_sort::sort_by_time(vec!(x)).pop().unwrap())
    // [code smell] Fix date sorting individual issue events
    // This will fix the line above.  Probably just means splitting out
    // part of date_sort::sort_by_time
  }

  fn read_from_map(map:&json::Object) -> Option<Issue>{
    let version_opt = get_string_for_key(map, VERSION_KEY);
    let version:int = if version_opt.is_none() {
                    fail!("No version on json for an issue.");
                  }else{
                    from_str::<int>(version_opt.unwrap().as_slice()).unwrap()
		  };
    if version == 1 {
      let title_opt = get_string_for_key(map, TITLE_KEY);
      title_opt.and_then (|title| {
        let body_opt = get_string_for_key(map, BODY_KEY);
        body_opt.and_then (|body| {
          let author_opt = get_string_for_key(map, AUTHOR_KEY);
          author_opt.and_then (|author| {
            let branch_opt = get_string_for_key(map, BRANCH_KEY);
            branch_opt.and_then (|branch| {
              let id_opt = get_string_for_key(map, ID_KEY);
              id_opt.and_then (|id| {
                let events = map.find(&I_EVENT_KEY.to_string().into_string()).map_or(vec!(),
                                                                      Issue::load_events);
		let status = map.find(&STATE_KEY.to_string().into_string())
                                  .map_or(IssueStatus::default(), |json| {
		  IssueStatus::from_json(json)
                });
                let time_opt = get_string_for_key(map, TIME_KEY);
                time_opt.and_then (|time| {
                  let timeResult = time::strptime(time.as_slice(),TIME_FORMAT);
                  match timeResult {
                    Ok(tm) => Some(Issue{title:title.clone(), body_text:body.clone(), 
                                        author:author.clone(), 
                                        creation_time:tm, id:id.clone(),
                                        events:events.clone(),
                                        branch:branch.clone(), status:status.clone()}),
                    Err(_) => None
                  }
                })
              })
    	    })
          })
        })
      })
    }else{
      None
    }
  }

  fn load_events(json:&json::Json) -> Vec<IssueTimelineEvent> {
    match *json {
      json::List(ref list) => {
        let eventJson_opts = list.clone();
        eventJson_opts.iter().filter_map(IssueTimelineEvent::from_json).collect()
      }
      _ => vec!() 
    }
  }

  pub fn new(title:String, body:String, author:String) -> Issue{
    let branch = vcs_status::current_branch().unwrap_or("<unknown>".to_string());
    Issue{title:title,
           body_text:body,
           author:author,
           id:generate_id(),
           creation_time:time::now(),
           events:vec!(),
           branch:branch,
           status:IssueStatus::default()}
  }

}

impl json::ToJson for IssueTag{
  fn to_json(&self) -> json::Json {
    let mut map:json::Object = treemap::TreeMap::new();
    map.insert(TIME_KEY.to_string().into_string(), json_time(&self.time));
    map.insert(AUTHOR_KEY.to_string().into_string(), json::String(self.author.to_string().into_string()));
    map.insert(NAME_KEY.to_string().into_string(), json::String(self.tag_name.to_string().into_string()));
    map.insert(ENABLED_KEY.to_string().into_string(), json::Boolean(self.enabled));
    map.insert(ID_KEY.to_string().into_string(), json::String(self.change_id.to_string().into_string()));
    json::Object(map)
  }
}

impl IssueTag{
  pub fn from_json(json:&json::Json) -> Option<IssueTag> {
    match json {
      &json::Object(ref map) => IssueTag::read_from_map(map),
      _ => None
    }
  }
  
  fn read_from_map(map:&json::Object) -> Option<IssueTag> {
    let name_opt = get_string_for_key(map, NAME_KEY);
    name_opt.and_then(|tname| {
      let author_opt = get_string_for_key(map, AUTHOR_KEY);
      author_opt.and_then(|author| {
        let enabled_opt = IssueTag::read_enabled(map);
        enabled_opt.and_then(|enabled| {
          let id_opt = get_string_for_key(map, ID_KEY);
          id_opt.and_then(|id| {
            let time_opt = get_string_for_key(map, TIME_KEY);
            time_opt.and_then(|timeStr| {
              let timeResult = time::strptime(timeStr.as_slice(), TIME_FORMAT);
              match timeResult {
                Ok(time) => 
                  Some(IssueTag{time:time,
                                author:author.to_string(),
                                enabled:enabled,
                                change_id:id.to_string(),
                                tag_name:tname.to_string()}),
                _ => None
              }
            })
          })
        })
      })
    })
  }
  
  fn read_enabled(map:&json::Object) -> Option<bool> {
    let e_opt = map.find(&ENABLED_KEY.to_string().into_string());
    e_opt.and_then(|json| {
      match json {
        &json::Boolean(b) => Some(b),
        _ => None
      }
    })
  }

  pub fn new(name:String, author:String, enabled:bool) -> IssueTag{
    IssueTag{time:time::now(), author:author, enabled:enabled,
             tag_name:name, change_id:generate_id()}
  }
}

impl json::ToJson for IssueComment{
  fn to_json(&self) -> json::Json {
    let mut map:json::Object = treemap::TreeMap::new();
    map.insert(BODY_KEY.to_string().into_string(), json::String(self.body_text.to_string().into_string()));
    map.insert(TIME_KEY.to_string().into_string(), 
               json::String(time::strftime(TIME_FORMAT, &self.creation_time).to_string().into_string()));
    map.insert(AUTHOR_KEY.to_string().into_string(), json::String(self.author.to_string().into_string()));
    map.insert(BRANCH_KEY.to_string().into_string(), json::String(self.branch.to_string().into_string()));
    map.insert(ID_KEY.to_string().into_string(), json::String(self.id.to_string().into_string()));
    json::Object(map) 
  }
}

impl IssueComment{
  pub fn from_json(json:&json::Json) -> Option<IssueComment> {
    match json {
      &json::Object(ref map) => IssueComment::read_from_map(map),
      _ => None
    }
  }
  
  fn read_from_map(map:&json::Object) -> Option<IssueComment> {
    let body_opt = get_string_for_key(map, BODY_KEY);
    body_opt.and_then (|body| {
      let author_opt = get_string_for_key(map, AUTHOR_KEY);
      author_opt.and_then (|author| {
        let branch_opt = get_string_for_key(map, BRANCH_KEY);
	branch_opt.and_then (|branch| {
          let time_opt = get_string_for_key(map, TIME_KEY);
          time_opt.and_then (|time| {
            let time_result = time::strptime(time.as_slice(),TIME_FORMAT);
            match time_result {
              Ok(tm) => Some(IssueComment{body_text:body.clone(),
                                    author:author.clone(),
                                    creation_time:tm,
                                    branch:branch.clone(),
                                    id:get_string_for_key(map, ID_KEY)
                                          .unwrap_or(generate_id())}),
              Err(_) => None
            }
          })
        })
      })
    })
  }
  
  pub fn new(author:String, body:String) -> IssueComment{
    let branch = vcs_status::current_branch().unwrap_or("<unknown>".to_string());
    IssueComment{author:author, body_text:body, creation_time:time::now(),
                  branch: branch, id:generate_id()}
  }
}

impl json::ToJson for IssueTimelineEvent{
  fn to_json(&self) -> json::Json {
    let data:Vec<json::Json> = vec!(json::String(self.event_type().to_string().into_string()),
                                    self.event_data());
    json::List(data)
  }
}

impl IssueTimelineEvent{
  pub fn event_type(&self) -> String {
    match self {
      &TimelineComment(_) => "comment",
      &TimelineTag(_) => "tag"
    }.to_string()
  }

  pub fn event_data(&self) -> json::Json {
    match self {
      &TimelineComment(ref comment) => comment.to_json(),
      &TimelineTag(ref tag) => tag.to_json()
    }
  }

  pub fn from_json(json:&json::Json) -> Option<IssueTimelineEvent> {
    match json {
      &json::List(ref jlist) => {
        if jlist.len() != 2 {
          None
        }else{
          match jlist[0] {
            json::String(ref cmt) if cmt.as_slice() == "comment" => IssueComment::from_json(&jlist[1])
                                                    .map(|x| TimelineComment(x)),
            json::String(ref tg) if tg.as_slice() == "tag" => IssueTag::from_json(&jlist[1])
                                            .map(|x| TimelineTag(x)),
            _ => None
          }
        }
      }
      otherJson => {
        //really old versions had comments only and did not use list format
        IssueComment::from_json(otherJson).map(|x| TimelineComment(x))
      }
    }
  }

  pub fn time<'x>(&'x self) -> &'x time::Tm {
    match self {
      &TimelineComment(ref comment) => &comment.creation_time,
      &TimelineTag(ref tag) => &tag.time
    }
  }

  pub fn id<'x>(&'x self) -> &'x str {
    match self {
      &TimelineComment(ref comment) => comment.id.as_slice(),
      &TimelineTag(ref tag) => tag.change_id.as_slice()
    }
  }
}

impl json::ToJson for IssueStatus{
  fn to_json(&self) -> json::Json {
    let mut map:json::Object = treemap::TreeMap::new();
    map.insert(NAME_KEY.to_string().into_string(), self.name.to_string().into_string().to_json());
    map.insert(TIME_KEY.to_string().into_string(), json_time(&self.last_change_time));
    json::Object(map)
  }
}

impl IssueStatus{
  pub fn from_json(json:&json::Json) -> IssueStatus {
    match json {
      &json::Object(ref map) => {
        get_string_for_key(map, NAME_KEY).and_then (|name| {
          get_string_for_key(map, TIME_KEY).and_then (|time| {
            match time::strptime(time.as_slice(), TIME_FORMAT) {
              Ok(tm) => Some(IssueStatus{name:name.clone(), last_change_time:tm}),
              Err(_) => None
            }
          })
        }).unwrap_or(IssueStatus::default())
      }
      _ => IssueStatus::default()
    }
  }

  pub fn default() -> IssueStatus{
    IssueStatus{name:DEFAULT_STATUS_NAME.to_string(), last_change_time:time::empty_tm()}
  }
}

pub fn generate_id() -> String {
  let ctime = time::get_time();
  ctime.sec.to_string().append(ctime.nsec.to_string().as_slice())
}

fn json_time(time:&time::Tm) -> json::Json {
  json::String(time::strftime(TIME_FORMAT, time).to_string().into_string())
}

#[test]
pub fn issue_equality(){
  let i1 = Issue::new("A".to_string(), "B".to_string(), "C".to_string());
  let mut i2 = Issue::new("X".to_string(), "Y".to_string(), "Z".to_string());
  i2.id = i1.id.clone();  //hackery because ids are generated by Issue::new
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

  let json = issue.to_json();

  let read_result = Issue::from_json(&json);

  assert!(read_result.is_some());

  let read_issue = read_result.unwrap();

  assert!(read_issue == issue);
  assert!(read_issue.title == title);
  assert!(read_issue.author == author);
  assert!(read_issue.body_text == body);
  assert!(read_issue.id == issue.id);
  assert!(time::strftime(TIME_FORMAT, &read_issue.creation_time) == 
          time::strftime(TIME_FORMAT, &issue.creation_time));
}
