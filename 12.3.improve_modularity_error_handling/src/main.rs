use std::env;
use std::process;
use minigrep::Config;

fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::new(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {}", err);
      process::exit(1);
   });

   println!("Searching for {}", config.query);
   println!("In file {}", config.filename);

   // run(config).unwrap_or_else(|err| {
   //    println!("Application error: {}", err);
   //    process::exit(1);
   // });

   if let Err(e) = minigrep::run(config) {
      println!("Application error: {}", e);
      process::exit(1);
   } 
   /*
      We use if let rather than unwrap_or_else to check whether run returns an Err value 
      and call process::exit(1) if it does. 
      
      The run function doesn’t return a value that we want to unwrap in the same way that 
      Config::new returns the Config instance. 
      
      Because run returns () in the success case, we only care about detecting an error, 
      so we don’t need unwrap_or_else to return the unwrapped value because it would 
      only be ().
   */
   
   // let config = Config::new(&args);
    // match config {
   //    Ok(config) => {
   //       println!("Searching for {}", config.query);
   //       println!("In file {}", config.filename);
      
   //       let contents = fs::read_to_string(config.filename)
   //                        .expect("Something went wrong reading the file");
                        
   //       println!("With text:\n{}", contents);
   //    }

   //    Err(str) => {
   //       println!("Error: {}", str);
   //    }
   // }
  

}
