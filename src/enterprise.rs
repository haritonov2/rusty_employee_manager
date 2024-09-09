use std::collections::HashMap;

pub trait CompanyOps {
    fn add_employee(&self, dep: &str, name: &str);
    fn get_dep_employees(&self, dep: &str);
}

pub struct Company {
    departments: HashMap<String, Vec<String>>
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }
}

impl CompanyOps for Company {
    fn add_employee(&self, dep: &str, name: &str) {}

    fn get_dep_employees(&self, dep: &str) {}
}
