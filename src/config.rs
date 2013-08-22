use file_util;
use extra::json;

#[cfg(target_os = "linux")]
static CONFIG_FILE:&'static str = "~/.evict/config";
static DIR_KEY:&'static str = "directory";
static AUTHOR_KEY:&'static str = "author";


struct Config{
  directory:~str,
  author:Option<~str>
}

pub fn loadConfig() -> Config {
  if(file_util::fileExists(CONFIG_FILE)){
    readConfig()
  }else{
    defaultConfig()
  }
}

pub fn defaultConfig() -> Config {
  Config{directory:~".evict", author:None}
}

fn readConfig() -> Config {
  let jsonStr = file_util::readStringFromFile(CONFIG_FILE);
  let jsonOpt = do jsonStr.chain |string| {
                  match json::from_str(string){
                    Ok(json) => Some(json),
		    Err(_) => None
		  }
                };
  jsonOpt.map_move_default(defaultConfig(), jsonToConfig)
}

fn jsonToConfig(json:json::Json) -> Config {
  match json {
    json::Object(map) => do map.find(&DIR_KEY.to_owned())
                               .map_move_default(defaultConfig()) |dir| {
                           Config{directory:dir.to_str(), 
			                    author:map.find(&AUTHOR_KEY.to_owned())
					              .map(|x| x.to_str())}
                         },
    _ => defaultConfig()
  }
}
