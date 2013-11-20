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
use extra::{json, treemap, time};
use evict;
use vcs_status;
use extra::json::ToJson;
use status_storage::DEFAULT_STATUS_NAME;
pub static TIME_FORMAT:&'static str = "%F %Y at %T";

pub static BODY_KEY:&'static str = "bodyText";
pub static TIME_KEY:&'static str = "time";
pub static AUTHOR_KEY:&'static str = "author";
pub static TITLE_KEY:&'static str = "title";
pub static ID_KEY:&'static str = "id";
pub static VERSION_KEY:&'static str = "evict-version";
pub static COMMENTS_KEY:&'static str = "comments";
pub static BRANCH_KEY:&'static str = "branch";
pub static STATE_KEY:&'static str = "status";
pub static NAME_KEY:&'static str = "name";
#[deriving(Clone)]
pub struct IssueComment{
  creationTime: time::Tm,
  author:~str,
  bodyText:~str,
  branch:~str,
  id:~str
}
#[deriving(Clone, Eq)]
pub struct IssueStatus{
  name:~str,
  lastChangeTime: time::Tm
}
#[deriving(Clone)]
pub struct Issue{
  title:~str,
  creationTime: time::Tm,
  author:~str,

  bodyText:~str,
  id:~str,
  comments:~[~IssueComment],
  branch:~str,
  status:~IssueStatus
}

impl Eq for IssueComment{
  fn eq(&self, other:&IssueComment) -> bool {
    return self.author == other.author
           && self.bodyText == other.bodyText;
  }
}

impl Eq for Issue{
  fn eq(&self, other:&Issue) -> bool {
    return self.id == other.id;
  }
}

impl IssueStatus{
  pub fn new(name:~str) -> IssueStatus {
    IssueStatus{name:name, lastChangeTime:time::now()}
  }
}

fn get_string_for_key(map:&json::Object, key:&str) -> Option<~str>{
  let valueOpt = map.find(&key.to_owned());
  do valueOpt.and_then |value| {
    match value {
      &json::String(ref strVal) => Some(strVal.to_owned()),
      _ => None
    }
  }
}

impl Issue{

  pub fn add_comment(&mut self, comment:~IssueComment) {
    self.comments.push(comment)
  }

  pub fn to_json(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(VERSION_KEY.to_owned(), json::String(evict::CURRENT_VERSION.to_str()));
    map.insert(TITLE_KEY.to_owned(), json::String(self.title.to_owned()));
    map.insert(BODY_KEY.to_owned(), json::String(self.bodyText.to_owned()));
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.creationTime)));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(ID_KEY.to_owned(), json::String(self.id.to_owned()));
    map.insert(COMMENTS_KEY.to_owned(), json::List(do self.comments.map |c| {c.to_json()}));
    map.insert(BRANCH_KEY.to_owned(), json::String(self.branch.to_owned()));
    map.insert(STATE_KEY.to_owned(), self.status.to_json());
    json::Object(map)
  }

  pub fn no_comment_json(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(VERSION_KEY.to_owned(), json::String(evict::CURRENT_VERSION.to_str()));
    map.insert(TITLE_KEY.to_owned(), json::String(self.title.to_owned()));
    map.insert(BODY_KEY.to_owned(), json::String(self.bodyText.to_owned()));
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.creationTime)));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(ID_KEY.to_owned(), json::String(self.id.to_owned()));
    map.insert(BRANCH_KEY.to_owned(), json::String(self.branch.to_owned()));
    map.insert(STATE_KEY.to_owned(), self.status.to_json());
    json::Object(map)
  }

  pub fn from_json(json:&json::Json) -> Option<~Issue> {
    match json {
      &json::Object(ref map) => Issue::read_from_map(map.clone()),
      _ => None
    }
  }

  fn read_from_map(map:~json::Object) -> Option<~Issue>{
    let versionOpt = get_string_for_key(map, VERSION_KEY);
    let version:int = if(versionOpt.is_none()){
                    fail!("No version on json for an issue.");
                  }else{
                    from_str::<int>(versionOpt.unwrap()).unwrap()
		  };
    if (version == 1) {
      let titleOpt = get_string_for_key(map, TITLE_KEY);
      do titleOpt.and_then |title| {
        let bodyOpt = get_string_for_key(map, BODY_KEY);
        do bodyOpt.and_then |body| {
          let authorOpt = get_string_for_key(map, AUTHOR_KEY);
          do authorOpt.and_then |author| {
            let branchOpt = get_string_for_key(map, BRANCH_KEY);
	    do branchOpt.and_then |branch| {
              let idOpt = get_string_for_key(map, ID_KEY);
              do idOpt.and_then |id| {
                let comments = map.find(&COMMENTS_KEY.to_owned()).map_default(~[],
                                                                      Issue::load_comments);
		let status = do map.find(&STATE_KEY.to_owned())
                                  .map_default(IssueStatus::default()) |json| {
		  IssueStatus::from_json(json)
                };
                let timeOpt = get_string_for_key(map, TIME_KEY);
                do timeOpt.and_then |time| {
                  let timeResult = time::strptime(time,TIME_FORMAT);
                  match timeResult {
                    Ok(tm) => Some(~Issue{title:title.clone(), bodyText:body.clone(), 
                                        author:author.clone(), 
                                        creationTime:tm, id:id.clone(),
                                        comments:comments.clone(),
                                        branch:branch.clone(), status:~status.clone()}),
                    Err(_) => None
                  }
                }
              }
    	    }
          }
        }
      }
    }else{
      None
    }
  }

  fn load_comments(json:&json::Json) -> ~[~IssueComment] {
    match *json {
      json::List(ref list) => {
	                         let commentJsonOpts = list.clone();
                                 let mut commentJson = commentJsonOpts.map(
                                                                   IssueComment::from_json);
                                 commentJson.retain(|comment| {comment.is_some()});
                                 commentJson.map(|comment| {comment.clone().unwrap()})
                               }
      _ => ~[]
    }
  }

  pub fn new(title:~str, body:~str, author:~str) -> ~Issue{
    let branch = vcs_status::current_branch().unwrap_or(~"<unknown>");
    ~Issue{title:title,
           bodyText:body,
           author:author,
           id:generate_id(),
           creationTime:time::now(),
           comments:~[],
           branch:branch,
           status:~IssueStatus::default()}
  }

}

