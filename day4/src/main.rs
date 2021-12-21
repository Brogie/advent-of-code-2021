use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
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
    let (bingo, mut boards) = load_boards();

    for round_number in bingo {
        for mut board in &mut boards {
            //Cross off the values that have passed
            mark_number(round_number, &mut board);
            //Check win state
            if is_win(board) {
                return calculate_win(board) * round_number;
            }
        }
    }

    return 0;
}

fn task2() -> u32 {
    let (bingo, mut boards) = load_boards();
    let mut finished_boards = 0;
    let total_boards = boards.len();

    for round_number in bingo {
        for mut board in &mut boards {
            //Cross off the values that have passed
            mark_number(round_number, &mut board);
            //Check win state
            if is_win(board) {
                finished_boards += 1;
                if finished_boards == total_boards {
                    println!("{:?}", board);
                    println!("{} * {}", calculate_win(board), round_number);
                    return (calculate_win(board) * round_number) - round_number;
                }
            }
        }
        finished_boards = 0;
    }

    return 0;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn load_boards() -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
    let mut boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let input = lines_from_file("input/input.txt");

    let bingo = input[0].split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    boards.push(Vec::new());
    let mut board_index = 0;
    for i in 2..input.len() {
        if input[i] != "" {
            boards[board_index].push(
                input[i].split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()
            );
        } else {
            boards.push(Vec::new());
            board_index += 1;
        }
    }
    return (bingo, boards)
}

fn calculate_win(board: &mut Vec<Vec<u32>>) -> u32 {
    let mut output = 0;

    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if board[x][y] != 9999 {
                output += board[x][y];
            }
        }
    }

    return output
}

fn is_win(board: &mut Vec<Vec<u32>>) -> bool {
    //check horiz
    for x in 0..board.len() {
        let mut win = true;
        for y in 0..board[0].len() {
            if board[x][y] != 9999 {
                win = false;
                break
            }
        }
        if win { return true }
    }

    //check vert
    for x in 0..board.len() {
        let mut win = true;
        for y in 0..board[0].len() {
            if board[y][x] != 9999 {
                win = false;
                break
            }
        }
        if win { return true }
    }

    false
}

fn mark_number(round_number: u32, board: &mut &mut Vec<Vec<u32>>) {
    for row in 0..board.len() {
        for column in 0..board[0].len() {
            if board[row][column] == round_number {
                board[row][column] = 9999;
            }
        }
    }
}


