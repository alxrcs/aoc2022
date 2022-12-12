use std::fmt::Debug;

// Rock Paper Scissors map to A B C
// Use an enum to represent the three options
#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

// Represent the game state
struct GameState {
    // The player's move
    player_move: Move,
    // The computer's move
    computer_move: Move,
}
// Represent the game result
enum GameResult {
    Win,
    Lose,
    Draw,
}

struct GameState2 {
    // The computer's move
    computer_move: Move,
    // The desired outcome
    outcome: GameResult,
}

impl Debug for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win => write!(f, "Win"),
            Self::Lose => write!(f, "Lose"),
            Self::Draw => write!(f, "Draw"),
        }
    }
}

// The game logic
fn play_game(player_move: Move, computer_move: Move) -> GameResult {
    // Determine the result
    match (player_move, computer_move) {
        (Move::Rock, Move::Rock) => GameResult::Draw,
        (Move::Rock, Move::Paper) => GameResult::Lose,
        (Move::Rock, Move::Scissors) => GameResult::Win,
        (Move::Paper, Move::Rock) => GameResult::Win,
        (Move::Paper, Move::Paper) => GameResult::Draw,
        (Move::Paper, Move::Scissors) => GameResult::Lose,
        (Move::Scissors, Move::Rock) => GameResult::Lose,
        (Move::Scissors, Move::Paper) => GameResult::Win,
        (Move::Scissors, Move::Scissors) => GameResult::Draw,
    }
}

fn get_shape_score(player_move: Move) -> i32 {
    match player_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn get_outcome_score(result: GameResult) -> i32 {
    match result {
        GameResult::Lose => 0,
        GameResult::Draw => 3,
        GameResult::Win => 6,
    }
}

fn get_round_score(player_move: Move, computer_move: Move) -> i32 {
    let shape_score = get_shape_score(player_move);
    let outcome_score = get_outcome_score(play_game(player_move, computer_move));
    shape_score + outcome_score
}

fn get_action_for_outcome(computer_move: Move, result: GameResult) -> Move {
    match (computer_move, result) {
        (Move::Rock, GameResult::Win) => Move::Paper,
        (Move::Rock, GameResult::Lose) => Move::Scissors,
        (Move::Rock, GameResult::Draw) => Move::Rock,
        (Move::Paper, GameResult::Win) => Move::Scissors,
        (Move::Paper, GameResult::Lose) => Move::Rock,
        (Move::Paper, GameResult::Draw) => Move::Paper,
        (Move::Scissors, GameResult::Win) => Move::Rock,
        (Move::Scissors, GameResult::Lose) => Move::Paper,
        (Move::Scissors, GameResult::Draw) => Move::Scissors,
    }
}

// The main function
fn main() {
    // Create a new game state
    let game_state = GameState {
        player_move: Move::Rock,
        computer_move: Move::Rock,
    };

    let mut game_state_2 = GameState2 {
        computer_move: Move::Rock,
        outcome: GameResult::Win,
    };

    let mut total_score = 0;

    // Parse the input from a file
    // Each line has two moves, separated by a space
    // The first move is the player's move
    // The second move is the computer's move
    let input = std::fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        // Split the line into two moves
        let mut moves = line.split_whitespace();

        // Parse the computer's move
        game_state_2.computer_move = match moves.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move"),
        };

        game_state_2.outcome = match moves.next().unwrap() {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("Invalid move"),
        };

        let required_move =
            get_action_for_outcome(game_state_2.computer_move, game_state_2.outcome);

        // game_state.computer_move = match moves.next().unwrap() {
        //     "A" => Move::Rock,
        //     "B" => Move::Paper,
        //     "C" => Move::Scissors,
        //     _ => panic!("Invalid move"),
        // };

        // game_state.player_move = match moves.next().unwrap() {
        //     "X" => Move::Rock,
        //     "Y" => Move::Paper,
        //     "Z" => Move::Scissors,
        //     _ => panic!("Invalid move"),
        // };

        // Play the game
        let result = play_game(game_state_2.computer_move, required_move);
        let round_score = get_round_score(required_move, game_state_2.computer_move);
        println!(
            "{:?} vs {:?} = {:?}",
            required_move, game_state_2.computer_move, result
        );
        println!("Round score: {}", round_score);
        total_score += round_score;
    }

    println!("Total score: {}", total_score);
}
