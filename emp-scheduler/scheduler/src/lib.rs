use std::collections::HashMap;

use std::fmt;

const OPERATIONAL_DAYS: usize = 7;

const DAY_NUM_SHIFTS: usize = 3;

const SHIFT_MIN_EMPS: usize = 2;

const EMP_MAX_DAYS: usize = 5;

// const EMP_MAX_SHIFTS: usize = 1;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Days {

    pub fn iter() -> impl Iterator<Item = Days> {
        [Self::Monday, Self::Tuesday, Self::Wednesday, Self::Thursday,
            Self::Friday, Self::Saturday, Self::Sunday
        ].iter().copied()
    }
}

impl fmt::Display for Days {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Monday => write!(f, "Monday"),
            Self::Tuesday => write!(f, "Tuesday"),
            Self::Wednesday => write!(f, "Wednesday"),
            Self::Thursday => write!(f, "Thursday"),
            Self::Friday => write!(f, "Friday"),
            Self::Saturday => write!(f, "Saturday"),
            Self::Sunday => write!(f, "Sunday"),
        }
    }
    
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Shifts {
    Morning,
    Afternoon,
    Evening,
}

impl Shifts {

    pub fn iter() -> impl Iterator<Item = Shifts> {
        [Self::Morning, Self::Afternoon, Self::Evening].iter().copied()
    }
}

impl fmt::Display for Shifts {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Morning => write!(f, "Morning"),
            Self::Afternoon => write!(f, "Afternoon"),
            Self::Evening => write!(f, "Evening"),
        }
    }
    
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Employee {
    pub name: String,

    pub preferred_shift: Shifts,
}

impl fmt::Display for Employee {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} {}", self.name, self.preferred_shift)
    }
}

#[derive(Default)]
pub struct Scheduler {

    pub employees: Vec<Employee>,
}

impl Scheduler {

