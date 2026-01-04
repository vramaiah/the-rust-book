use std::io;

use question_3::company::{Company};

fn add_employee(company: &mut Company) {
    println!("Enter employee name: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let employee = buffer.trim().to_string();

    println!("Enter department name: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let department = buffer.trim().to_string();
    company.add_employee_to_department(&employee, &department);
}

fn add_department(company: &mut Company) {
    println!("Enter department name: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();
    company.add_department(&buffer);
}


fn list_employees(company: &Company) {
    let employees = company.get_all_employees();
    println!("This company's employees are: ");
    for &employee in &employees {
      println!(" * {employee}");
    }
}

fn list_employees_in_department(company: &Company) {
    // Get a department ID
    println!("Enter a department's name: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let department = buffer.trim().to_string();

    let employees = company.get_all_people_from_department(&department);
    println!("{department}'s employees are: ");
    for &employee in &employees {
        println!(" * {employee}");
    }
}

fn main() {
    let mut company = Company::new();
    loop {
        let mut buffer = String::new();
        println!("\nChose an action:\n(1): Add employee");
        println!("(2): Add department");
        println!("(3): List all employees");
        println!("(4): List employees in department");
        println!("(q): Quit");
        io::stdin().read_line(&mut buffer).unwrap();
        let action = buffer.trim();

        let company_m = &mut company;
        match action {
            "1" => add_employee(company_m),
            "2" => add_department(company_m),
            "3" => list_employees(&company_m),
            "4" => list_employees_in_department(&company_m),
            "q" => break,
            "Q" => break,
            _ => {
                println!("Chose a valid option");
            }
        }
    }
}
