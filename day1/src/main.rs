use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    return numbers;
}

fn task1() -> i32 {
    let numbers = load_from_file("input/input.txt");
    let mut increase_count = 0;

    for i in 1..numbers.len() {
        if numbers[i] > numbers[i-1] { increase_count += 1 }
    }

    return increase_count;
}

fn task2() -> i32 {
    let numbers = load_from_file("input/input.txt");
    let mut increase_count = 0;

    for i in 3..numbers.len() {
        if numbers[i-3] + numbers[i-2] + numbers[i-1] < numbers[i-2] + numbers[i-1] + numbers[i] { increase_count += 1 }
    }

    return increase_count;
}

fn main() {
    println!("[Day 1]");
    println!("Task 1: {}", task1());
    println!("Task 2: {}", task2());
}
