use std::collections::HashMap;

pub trait CompanyOps {
    fn new() -> Self;
    fn add_employee(&self, dep: &str, name: &str) -> bool;
    fn get_dep_employees(&self, dep: &str) -> bool;
}

pub struct Company {
    departments: HashMap<String, Vec<String>>
}

impl CompanyOps for Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&self, dep: &str, name: &str) -> bool {
        false
    }

    fn get_dep_employees(&self, dep: &str) -> bool {
        false
    }
}
