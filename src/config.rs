use file_util;
use extra::json;
use extra::json::ToJson;
use extra::treemap;

static CONFIG_FILE:&'static str = ".evict/config";
static AUTHOR_KEY:&'static str = "author";
static STATUS_KEY:&'static str = "statuses";

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
    if(file_util::fileExists(CONFIG_FILE)){
      Config::readRepoConf()
    }else{
      Config::default()
    }
  }
  
  pub fn default() -> Config {
    Config{author:None}
  }
  
  fn readRepoConf() -> Config {
    let jsonStr = file_util::readStringFromFile(CONFIG_FILE);
    let jsonOpt = do jsonStr.chain |string| {
                    match json::from_str(string){
                      Ok(json) => Some(json),
                      Err(_) => None
                    }
                  };
    jsonOpt.map_move_default(Config::default(), Config::jsonToConfig)
  }
  
  fn jsonToConfig(json:json::Json) -> Config {
    match json {
      json::Object(map) => Config{author:map.find(&AUTHOR_KEY.to_owned())
                                            .chain(|x| extractString(x)),
                           },
      _ => Config::default()
    }
  }

  pub fn save(&self){
    let jsonStr = self.to_json().to_pretty_str();
    file_util::writeStringToFile(jsonStr, CONFIG_FILE, true);
  }
}

fn extractString(json:&json::Json) -> Option<~str> {
  match json {
    &json::String(ref string) => Some(string.to_owned()),
    _ => None
  }
}
