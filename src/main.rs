extern crate bcrypt;

use bcrypt::{hash_with_result, Version};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).is_some() {
        let arg = args.get(1).unwrap();
        let output = hash_with_result(arg, 8).unwrap();
        let output = output.format_for_version(Version::TwoA);
        println!("{}", output);
    } else {
        println!("OBS! No password argument");
    }
    print_sql_and_prompt();
}

fn print_sql_and_prompt() {
    // Make  the function prin
    let database = ask("Database name: ");
    let borrowernumber = ask("Borrowernumber: ");
    let cardnumber = ask("Cardnumber: ");
    let branchcode = ask("Branchcode: ");
    let categorycode = ask("Categorycode: ");
    println!("Remember your password enforcement or you might not be able to log in!");
    let password = ask("Password: ");
    let password = get_password(&password);
    let userid = ask("Userid: ");
    let firstname = ask("Firstname: ");

    println!("INSERT INTO `{database}`.`borrowers` (`borrowernumber`, `cardnumber`, `firstname`, `branchcode`, `categorycode`, `password`, `userid`) 
                                           VALUES ('{borrowernumber}', '{cardnumber}', '{firstname}',  '{branchcode}', '{categorycode}', '{password}', '{userid}');",
    database=database,
    borrowernumber=borrowernumber,
    cardnumber=cardnumber,
    branchcode=branchcode,
    categorycode=categorycode,
    password=password,
    userid=userid,
    firstname=firstname,
   );
}

fn get_password(passwd: &str) -> String {
    let output = hash_with_result(passwd, 8).expect("Expected password");
    output.format_for_version(Version::TwoA)
}

fn ask(question: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}", question);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
