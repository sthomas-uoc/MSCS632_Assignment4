use std::io;
use std::io::Write;

use scheduler::{Days, Shifts, Employee, Scheduler};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");

    buffer.trim().into()
}

fn main() {

    println!("Employee scheduler!");
    
    // Store employees
    let mut employees: Vec<Employee> = Vec::new();

    // Read employees
    println!("Enter employees and shift preferences");

    loop {
        let name = read_input("Enter Employee Name or 'run': ");
        if name.eq_ignore_ascii_case("run") {
            break;
        }
        if name.is_empty() {
            continue;
        }

        let preferred_shift = read_input("Enter Preferred Shift (M/A/E): ");

        let preferred_shift = match preferred_shift.to_lowercase().as_str() {
            "m" => Shifts::Morning,
            "a" => Shifts::Afternoon,
            "e" => Shifts::Evening,
            _ => {
                println!("Invalid shift! Setting as Morning");
                Shifts::Morning
            }
        };

        employees.push(Employee { name, preferred_shift });
    }

    // Scheduler
    let scheduler = Scheduler { employees };

    // Get Schedule
    let schedule = scheduler.get_schedule();

    match schedule {
        Ok(schedule) => {
            // Print schedule
            println!("Schedule calculated");
            for day in Days::iter() {
                println!("--- {} ---", day);
                for shift in Shifts::iter() {
                    let workers = &schedule[&day][&shift];
                    print!("{:<10}: ", format!("{:}", shift));
                    
                    let names: Vec<String> = workers.iter().map(|w| w.name.clone()).collect();
                    println!("{}", names.join(", "));
                }
            }
        },
        Err(e) => {
            // No schedule found
            println!("Error getting schedule: {}", e);
        }
    }

}
