pub struct Team {
    name: String,
    players: Vec<Box<super::player::Player>>
}

impl Team {
    const DEFAULT_SIZE: usize = 50;

    //fn default(name: string)
    //description: makes a default to with default number of players
    pub fn new(name: String) -> Self {
        let mut players = Vec::with_capacity(Team::DEFAULT_SIZE);
        for _ in 0..Team::DEFAULT_SIZE {
            players.push(Box::new(super::player::Player::default()));
        }

        Team { name, players }
    }
}