impl IssueComment{
  pub fn to_json(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(BODY_KEY.to_owned(), json::String(self.bodyText.to_owned()));
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.creationTime)));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(BRANCH_KEY.to_owned(), json::String(self.branch.to_owned()));
    map.insert(ID_KEY.to_owned(), json::String(self.id.to_owned()));
    json::Object(map) 
  }
  
  pub fn from_json(json:&json::Json) -> Option<~IssueComment> {
    match json {
      &json::Object(ref map) => IssueComment::read_from_map(map.clone()),
      _ => None
    }
  }

  fn read_from_map(map:~json::Object) -> Option<~IssueComment> {
    let bodyOpt = get_string_for_key(map, BODY_KEY);
    do bodyOpt.and_then |body| {
      let authorOpt = get_string_for_key(map, AUTHOR_KEY);
      do authorOpt.and_then |author| {
        let branchOpt = get_string_for_key(map, BRANCH_KEY);
	do branchOpt.and_then |branch| {
          let timeOpt = get_string_for_key(map, TIME_KEY);
          do timeOpt.and_then |time| {
            let timeResult = time::strptime(time,TIME_FORMAT);
            match timeResult {
              Ok(tm) => Some(~IssueComment{bodyText:body.clone(),
                                    author:author.clone(),
                                    creationTime:tm,
                                    branch:branch.clone(),
                                    id:get_string_for_key(map, ID_KEY)
                                          .unwrap_or(generate_id())}),
              Err(_) => None
            }
          }
        }
      }
    }
  }
  
  pub fn new(author:~str, body:~str) -> ~IssueComment{
    let branch = vcs_status::current_branch().unwrap_or(~"<unknown>");
    ~IssueComment{author:author, bodyText:body, creationTime:time::now(),
                  branch: branch, id:generate_id()}
  }
}

impl json::ToJson for IssueStatus{
  fn to_json(&self) -> json::Json {
    let mut map:~treemap::TreeMap<~str, json::Json> = ~treemap::TreeMap::new();
    map.insert(NAME_KEY.to_owned(), self.name.to_json());
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.lastChangeTime)));
    json::Object(map)
  }
}
impl IssueStatus{
  pub fn from_json(json:&json::Json) -> IssueStatus {
    match json {
      &json::Object(ref mapRef) => {
        let map = mapRef.clone();
        do get_string_for_key(map, NAME_KEY).and_then |name| {
          do get_string_for_key(map, TIME_KEY).and_then |time| {
            match time::strptime(time, TIME_FORMAT) {
              Ok(tm) => Some(IssueStatus{name:name.clone(), lastChangeTime:tm}),
              Err(_) => None
            }
          }
        }.unwrap_or(IssueStatus::default())
      }
      _ => IssueStatus::default()
    }
  }

  pub fn default() -> IssueStatus{
    IssueStatus{name:DEFAULT_STATUS_NAME.to_owned(), lastChangeTime:time::empty_tm()}
  }
}

pub fn generate_id() -> ~str {
  let cTime = time::get_time();
  cTime.sec.to_str() + cTime.nsec.to_str()
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

  let readResult = Issue::from_json(&json);

  assert!(readResult.is_some());

  let readIssue = readResult.unwrap();

  assert!(readIssue == issue);
  assert!(readIssue.title == title);
  assert!(readIssue.author == author);
  assert!(readIssue.bodyText == body);
  assert!(readIssue.id == issue.id);
  assert!(time::strftime(TIME_FORMAT, &readIssue.creationTime) == 
          time::strftime(TIME_FORMAT, &issue.creationTime));
}
