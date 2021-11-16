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
    
    println!("{}", contents);
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
   