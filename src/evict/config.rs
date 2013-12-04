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
use extra::json;
use extra::json::ToJson;
use extra::treemap;

static CONFIG_FILE:&'static str = ".evict/config";
static AUTHOR_KEY:&'static str = "author";

struct Config{
  author:Option<~str>,
}

impl ToJson for Config{
  fn to_json(&self) -> json::Json {
    let mut map:~treemap::TreeMap<~str, json::Json> = ~treemap::TreeMap::new();
    match self.author {
      Some(ref auth) => {map.insert(AUTHOR_KEY.to_owned(),json::String(auth.to_owned()));}
      None => {}
    };
    json::Object(map)
  }
}

impl Config{
  pub fn load() -> Config {
    if(file_util::file_exists(CONFIG_FILE)){
      Config::read_repo_config()
    }else{
      Config::default()
    }
  }
  
  pub fn default() -> Config {
    Config{author:None}
  }
  
  fn read_repo_config() -> Config {
    let jsonStr = file_util::read_string_from_file(CONFIG_FILE);
    let jsonOpt = jsonStr.and_then (|string| {
                    match json::from_str(string){
                      Ok(json) => Some(json),
                      Err(_) => None
                    }
                  });
    jsonOpt.map_default(Config::default(), Config::from_json)
  }
  
  fn from_json(json:json::Json) -> Config {
    match json {
      json::Object(map) => Config{author:map.find(&AUTHOR_KEY.to_owned())
                                            .and_then(|x| extract_string(x)),
                           },
      _ => Config::default()
    }
  }

  pub fn save(&self){
    let jsonStr = self.to_json().to_pretty_str();
    file_util::write_string_to_file(jsonStr, CONFIG_FILE, true);
  }
}

fn extract_string(json:&json::Json) -> Option<~str> {
  match json {
    &json::String(ref string) => Some(string.to_owned()),
    _ => None
  }
}
