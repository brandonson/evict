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

