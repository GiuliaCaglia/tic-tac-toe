use std::io;

fn main() {
    // Create Board
    let mut board: Vec<Vec<i8>> = vec![vec![0; 3]; 3];

    // While not win_condition
    let mut active_player: i8 = 1;
    let mut outcome: i8 = 0;
    while outcome == 0 {
        // Play round
        board = player_turn(&board, active_player);

        // Mark cell
        active_player *= -1;
        println!("-------------");
        outcome = get_win_condition(&board);
    }

    if outcome == 1 {
        println!("Player 1 wins!")
    } else if outcome == -1 {
        println!("Player 2 wins!")
    } else if outcome == 99 {
        println!("Undecided!")
    }
    print_board(&board);
}

fn player_turn(board: &Vec<Vec<i8>>, active_player: i8) -> Vec<Vec<i8>> {
    let mut valid_turn = false;
    let mut new_board = board.clone();
    while !valid_turn {
        print_board(&new_board);
        let new_cell: u8 = get_cell();
        // Mark cell
        let row: usize = (new_cell / 3) as usize;
        let col: usize = (new_cell % 3) as usize;
        if new_board[row][col] == 0 {
            new_board[row][col] = active_player;
            valid_turn = true
        } else {
            println!("Cell is already taken!")
        }

    }
    return new_board;
}

fn print_board(board: &Vec<Vec<i8>>) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn get_cell() -> u8 {
    print!("Please select cell:\n");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let choice: u8 = input.trim().to_string().parse().unwrap();
    return choice;
}

fn get_win_condition(board: &Vec<Vec<i8>>) -> i8 {
    let row_sum: Vec<i8> = board.iter().map(|row| row.iter().sum()).collect();
    let mut col_sum: Vec<i8> = vec![0; 3];
    let mut diag_sum: i8 = 0;
    let mut diag_sum_inv: i8 = 0;
    for row in board {
        for (col, value) in row.iter().enumerate() {
            col_sum[col] += value;
        }
    }
    for i in [0, 1, 2] {
        let index: usize = i as usize;
        diag_sum += board[index][index];
    }
    for (a, b) in [(0, 2), (1, 1), (2, 0)] {
        let index_a: usize = a as usize;
        let index_b: usize = b as usize;
        diag_sum_inv += board[index_a][index_b];
    }

    let win_player_1: Option<i8> = Some(3);
    let win_player_2: Option<i8> = Some(-3);
    let board_full: bool = check_board_full(&board);
    if row_sum.iter().max() == win_player_1.as_ref() || col_sum.iter().max() == win_player_1.as_ref() {
        return 1
    }
    else if diag_sum == 3 || diag_sum_inv == 3 {
        return 1
    } else if diag_sum_inv == -3 || diag_sum == -3 {
        return -1
    }
    else if row_sum.iter().min() == win_player_2.as_ref() || col_sum.iter().min() == win_player_2.as_ref() {
        return -1
    }
    else if board_full {
        return 99
    }
    else {
        return 0
    }
}

fn check_board_full(board: &Vec<Vec<i8>>) -> bool {
    let mut board_full = true;
    for row in board {
        for col in row {
            if *col == 0 {
                board_full = false
            }
        }
    }

    return board_full;
}