use std::io;
use rusty_employee_manager::Company;
use rusty_employee_manager::{add_cmd, show_cmd};

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
