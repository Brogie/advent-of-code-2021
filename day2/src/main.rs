use std::fs::File;
use std::io::{BufRead, BufReader};

fn task1() -> i32 {
    let reader = BufReader::new(File::open("input/input.txt").expect("Cannot open file.txt"));
    let mut depth = 0;
    let mut breadth = 0;

    for line in reader.lines() {
        let order_string = line.unwrap();
        let order: Vec<&str> = order_string.split(" ").collect();
        match order[0] {
            "up" => depth -= order[1].parse::<i32>().unwrap(),
            "down" => depth += order[1].parse::<i32>().unwrap(),
            "forward" => breadth += order[1].parse::<i32>().unwrap(),
            _ => println!("reeee")
        }
    }

    return depth * breadth;
}

fn task2() -> i32 {
    let reader = BufReader::new(File::open("input/input.txt").expect("Cannot open file.txt"));
    let mut depth = 0;
    let mut breadth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let order_string = line.unwrap();
        let order: Vec<&str> = order_string.split(" ").collect();
        match order[0] {
            "up" => aim -= order[1].parse::<i32>().unwrap(),
            "down" => aim += order[1].parse::<i32>().unwrap(),
            "forward" => {
                breadth += order[1].parse::<i32>().unwrap();
                depth += aim * order[1].parse::<i32>().unwrap();
            },
            _ => println!("reeee")
        }
    }

    return depth * breadth;
}

fn main() {
    println!("Task 1: {}", task1());
    println!("Task 2: {}", task2());
}