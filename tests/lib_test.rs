use rusty_employee_manager::CompanyOps;
use rusty_employee_manager::{ add_cmd, show_cmd };

struct StubCompany;

impl StubCompany {
    pub fn new() -> Self {
        Self {}
    }
}

impl CompanyOps for StubCompany {
    fn add_employee(&self, _dep: &str, _name: &str) {}

    fn get_dep_employees(&self, _dep: &str) {}
}

#[test]
fn test_add_cmd_4_words_validation() {
    let company = StubCompany::new();
    let invalid_words = vec!("Add", "Sasha", "to");

    assert_eq!(add_cmd(&company, invalid_words), false);
}

#[test]
fn test_add_cmd_no_to_word() {
    let company = StubCompany::new();
    let invalid_words = vec!("Add", "Sasha", "from", "QA");

    assert_eq!(add_cmd(&company, invalid_words), false);
}

#[test]
fn test_add_cmd() {
    let company = StubCompany::new();
    let words = vec!("Add", "Sasha", "to", "QA");

    assert!(add_cmd(&company, words));
}

#[test]
fn test_show_cmd_2_words_validation() {
    let company = StubCompany::new();
    let invalid_words = vec!("Show", "QA", "list");

    assert_eq!(show_cmd(&company, invalid_words), false);
}

#[test]
fn test_show_cmd() {
    let company = StubCompany::new();
    let words = vec!("Show", "Dev");

    assert!(show_cmd(&company, words));
}
