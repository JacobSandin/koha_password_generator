extern crate bcrypt;

use bcrypt::{hash_with_result, Version};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).is_some() {
        let arg = args.get(1).unwrap();
        if args.contains(&"sql".to_string()) {
            print_sql_and_prompt();
        } else {
            let output = hash_with_result(arg, 8).unwrap();
            let output = output.format_for_version(Version::TwoA);
            println!("{}", output);
        }
    } else {
        println!("OBS! No password argument");
    }
}

fn print_sql_and_prompt() {
    // Make  the function prin
    //    let database = ask("Database name: ");
    let borrowernumber = ask("Borrowernumber: ");
    let cardnumber = ask("Cardnumber: ");
    let branchcode = ask("Branchcode: ");
    let categorycode = ask("Categorycode: ");
    println!("Remember your password enforcement or you might not be able to log in!");
    let password = ask("Password: ");
    let password = get_password(&password);
    let userid = ask("Userid: ");
    let firstname = ask("Firstname: ");
    let surname = ask("Surname: ");
    let dateenrolled = "2020-10-04";
    let dateexpiry = "2852-10-04";
    let updated_on = "2020-10-04";

    println!(
        "INSERT INTO `borrowers` (
    `borrowernumber`, 
    `cardnumber`, 
    `surname`, 
    `firstname`, 
    `branchcode`, 
    `categorycode`, 
    `dateenrolled`, 
    `dateexpiry`, 
    `password`, 
    `flags`, 
    `userid`, 
    `updated_on`
    ) 
VALUES (
    '{borrowernumber}', 
    '{cardnumber}', 
    '{surname}', 
    '{firstname}', 
    '{branchcode}', 
    '{categorycode}', 
    '{dateenrolled}', 
    '{dateexpiry}', 
    '{password}', 
    '1', 
    '{userid}', 
    '{updated_on}'
    );",
        borrowernumber = borrowernumber,
        cardnumber = cardnumber,
        branchcode = branchcode,
        categorycode = categorycode,
        dateenrolled = dateenrolled,
        dateexpiry = dateexpiry,
        password = password,
        userid = userid,
        firstname = firstname,
        surname = surname,
        updated_on = updated_on,
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
