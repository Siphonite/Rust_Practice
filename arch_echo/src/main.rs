use std::env; 

fn main(){ 
 
  let intake: Vec<String> = env::args().collect();  
   
if intake.len() < 3 {
    eprintln!("not enough arguments");
    std::process::exit(1);  
  }
  
  for (i,arg) in intake[1..].iter().enumerate(){
  println!("{}.{}",i,arg);

  } 
}