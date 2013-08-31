use config;

pub fn defaultAuthor(args:~[~str], config:config::Config) -> int {
  if(args.len() > 1){
    println("default-author usage: evict default-author [new-author]");
    1
  }else if(args.len() == 0){
    match config.author {
      Some(author) => println(author),
      None => println("No author set")
    };
    0
  }else{
    config::Config{author:Some(args[0]), .. config}.save();
    0
  }
}

