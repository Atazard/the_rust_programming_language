use std::{collections::HashMap, io::stdin};

fn main() {
    let list = [4, 3, 1, 4, 2];

    let average = find_average(&list);
    println!("average: {}", average);

    let median = find_median(&list);
    println!("median: {}", median);

    let mode = find_mode(&list);
    println!("mode: {}", mode);

    println!();

    let string = "test".to_string();
    let pig_string = convert_to_pig(string);
    println!("pig string: {}", pig_string);

    println!();

    //TODO figure out the last excersise
    let _departments = ["Engineering", "Sales", "HR"];
    let mut company = HashMap::new();

    loop {
        print_menu();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line.");
        let input = input.trim();
        let input_parts: Vec<&str> = input.split_whitespace().collect();

        if input_parts.is_empty() {
            println!("Invalid command. Try again.\n");
            continue;
        }

        match input_parts[0].to_lowercase().as_str() {
            "add" => {
                if input_parts.len() < 4 || input_parts[2].to_lowercase() != "to" {
                    println!(
                        "Invalid command. Please use the format: 'Add [Name] to [Department]'\n"
                    );
                    continue;
                }

                add_employee_to_department(&mut company, input_parts[1], input_parts[3]);
            }
            "list" => {
                if input_parts.len() < 2 {
                    println!("Invalid command. Please use 'List [Department]' or 'List All'\n");
                    continue;
                }

                match input_parts[1].to_lowercase().as_str() {
                    "all" => {
                        list_all_employees(&company);
                    }
                    department => {
                        list_employees_of_department(&company, department);
                    }
                }
            }
            "exit" => {
                println!("Goodbye.\n");
                break;
            }
            _ => {
                println!("Invalid command. Try again.\n");
            }
        }
    }
}

fn list_employees_of_department(company: &HashMap<String, Vec<String>>, department: &str) {
    if let Some(employees) = company.get(department) {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();

        println!("Employees in {}:", department);
        for employee in sorted_employees {
            println!("\t{}", employee);
        }
        println!();
    } else {
        println!("Department: {} not found!", department);
    }
}

fn list_all_employees(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<&String> = company.keys().collect();

    if departments.is_empty() {
        println!("No employees in any department!");
    }

    departments.sort();

    for department in departments {
        list_employees_of_department(company, department);
    }
}

fn add_employee_to_department(
    company: &mut HashMap<String, Vec<String>>,
    name: &str,
    department: &str,
) {
    company
        .entry(department.to_string())
        .or_insert(Vec::new())
        .push(name.to_string());
    println!("Added {} to {}\n", name, department);
}

fn print_menu() {
    println!("1. Add employee to department. ('Add [Name] to [Department]')");
    println!("2. List employees. ('List [Department]' or 'List all')");
    println!("0. Exit.");
    println!("Enter command:");
}

#[test]
fn test_convert_to_pig() {
    assert_eq!(convert_to_pig("first".to_string()), "irst-fay");
    assert_eq!(convert_to_pig("apple".to_string()), "apple-hay");
}

fn convert_to_pig(string: String) -> String {
    let tmp = &string[..1];
    match tmp {
        "a" | "e" | "i" | "o" | "u" => string + "-hay",
        _ => string[1..].to_string() + "-" + tmp + "ay",
    }
}

fn find_mode(list: &[i32]) -> i32 {
    let mut num_count = HashMap::new();
    for i in list {
        let count = num_count.entry(i).or_insert(0);
        *count += 1;
    }

    // println!("num_count: {:#?}", num_count);

    // let tmp: i32 = num_count.values().max_by_key(|x| x.abs()); I can't figure this out at this point

    // Let's see what stackoverflow has to offer:
    // let tmp = num_count
    //     .iter()
    //     .max_by(|a, b| a.1.cmp(b.1))
    //     .map(|(k, _v)| k)
    //     .unwrap();
    // *tmp

    // I think I understand this a bit better
    let tmp = num_count.iter().max_by_key(|x| x.1).unwrap(); // x.0 and x.1 appear to be the same
    **tmp.0
}

#[test]
fn test_find_mode() {
    assert_eq!(find_mode(&[1]), 1);
    assert_eq!(find_mode(&[1, 2, 1]), 1);
    // assert_eq!(find_mode(&[1, 2]), 2); // this sometimes returns 1 and sometimes 2
}

#[test]
fn test_find_median() {
    assert_eq!(find_median(&[1, 2, 3, 4, 5]), 3);
    assert_eq!(find_median(&[5, 4, 3, 2, 1]), 3);
    assert_eq!(find_median(&[3, 1, 4, 5, 2]), 3);
    assert_eq!(find_median(&[1, 2, 3, 4, 5, 6]), 4);
}

fn find_median(list: &[i32]) -> i32 {
    let mut v = list.to_vec();
    v.sort();
    v[v.len() / 2]
}

#[test]
fn test_find_average() {
    assert_eq!(find_average(&[1, 2, 3]), 2.0);
    assert_eq!(find_average(&[1, 2, 3, 4, 5]), 3.0);
    assert_eq!(find_average(&[1, 2, 3, 4, 5, 6, 7]), 4.0);
}

fn find_average(list: &[i32]) -> f32 {
    let v = list.to_vec();
    let mut sum = 0.0;
    for i in &v {
        sum += *i as f32;
    }
    sum / v.len() as f32
}
