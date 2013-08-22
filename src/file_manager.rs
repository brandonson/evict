use extra;
use issue::Issue;
use file_util;
use extra::json;
#[cfg(not(test))]
pub static EVICT_DIRECTORY:&'static str = ".evict/";
#[cfg(test)]
pub static EVICT_DIRECTORY:&'static str = ".evict-test/";

static EXTENSION:&'static str = ".ebtd";

static LOCAL_EXT:&'static str = ".ebtdlocal";

static ACTIVE_ISSUE_FILENAME_PART:&'static str = "issues";
static LOCAL_ISSUE_SUFFIX:&'static str = "-local";

pub fn activeIssueFilename() -> ~str {fmt!("%s%s%s",EVICT_DIRECTORY, 
                                                ACTIVE_ISSUE_FILENAME_PART, 
                                                EXTENSION)}

pub fn localIssueFilename (branchname:&str) -> ~str { 
  fmt!("%s%s%s%s", EVICT_DIRECTORY,
                 branchname,
                 LOCAL_ISSUE_SUFFIX,
                 LOCAL_EXT)
}  

pub fn committableIssueFilename(branchname:&str) -> ~str {
  fmt!("%s%s%s", EVICT_DIRECTORY, branchname, LOCAL_EXT)
}

pub fn writeCommittableIssues(branchname:&str, issues:&[~Issue]) -> bool {
  writeIssuesToFile(issues, committableIssueFilename(branchname), true)
}

pub fn writeLocalIssues(branchname:&str, issues:&[~Issue]) -> bool {
  writeIssuesToFile(issues, localIssueFilename(branchname), true)
}

pub fn commitIssues(issues:&[~Issue]) -> bool {
  writeIssuesToFile(issues, activeIssueFilename(), true)
}

pub fn writeIssuesToFile(issues:&[~Issue], filename:&str, overwrite:bool) -> bool {
  let jsonList = do issues.map |issue| {issue.getJson()};
  let strval = json::to_pretty_str(&json::List(jsonList));
  file_util::writeStringToFile(strval, filename, overwrite)
}

pub fn readCommittableIssues(branchname:&str) -> ~[~Issue] {
  readIssuesFromFile(committableIssueFilename(branchname))
}

pub fn readLocalIssues(branchname:&str) -> ~[~Issue] {
  readIssuesFromFile(localIssueFilename(branchname))
}

pub fn readCommittedIssues() -> ~[~Issue] {
  readIssuesFromFile(activeIssueFilename())
}

pub fn readIssuesFromFile(filename:&str) -> ~[~Issue] {
  let strvalOpt = file_util::readStringFromFile(filename);
  match strvalOpt{
    Some(strval) => readIssuesFromString(strval),
    None => ~[]
  }
}

fn readIssuesFromString(strval:&str) -> ~[~Issue] {
  let json = extra::json::from_str(strval);
  match json {
    Ok(jsonVal) => readIssuesFromJson(jsonVal),
    Err(_) => ~[]
  }
}

fn readIssuesFromJson(json:extra::json::Json) -> ~[~Issue] {
  match json {
    extra::json::List(ref jsonVals) => 
      do jsonVals.iter().filter_map |jsval| {
        Issue::fromJson(jsval)
      }.collect(),
    _ => ~[]
  }
}

#[test]
pub fn writeReadIssueFile(){
  let testName = ~"writeReadIssueFileTest";
  let issues = ~[Issue::new(~"A", ~"B", ~"C", ~"D")];
  writeIssuesToFile(issues, testName, false);
  let read = readIssuesFromFile(testName);
  file_util::deleteFile(testName);
  assert!(issues == read);
}
