use std::io;
use std::io::Write;

const UNICODE_A: u8 = 97;
const BOARD_BLANK: char = '.';
const PLAYER_1: char = 'B';
const PLAYER_2: char = 'W';
const INVALID_MOVE_ERR: &str = "Invalid move. Try again.";
fn main() {
    let mut board = create_board();
    let (mut black_done, mut white_done) = (false, false);
    let mut curr_player = PLAYER_1;

    loop {
        if !valid_move_exists(&board, curr_player) {
            if curr_player == PLAYER_1 {
                black_done = true;
            } else {
                white_done = true;
            }
            println!("{} player has no valid move.", curr_player);

            // Check if the game is over
            if black_done && white_done {
                let (black_score, white_score) = tally_score(&board);
                display_board(&board);

                match black_score.cmp(&white_score) {
                    std::cmp::Ordering::Less => {
                        println!("White wins by {} points!", white_score - black_score)
                    }
                    std::cmp::Ordering::Greater => {
                        println!("Black wins by {} points!", black_score - white_score)
                    }
                    std::cmp::Ordering::Equal => println!("Draw!"),
                }

                break;
            }

            curr_player = if curr_player == PLAYER_1 {
                PLAYER_2
            } else {
                PLAYER_1
            };
            continue;
        }

        display_board(&board);

        print!("Enter move for colour {} (RowCol): ", curr_player);
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();

        match modify_board(&mut board, input, curr_player) {
            Ok(_) => (),
            Err(_) => {
                println!("{}", INVALID_MOVE_ERR);
                continue;
            }
        }

        curr_player = if curr_player == PLAYER_1 {
            PLAYER_2
        } else {
            PLAYER_1
        };
    }
}

fn create_board() -> [[char; 8]; 8] {
    let mut board = [[BOARD_BLANK; 8]; 8];

    board[3][3] = PLAYER_2;
    board[3][4] = PLAYER_1;
    board[4][3] = PLAYER_1;
    board[4][4] = PLAYER_2;

    board
}

fn display_board(board: &[[char; 8]; 8]) {
    println!("  abcdefgh");

    for (index, &row) in board.iter().enumerate() {
        print!("{} ", char::from(index as u8 + UNICODE_A));
        for &cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}

fn modify_board(board: &mut [[char; 8]; 8], user_move: &str, colour: char) -> Result<(), ()> {
    let mut user_moves = user_move.chars();

    let row = match user_moves.next() {
        Some(c) if (c as u8) >= UNICODE_A => (c as u8 - UNICODE_A) as usize,
        _ => return Err(()),
    };

    let col = match user_moves.next() {
        Some(c) if (c as u8) >= UNICODE_A => (c as u8 - UNICODE_A) as usize,
        _ => return Err(()),
    };

    if user_moves.next() != None {
        return Err(());
    }

    if row > 7 || col > 7 {
        return Err(());
    }

    if board[row as usize][col as usize] != BOARD_BLANK {
        return Err(());
    }

    let mut valid_move = false;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let (mut row, mut col) = (row as i8 + i, col as i8 + j);
            let mut positions_to_flip = Vec::new();

            while row >= 0 && row < 8 && col >= 0 && col < 8 {
                match board[row as usize][col as usize] {
                    BOARD_BLANK => break,
                    c if c == colour => {
                        for &(r, c) in &positions_to_flip {
                            valid_move = true;
                            board[r as usize][c as usize] = colour;
                        }
                        break;
                    }
                    _ => positions_to_flip.push((row, col)),
                }
                row += i;
                col += j;
            }
        }
    }

    if !valid_move {
        return Err(());
    }

    board[row as usize][col as usize] = colour;

    Ok(())
}

fn valid_move_exists(board: &[[char; 8]; 8], colour: char) -> bool {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] != BOARD_BLANK {
                continue;
            }

            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let (mut row, mut col) = (row as i8 + i, col as i8 + j);
                    let mut has_positions_to_flip = false;

                    while row >= 0 && row < 8 && col >= 0 && col < 8 {
                        match board[row as usize][col as usize] {
                            BOARD_BLANK => break,
                            c if c == colour => {
                                if has_positions_to_flip {
                                    return true;
                                }
                                break;
                            }
                            _ => has_positions_to_flip = true,
                        }
                        row += i;
                        col += j;
                    }
                }
            }
        }
    }

    false
}

fn tally_score(board: &[[char; 8]; 8]) -> (i32, i32) {
    let mut black_count = 0;
    let mut white_count = 0;

    for &row in board.iter() {
        for &cell in row.iter() {
            match cell {
                PLAYER_1 => black_count += 1,
                PLAYER_2 => white_count += 1,
                _ => (),
            }
        }
    }

    (black_count, white_count)
}
