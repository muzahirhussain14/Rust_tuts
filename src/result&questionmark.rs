/*
Topic: Result and the question mark operator

Requirements:
    - Determine if an employee can access a building using a digital keycard.
    - Employes that can access the building are,
        - Maintenance crews
        - Marketing department employees
        - Managers
    - Other employees that work at the company are,
        - Line supervisors
        - Kitchen staff
        - Assembly technicians
    - Ensure that terminated employees cannot access the building regardless of their position

Notes:
    - Use an enum to represent all types of employees
    - Use a struct to store the employee type and whether they are still employed
    - Use a function that returns a Result to determine if the employee may enter the building
    - Print whether the employee may access the building
        - Must use a function that utilizes the question mark operator to do this
*/

enum EmployeeTypes {
    Maintenance,
    Marketing,
    Manager,
    Line,
    Kitchen,
    Assembly
}
enum EmployeeStatue {
    Employeed,
    Terminated
}

struct Employee {
    emp_type: EmployeeTypes,
    status: EmployeeStatue
}

fn try_access (employee: Employee) -> Result<(), String> {

    match employee.status {
        EmployeeStatue::Terminated => return Err("This Employee is terminated".to_owned()),
        _ => ()
    }

    match employee.emp_type {
        EmployeeTypes::Maintenance => Ok(()),
        EmployeeTypes::Marketing => Ok(()),
        EmployeeTypes::Manager => Ok(()),
        _ => Err("This employee cannot enter the building".to_owned())
    }
}

fn get_access(employee: Employee) -> Result<(), String> {

    try_access(employee)?;                 // if it receives the error, it will return from here

    println!("Access granted to this employee");
    Ok(())
}

fn main() {

    let employee_1 = Employee {
        emp_type: EmployeeTypes::Manager,
        status: EmployeeStatue::Employeed
    };

    let employee_2 = Employee {
        emp_type: EmployeeTypes::Line,
        status: EmployeeStatue::Employeed
    };

    let employee_3 = Employee {
        emp_type: EmployeeTypes::Marketing,
        status: EmployeeStatue::Terminated
    };

    print!("Employee 1: ");
    match get_access(employee_1) {
        Err(msg) => println!("{}", msg),
        _ => ()
    };

    print!("Employee 2: ");
    match get_access(employee_2) {
        Err(msg) => println!("{}", msg),
        _ => ()
    };

    print!("Employee 3: ");
    match get_access(employee_3) {
        Err(msg) => println!("{}", msg),
        _ => ()
    };
}