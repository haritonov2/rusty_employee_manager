use std::collections::HashMap;

pub trait CompanyOps {
    fn add_employee(&mut self, dep: &str, name: &str);
    fn get_dep_employees(&self, dep: &str) -> String;
}

pub struct Company {
    pub departments: HashMap<String, Vec<String>>
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }
}

impl CompanyOps for Company {
    fn add_employee(&mut self, dep: &str, name: &str) {
        let department = self.departments.
            entry(dep.to_string()).or_insert(Vec::new());

        department.push(name.to_string());
    }

    fn get_dep_employees(&self, dep: &str) -> String {
        let department = self.departments.get(dep);

        let result: String = match department {
            Some(v) => v.join(", "),
            None => String::new()
        };

        result
    }
}
