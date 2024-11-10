use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
    salary: f64,
}

impl std::cmp::PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age && self.salary == other.salary
    }
}

impl std::cmp::PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl std::cmp::Eq for Employee {}

impl std::cmp::Ord for Employee {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.salary > other.salary {
            Ordering::Greater
        } else if (self.salary - other.salary).abs() < 0.0001 {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

fn main() {
    let mut employees = (1..100)
        .map(|_| Employee {
            name: String::from("John"),
            age: 18,
            salary: 5000.0,
        })
        .collect::<Vec<Employee>>();

    employees.push(Employee {
        name: String::from("John"),
        age: 25,
        salary: 6000.0,
    });

    println!("{:?}", employees.iter().max());
}
