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

#[deriving(Clone, Eq)]
pub struct IssueComment{
  creation_time: time::Tm,
  author:~str,
  body_text:~str,
  branch:~str,
  id:~str
}

#[deriving(Clone, Eq)]
pub struct IssueTag{
  time: time::Tm,
  tag_name: ~str,
  enabled: bool,
  author: ~str,
  change_id: ~str
}

#[deriving(Clone, Eq)]
pub enum IssueTimelineEvent{
  TimelineComment(IssueComment),
  TimelineTag(IssueTag)
}

#[deriving(Clone, Eq)]
pub struct IssueStatus{
  name:~str,
  last_change_time: time::Tm
}

#[deriving(Clone)]
pub struct Issue{
  title:~str,
  creation_time: time::Tm,
  author:~str,

  body_text:~str,
  id:~str,
  events:~[IssueTimelineEvent],
  branch:~str,
  status:IssueStatus
}


impl Eq for Issue{
  fn eq(&self, other:&Issue) -> bool {
    return self.id == other.id;
  }
}

impl IssueStatus{
  pub fn new(name:~str) -> IssueStatus {
    IssueStatus{name:name, last_change_time:time::now()}
  }
}

fn get_string_for_key(map:&json::Object, key:&str) -> Option<~str>{
  let value_opt = map.find(&key.to_owned());
  value_opt.and_then (|value| {
    match value {
      &json::String(ref strVal) => Some(strVal.to_owned()),
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

  pub fn to_json(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(VERSION_KEY.to_owned(), json::String(evict::CURRENT_VERSION.to_str()));
    map.insert(TITLE_KEY.to_owned(), json::String(self.title.to_owned()));
    map.insert(BODY_KEY.to_owned(), json::String(self.body_text.to_owned()));
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.creation_time)));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(ID_KEY.to_owned(), json::String(self.id.to_owned()));
    map.insert(I_EVENT_KEY.to_owned(), json::List(self.events.map (|c| {c.to_json()})));
    map.insert(BRANCH_KEY.to_owned(), json::String(self.branch.to_owned()));
    map.insert(STATE_KEY.to_owned(), self.status.to_json());
    json::Object(map)
  }

  pub fn no_comment_json(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(VERSION_KEY.to_owned(), json::String(evict::CURRENT_VERSION.to_str()));
    map.insert(TITLE_KEY.to_owned(), json::String(self.title.to_owned()));
    map.insert(BODY_KEY.to_owned(), json::String(self.body_text.to_owned()));
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.creation_time)));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(ID_KEY.to_owned(), json::String(self.id.to_owned()));
    map.insert(I_EVENT_KEY.to_owned(), json::List(self.events.map (|c| {c.to_json()})));
    map.insert(BRANCH_KEY.to_owned(), json::String(self.branch.to_owned()));
    map.insert(STATE_KEY.to_owned(), self.status.to_json());
    json::Object(map)
  }

  pub fn from_json(json:&json::Json) -> Option<Issue> {
    match json {
      &json::Object(ref map) => Issue::read_from_map(*map),
      _ => None
    }
  }

  fn read_from_map(map:&json::Object) -> Option<Issue>{
    let version_opt = get_string_for_key(map, VERSION_KEY);
    let version:int = if version_opt.is_none() {
                    fail!("No version on json for an issue.");
                  }else{
                    from_str::<int>(version_opt.unwrap()).unwrap()
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
                let events = map.find(&I_EVENT_KEY.to_owned()).map_or(~[],
                                                                      Issue::load_events);
		let status = map.find(&STATE_KEY.to_owned())
                                  .map_or(IssueStatus::default(), |json| {
		  IssueStatus::from_json(json)
                });
                let time_opt = get_string_for_key(map, TIME_KEY);
                time_opt.and_then (|time| {
                  let timeResult = time::strptime(time,TIME_FORMAT);
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

  fn load_events(json:&json::Json) -> ~[IssueTimelineEvent] {
    match *json {
      json::List(ref list) => {
        let eventJson_opts = list.clone();
        let mut eventJson = eventJson_opts.map(IssueTimelineEvent::from_json);
        eventJson.retain(|e| {e.is_some()});
        eventJson.map(|e| {e.clone().unwrap()})
      }
      _ => ~[]
    }
  }

  pub fn new(title:~str, body:~str, author:~str) -> Issue{
    let branch = vcs_status::current_branch().unwrap_or(~"<unknown>");
    Issue{title:title,
           body_text:body,
           author:author,
           id:generate_id(),
           creation_time:time::now(),
           events:~[],
           branch:branch,
           status:IssueStatus::default()}
  }

}

impl json::ToJson for IssueTag{
  fn to_json(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(TIME_KEY.to_owned(), json_time(&self.time));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(NAME_KEY.to_owned(), json::String(self.tag_name.to_owned()));
    map.insert(ENABLED_KEY.to_owned(), json::Boolean(self.enabled));
    map.insert(ID_KEY.to_owned(), json::String(self.change_id.to_owned()));
    json::Object(map)
  }
}

impl IssueTag{
  pub fn from_json(json:&json::Json) -> Option<IssueTag> {
    match json {
      &json::Object(ref map) => IssueTag::read_from_map(*map),
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
              let timeResult = time::strptime(timeStr, TIME_FORMAT);
              match timeResult {
                Ok(time) => 
                  Some(IssueTag{time:time,
                                author:author.to_owned(),
                                enabled:enabled,
                                change_id:id.to_owned(),
                                tag_name:tname.to_owned()}),
                _ => None
              }
            })
          })
        })
      })
    })
  }
  
  fn read_enabled(map:&json::Object) -> Option<bool> {
    let e_opt = map.find(&ENABLED_KEY.to_owned());
    e_opt.and_then(|json| {
      match json {
        &json::Boolean(b) => Some(b),
        _ => None
      }
    })
  }

  pub fn new(name:~str, author:~str, enabled:bool) -> IssueTag{
    IssueTag{time:time::now(), author:author, enabled:enabled,
             tag_name:name, change_id:generate_id()}
  }
}

