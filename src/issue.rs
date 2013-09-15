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
use std::int;
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
  branch:~str
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

fn getStringForKey(map:&json::Object, key:&str) -> Option<~str>{
  let valueOpt = map.find(&key.to_owned());
  do valueOpt.chain |value| {
    match value {
      &json::String(ref strVal) => Some(strVal.to_owned()),
      _ => None
    }
  }
}

impl Issue{

  pub fn addComment(&mut self, comment:~IssueComment) {
    self.comments.push(comment)
  }

  pub fn getJson(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(VERSION_KEY.to_owned(), json::String(evict::CURRENT_VERSION.to_str()));
    map.insert(TITLE_KEY.to_owned(), json::String(self.title.to_owned()));
    map.insert(BODY_KEY.to_owned(), json::String(self.bodyText.to_owned()));
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.creationTime)));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(ID_KEY.to_owned(), json::String(self.id.to_owned()));
    map.insert(COMMENTS_KEY.to_owned(), json::List(do self.comments.map |c| {c.getJson()}));
    map.insert(BRANCH_KEY.to_owned(), json::String(self.branch.to_owned()));
    map.insert(STATE_KEY.to_owned(), self.status.to_json());
    json::Object(map)
  }

  pub fn fromJson(json:&json::Json) -> Option<~Issue> {
    match json {
      &json::Object(ref map) => Issue::readFromMap(map.clone()),
      _ => None
    }
  }

  fn readFromMap(map:~json::Object) -> Option<~Issue>{
    let versionOpt = getStringForKey(map, VERSION_KEY);
    let version:int = if(versionOpt.is_none()){
                    fail!("No version on json for an issue.");
                  }else{
                    int::from_str(versionOpt.unwrap()).unwrap()
		  };
    if (version == 1) {
      let titleOpt = getStringForKey(map, TITLE_KEY);
      do titleOpt.chain |title| {
        let bodyOpt = getStringForKey(map, BODY_KEY);
        do bodyOpt.chain |body| {
          let authorOpt = getStringForKey(map, AUTHOR_KEY);
          do authorOpt.chain |author| {
            let branchOpt = getStringForKey(map, BRANCH_KEY);
	    do branchOpt.chain |branch| {
              let idOpt = getStringForKey(map, ID_KEY);
              do idOpt.chain |id| {
                let comments = map.find(&COMMENTS_KEY.to_owned()).map_default(~[],
                                                                      Issue::loadComments);
		let status = do map.find(&STATE_KEY.to_owned())
                                  .map_default(IssueStatus::default()) |json| {
		  IssueStatus::from_json(*json)
                };
                let timeOpt = getStringForKey(map, TIME_KEY);
                do timeOpt.chain |time| {
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

  fn loadComments(json:& &json::Json) -> ~[~IssueComment] {
    match *json {
      &json::List(ref list) => {
	                         let commentJsonOpts = list.clone();
                                 let mut commentJson = commentJsonOpts.map(
                                                                   IssueComment::fromJson);
                                 commentJson.retain(|comment| {comment.is_some()});
                                 commentJson.map(|comment| {comment.clone().unwrap()})
                               }
      _ => ~[]
    }
  }

  pub fn new(title:~str, body:~str, author:~str, ident:~str) -> ~Issue{
    let branch = vcs_status::currentBranch().unwrap_or_default(~"<unknown>");
    ~Issue{title:title, bodyText:body, author:author, id:ident, creationTime:time::now(),
           comments:~[], branch:branch, status:~IssueStatus::default()}
  }
  pub fn generateId() -> ~str {
    let cTime = time::get_time();
    cTime.sec.to_str() + cTime.nsec.to_str()
  }
}

impl IssueComment{
  pub fn getJson(&self) -> json::Json {
    let mut map:~json::Object = ~treemap::TreeMap::new();
    map.insert(BODY_KEY.to_owned(), json::String(self.bodyText.to_owned()));
    map.insert(TIME_KEY.to_owned(), 
               json::String(time::strftime(TIME_FORMAT, &self.creationTime)));
    map.insert(AUTHOR_KEY.to_owned(), json::String(self.author.to_owned()));
    map.insert(BRANCH_KEY.to_owned(), json::String(self.branch.to_owned()));
    json::Object(map) 
  }
  
  pub fn fromJson(json:&json::Json) -> Option<~IssueComment> {
    match json {
      &json::Object(ref map) => IssueComment::readFromMap(map.clone()),
      _ => None
    }
  }

  fn readFromMap(map:~json::Object) -> Option<~IssueComment> {
    let bodyOpt = getStringForKey(map, BODY_KEY);
    do bodyOpt.chain |body| {
      let authorOpt = getStringForKey(map, AUTHOR_KEY);
      do authorOpt.chain |author| {
        let branchOpt = getStringForKey(map, BRANCH_KEY);
	do branchOpt.chain |branch| {
          let timeOpt = getStringForKey(map, TIME_KEY);
          do timeOpt.chain |time| {
            let timeResult = time::strptime(time,TIME_FORMAT);
            match timeResult {
              Ok(tm) => Some(~IssueComment{bodyText:body.clone(), 
                                    author:author.clone(), 
                                    creationTime:tm, branch:branch.clone()}),
              Err(_) => None
            }
          }
        }
      }
    }
  }
  
  pub fn new(author:~str, body:~str) -> ~IssueComment{
    let branch = vcs_status::currentBranch().unwrap_or_default(~"<unknown>");
    ~IssueComment{author:author, bodyText:body, creationTime:time::now(),
                  branch: branch}
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
        do getStringForKey(map, NAME_KEY).chain |name| {
          do getStringForKey(map, TIME_KEY).chain |time| {
            match time::strptime(time, TIME_FORMAT) {
              Ok(tm) => Some(IssueStatus{name:name.clone(), lastChangeTime:tm}),
              Err(_) => None
            }
          }
        }.unwrap_or_default(IssueStatus::default())
      }
      _ => IssueStatus::default()
    }
  }

  pub fn default() -> IssueStatus{
    IssueStatus{name:DEFAULT_STATUS_NAME.to_owned(), lastChangeTime:time::empty_tm()}
  }
}

#[test]
pub fn issueEquality(){
  let i1 = Issue::new(~"A", ~"B", ~"C", ~"D");
  let i2 = Issue::new(~"X", ~"Y", ~"Z", ~"D");
  let i3 = Issue::new(~"D", ~"E", ~"F", ~"G");
  //identify by ids
  assert!(i1 == i2);
  assert!(i2 != i3);
}

#[test]
pub fn writeReadIssue(){
  let title = ~"Foo";
  let body = ~"Body";
  let author = ~"Author";
  let ident = ~"TestIdent"; //don't generate here, not necessary

  let issue = Issue::new(title.to_owned(), 
                          body.to_owned(),
			  author.to_owned(), 
                          ident.to_owned());

  let json = issue.getJson();

  let readResult = Issue::fromJson(&json);

  assert!(readResult.is_some());

  let readIssue = readResult.unwrap();

  assert!(readIssue == issue);
  assert!(readIssue.title == title);
  assert!(readIssue.author == author);
  assert!(readIssue.bodyText == body);
  assert!(readIssue.id == ident);
  assert!(time::strftime(TIME_FORMAT, &readIssue.creationTime) == 
          time::strftime(TIME_FORMAT, &issue.creationTime));
}
