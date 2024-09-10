# Rusty Employee Manager

A command-line interface for managing employee assignments within a company, implemented in Rust. This tool allows users to add employees to departments and retrieve sorted lists of employees.

## Features

- **Add Employee**: Add employees to departments with commands like `Add [Employee name] to [department name]`.
- **Show Employees**: Retrieve a comma-separated list of employees in a specified department with commands like `Show [department name]`.
- **Interactive CLI**: User-friendly text interface for managing and displaying employee data.

## Example output

```txt
Welcome to an employee manager!
Use two types of commands:
"Add [Employee name] to [department name]" ===> Example: "Add Amir to Sales"
OR
"Show [department name]" ===> Example: "Show [department name]"
OR
"Exit"
Please enter a command:
Add Sasha to QA
Employee "Sasha" is added to "QA"
Please enter a command:
Add Alex to QA
Employee "Alex" is added to "QA"
Please enter a command:
Show QA
Sasha, Alex
Please enter a command:
exit
Goodbye!
```


## Installation

To build and run this project, ensure you have Rust installed. You can then use Cargo to manage and run the project:

```sh
cargo build
cargo run
```

## Usage

Run the application and follow the on-screen instructions. Here are some example commands you can use:

* Add an employee: `Add [Employee name] to [department name]`
  * Example: `Add Amir to Sales`
* Show employees: `Show [department name]`
  * Example: `Show Sales`
* Exit: Type `Exit` to close the application.

## Testing

```sh
cargo test
```

The tests cover various aspects of the functionality, including command parsing and employee management.

## License
This project is licensed under the MIT License. See the LICENSE file for details.



