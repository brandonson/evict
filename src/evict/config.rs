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
use serialize::json;
use serialize::json::ToJson;
use std::collections::BTreeMap;

static CONFIG_FILE:&'static str = ".evict/config";
static AUTHOR_KEY:&'static str = "author";

pub struct Config{
  pub author:Option<String>,
}

impl ToJson for Config{
  fn to_json(&self) -> json::Json {
    let mut map:BTreeMap<String, json::Json> = BTreeMap::new();
    match self.author {
      Some(ref auth) => {
        map.insert(AUTHOR_KEY.to_string(),json::Json::String(auth.to_string()));
      }
      None => {}
    };
    json::Json::Object(map)
  }
}

impl Config{
  pub fn load() -> Config {
    if file_util::file_exists(CONFIG_FILE) {
      Config::read_repo_config()
    }else{
      Config::default()
    }
  }
  
  pub fn default() -> Config {
    Config{author:None}
  }
  
  fn read_repo_config() -> Config {
    let json_str = file_util::read_string_from_file(CONFIG_FILE);
    let json_opt = json_str.and_then (|string| {
                    match json::from_str(string.as_str()){
                      Ok(json) => Some(json),
                      Err(_) => None
                    }
                  });
    json_opt.map_or(Config::default(), Config::from_json)
  }
  
  fn from_json(json:json::Json) -> Config {
    match json {
      json::Json::Object(map) => Config{author:map.get(&AUTHOR_KEY.to_string())
                                            .and_then(|x| extract_string(x)),
                           },
      _ => Config::default()
    }
  }

  pub fn save(&self){
    let json_str = format!("{}", self.to_json().pretty());
    file_util::write_string_to_file(json_str.as_str(), CONFIG_FILE, true);
  }
}

fn extract_string(json:&json::Json) -> Option<String> {
  match json {
    &json::Json::String(ref string) => Some(string.to_string()),
    _ => None
  }
}
