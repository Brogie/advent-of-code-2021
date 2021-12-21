use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let result1 = task1();
    let time1 = now.elapsed().as_millis();

    now = Instant::now();
    let result2 = task2();
    let time2 = now.elapsed().as_millis();


    println!("Task 1: {} in {}ms", result1, time1);
    println!("Task 2: {} in {}ms", result2, time2);
}

fn task1() -> u32 {
    let file = File::open("input/input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut fish: Vec<u64> =
        reader
            .lines().next().unwrap().unwrap()
            .split(',').map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

    for _round in 0..80 {
        //println!("Round: {}", _round + 1);
        let mut new_fish = 0;

        //breed fish
        for i in 0..fish.len() {
            if fish[i] == 0 {
                new_fish += 1;
                fish[i] = 6;
            } else {
                fish[i] -= 1;
            }
        }
        //add new fish
        fish.append(&mut vec![8; new_fish]);
        // println!("{:?}", fish);
    }

    return fish.len() as u32;
}

fn task2() -> u64 {
    let file = File::open("input/input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let input: Vec<u64> =
        reader
            .lines().next().unwrap().unwrap()
            .split(',').map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

    let mut fish:Vec<u64> = vec![0; 9];

    for i in 0..input.len() {
        fish[input[i] as usize] += 1;
    }

    for _round in 0..256 {
        //println!("Round: {}", _round);
        let new_fish = fish[0];

        for i in 1..fish.len() {
            fish[i-1] = fish[i];
        }

        fish[8] = new_fish;
        fish[6] += new_fish;
        //println!("{:?}", fish)
    }

    let mut output = 0;

    for i in 0..fish.len() {
        output += fish[i];
    }

    return output;
}
