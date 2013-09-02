use issue::Issue;

pub fn findMatchingIssues(idEndPart:&str, searchIn:&[~Issue]) -> ~[~Issue] {
  let mut matching:~[~Issue] = ~[];
  for issue in searchIn.iter() {
    if issue.id.ends_with(idEndPart) {
      matching.push(issue.clone());
    }
  }
  matching
}

pub fn updateIssue(idEndPart:&str, searchIn:~[~Issue],update:&fn(~Issue) -> ~Issue) 
  -> ~[~Issue] {
  let matching  = findMatchingIssues(idEndPart, searchIn);
  if(matching.len() > 1){
    println("Multiple matching issues found:");
    for issue in matching.iter() {
      println(fmt!("%s (%s)", issue.id, issue.title));
    }
    searchIn
  }else{
    let mut filtered:~[~Issue] = searchIn.move_iter()
                                         .filter(|x| x.id != matching[0].id)
                                         .collect();
    filtered.push(update(matching[0]));
    filtered
  }
}

