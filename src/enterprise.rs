use std::collections::HashMap;

pub struct Company {
    _departments: HashMap<String, Vec<String>>
}

impl Company {
    pub fn new() -> Company {
        Company {
            _departments: HashMap::new(),
        }
    }

    pub fn add_employee(&self, _dep: String, _name: String) {
        // TBD
    }

    pub fn get_dep_employees(&self, _dep: String) {
        // TBD
    }
}