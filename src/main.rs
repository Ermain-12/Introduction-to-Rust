extern crate regex;

use regex::Regex;

fn main() {
    println!("Hello, world!");
    println!("Hello, Rust");

   let home_team = "Real Madrid";
   let result = " beat ";
   let away_team = "Barcelona";

   let full_line = home_team.to_owned() + result + away_team;

   println!("{}", full_line);

   // Vectors 
   let mut my_vec: Vec<i32> = (0..10).collect();
   println!("{:?}", my_vec);

   // Add values to the vector
   my_vec.push(13);
   my_vec.push(21);
   println!("{:?}", my_vec);

   // Remove the last value
   let mut twenty_one = my_vec.pop();
   println!("twenty_one = {:?}", twenty_one);
   println!("{:?}", my_vec);

   let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
   println!("Did our date match? {}", re.is_match("2014-01-01"));
}
