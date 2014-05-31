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
use config;

pub fn default_author(args:~[String]) -> int {
  if args.len() > 1 {
    println!("default-author usage: evict default-author [new-author]");
    1
  }else{
    let config = config::Config::load(); 
    if args.len() == 0 {
      match config.author {
        Some(author) => println!("{}", author),
        None => println!("No author set")
      };
      0
    }else{
      config::Config{author:Some(args[0]), .. config}.save();
      0
    }
  }
}

