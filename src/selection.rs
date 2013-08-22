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
