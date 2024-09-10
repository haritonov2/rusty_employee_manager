mod company;

pub use company::Company;
pub use company::CompanyOps;

const TO: &str = "to";

pub fn add_cmd(company: &mut dyn CompanyOps, words: Vec<&str>) -> bool {
    if words.len() != 4 || words[2] != TO {
        return false
    }

    company.add_employee(words[3], words[1]);

    println!("Employee \"{}\" is added to \"{}\"", words[1], words[3]);

    true
}

pub fn show_cmd(company: &dyn CompanyOps, words: Vec<&str>) -> bool {
    if words.len() != 2 {
        return false
    }

    let list = company.get_dep_employees(words[1]);

    println!("{}", list);

    true
}
