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
use issue::{Issue, IssueTimelineEvent};
use issue::{TimelineComment, TimelineTag};
use extra::time;
use std::mem::swap;

priv enum TimeSorted{
  TimeSortedIssue(Issue),
  TimeSortedEvent(IssueTimelineEvent)
}

impl TimeSorted{
  fn creation<'x>(&'x self) -> &'x time::Tm {
    match self {
      &TimeSortedIssue(ref issue) => &issue.creation_time,
      &TimeSortedEvent(ref evt) => match evt {
        &TimelineComment(ref comment) => &comment.creation_time,
        &TimelineTag(ref tag) => &tag.time
      }
    }
  }
  
  fn unwrap_to_issue(self) -> Issue {
    match self {
      TimeSortedIssue(i) => i,
      _ => fail!("Tried to get issue from something that wasn't a TimeSortedIssue")
    }
  }

  fn unwrap_to_event(self) -> IssueTimelineEvent{
    match self {
      TimeSortedEvent(e) => e,
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

fn ts_ordering(a:&TimeSorted, b:&TimeSorted) -> Ordering {
  if a.eq(b) {
    Equal
  }else if a.lt(b) {
    Less
  }else{
    Greater
  }
}

pub fn sort_by_time(issues:~[Issue]) -> ~[Issue]{
  let mut wrapped:~[TimeSorted] = 
                             issues.move_iter().map(|x| TimeSortedIssue(x)).collect();

  wrapped.sort_by(ts_ordering);
  
  let mut sorted:~[Issue] = wrapped.move_iter().map(|x| x.unwrap_to_issue()).collect();
  
  for x in sorted.mut_iter() {
    let mut events:~[IssueTimelineEvent] = ~[];
    swap(&mut events, &mut x.events);
    
    let mut wrappedComments:~[TimeSorted] = events.move_iter().map(|x| TimeSortedEvent(x)).collect();
    wrappedComments.sort_by(ts_ordering);
    events = wrappedComments.move_iter().map(|x| x.unwrap_to_event()).collect();
    swap(&mut events, &mut x.events);
  }
  sorted
}

