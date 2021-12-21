use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn task1() -> u32 {
    let reader = BufReader::new(File::open("input/input.txt").expect("Cannot open file.txt"));
    let mut sum = vec![0; 12];
    let mut total_lines = 0;

    for line in reader.lines() {
        let number: Vec<u32> = line.unwrap()
            .chars()
            .map(|character| character.to_digit(10).unwrap())
            .collect();

        for i in 0..number.len() {
            sum[i] += number[i];
        }

        total_lines += 1;
    }

    let mut gamma:u32 = 0;
    let mut epsilon:u32 = 0;

    for i in 0..sum.len() {
        if sum[i] > total_lines / 2 {
            gamma = gamma | (1<<(11-i));
        } else {
            epsilon = epsilon | (1<<(11-i));
        }
    }

    return epsilon * gamma;
}

fn task2() -> u32 {
    let reader = BufReader::new(File::open("input/input.txt").expect("Cannot open file.txt"));
    let mut oxygen:Vec<u32> = Vec::new();
    let mut co2:Vec<u32> = Vec::new();

    for line in reader.lines() {
        let number_string = line.unwrap();
        let number_value = isize::from_str_radix(&*number_string, 2).unwrap() as u32;
        oxygen.push(number_value);
        co2.push(number_value);
    }

    for i in 0..12 {
        let oxygen_total_of_ones = sum_of_column(&oxygen, i);
        let co2_total_of_ones = sum_of_column(&co2, i);

        let mut oxygen_bit = 0;
        let mut co2_bit = 0;

        if oxygen_total_of_ones >= oxygen.len() as u32 / 2 {
            oxygen_bit = 1;
        }

        if co2_total_of_ones < co2.len() as u32 / 2 {
            co2_bit = 1;
        }

        if oxygen.len() > 1 {
            oxygen.retain(|&number| {
                number & (1 << (11 - i)) == (oxygen_bit << (11 - i))
            });
        }

        if co2.len() > 1 {
            co2.retain(|&number| {
                number & (1 << (11 - i)) == (co2_bit << (11 - i))
            });
        }
    }

    return oxygen[0] * co2[0];

}

fn sum_of_column(to_sum:&Vec<u32>, column:u8) -> u32{
    let mut output = 0;
    for number in to_sum {
        if number & 1 << 11 - column != 0 {
            output += 1;
        }
    }

    return output;
}

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
