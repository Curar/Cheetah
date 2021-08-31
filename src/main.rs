// Cheeter
// A program written to search for e-mail addresses from main web pages or other
// uses regular expressions (https://docs.rs/regex/)
// Posted by Curar in 2021 for fun

use std::io;
use regex::Regex;
use reqwest;
#[tokio::main]


async fn main() -> Result<(), reqwest::Error> {
     
      fn info() {
      // Info 
      //
      let info = r#"=====================================
                  =    e-mail scaner by curar   =
                  =     https://www.rust-lang.org     =
                  =      https://github.com/curar     =
                  =   This is my educational project  =
                  ====================================="#;
    
        println!("{}", info);
      } 

     //Adding data
     //
     loop {
     info();
     let mut serwer = String::new();
     
     // Check the server, enter the address
     //
     println!("Enter a website address to check email addresses http://127.0.0.1 :");
     
     io::stdin().read_line(&mut serwer)
        .ok()
        .expect("Failed to read line");

     // Regular expression matching any e-mail address
     //
     let szukane_dane = Regex::new(r"([\w-]+(?:\.[\w-]+)*)@((?:[\w-]+\.)*\w[\w-]{0,80})\.([a-z]{2,6}(?:\.[a-z]{2})?)").unwrap();
    
     // Now we are downloading data from the selected server
     let pobieraj_dane_zwww = reqwest::get(&serwer)
        .await?
        .text()
        .await?;
     // Loop collecting data 
     for i in szukane_dane.captures_iter(&pobieraj_dane_zwww) {

        println!("I found the address: {}", &i[0]);

     }
     }
}
