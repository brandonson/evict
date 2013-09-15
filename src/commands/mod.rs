use std;
use file_manager;
use config;

mod create;
mod clear;
mod list;
mod sync;
mod delete;
mod comment;
mod merge;
mod new_status;
mod default_author;
mod set_status;
mod default_status;

/* A command takes a list of argument strings,
 * performs some action, then returns an
 * exit code.
 */
type Command = ~fn (~[~str], config::Config) -> int;

pub fn executeCommand(command:&~str, 
                      commandList:&~std::container::Map<~str, Command>, 
                      argList: ~[~str], config:config::Config) -> bool{
  match commandList.find(command) {
    Some(cmd) => {let exit = (*cmd)(argList, config); std::os::set_exit_status(exit); true}
    None => {
     std::io::println(fmt!("Command %s not found", *command)); 
     std::os::set_exit_status(1); 
     false
    } 
  }
}

pub fn standardCommands() -> ~std::container::Map<~str, Command> {
  let mut hmap:~std::hashmap::HashMap<~str, Command> = ~std::hashmap::HashMap::new();
  hmap.insert(~"create", create::createIssue);
  hmap.insert(~"clear", clear::clearData);
  hmap.insert(~"init", init);
  hmap.insert(~"list", list::listIssues); 
  hmap.insert(~"delete", delete::deleteIssue);
  hmap.insert(~"comment", comment::newComment); 
  hmap.insert(~"merge", merge::mergeBranches);
  hmap.insert(~"sync", sync::syncIssues);
  hmap.insert(~"new-status", new_status::newStatus);
  hmap.insert(~"default-author", default_author::defaultAuthor);
  hmap.insert(~"set-status", set_status::setStatus);
  hmap.insert(~"default-status", default_status::defaultStatus);
  hmap as ~std::container::Map<~str, Command>
}

pub fn init(_:~[~str], _:config::Config) -> int {
  let res = std::os::make_dir(&Path(file_manager::EVICT_DIRECTORY), 
                                    0400 | 0200 | 0040 | 0020 | 0004);
  if(res){0}else{1}
}

pub fn prompt(prompt:&str) -> ~str{
  std::io::print(prompt);
  std::io::stdin().read_line()
}

pub fn getAuthor() -> ~str {
  let config = config::Config::load();
  match config.author {
    Some(author) => author,
    None => prompt("Author: ")
  }
}

pub fn editFile(filename:&str) -> bool{
  match std::os::getenv("EDITOR") {
    Some(editorName) => {
      std::run::process_status(editorName, &[filename.to_owned()]);
      true
    }
    None => false
  }
}
