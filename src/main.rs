use std::io;
use std::mem::transmute;

// How many players are playing? (1 or 2)
const PLAYERS: i8 = 1;

// Game
fn main() {
    let mut game_field = ["0","1","2","3","4","5","6","7","8"];
    // X always starts
    let mut turn = "O";
    let mut game_over = false;
    while !game_over {
        // Switch turns
        turn = match turn {
            "X" => "O",
            "O" => "X",
            _ => panic!("Error when switching turns")
        };

        // Print the game field
        print_field(game_field);

        println!("{} turn", turn);
        if PLAYERS == 1 && turn == "O" {
            let mut best_move = find_best_move(game_field);
            game_field[best_move] = turn;
            println!("AI played {}", best_move);
        } else {
            game_field[make_move(game_field)] = turn;
        }

        if check_win(game_field) {
            print_field(game_field);
            println!("{} wins!", turn);
            game_over = true;
        } else if check_draw(game_field) {
            print_field(game_field);
            println!("Draw!");
            game_over = true;
        } else {
            game_over = false;
        }
    }
}

fn print_field(game_field: [&str; 9]) {
    println!("{} {} {}", game_field[0], game_field[1], game_field[2]);
    println!("{} {} {}", game_field[3], game_field[4], game_field[5]);
    println!("{} {} {}", game_field[6], game_field[7], game_field[8]);
}

fn make_move(game_field: [&str; 9]) -> usize {
    println!("Enter a move: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid move!");
            make_move(game_field)
        }
    };
    if input > 8 {
        println!("Invalid move!");
        make_move(game_field);
    } else if game_field[input] == "X" || game_field[input] == "O" {
        println!("This field is already taken!");
        make_move(game_field);
    } else {
        return input;
    }

    return 7;
}

fn check_draw(game_field: [&str; 9]) -> bool {
    return match game_field {
        ["X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O"] => true,
        _ => false
    }
}

fn check_win(game_field: [&str; 9]) -> bool {
    const WIN_COMBOS : [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6]
    ];
    for combo in WIN_COMBOS.iter() {
        if game_field[combo[0]] == game_field[combo[1]] && game_field[combo[1]] == game_field[combo[2]] {
            return true;
        }
    }
    return false;
}

// Minimax algorithm
fn check_game_over(game_field: [&str; 9]) -> Option<i32> {
    return if check_win_mm(game_field, "O") {
        Some(10)
    } else if check_win_mm(game_field, "X") {
        Some(-10)
    } else if check_draw_mm(game_field) {
        Some(0)
    } else {
        None
    }
}

fn check_win_mm(game_field: [&str; 9], turn: &str) -> bool {
    const WIN_COMBOS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6]
    ];

    for combo in WIN_COMBOS.iter() {
        if game_field[combo[0]] == turn && game_field[combo[1]] == turn && game_field[combo[2]] == turn {
            return true;
        }
    }
    return false;
}

fn check_draw_mm(game_field: [&str; 9]) -> bool {
    return match game_field {
        ["X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O", "X" | "O"] => true,
        _ => false
    }
}

fn minimax(game_field: [&str; 9], depth: i32, turn: &str) -> i32 {
    let score = check_game_over(game_field);
    if score.is_some() {
        let new_score = score.unwrap();
        if new_score == 10 {
            return new_score - depth;
        } else if new_score == -10 {
            return new_score + depth;
        } else {
            return new_score;
        }
    } else {
        return if turn == "O" {
            let mut best_move = -f64::INFINITY;
            for i in 0..9 {
                if game_field[i] == " " {
                    let mut new_board = game_field;
                    new_board[i] = "O";
                    let move_value: i32 = minimax(new_board, depth + 1, "X");
                    new_board[i] = " ";
                    best_move = max(best_move, move_value as f64);
                }
            }
            best_move as i32
        } else {
            let mut best_move = f64::INFINITY;
            for i in 0..9 {
                if game_field[i] == " " {
                    let mut new_board = game_field;
                    new_board[i] = "X";
                    let move_value = minimax(new_board, depth + 1, "O");
                    new_board[i] = " ";
                    best_move = min(best_move, move_value as f64);
                }
            }
            best_move as i32
        }
    }
}

fn find_best_move(game_field: [&str; 9]) -> usize {
    let mut best_move: i32 = -1;
    let mut best_value = -f64::INFINITY;
    let mut new_board = make_new_board(game_field);
    for i in 0..9 {
        if new_board[i] == " " {
            new_board[i] = "O";
            let move_value = minimax(new_board, 0, "X");
            new_board[i] = " ";
            if move_value > best_value as i32 {
                best_move = i as i32;
                best_value = move_value as f64;
            }
        }
    }
    return best_move as usize;
}

fn max(v1: f64, v2: f64) -> f64 {
    if v1 > v2 {
        return v1;
    } else {
        return v2;
    }
}

fn min(v1: f64, v2: f64) -> f64 {
    if v1 < v2 {
        return v1;
    } else {
        return v2;
    }
}

fn make_new_board(game_field: [&str; 9]) -> [&str; 9] {
    let mut new_board = game_field;
    for i in 0..9 {
        if new_board[i] != "X" && new_board[i] != "O" {
            new_board[i] = " ";
        }
    }
    return new_board;
}