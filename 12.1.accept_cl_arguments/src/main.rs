use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();
   //println!("{:?} \n\t{} \n\t{}", args, args[1], args[2]);

   let query = &args[1];
   let filename = &args[2];

   println!("Searching for {}", query);
   println!("In file {}", filename);
}
