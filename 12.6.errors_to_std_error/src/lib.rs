use std::fs; 
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { 
   println!("++++ case sensitive: {}", config.case_sensitive);  
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
    pub fn new(args: &[String]) -> Result<Config, &str> {
       if args.len() < 3 {
          return Err("not enough arguments");
       }
 
       let query = args[1].clone();
       let filename = args[2].clone();
       
      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
      
      // let key = "CASE_SENSITIVE";
      //  match env::var(key) {
      //    Ok(val) =>  if val == "1" {
      //       Ok(Config { query, filename, case_sensitive: true })
      //    } else {
      //       Ok(Config { query, filename, case_sensitive: false })
      //    },
      //       // case_sensitive = val, //println!("---- {}: {:?}", key, val),
      //    Err(e) => e.description(), //"error:", //format!("---couldn't interpret {}: {}", key, e),
      // }

       
       println!(".... case sensitive: {}", case_sensitive);  

      Ok(Self { query, filename, case_sensitive })
    }
 }
  
 pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
 // pub fn search(query: &str, contents: &str) -> Vec<&str> {
    // tell Rust that the data returned by the searchma function will live 
    // as long as the data passed into the search function in the contents argument
    
    let mut results = Vec::new();

    for line in contents.lines() {
       if line.contains(query){  
         results.push(line);
       }
    }
    results
 }

 pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 
   let query = &query.to_lowercase(); // we need to add an ampersand because 
                                      // the signature of contains is defined to take 
                                      // a string slice.  
   let mut results = Vec::new();
       
   for line in contents.lines() {
      if line.to_lowercase().contains(query){  
        results.push(line);
      }
   }
   results
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
