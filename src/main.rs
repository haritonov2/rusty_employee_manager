mod enterprise;

use std::io;
use enterprise::Company;

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

        if cmd_words.len() < 2 {
            println!("Invalid command: {}", raw_cmd);
            continue;
        }

        match cmd_words[0].to_lowercase().as_str() {
            "add" => {
                company.add_employee(String::from("QA"), String::from("Sasha"))
            },
            "show" => company.get_dep_employees(String::from("QA")),
            "exit" => break,
            _ => {
                println!("Invalid command: {}", raw_cmd);
                println!("A command should start with: \"Add\", \"Show\" or \"Exit\"");
                continue;
            },
        }
    }

    println!("Goodbye!");
}

// Add [Employee name] to [dep name]
// Add Amir to Sales

// Show [dep name]
