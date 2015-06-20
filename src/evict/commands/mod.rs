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
use std::io::BufReader;
use std::collections::hash_map::HashMap;
use std::process;

use file_util;
use file_manager;

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
//mod parse;

/* A command takes a list of argument strings,
 * performs some action, then returns an
 * exit code.
 */
pub type Command = fn (Vec<String>) -> isize;

pub fn execute_command(command:&String, 
                      commandList:&HashMap<String, Command>, 
                      argList: Vec<String>) -> bool{
  // [quality] This should be done without hardcoding init as the exception
  if command != &"init".to_string() && 
     !file_util::file_exists(file_manager::EVICT_DIRECTORY) {
    println!("There is no evict directory.  Run evict init.");
    std::env::set_exit_status(2);
    return false;
  }
  match commandList.find(command) {
    Some(cmd) => {let exit = (*cmd)(argList); std::env::set_exit_status(exit); true}
    None => {
     println!("Command {} not found", command); 
     std::env::set_exit_status(1); 
     false
    } 
  }
}

pub fn standard_commands() -> HashMap<String, Command> {
  let mut hmap:HashMap<String, Command> = HashMap::new();
  hmap.insert("create".into_string(), create::create_issue);
  hmap.insert("clear".into_string(), clear::clear_data);
  hmap.insert("init".into_string(), init::initialize);
  hmap.insert("list".into_string(), list::list_issues); 
  hmap.insert("delete".into_string(), delete::delete_issue);
  hmap.insert("comment".into_string(), comment::new_comment); 
  hmap.insert("new-status".into_string(), new_status::new_status);
  hmap.insert("default-author".into_string(), default_author::default_author);
  hmap.insert("set-status".into_string(), set_status::set_status);
  hmap.insert("default-status".into_string(), default_status::default_status);
  hmap.insert("tag".into_string(), tag::tag);
  hmap.insert("untag".into_string(), tag::untag);
  //hmap.insert("parse".into_string(), parse::parse_issues);
  
  hmap
}

pub fn prompt(prompt:&str) -> String{
  print!("{}", prompt);
  //TODO do we need to check this?
  let withNewline = BufReader::new(stdin()).read_line().unwrap();
  withNewline.replace("\n", "").replace("\r", "")
}

pub fn get_author() -> String {
  let config = config::Config::load();
  match config.author {
    Some(author) => author,
    None => prompt("Author: ")
  }
}

pub fn edit_file(filename:&str) -> bool{
  match std::env::var("EDITOR") {
    Ok(editorName) => {
      let mut editor_command = process::Command::new(editorName.clone());
      editor_command.arg(filename);
      editor_command.stdin(process::Stdio::inherit());
      editor_command.stdout(process::Stdio::inherit());
      editor_command.stderr(process::Stdio::inherit());

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
    _ => false
  }
}
