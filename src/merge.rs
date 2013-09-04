use std::hashmap::HashMap;
use issue::{Issue,IssueComment};
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
  let (incomingOpt, mergeIntoOpt) = issues;
  if(incomingOpt.is_some() && mergeIntoOpt.is_some()){
    let incoming = incomingOpt.unwrap();
    let mergeInto = mergeIntoOpt.unwrap();
    let newComments = mergeComments(incoming.comments.clone(), 
                                    mergeInto.comments.clone());
    let status = incoming.status.clone();
    ~Issue{comments:newComments, status:status, .. *incoming}
  }else if(incomingOpt.is_some()){
    incomingOpt.unwrap()
  }else{
    mergeIntoOpt.unwrap()
  }
}

fn mergeComments(incoming:~[~IssueComment], mergeInto:~[~IssueComment]) -> ~[~IssueComment] {
  let mut joined = incoming + mergeInto;
  let mut merged:~[~IssueComment] = ~[];
  while(joined.len() > 0) {
    match joined.iter().min_by(|icomment| icomment.creationTime.to_timespec())
                       .chain(|minimum| joined.position_elem(minimum)) {
      Some(pos) => {
        merged.push(joined.swap_remove(pos));
      }
      None => {}
    }
  }
  merged
}

