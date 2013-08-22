use std;
use file_manager;

mod create;
mod clear;
mod list;
mod sync;
mod delete;
mod comment;
/* A command takes a list of argument strings,
 * performs some action, then returns an
 * exit code.
 */
type Command = ~fn (~[~str]) -> int;

pub fn executeCommand(command:&~str, 
                      commandList:&~std::container::Map<~str, Command>, 
                      argList: ~[~str]) -> bool{
  match commandList.find(command) {
    Some(cmd) => {let exit = (*cmd)(argList); std::os::set_exit_status(exit); true}
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
  //TODO implement below commands
  hmap.insert(~"sync", sync::syncIssues);
  hmap as ~std::container::Map<~str, Command>
}

pub fn init(_:~[~str]) -> int {
  let res = std::os::make_dir(&Path(file_manager::EVICT_DIRECTORY), 
                                    0400 | 0200 | 0040 | 0020 | 0004);
  if(res){0}else{1}
}

pub fn prompt(prompt:&str) -> ~str{
  std::io::print(prompt);
  std::io::stdin().read_line()
}

pub fn getAuthor() -> ~str {
  prompt("Author: ")
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
