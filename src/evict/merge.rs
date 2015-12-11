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
use std::collections::HashMap;
use issue::{Issue,IssueTimelineEvent};
use std::vec::Vec;

pub fn merge_issues(incoming:Vec<Issue>,merge_into:Vec<Issue>) -> Vec<Issue> {
  let mut ident_map:HashMap<String, (Option<Issue>, Option<Issue>)> = HashMap::new();
  for issue in incoming.into_iter() {
    ident_map.insert(issue.id().to_string(), (Some(issue), None));
  }
  for issue in merge_into.into_iter() {
    match ident_map.remove(issue.id()) {
      Some((i, _)) => ident_map.insert(issue.id().to_string(), (i, Some(issue))),
      None => ident_map.insert(issue.id().to_string(), (None, Some(issue)))
    };
  }
  let mut merged:Vec<Issue> = vec!();
  merged.reserve(ident_map.len());

  for (_, value) in ident_map.into_iter() {
    merged.push(merge_pair(value));
  }
  merged
}

fn merge_pair(issues:(Option<Issue>, Option<Issue>)) -> Issue {
  let (incoming_opt, merge_into_opt) = issues;
  if incoming_opt.is_some() && merge_into_opt.is_some() {
    let incoming = incoming_opt.unwrap();
    let merge_into = merge_into_opt.unwrap();
    let new_events = merge_events(incoming.events.clone(),
                                  merge_into.events.clone());

    let incomingTime = incoming.status.last_change_time.0.to_timespec();
    let intoTime = incoming.status.last_change_time.0.to_timespec();
    let status = if incomingTime > intoTime {
                      incoming.status.clone()
                 } else {
                      merge_into.status.clone()
                 };
    Issue{events:new_events, status:status, .. incoming}
  }else if incoming_opt.is_some() {
    incoming_opt.unwrap()
  }else{
    merge_into_opt.unwrap()
  }
}

fn merge_events(mut incoming:Vec<IssueTimelineEvent>,
                merge_into:Vec<IssueTimelineEvent>) -> Vec<IssueTimelineEvent> {
  incoming.extend(merge_into.into_iter());
  let mut merged:Vec<IssueTimelineEvent> = vec!();
  while incoming.len() > 0 {
    match incoming.iter().min_by_key(|ievt| ievt.time().to_timespec())
                         .and_then(|minimum| incoming.iter().position(|x| x == minimum)) {
      Some(pos) => {
        //unwrap here is fine, we know pos is within index range.
        merged.push(incoming.swap_remove(pos));
      }
      None => {}
    }
  }
  merged.dedup();
  merged
}

