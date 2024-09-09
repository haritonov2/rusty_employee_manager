mod enterprise;

pub use enterprise::Company;
pub use enterprise::CompanyOps;

const TO: &str = "to";

pub fn add_cmd(company: &dyn CompanyOps, words: Vec<&str>) -> bool {
    if words.len() != 4 || words[2] != TO {
        return false
    }

    company.add_employee(words[3], words[1]);

    true
}

pub fn show_cmd(company: &dyn CompanyOps, words: Vec<&str>) -> bool {
    if words.len() != 2 {
        return false
    }

    company.get_dep_employees(words[1]);

    true
}
