use std::io;

struct Game {
    board: [[char; 3]; 3],
    current_player: bool,
    players: [char; 2],
}

fn main() {
    let mut game = Game {
        board: [[' '; 3]; 3],
        current_player: true,
        players: ['X', 'O'],
    };
    println!("Tic Tac Toe Game");
    println!("Player 1: X");
    println!("Player 2: O");
    println!("Enter your move in the format 'row col' (0-indexed).");
    run(&mut game);
}

fn run(game: &mut Game) {
    while !game_over(game) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let x: usize = parts.next().unwrap().trim().parse().unwrap();
        let y: usize = parts.next().unwrap().trim().parse().unwrap();
        if x > game.board.len() - 1 || y > game.board[0].len() - 1
        || game.board[x][y] != ' ' {
            println!("aim out of range or taken, try again");
            continue;
        }
        game.board[x][y] = game.players[
            if game.current_player == true {0} else {1}
        ];
        game.current_player = if game.current_player == true {false} else {true};
        print_board(game);
    }
}

fn game_over(game : &mut Game) -> bool {
    if game.players.is_empty() { return true; }
    if check_end_game(game) { return true; }
    for i in game.board {
        for y in i {
            if y == ' ' {
                return false
            } else { continue }
        }
    }
    print!("tie");
    return true
}

fn print_board(game: &mut Game) {
    let board = format!("{}|{}|{}\n{}|{}|{}\n{}|{}|{}\n",
        game.board[0][0], game.board[0][1], game.board[0][2],
        game.board[1][0], game.board[1][1], game.board[1][2],
        game.board[2][0], game.board[2][1], game.board[2][2],);
    println!("{}", board);
}

fn check_end_game(game : &mut Game) -> bool {
    let player: char = if game.current_player == false { game.players[0] } else { game.players[1] };
    let board: [[char; 3]; 3] = game.board;
    if board[0][0] == player {
        if board[0][1] == player {
            if board[0][2] == player {
                println!("Player {} WON", player);
                return true;
            }
        }
        if board[1][1] == player {
            if board[2][2] == player {
                println!("Player {} WON", player);
                return true;
            }
        }
    }
    if board[1][0] == player {
        if board[1][1] == player {
            if board[1][2] == player {
                println!("Player {} WON", player);
                return true;
            }
        }
    }
    if board[2][0] == player {
        if board[2][1] == player {
            if board[2][2] == player {
                println!("Player {} WON", player);
                return true;
            }
        }
        if board[1][1] == player {
            if board[0][2] == player {
                println!("Player {} WON", player);
                return true;
            }
        }
    }
    return false
}