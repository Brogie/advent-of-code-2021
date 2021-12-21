use std::{cmp, fs};
use std::time::Instant;
use regex::Regex;

struct VentLine {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32
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

fn task1() -> u32 {
    let input = fs::read_to_string("input/sample.txt").expect("Unable to read file");
    let re = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)").unwrap();
    let mut grid:Vec<Vec<u32>> = vec![vec![0; 20]; 20];

    for capture in re.captures_iter(&input) {
        let vent = VentLine {
            x1: (&capture[1]).parse().unwrap(),
            y1: (&capture[2]).parse().unwrap(),
            x2: (&capture[3]).parse().unwrap(),
            y2: (&capture[4]).parse().unwrap()
        };

        if vent.y1 == vent.y2 {
            let (x_start, x_length) = get_range(vent.x1, vent.x2);
            for x in 0..x_length {
                grid[vent.y1 as usize][(x + x_start) as usize] = grid[vent.y1 as usize][(x + x_start) as usize] + 1;
            }
        }

        if vent.x1 == vent.x2 {
            let (y_start, y_length) = get_range(vent.y1, vent.y2);
            for y in 0..y_length {
                grid[(y + y_start) as usize][vent.x1 as usize] = grid[(y + y_start) as usize][vent.x1 as usize]+ 1;
            }
        }
    }

    let mut total_overlaps = 0;

    for y in 0..grid.len(){
        for x in 0..grid[0].len() {
            if grid[x][y] > 1 {
                total_overlaps = total_overlaps + 1;
            }
        }
    }

    return total_overlaps;
}

fn task2() -> i32 {
    let input = fs::read_to_string("input/sample.txt").expect("Unable to read file");
    let re = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)").unwrap();
    let mut grid:Vec<Vec<u32>> = vec![vec![0; 20]; 20];

    for capture in re.captures_iter(&input) {
        let vent = VentLine {
            x1: (&capture[1]).parse().unwrap(),
            y1: (&capture[2]).parse().unwrap(),
            x2: (&capture[3]).parse().unwrap(),
            y2: (&capture[4]).parse().unwrap()
        };

        let (mut x, mut y) = (vent.x1, vent.y1);

        grid[y as usize][x as usize] = grid[y as usize][x as usize] + 1;

        while x != vent.x2 && y != vent.y2{
            let (new_x, new_y) = next_coord(x,y,vent.x2, vent.y2);
            x = new_x;
            y = new_y;
            grid[y as usize][x as usize] = grid[y as usize][x as usize] + 1;
        }

        //grid[y as usize][x as usize] = grid[y as usize][x as usize] + 1;

    }

    for i in 0..grid.len() {
        println!("{:?}", grid[i]);
    }

    let mut total_overlaps = 0;

    for y in 0..grid.len(){
        for x in 0..grid[0].len() {
            if grid[x][y] > 1 {
                total_overlaps = total_overlaps + 1;
            }
        }
    }

    return total_overlaps;
}

fn next_coord(mut x: u32, mut y:u32, target_x:u32, target_y:u32) -> (u32, u32) {
    if x < target_x {
        x = x + 1;
    } else if x > target_x {
        x = x - 1;
    }

    if y < target_y {
        y = y + 1;
    } else if y > target_y {
        y = y - 1;
    }

    return (x,y);
}

fn get_range(start: u32, end: u32) -> (u32, u32) {
    return (cmp::min(start, end), ((start as i32 - end as i32).abs() + 1) as u32);
}
