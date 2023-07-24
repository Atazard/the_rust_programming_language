use std::collections::HashMap;

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

    let _departments = ["Engineering", "Sales", "HR"];
    //TODO figure out the last excersise
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