impl json::ToJson for IssueComment{
  fn to_json(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(BODY_KEY.to_owned(), json::String(self.body_text.to_owned()));
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.creation_time)));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(BRANCH_KEY.to_owned(), json::String(self.branch.to_owned()));
    map.insert(ID_KEY.to_owned(), json::String(self.id.to_owned()));
    json::Object(map) 
  }
}

impl IssueComment{
  pub fn from_json(json:&json::Json) -> Option<IssueComment> {
    match json {
      &json::Object(ref map) => IssueComment::read_from_map(map.clone()),
      _ => None
    }
  }
  
  fn read_from_map(map:~json::Object) -> Option<IssueComment> {
    let body_opt = get_string_for_key(map, BODY_KEY);
    body_opt.and_then (|body| {
      let author_opt = get_string_for_key(map, AUTHOR_KEY);
      author_opt.and_then (|author| {
        let branch_opt = get_string_for_key(map, BRANCH_KEY);
	branch_opt.and_then (|branch| {
          let time_opt = get_string_for_key(map, TIME_KEY);
          time_opt.and_then (|time| {
            let time_result = time::strptime(time,TIME_FORMAT);
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
  
  pub fn new(author:~str, body:~str) -> IssueComment{
    let branch = vcs_status::current_branch().unwrap_or(~"<unknown>");
    IssueComment{author:author, body_text:body, creation_time:time::now(),
                  branch: branch, id:generate_id()}
  }
}

impl json::ToJson for IssueTimelineEvent{
  fn to_json(&self) -> json::Json {
    let mut data:~[json::Json] = ~[];
    data.push(json::String(self.event_type()));
    data.push(self.event_data());
    json::List(data)
  }
}

impl IssueTimelineEvent{
  pub fn event_type(&self) -> ~str {
    match self {
      &TimelineComment(_) => ~"comment",
      &TimelineTag(_) => ~"tag"
    }
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
    let mut map:~treemap::TreeMap<~str, json::Json> = ~treemap::TreeMap::new();
    map.insert(NAME_KEY.to_owned(), self.name.to_json());
    map.insert(TIME_KEY.to_owned(), json_time(&self.last_change_time));
    json::Object(map)
  }
}

impl IssueStatus{
  pub fn from_json(json:&json::Json) -> IssueStatus {
    match json {
      &json::Object(ref map_ref) => {
        let map = map_ref.clone();
        get_string_for_key(map, NAME_KEY).and_then (|name| {
          get_string_for_key(map, TIME_KEY).and_then (|time| {
            match time::strptime(time, TIME_FORMAT) {
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
    IssueStatus{name:DEFAULT_STATUS_NAME.to_owned(), last_change_time:time::empty_tm()}
  }
}

pub fn generate_id() -> ~str {
  let ctime = time::get_time();
  ctime.sec.to_str() + ctime.nsec.to_str()
}

fn json_time(time:&time::Tm) -> json::Json {
  json::String(time::strftime(TIME_FORMAT, time))
}

#[test]
pub fn issueEquality(){
  let i1 = Issue::new(~"A", ~"B", ~"C");
  let mut i2 = Issue::new(~"X", ~"Y", ~"Z");
  i2.id = i1.id.clone();  //hackery because ids are generated by Issue::new
  let i3 = Issue::new(~"D", ~"E", ~"F");
  //identify by ids
  assert!(i1 == i2);
  assert!(i2 != i3);
}

#[test]
pub fn writeReadIssue(){
  let title = ~"Foo";
  let body = ~"Body";
  let author = ~"Author";

  let issue = Issue::new(title.to_owned(), 
                          body.to_owned(),
			  author.to_owned());

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
