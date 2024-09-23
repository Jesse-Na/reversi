use std::io;
use std::io::Write;

const UNICODE_A: u8 = 97;
const BOARD_BLANK: char = '.';
const INVALID_MOVE_ERR: &str = "Invalid move. Try again.";
fn main() {
    let mut board = create_board();

    display_board(&board);

    loop {
        // Black's Turn
        loop {
            print!("Enter move for colour B (RowCol): ");
            io::stdout().flush().expect("Failed to flush stdout.");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            let input = input.trim();

            match modify_board(&mut board, input, 'B') {
                Ok(_) => break,
                Err(_) => {
                    println!("{}", INVALID_MOVE_ERR);
                    display_board(&board);
                }
            }
        }

        display_board(&board);

        // White's Turn
        loop {
            print!("Enter move for colour W (RowCol): ");
            io::stdout().flush().expect("Failed to flush stdout.");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            let input = input.trim();

            match modify_board(&mut board, input, 'W') {
                Ok(_) => break,
                Err(_) => {
                    println!("{}", INVALID_MOVE_ERR);
                    display_board(&board);
                }
            }
        }

        display_board(&board);

        // Check if the game is over

    }
}

fn create_board() -> [[char; 8]; 8] {
    let mut board = [[BOARD_BLANK; 8]; 8];

    board[3][3] = 'W';
    board[3][4] = 'B';
    board[4][3] = 'B';
    board[4][4] = 'W';

    board
}

fn display_board(board: &[[char; 8]; 8]) {

    println!("  abcdefgh");

    for (index, &row) in board.iter().enumerate() {
        print!("{} ", char::from(index as u8 + UNICODE_A));
        for cell in row.iter() {
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
            let mut positions_to_flip: Vec<(usize, usize)> = Vec::new();

            while row >= 0 && row < 8 && col >= 0 && col < 8 {
                match board[row as usize][col as usize] {
                    BOARD_BLANK => break,
                    c if c == colour => {
                        for &(r, c) in &positions_to_flip {
                            valid_move = true;
                            board[r][c] = colour;
                        }
                        break;
                    }
                    _ => positions_to_flip.push((row as usize, col as usize)),
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
