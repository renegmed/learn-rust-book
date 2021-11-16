use std::fs; 
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { 
  
    /*
       Box<dyn Error> means the function will return a type that implements the 
       Error trait, but we don’t have to specify what particular type the return 
       value will be. This gives us flexibility to return error values that may be 
       of different types in different error cases. 
       
       The dyn keyword is short for “dynamic.”
    */
    let contents = fs::read_to_string(config.filename)?; 

    let results = if config.case_sensitive {
       search(&config.query, &contents)
    } else {
       search_case_insensitive(&config.query, &contents)
    };     

    for line in results {
       println!("{}", line);
    }
    
    Ok(())
 }
 
 pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
 }
 
 impl Config {
    /*
       env::Args is a type of iterator, args will be mutated

       We also needed to specify that the string slice error type 
       can now only have the 'static lifetime. Because we’re only 
       ever returning string literals, this was true before. 
       
       However, when we had a reference in the parameters, there 
       was the possibility that the reference in the return type 
       could have had the same lifetime as the reference in the parameters. 
       
       The rules that we discussed in the “Lifetime Elision” section of 
       Chapter 10 applied, and we weren’t required to annotate the lifetime 
       of &str. 
       
       With the change to args, the lifetime elision rules no longer apply, 
       and we must specify the 'static lifetime.
    */
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
       args.next();  // skip first element
       
       let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
       };

       let filename = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a file name"),
       };

      //  if args.len() < 3 {
      //     return Err("not enough arguments");
      //  }
 
      //  let query = args[1].clone();
      //  let filename = args[2].clone();
       
      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();  
       
      // println!(".... case sensitive: {}", case_sensitive);  

      Ok(Self { query, filename, case_sensitive })
    }
 }
  
 pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
 // pub fn search(query: &str, contents: &str) -> Vec<&str> {
    // tell Rust that the data returned by the searchma function will live 
    // as long as the data passed into the search function in the contents argument
    
   //  let mut results = Vec::new();

   //  for line in contents.lines() {
   //     if line.contains(query){  
   //       results.push(line);
   //     }
   //  }
   //  results

   contents 
      .lines()
      .filter(|line| line.contains(query))
      .collect()
 }

 pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 
   let query = &query.to_lowercase(); // we need to add an ampersand because 
                                      // the signature of contains is defined to take 
                                      // a string slice.  
   // let mut results = Vec::new();
       
   // for line in contents.lines() {
   //    if line.to_lowercase().contains(query){  
   //      results.push(line);
   //    }
   // }
   // results
   contents
      .lines() 
      .filter(|line| line.to_lowercase().contains(query))
      .collect()
}

 #[cfg(test)]
 mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
       let query = "duct";
       let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
      assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
       let query = "rUsT";
       let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
      assert_eq!(vec!["Rust:", "Trust me."], 
      search_case_insensitive(query, contents));
    }
 }
