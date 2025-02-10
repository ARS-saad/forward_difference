use std::io;
use std::io::BufRead;

pub fn input() -> Vec<i32> {
    let mut input = String::new();
    let stdin = io::stdin();

    println!("Give some value: ");
    stdin
        .lock()
        .read_line(&mut input)
        .expect("There have no value");

    let elements: Vec<&str> = input.trim().split_whitespace().collect();

    let mut num_arr: Vec<i32> = Vec::new();
    for element in &elements {
        num_arr.push(element.parse::<i32>().unwrap());
    }
    // println!("{:?}", elements);
    num_arr
}