    pub fn get_schedule(self) -> Result<HashMap<Days, HashMap<Shifts, Vec<Employee>>>, String> {

        if self.employees.len() * EMP_MAX_DAYS  < OPERATIONAL_DAYS * DAY_NUM_SHIFTS * SHIFT_MIN_EMPS {
            // println!("Not enough employees to schedule for the entire week.");
            return Err("Not enough employees".into());
        }

        
        // for emp in &self.employees {
        //     println!("Employee: {}", emp);
        // }

        let mut shifts = HashMap::<Days, HashMap::<Shifts, Vec<Employee>>>::new();
        let mut emp_days = HashMap::<Employee, usize>::new();

        /*
        for emp in &self.employees {
            for day in Days::iter() {
                if EMP_MAX_DAYS > *emp_days.get(emp).unwrap_or(&0) {
                    for shift in Shifts::iter() {
                        if emp.preferred_shift == shift {
                           let mut day_shifts = shifts.entry(day).or_insert(HashMap::new());
                           let mut shift_emps = day_shifts.entry(shift).or_insert(Vec::new());

                           if shift_emps.len() < SHIFT_MIN_EMPS {
                               shift_emps.push(emp.clone());
                               *emp_days.entry(emp.clone()).or_insert(0) += 1;
                               break; //'day;
                           } 
                        }
                    } 
                } else {
                    break;
                }
            }
        }

        for day in Days::iter() {
            let mut day_shifts = shifts.entry(day).or_insert(HashMap::new());
            let mut day_employees: Vec<Employee> = Vec::new();

            for shift in Shifts::iter() {
                let mut shift_emps = day_shifts.entry(shift).or_insert(Vec::new());

                for shift_emp in shift_emps.iter() {
                    day_employees.push(shift_emp.clone());
                }
            }
            for shift in Shifts::iter() {
                let mut shift_emps = day_shifts.entry(shift).or_insert(Vec::new());

                if shift_emps.len() < SHIFT_MIN_EMPS {
                    let mut available_employees: Vec<_> = self.employees.iter().filter(|e| {
                       *emp_days.get(e).unwrap_or(&0) < EMP_MAX_DAYS && !day_employees.contains(e)
                    }).collect();

                    if available_employees.len() + shift_emps.len() < SHIFT_MIN_EMPS {
                        // println!("Not enough employees to schedule for {} - {}", day, shift);
                        return Err("Not enough employees".into());
                    }
                    
                    let new_emps = available_employees.into_iter()
                        .take(SHIFT_MIN_EMPS - shift_emps.len()).cloned().collect::<Vec<_>>();

                    for emp in new_emps.iter() {
                        day_employees.push(emp.clone());
                        *emp_days.entry(emp.clone()).or_insert(0) += 1;

                    }
                    shift_emps.extend(new_emps);
                }
            }
        }

        println!("Shifts: {:?}", shifts);
        println!("Emps: {:?}", emp_days);
        */

        
        for day in Days::iter() {
            // let employees = &self.employees.clone();
            let mut day_shifts = HashMap::<Shifts, Vec<Employee>>::new();
            let mut day_employees: Vec<Employee> = Vec::new();

            for shift in Shifts::iter() {
                let mut available_employees: Vec<_> = self.employees.iter().filter(|e| {
                   *emp_days.get(e).unwrap_or(&0) < EMP_MAX_DAYS && !day_employees.contains(e) && e.preferred_shift == shift
                }).collect();

                // Sort the list by the least scheduled employees
                available_employees.sort_by_key(|e| *emp_days.get(e).unwrap_or(&0));
                
                let mut shift_employees: Vec<_> = available_employees.into_iter().take(SHIFT_MIN_EMPS).cloned().collect();
                if shift_employees.len() < SHIFT_MIN_EMPS {
                    let mut available_employees: Vec<_> = self.employees.iter().filter(|e| {
                        *emp_days.get(e).unwrap_or(&0) < EMP_MAX_DAYS && !shift_employees.contains(e) && !day_employees.contains(e)
                    }).collect();

                    // Sort the list by the least scheduled employees
                    available_employees.sort_by_key(|e| *emp_days.get(e).unwrap_or(&0));
                    
                    if available_employees.len() + shift_employees.len() < SHIFT_MIN_EMPS {
                        // println!("Not enough employees to schedule for {} - {}", day, shift);
                        // println!("State: {:?}", shifts);
                        // println!("Employees: {:?}", emp_days);
                        return Err("Not enough employees".into());
                    }

                    shift_employees.extend(available_employees.into_iter()
                        .take(SHIFT_MIN_EMPS - shift_employees.len()).cloned().collect::<Vec<_>>());
                }
                for e in &shift_employees {
                    *emp_days.entry(e.clone()).or_insert(0) += 1;
                    day_employees.push(e.clone());
                }
                day_shifts.insert(shift, shift_employees);
                // println!("Day: {}, Shift: {}", day, shift);
            }
            shifts.insert(day, day_shifts);
        }
        

        Ok(shifts)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rng;
    use rand::prelude::IteratorRandom;

    fn get_random_shift() -> Shifts {
        let mut rng = rng();
        Shifts::iter().choose(&mut rng).unwrap()
    }

    fn create_employees(count: usize) -> Vec<Employee> {
        (0..count).map(|i| {
            Employee {
                name: format!("Emp {}", i),
                preferred_shift: get_random_shift(),
            }
        }).collect()
    }

    #[test]
    fn test_not_enough_employees() {
        let emps = create_employees(8);
        let mut scheduler = Scheduler { employees: emps };

        let result = scheduler.get_schedule();
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Not enough employees".into()));
    }

    #[test]
    fn test_schedule_possible() {
        let emps = create_employees(10);
        let mut scheduler = Scheduler { employees: emps };

        let result = scheduler.get_schedule();
        assert!(result.is_ok());
    }

    #[test]
    fn test_rules() {
        let emps = create_employees(10);

        let mut scheduler = Scheduler { employees: emps };

        let schedule = scheduler.get_schedule().expect("Schedule should be created");

        let mut emp_shifts = HashMap::new();

        for (day, day_shifts) in &schedule {
           let mut day_employees = Vec::new();

           for (shift, employees) in day_shifts {
               assert!(employees.len() >= SHIFT_MIN_EMPS, "Shift size too small, {}, {}", day, shift);

               for emp in employees {
                   *emp_shifts.entry(emp.name.clone()).or_insert(0) += 1;

                   day_employees.push(emp.name.clone());
               }
           }

           let day_emp_count = day_employees.len();
           day_employees.sort();
           day_employees.dedup();

           assert_eq!(day_emp_count, day_employees.len(), "Duplicate shift on {}", day);
        }

        for (name, count) in emp_shifts {
            assert!(count <= EMP_MAX_DAYS, "Employee {} worked too many days {}", name, count);
        }
    }
}
