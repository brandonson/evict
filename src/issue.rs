use extra::{json, treemap, time};
use evict;
use std::int;
use vcs_status;

pub static TIME_FORMAT:&'static str = "%F %Y at %T";

pub static BODY_KEY:&'static str = "bodyText";
pub static TIME_KEY:&'static str = "time";
pub static AUTHOR_KEY:&'static str = "author";
pub static TITLE_KEY:&'static str = "title";
pub static ID_KEY:&'static str = "id";
pub static VERSION_KEY:&'static str = "evict-version";
pub static COMMENTS_KEY:&'static str = "comments";
pub static BRANCH_KEY:&'static str = "branch";

#[deriving(Clone)]
pub struct IssueComment{
  creationTime: time::Tm,
  author:~str,
  bodyText:~str,
  branch:~str
}

#[deriving(Clone)]
pub struct Issue{
  title:~str,
  creationTime: time::Tm,
  author:~str,

  bodyText:~str,
  id:~str,
  comments:~[~IssueComment],
  branch:~str
}

impl Eq for IssueComment{
  fn eq(&self, other:&IssueComment) -> bool {
    return self.author == other.author
           && self.bodyText == other.bodyText;
  }
  fn ne(&self, other:&IssueComment) -> bool {
    !self.eq(other)
  }
}

impl Eq for Issue{
  fn eq(&self, other:&Issue) -> bool {
    return self.id == other.id;
  }
  fn ne(&self, other:&Issue) -> bool {
    !self.eq(other)
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
                let timeOpt = getStringForKey(map, TIME_KEY);
                do timeOpt.chain |time| {
                  let timeResult = time::strptime(time,TIME_FORMAT);
                  match timeResult {
                    Ok(tm) => Some(~Issue{title:title.clone(), bodyText:body.clone(), 
                                        author:author.clone(), 
                                        creationTime:tm, id:id.clone(),
                                        comments:comments.clone(),
                                        branch:branch.clone()}),
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
           comments:~[], branch:branch}
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
