use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let result1 = task1();
    let time1 = now.elapsed().as_millis();

    println!("Task 1: {} in {}ms", result1, time1);

    now = Instant::now();
    let result2 = task2();
    let time2 = now.elapsed().as_millis();

    println!("Task 2: {} in {}ms", result2, time2);
}

fn task1() -> i32 {
    let file = File::open("input/input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let crabs: Vec<i32> =
        reader.lines().next().unwrap().unwrap()
            .split(',').map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

    let mut smallest_fuel_use = 999999;
    for i in 0..crabs.len() {
        let mut fuel_used = 0;
        let alignment = crabs[i];

        for j in 0..crabs.len() {
            fuel_used += (alignment - crabs[j]).abs();
            if fuel_used > smallest_fuel_use {
                break;
            }
        }

        if fuel_used < smallest_fuel_use {
            smallest_fuel_use = fuel_used;
        }
    }

    return smallest_fuel_use;
}

fn task2() -> u64 {
    let file = File::open("input/input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let crabs: Vec<i32> =
        reader.lines().next().unwrap().unwrap()
            .split(',').map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

    let mut biggest = 0;

    for i in 0..crabs.len() {
        if crabs[i] > biggest {
            biggest = crabs[i];
        }
    }

    let mut smallest_fuel_use = 18446744073709551615;
    for i in 0..=biggest {
        let mut fuel_used = 0;
        let alignment = i;

        for j in 0..crabs.len() {
            fuel_used += factorialish((alignment - crabs[j]).abs() as u64);
            if fuel_used > smallest_fuel_use {
                break;
            }
            //println!("Move from {} to {}: {} fuel,", crabs[j], alignment, fuel_used);
        }

        if fuel_used < smallest_fuel_use {
            smallest_fuel_use = fuel_used;
        }
    }

    return smallest_fuel_use;
}

fn factorialish(b: u64) -> u64 {
    let mut output = 0;
    for i in 1..=b {
        output += i;
    }

    return output
}