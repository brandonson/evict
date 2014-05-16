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
use libc;
use config;
use std::io::stdin;
use std::io::BufferedReader;
use collections::hashmap::HashMap;
use std::io::process;

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
mod tag;
mod parse;

/* A command takes a list of argument strings,
 * performs some action, then returns an
 * exit code.
 */
pub type Command = fn (~[~str]) -> int;

pub fn execute_command(command:&~str, 
                      commandList:&HashMap<~str, Command>, 
                      argList: ~[~str]) -> bool{
  match commandList.find(command) {
    Some(cmd) => {let exit = (*cmd)(argList); std::os::set_exit_status(exit); true}
    None => {
     println!("Command {} not found", command); 
     std::os::set_exit_status(1); 
     false
    } 
  }
}

pub fn standard_commands() -> Box<HashMap<~str, Command>> {
  let mut hmap:Box<HashMap<~str, Command>> = box HashMap::new();
  hmap.insert("create".to_owned(), create::create_issue);
  hmap.insert("clear".to_owned(), clear::clear_data);
  hmap.insert("init".to_owned(), init::initialize);
  hmap.insert("list".to_owned(), list::list_issues); 
  hmap.insert("delete".to_owned(), delete::delete_issue);
  hmap.insert("comment".to_owned(), comment::new_comment); 
  hmap.insert("new-status".to_owned(), new_status::new_status);
  hmap.insert("default-author".to_owned(), default_author::default_author);
  hmap.insert("set-status".to_owned(), set_status::set_status);
  hmap.insert("default-status".to_owned(), default_status::default_status);
  hmap.insert("tag".to_owned(), tag::tag);
  hmap.insert("untag".to_owned(), tag::untag);
  hmap.insert("parse".to_owned(), parse::parse_issues);
  
  hmap
}

pub fn prompt(prompt:&str) -> ~str{
  print!("{}", prompt);
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
      let mut editor_command = process::Command::new(editorName.clone());
      editor_command.arg(filename);
      editor_command.stdin(process::InheritFd(libc::STDIN_FILENO));
      editor_command.stdout(process::InheritFd(libc::STDOUT_FILENO));
      editor_command.stderr(process::InheritFd(libc::STDERR_FILENO));

      let editor = editor_command.spawn();

      if editor.is_err() {
        println!("Couldn't launch editor {}", editorName);
        false
      }else{
        let wait_res = editor.ok().unwrap().wait();
        if !wait_res.is_ok() {
          println!("Something went wrong with the editor");
          false
        }else{
          wait_res.ok().unwrap().success()
        }
      }
    }
    None => false
  }
}
