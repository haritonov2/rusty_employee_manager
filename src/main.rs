mod enterprise;

use std::io;
use enterprise::Company;
use enterprise::CompanyOps;

const TO: &str = "to";

fn main() {
    print!("Welcome to an employee manager!\r\n");
    print!("Use two types of commands:\r\n");
    print!("\"Add [Employee name] to [department name]\" ===> Example: \"Add Amir to Sales\"\r\n");
    print!("OR\r\n");
    print!("\"Show [department name]\" ===> Example: \"Show [department name]\"\r\n");
    print!("OR\r\n");
    print!("\"Exit\"");

    let company = Company::new();

    loop {
        println!("Please enter a command:");

        let mut raw_cmd = String::new();

        io::stdin().
            read_line(&mut raw_cmd).
            expect("Failed to read a command.");

        let cmd_words: Vec<&str> = raw_cmd.trim().split_whitespace().collect();

        let is_valid = match cmd_words[0].to_lowercase().as_str() {
            "add"  => add_cmd(&company, cmd_words),
            "show" => show_cmd(&company, cmd_words),
            "exit" => break,
            _ => false,
        };

        if !is_valid {
            println!("Invalid command: {}", raw_cmd);
        }
    }

    println!("Goodbye!");
}

fn add_cmd(company: &Company, words: Vec<&str>) -> bool {
    let is_valid = true;

    if words.len() != 4 || words[2] != TO {
        return false
    }

    // company.add_employee(String::from("QA"), String::from("Sasha"))

    is_valid
}

fn show_cmd(company: &Company, words: Vec<&str>) -> bool {
    let is_valid = true;

    // company.get_dep_employees(String::from("QA")),

    is_valid
}

#[cfg(test)]
#[test]
fn test_add_cmd_4_words_validation() {
    let company = Company::new();
    let invalid_words = vec!("Add", "QA", "to");

    assert_eq!(add_cmd(&company, invalid_words), false);
}

#[test]
fn test_add_cmd_no_to_word() {
    let company = Company::new();
    let invalid_words = vec!("Add", "QA", "from", "team");

    assert_eq!(add_cmd(&company, invalid_words), false);
}
