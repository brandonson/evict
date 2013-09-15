use issue::Issue;

struct TimeSortedIssue<'self>(&'self Issue);

impl<'self> Ord for TimeSortedIssue<'self>{
  fn lt(&self, other:&TimeSortedIssue) -> bool{
    (*self).creationTime.to_timespec().lt(&(*other).creationTime.to_timespec())
  }
}
impl<'self> Eq for TimeSortedIssue<'self>{
  fn eq(&self, other:&TimeSortedIssue) -> bool {
    *(*self) == *(*other)
  }
}
pub fn sortByTime<'a>(issues:&'a [~Issue]) -> ~[&'a Issue]{
  let mut wrapped:~[TimeSortedIssue<'a>] = 
                             issues.iter().map(|x| TimeSortedIssue(&**x)).collect();

  let mut sorted:~[&'a Issue] = ~[];
  while(wrapped.len() > 0){
    let pos = wrapped.position_elem(wrapped.iter().min().unwrap());
    let timeIssue = wrapped.swap_remove(pos.unwrap());
    sorted.push(*timeIssue);
  }
  sorted
}

