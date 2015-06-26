#![allow(non_snake_case)]

#![feature(convert)]
#![feature(path_ext)]
#![feature(libc)]
#![feature(collections)]
#![feature(iter_cmp)]
#![feature(custom_derive, plugin)]

#![plugin(serde_macros)]

extern crate genfsm as fsm;
extern crate collections;
extern crate time;
extern crate serde;
extern crate libc;
#[macro_use]
extern crate error_type;

pub mod issue;
pub mod file_manager;
pub mod commands;
pub mod file_util;
pub mod vcs_status;
pub mod merge;
pub mod selection;
pub mod config;
pub mod status_storage;
pub mod date_sort;
pub mod serdetime;

/*
pub mod source{
  pub mod parse;
  pub mod file_parser;
  pub mod recursive_parser;
}
*/

pub mod evict{
  pub static CURRENT_VERSION:usize = 1;
}

#[cfg(not(test))]
fn main(){
  let args = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 {
    // < 2 because the first arg is the name of the binary
    println!("No command given");
  }else{
    let cmd_args = args.iter().skip(2).map(|s_ref| s_ref.to_string()).collect();
    
    let cmd = &args[1];
    commands::execute_command(cmd, &commands::standard_commands(), cmd_args);
  }
}
