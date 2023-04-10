pub struct Monster {
    stats: Stats,
    friends: Vec<Friend>,
}

pub struct Stats {
    hp: u8,
    sp: u8,
}

pub struct Friend {
    loyalty: u8,
}

impl Stats {
    pub fn heal(&mut self, loyalty: u8) {
        self.hp += loyalty;
        self.sp -= loyalty;
    }
}

impl Monster {
    pub fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.stats.heal(friend.loyalty);
            println!("Healing for: {}", friend.loyalty);
        }
    }
}

fn main() {}
