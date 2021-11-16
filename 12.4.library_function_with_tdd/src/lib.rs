use std::fs; 
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {   
    /*
       Box<dyn Error> means the function will return a type that implements the 
       Error trait, but we don’t have to specify what particular type the return 
       value will be. This gives us flexibility to return error values that may be 
       of different types in different error cases. 
       
       The dyn keyword is short for “dynamic.”
    */
    let contents = fs::read_to_string(config.filename)?;                    
                 
    //println!("With text:\n{}", contents.unwrap());
    for line in search(&config.query, &contents) {
       println!("{}", line);
    }
    
    Ok(())
 }
 
 pub struct Config {
    pub query: String,
    pub filename: String,
 }
 
 impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
       if args.len() < 3 {
          return Err("not enough arguments");
       }
 
       let query = args[1].clone();
       let filename = args[2].clone(); 
       Ok(Self { query, filename })
    }
 }
 
 // fn parse_config(args: &[String]) -> Config {
 
 //    let query = args[1].clone();
 //    let filename = args[2].clone(); 
 //    Config { query, filename }  
 // }
 
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

 #[cfg(test)]
 mod tests {
    use super::*;

    #[test]
    fn one_result() {
       let query = "duct";
       let contents = "\
Rust: 
safe, fast, productive.
Pick three.";
      assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
 }