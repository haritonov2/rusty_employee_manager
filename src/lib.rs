mod enterprise;

use enterprise::Company;
use enterprise::CompanyOps;

const TO: &str = "to";

pub fn add_cmd(company: &Company, words: Vec<&str>) -> bool {
    let is_valid = true;

    if words.len() != 4 || words[2] != TO {
        return false
    }

    company.add_employee(words[3], words[1]);

    is_valid
}

pub fn show_cmd(company: &Company, words: Vec<&str>) -> bool {
    let is_valid = true;

    // company.get_dep_employees(String::from("QA")),

    is_valid
}

#[cfg(test)]
#[test]
fn test_add_cmd_4_words_validation() {
    let company = Company::new();
    let invalid_words = vec!("Add", "Sasha", "to");

    assert_eq!(add_cmd(&company, invalid_words), false);
}

#[test]
fn test_add_cmd_no_to_word() {
    let company = Company::new();
    let invalid_words = vec!("Add", "Sasha", "from", "QA");

    assert_eq!(add_cmd(&company, invalid_words), false);
}

#[test]
fn test_add_cmd() {
    let company = Company::new();
    let invalid_words = vec!("Add", "Sasha", "to", "QA");

    assert!(add_cmd(&company, invalid_words));
}
