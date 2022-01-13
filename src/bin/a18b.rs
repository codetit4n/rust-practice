// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    MaintenanceCrew,
    MartketingDept,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    emp_type: EmployeeType,
    still_employed: Status,
}

fn building_access(emp: &Employee) -> Result<(), String> {
    match emp.still_employed {
        Status::Terminated => return Err("Employee terminated!!!".to_owned()),
        _ => (),
    }
    match emp.emp_type {
        EmployeeType::MaintenanceCrew => Ok(()),
        EmployeeType::MartketingDept => Ok(()),
        EmployeeType::Manager => Ok(()),
        _ => Err("access denied!!!".to_owned()),
    }
}

fn print_access(emp: &Employee) -> Result<(), String> {
    let attempt = building_access(emp)?;
    println!("access ok");
    Ok(())
}
fn main() {
    let emp1 = Employee {
        emp_type: EmployeeType::Manager,
        still_employed: Status::Active,
    };
    match print_access(&emp1) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }
}
