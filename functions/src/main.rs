fn main() {
    print_labeled_measurement(add_one(five()), 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn add_one(x : i32) -> i32 {
    x + 1
}