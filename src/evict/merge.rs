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
use std::hashmap::HashMap;
use issue::{Issue,IssueTimelineEvent};

pub fn merge_issues(incoming:~[Issue],mergeInto:~[Issue]) -> ~[Issue] {
  let mut identMap:HashMap<~str, (Option<Issue>, Option<Issue>)> = HashMap::new();
  for issue in incoming.move_iter() {
    identMap.insert(issue.id.to_owned(), (Some(issue), None));
  }
  for issue in mergeInto.move_iter() {
    match identMap.pop(&issue.id) {
      Some((i, _)) => identMap.insert(issue.id.to_owned(), (i, Some(issue))),
      None => identMap.insert(issue.id.to_owned(), (None, Some(issue)))
    };
  }
  let mut merged:~[Issue] = ~[];
  merged.reserve(identMap.len());

  for (_, value) in identMap.move_iter() {
    merged.push(merge_pair(value));
  }
  merged
}

fn merge_pair(issues:(Option<Issue>, Option<Issue>)) -> Issue {
  let (incomingOpt, mergeIntoOpt) = issues;
  if(incomingOpt.is_some() && mergeIntoOpt.is_some()){
    let incoming = incomingOpt.unwrap();
    let mergeInto = mergeIntoOpt.unwrap();
    let new_events = merge_events(incoming.events.clone(), 
                                  mergeInto.events.clone());

    let status = if(incoming.status.lastChangeTime.to_timespec()
                     .gt(&mergeInto.status.lastChangeTime.to_timespec())){
                      incoming.status.clone()
                 } else {
                      mergeInto.status.clone()
                 };
    Issue{events:new_events, status:status, .. incoming}
  }else if(incomingOpt.is_some()){
    incomingOpt.unwrap()
  }else{
    mergeIntoOpt.unwrap()
  }
}

fn merge_events(incoming:~[IssueTimelineEvent],
                  mergeInto:~[IssueTimelineEvent]) -> ~[IssueTimelineEvent] {
  let mut joined = incoming + mergeInto;
  let mut merged:~[IssueTimelineEvent] = ~[];
  while(joined.len() > 0) {
    match joined.iter().min_by(|ievt| ievt.time().to_timespec())
                       .and_then(|minimum| joined.position_elem(minimum)) {
      Some(pos) => {
        merged.push(joined.swap_remove(pos));
      }
      None => {}
    }
  }
  merged.dedup();
  merged
}

