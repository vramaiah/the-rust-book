pub mod company {
    use std::collections::HashMap;

    pub struct Company {
        data: HashMap<String, Vec<String>>
    }

    impl Company {
        pub fn new() -> Company {
            Company {
                data: HashMap::new(),
            }
        }

        /// Adds a department to the company
        pub fn add_department(&mut self, name: &str) {
            self.data.insert(name.to_string(), Vec::new());
        }

        pub fn add_employee_to_department(& mut self, employee: &str, department: &str) {
            self.data
                .get_mut(&department.to_string())
                .expect("Provided department was not valid")
                .push(employee.to_string());
        }

        pub fn get_all_employees(&self) -> Vec<&String> {
            let mut names = Vec::new();
            for (_, employees) in &self.data {
                for employee in employees {
                    names.push(employee);
                }
            }
            names.sort();
            names
        }

        pub fn get_all_people_from_department(&self, department: &str) -> Vec<&String> {
            let mut names = Vec::new();
            let employees = self
                .data
                .get(&department.to_string())
                .expect("Provided department was not valid");
            for employee in employees {
                names.push(employee);
            }
            names.sort();
            names
        }

    }
}
