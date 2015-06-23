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

use file_manager;
use source::recursive_parser::parse_directory;
use source::parse::SourceSearcher;
use std::path::Path;
use commands;
use fsm::NextState::*;
use fsm;
use fsm::StateMachine;

struct Flags{
  source_dir:Option<String>
}

pub fn parse_issues(args:Vec<String>) -> isize{
  let mut stateMachine = StateMachine::new(std_handler,
                                           Flags{source_dir:None});

  for argVal in args.into_iter() {
    stateMachine.process(argVal)
  }

  let flags = stateMachine.move_state();

  let author = commands::get_author();
  let result = parse_directory(&SourceSearcher::new_default_searcher(author),
                               Path::new(flags.source_dir.unwrap_or("".to_string())));
  file_manager::write_issues(result.new_issues.as_str());
  for errstr in result.failures.iter(){
    println!("Parser error: {}", errstr);
  }
  0
}

fn std_handler(flags:Flags, input:String) -> fsm::NextState<Flags, String> {
  match input.as_str() {
    "--src-dir" => ChangeState(get_source_dir, flags),
    _ => Continue(flags)
  }
}

fn get_source_dir(_:Flags, input:String) -> fsm::NextState<Flags, String>{
  ChangeState(std_handler, Flags{source_dir:Some(input)})
}
