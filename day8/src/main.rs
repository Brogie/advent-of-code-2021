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

    let mut output = 0;

    for line in reader.lines() {
        for display in line.unwrap().split(" | ").nth(1).unwrap().split_whitespace() {

            if display.len() != 5 && display.len() != 6 {
                output += 1;
            }
        }
        //println!("{:?}", line.unwrap().split(" | ").nth(1))
    }

    output
}

fn task2() -> i32 {
    let file = File::open("input/input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut output = 0;

    for line in reader.lines() {
        let line_string = line.unwrap();
        let mut known_numbers: Vec<String> = ["","","","","","","","","",""].iter().map(|&s|s.into()).collect();
        let mut code = Vec::new();
        let mut fives = Vec::new();
        let mut sixes = Vec::new();

        let mut display = line_string.split(" | ");
        // Load numbers and map known values
        for broken_number in display.nth(0).unwrap().split_whitespace() {
            match broken_number.len() {
                2 => known_numbers[1] = broken_number.parse().unwrap(),
                3 => known_numbers[7] = broken_number.parse().unwrap(),
                4 => known_numbers[4] = broken_number.parse().unwrap(),
                7 => known_numbers[8] = broken_number.parse().unwrap(),
                5 => fives.push(broken_number.to_owned()),
                6 => sixes.push(broken_number.to_owned()),
                _ => {}
            }
        }
        //Load result
        display = line_string.split(" | ");
        for output_number in display.nth(1).unwrap().split_whitespace() {
            code.push(output_number)
        }

        // Unknown numbers 2,3,5,6,9,0
        // Find 6
        let mut remove_index = 0;
        for i in 0..sixes.len() {
            if minus_length(sixes[i].to_owned(), known_numbers[1].to_owned()) == 5 {
                known_numbers[6] = sixes[i].to_owned();
                remove_index = i;
                break;
            }
        }
        sixes.remove(remove_index);

        // Find 5
        for i in 0..fives.len() {
            if minus_length(fives[i].to_owned(), known_numbers[6].to_owned()) == 0 {
                known_numbers[5] = fives[i].to_owned();
                remove_index = i;
                break;
            }
        }
        fives.remove(remove_index);

        // Find 9
        let nine_key = minus(known_numbers[6].to_owned(), known_numbers[5].to_owned());
        let mut remove_index = 0;
        for i in 0..sixes.len() {
            if minus_length(sixes[i].to_owned(), nine_key.to_owned()) == 6 {
                known_numbers[9] = sixes[i].to_owned();
                remove_index = i;
                break;
            }
        }
        sixes.remove(remove_index);

        // Find 0
        known_numbers[0] = sixes[0].to_owned();

        // Find 2
        for i in 0..fives.len() {
            if minus_length(fives[i].to_owned(), known_numbers[5].to_owned()) == 2 {
                known_numbers[2] = fives[i].to_owned();
                remove_index = i;
                break;
            }
        }
        fives.remove(remove_index);

        // Find 3
        known_numbers[3] = fives[0].to_owned();

        let mut decrypted:String = String::new();
        for i in 0..code.len() {
            for j in 0..known_numbers.len() {
                if is_match(code[i].to_owned(), known_numbers[j].to_owned()) {
                    decrypted = format!("{}{}", decrypted, j);
                }
            }
        }

        output += decrypted.parse::<i32>().unwrap();
    }

    output
}

fn is_match(a: String, b:String) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for letter in b.chars() {
        if !a.contains(letter) {
            return false;
        }
    }

    true
}

fn minus_length(mut a: String, b: String) -> u8 {
    for letter in b.chars() {
        a = a.replace(letter, "")
    }

    a.len() as u8
}

fn minus(mut a: String, b: String) -> String {
    for letter in b.chars() {
        a = a.replace(letter, "")
    }

    a
}
