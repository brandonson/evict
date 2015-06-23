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
use file_util;
use serde::json::Serializer as JsonSerializer;
use serde::json::Deserializer as JsonDeserializer;
use serde::json::Error as JsonDeserializationError;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::io::Result as IoResult;
use std::io::Read;

use std::fs::File;

static CONFIG_FILE:&'static str = ".evict/config";

#[derive(Serialize, Deserialize)]
pub struct Config{
  pub author:Option<String>,
}

impl Config{
  pub fn load() -> Config {
    if file_util::file_exists(CONFIG_FILE) {
      match Config::read_repo_config() {
        Ok(conf) => conf,
        Err(_) => Config::default()
      }
    }else{
      Config::default()
    }
  }
  
  pub fn default() -> Config {
    Config{author:None}
  }
  
  fn read_repo_config() -> Result<Config, JsonDeserializationError> {
    let file = try!(File::open(CONFIG_FILE));
    let mut deser = try!(JsonDeserializer::new(file.bytes()));
    Config::deserialize(&mut deser)
  }
  
  pub fn save(&self) -> IoResult<()> {
    let file = try!(File::create(CONFIG_FILE));
    let mut writer = JsonSerializer::pretty(file);
    self.serialize(&mut writer)
  }
}
