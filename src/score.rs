pub struct Score {
    player: u32,
    computer: u32,
    draws: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            player: 0,
            computer: 0,
            draws: 0,
        }
    }

    pub fn get_gamecount(&self) -> u32 {
        self.player + self.computer + self.draws
    }

    pub fn to_string(&self) -> String {
        format!("Player: {}\nComputer: {}\nDraws: {}\nGamecount: {}", self.player, self.computer, self.draws, self.get_gamecount())
    }

    pub fn incr_player(&mut self) {
        self.player += 1;
    }

    pub fn incr_comp(&mut self) {
        self.computer += 1;
    }

    pub fn incr_draw(&mut self) {
        self.draws += 1;
    }
}