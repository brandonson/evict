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

priv struct TimeSortedIssue(~Issue);
priv struct TimeSortedComment(~IssueComment);

priv trait CreationTimeTracker {
  fn creation(&self) -> &time::Tm;
}

impl CreationTimeTracker for TimeSortedIssue {
  fn creation(&self) -> &time::Tm {(*self).creationTime}
}
impl CreationTimeTracker for TimeSortedComment{
  fn creation(&self) -> &time::Tm {(*self).creationTime}
}

impl<T:CreationTimeTracker> Ord for T{
  fn lt(&self, other:&T) -> bool{
    (*self).creation().to_timespec().lt(&(*other).creation().to_timespec())
  }
}

impl<T:CreationTimeTracker> Eq for T{
  fn eq(&self, other:&T) -> bool {
    (*self).creation().to_timespec() == (*other).creation().to_timespec()
  }
}

fn ctt_sort_le<T:CreationTimeTracker>(a:&T, b:&T) -> bool {
  a.le(b)
}

pub fn sort_by_time(issues:~[~Issue]) -> ~[~Issue]{
  let mut wrapped:~[TimeSortedIssue] = 
                             issues.move_iter().map(|x| TimeSortedIssue(x)).collect();

  sort::quick_sort(wrapped, ctt_sort_le);
  
  let mut sorted:~[~Issue] = wrapped.move_iter().map(|x| *x).collect();
  
  for x in sorted.mut_iter() {
    sort::quick_sort(x.comments);
  }
  sorted
}

