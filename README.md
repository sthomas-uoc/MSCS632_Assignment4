# Code for MSCS 632 Assignment 4
Implement Employee Scheduler n Python and Rust

## Build and run instructions

### Python
The Python implementation is in the file employee_scheduler.py
As this is a Python program, Python version 3 or higher is required.

To run the project run `python employee_scheduler.py`

The application guides the user through the steps to input employees and run the scheduler.

### Rust
The Rust implementation is in the Cargo worspace `emp-scheduler`

The implementation is split into 3 projects, the core logic as the library project `scheduler`
and the CLI and GUI implementations.

The CLI implementation is available as the workspace project `scheduler-cli`

To run the CLI application, from the cargo workspace run `cargo run -p scheduler-cli` --release

The application guides the user through the steps to input employees and run the scheduler.

The GUI implementation is available as the workspace project `scheduler-ui`

The egui library and eframe framework are required to run the application.

Requirement for eframe are avaiable at https://github.com/emilk/egui/tree/main/crates/eframe

To run the GUI application, from the Cargo workspace run `cargo run -p scheduler-ui --release`

The top section of the application allows for adding an employee by 
providing their name and shift preference and clicking the `Add Employee` button.
The number of employees added is displayed as a label. All added employees can be
removed by clicking the `Clear Employee` button. Once employees are added, 
clicking on the `Get Schedule` button provides the schedule in a table
under the `Schedule` section. If a schedule could not be generated, the label in the
`Schedule` section will highlight the error.

