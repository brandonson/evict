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
use issue::{Issue,IssueComment};
use extra::sort;
use extra::time;
use std::util::swap;

priv enum TimeSorted{
  TimeSortedIssue(~Issue),
  TimeSortedComment(~IssueComment)
}

impl TimeSorted{
  fn creation<'x>(&'x self) -> &'x time::Tm {
    match self {
      &TimeSortedIssue(ref issue) => &issue.creationTime,
      &TimeSortedComment(ref comment) => &comment.creationTime
    }
  }
  
  fn unwrap_to_issue(self) -> ~Issue {
    match self {
      TimeSortedIssue(i) => i,
      _ => fail!("Tried to get issue from something that wasn't a TimeSortedIssue")
    }
  }

  fn unwrap_to_comment(self) -> ~IssueComment{
    match self {
      TimeSortedComment(c) => c,
      _ => fail!("Tried to get comment from something that wasn't a TimeSortedComment")
    }
  }
}

impl Ord for TimeSorted{
  fn lt(&self, other:&TimeSorted) -> bool{
    (*self).creation().to_timespec().lt(&(*other).creation().to_timespec())
  }
}

impl Eq for TimeSorted{
  fn eq(&self, other:&TimeSorted) -> bool {
    (*self).creation().to_timespec() == (*other).creation().to_timespec()
  }
}

fn sort_le(a:&TimeSorted, b:&TimeSorted) -> bool {
  a.le(b)
}

pub fn sort_by_time(issues:~[~Issue]) -> ~[~Issue]{
  let mut wrapped:~[TimeSorted] = 
                             issues.move_iter().map(|x| TimeSortedIssue(x)).collect();

  sort::quick_sort(wrapped, sort_le);
  
  let mut sorted:~[~Issue] = wrapped.move_iter().map(|x| x.unwrap_to_issue()).collect();
  
  for x in sorted.mut_iter() {
    let mut comments:~[~IssueComment] = ~[];
    swap(&mut comments, &mut x.comments);
    
    let mut wrappedComments:~[TimeSorted] = comments.move_iter().map(|x| TimeSortedComment(x)).collect();
    sort::quick_sort(wrappedComments, sort_le);
    comments = wrappedComments.move_iter().map(|x| x.unwrap_to_comment()).collect();
    swap(&mut comments, &mut x.comments);
  }
  sorted
}

