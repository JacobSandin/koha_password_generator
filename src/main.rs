extern crate bcrypt;

use bcrypt::{hash_with_result, Version};
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).is_some() {
        let arg =  args.get(1).unwrap();
        let output = hash_with_result(arg,8).unwrap();
        let output =output.format_for_version(Version::TwoA);
        println!("{}",output);
    } else {
        println!("OBS! No password argument");
    }
}
