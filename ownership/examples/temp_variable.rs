pub struct Player {
    score: i32,
}

impl Player {
    pub fn set_score(&mut self, score: i32) {
        self.score = score
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn new() -> Self {
        Player { score: 0 }
    }
}

fn main() {
    let mut player1 = Player::new();
    player1.set_score(player1.score() + 1);
    println!("score: {}", player1.score())
}
