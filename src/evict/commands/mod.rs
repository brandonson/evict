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
use std;
use config;
use std::io::{stdin,stdout};
use std::io::buffered::BufferedReader;
use std::io::Writer;

mod init;
mod create;
mod clear;
mod list;
mod delete;
mod comment;
mod new_status;
mod default_author;
mod set_status;
mod default_status;

/* A command takes a list of argument strings,
 * performs some action, then returns an
 * exit code.
 */
type Command = fn (~[~str]) -> int;

pub fn execute_command(command:&~str, 
                      commandList:&~std::container::Map<~str, Command>, 
                      argList: ~[~str]) -> bool{
  match commandList.find(command) {
    Some(cmd) => {let exit = (*cmd)(argList); std::os::set_exit_status(exit); true}
    None => {
     println(format!("Command {} not found", *command)); 
     std::os::set_exit_status(1); 
     false
    } 
  }
}

pub fn standard_commands() -> ~std::container::Map<~str, Command> {
  let mut hmap:~std::hashmap::HashMap<~str, Command> = ~std::hashmap::HashMap::new();
  hmap.insert(~"create", create::create_issue);
  hmap.insert(~"clear", clear::clear_data);
  hmap.insert(~"init", init::initialize);
  hmap.insert(~"list", list::list_issues); 
  hmap.insert(~"delete", delete::delete_issue);
  hmap.insert(~"comment", comment::new_comment); 
  hmap.insert(~"new-status", new_status::new_status);
  hmap.insert(~"default-author", default_author::default_author);
  hmap.insert(~"set-status", set_status::set_status);
  hmap.insert(~"default-status", default_status::default_status);
  hmap as ~std::container::Map<~str, Command>
}

pub fn prompt(prompt:&str) -> ~str{
  let mut out = stdout();
  out.write(prompt.as_bytes());
  out.flush();
  //TODO do we need to check this?
  let withNewline = BufferedReader::new(stdin()).read_line().unwrap();
  withNewline.replace("\n", "").replace("\r", "")
}

pub fn get_author() -> ~str {
  let config = config::Config::load();
  match config.author {
    Some(author) => author,
    None => prompt("Author: ")
  }
}

pub fn edit_file(filename:&str) -> bool{
  match std::os::getenv("EDITOR") {
    Some(editorName) => {
      std::run::process_status(editorName, &[filename.to_owned()]);
      true
    }
    None => false
  }
}
