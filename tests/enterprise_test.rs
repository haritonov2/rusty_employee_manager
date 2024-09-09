use std::collections::HashMap;
use rusty_employee_manager::{Company, CompanyOps};

#[test]
fn test_manage_employee_to_empty_dep() {
    let mut company = Company::new();

    company.add_employee("Sales", "Alex");

    assert_eq!(company.get_dep_employees("Sales"), "Alex");
}

#[test]
fn test_manage_employee_to_populated_dep() {
    let mut departments = HashMap::new();

    departments.insert(
        String::from("Sales"),
        vec!(String::from("Tony"), String::from("Anna"))
    );

    let mut company = Company {
        departments
    };

    company.add_employee("Sales", "Alex");

    assert_eq!(company.get_dep_employees("Sales"), "Tony, Anna, Alex");
}
