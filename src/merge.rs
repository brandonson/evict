use std::hashmap::HashMap;
use issue::Issue;
pub fn mergeIssues(incoming:~[~Issue],mergeInto:~[~Issue]) -> ~[~Issue] {
  let mut identMap:HashMap<~str, (Option<~Issue>, Option<~Issue>)> = HashMap::new();
  for issue in incoming.move_iter() {
    identMap.insert(issue.id.to_owned(), (Some(issue), None));
  }
  for issue in mergeInto.move_iter() {
    match identMap.pop(&issue.id) {
      Some((i, _)) => identMap.insert(issue.id.to_owned(), (i, Some(issue))),
      None => identMap.insert(issue.id.to_owned(), (None, Some(issue)))
    };
  }
  let mut merged:~[~Issue] = ~[];
  merged.reserve(identMap.len());

  for (_, value) in identMap.move_iter() {
    merged.push(mergePair(value));
  }
  merged
}

fn mergePair(issues:(Option<~Issue>, Option<~Issue>)) -> ~Issue {
  let (incoming, mergeInto) = issues;
  if(incoming.is_some() && mergeInto.is_some()){
    //TODO actually merge
    incoming.unwrap()
  }else if(incoming.is_some()){
    incoming.unwrap()
  }else{
    mergeInto.unwrap()
  }
}
