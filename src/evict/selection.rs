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

pub fn find_matching_issues(idEndPart:&str, searchIn:&[~Issue]) -> ~[~Issue] {
  let mut matching:~[~Issue] = ~[];
  for issue in searchIn.iter() {
    if issue.id.ends_with(idEndPart) {
      matching.push(issue.clone());
    }
  }
  matching
}

pub fn update_issue(idEndPart:&str, searchIn:~[~Issue],update:|~Issue| -> ~Issue)
  -> ~[~Issue] {
  let matching  = find_matching_issues(idEndPart, searchIn);
  if(matching.len() != 1){
    println("Found 0 or >1 matching issues:");
    for issue in matching.iter() {
      println(format!("{} ({})", issue.id, issue.title));
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

