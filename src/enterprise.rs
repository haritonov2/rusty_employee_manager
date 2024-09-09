use std::collections::HashMap;

pub struct Company {
    departments: HashMap<String, Vec<String>>
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    pub fn add_employee(&self, dep: &str, name: &str) {
        // TBD
    }

    pub fn get_dep_employees(&self, dep: &str) {
        // TBD
    }
}
