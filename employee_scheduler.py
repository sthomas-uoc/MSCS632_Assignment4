from enum import Enum
from collections import defaultdict

OPERATIONAL_DAYS = 7

DAY_NUM_SHIFTS = 3

SHIFT_MIN_EMPS = 2

EMP_MAX_DAYS = 5

class Days(Enum):
    MONDAY = 1
    TUESDAY = 2
    WEDNESDAY = 3
    THURSDAY = 4
    FRIDAY = 5
    SATURDAY = 6
    SUNDAY = 7

    def __str__(self):
        return f"{self.name.capitalize()}"

class Shifts(Enum):
    MORNING = 1
    AFTERNOON = 2
    EVENING = 3

    def __str__(self):
        return f"{self.name.capitalize()}"

class Employee:

    def __init__(self, name, preferred_shift):
        self.name = name
        self.preferred_shift = preferred_shift

class EmployeeScheduler:

    def __init__(self, employees):
        self.employees = employees

    def get_schedule(self):

        if len(self.employees) * EMP_MAX_DAYS < OPERATIONAL_DAYS * DAY_NUM_SHIFTS * SHIFT_MIN_EMPS:
            raise ValueError("Not enough employees")

        shifts = defaultdict()
        emp_days = defaultdict(int) # Values default to 0

        for day in Days:

            day_shifts = defaultdict()
            day_employees = []

            for shift in Shifts:
                available_employees = [
                    emp.name for emp in self.employees
                    if emp_days.setdefault(emp.name, 0) < EMP_MAX_DAYS
                    and emp.name not in day_employees
                    and emp.preferred_shift == shift
                ]

                # Sort the list by the least scheduled employees
                available_employees = sorted(available_employees, key=lambda emp: emp_days.get(emp, 0))

                shift_employees = available_employees[:SHIFT_MIN_EMPS]

                if len(shift_employees) < SHIFT_MIN_EMPS:
                    available_employees = [
                        emp.name for emp in self.employees
                        if emp_days.setdefault(emp.name, 0) < EMP_MAX_DAYS
                        and emp.name not in day_employees
                        and emp.name not in shift_employees
                    ]
                    
                    # Sort the list by the least scheduled employees
                    available_employees = sorted(available_employees, key=lambda emp: emp_days.get(emp, 0))

                    if len(available_employees) + len(shift_employees) < SHIFT_MIN_EMPS:
                        raise ValueError("Not enough employees")

                    shift_employees.extend(available_employees[:SHIFT_MIN_EMPS - len(shift_employees)])

                for emp in shift_employees:
                    emp_days[emp] += 1
                    day_employees.append(emp)

                day_shifts[shift] = shift_employees

            shifts[day] = day_shifts

        return shifts
        
def read_enum(enum_type, prompt):

    while True:
        user_input = input(prompt)

        try:
            enum_input = enum_type[user_input.upper()]
            return enum_input
        except KeyError:
            print(f"Invalid value for {enum_type}")

    
if __name__ == "__main__":

    print("Employee Scheduler!")

    # employees = [
    #     Employee("1", Shifts.MORNING),
    #     Employee("2", Shifts.MORNING),
    #     Employee("3", Shifts.MORNING),
    #     Employee("4", Shifts.MORNING),
    #     Employee("5", Shifts.MORNING),
    #     Employee("11", Shifts.MORNING),
    #     Employee("22", Shifts.MORNING),
    #     Employee("33", Shifts.MORNING),
    #     Employee("44", Shifts.MORNING),
    #     Employee("55", Shifts.MORNING),
    #     Employee("111", Shifts.MORNING),
    # ]

    employees = []

    print("Enter employees and shift preferences")

    while True:

        name = input("Enter Employee Name or 'run': ")
        if name.lower() == "run":
            break
        elif not name:
            continue

        preferred_shift = input("Enter Preferred Shift (M/A/E): ")

        match preferred_shift.lower():
            case "m":
                preferred_shift = Shifts.MORNING
            case "a":
                preferred_shift = Shifts.AFTERNOON
            case "e":
                preferred_shift = Shifts.EVENING
            case _:
                print("Invalid shift! Setting as Morning")
                preferred_shift = Shifts.MORNING

        employees.append(Employee(name, preferred_shift))

    # Scheduler
    scheduler = EmployeeScheduler(employees)

    # Get Schedule
    try:
        schedule = scheduler.get_schedule()

        print(f"Schedule calculated")

        for day in Days:
            print(f"--- {day} ---")
            for shift in Shifts:
                emps = schedule[day][shift]
                print(f"{shift:<10} : {', '.join(emps)}")

    except Exception as e:
        # No schedule found
        print(f"Error getting schedule: {e}")
