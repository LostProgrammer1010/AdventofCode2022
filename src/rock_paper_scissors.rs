use std::collections::HashMap;
use std::fs;
fn parse_file() -> String {
    let content = fs::read_to_string("rock_paper_scissor.txt").expect("Error reading the file");

    content
}

#[derive(Debug)]
struct StratOne {
    opponent_play: i32,
    your_play: i32,
}

impl StratOne {
    pub fn build(game: &str) -> Result<StratOne, &str> {
        let mut plays = game.split(" ");
        let opponent_play = match plays.next() {
            Some("A") => 1,
            Some("B") => 2,
            Some("C") => 3,
            _ => return Err("Not a string"),
        };
        let your_play = match plays.next() {
            Some("X") => 1,
            Some("Y") => 2,
            Some("Z") => 3,
            _ => return Err("Not a string"),
        };
        Ok(StratOne {
            opponent_play,
            your_play,
        })
    }

    pub fn calculate_game_points(self) -> i32 {
        let mut total_points = 0;
        if self.opponent_play == self.your_play {
            total_points += 3;
        } else if self.opponent_play == 1 && self.your_play == 2 {
            total_points += 6;
        } else if self.opponent_play == 2 && self.your_play == 3 {
            total_points += 6;
        } else if self.opponent_play == 3 && self.your_play == 1 {
            total_points += 6;
        }
        total_points + self.your_play
    }
}
#[derive(Debug)]
struct StratTwo {
    opponent_play: i32,
    game_outcome: i32,
}

impl StratTwo {
    pub fn build(game: &str) -> Result<StratTwo, &str> {
        let mut plays = game.split(" ");
        let opponent_play = match plays.next() {
            Some("A") => 1,
            Some("B") => 2,
            Some("C") => 3,
            _ => return Err("Not a string"),
        };
        let game_outcome = match plays.next() {
            Some("X") => 0,
            Some("Y") => 3,
            Some("Z") => 6,
            _ => return Err("Not a string"),
        };
        Ok(StratTwo {
            opponent_play,
            game_outcome,
        })
    }

    pub fn calculate_game_points(self) -> i32 {
        let your_play_to_lose: HashMap<i32, i32> = HashMap::from([(1, 3), (2, 1), (3, 2)]);
        let your_play_to_win: HashMap<i32, i32> = HashMap::from([(1, 2), (2, 3), (3, 1)]);
        if self.game_outcome == 3 {
            return self.game_outcome + self.opponent_play;
        } else if self.game_outcome == 6 {
            return self.game_outcome + your_play_to_win.get(&self.opponent_play).unwrap();
        }
        *your_play_to_lose.get(&self.opponent_play).unwrap()
    }
}

pub fn run() {
    let mut all_games: Vec<StratTwo> = Vec::new();
    let content = parse_file();
    for game in content.split("\n") {
        match StratTwo::build(game) {
            Ok(x) => all_games.push(x),
            Err(_) => (),
        }
    }
    let mut total_points = 0;
    for game in all_games {
        total_points += game.calculate_game_points()
    }
    println!("{total_points}");
}
