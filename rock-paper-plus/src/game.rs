use mpz_circuits::types::Value;

#[derive(Clone, Copy)]
pub enum GameAction {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl GameAction {
    pub fn from(i: usize) -> Option<Self> {
        match i {
            0 => Some(GameAction::Rock),
            1 => Some(GameAction::Paper),
            2 => Some(GameAction::Scissors),
            3 => Some(GameAction::Lizard),
            4 => Some(GameAction::Spock),
            _ => None,
        }
    }
}

impl Into<Value> for GameAction {
    fn into(self) -> Value {
        let mut bits = vec![Value::Bit(false); 5];
        bits[self as usize] = Value::Bit(true);

        Value::Array(bits)
    }
}

#[derive(Clone, Copy)]
pub enum GameResult {
    Player1,
    Player2,
    Draw,
}

impl Into<Value> for GameResult {
    fn into(self) -> Value {
        let mut bits = vec![Value::Bit(false); 3];

        match self {
            GameResult::Player1 => bits[0] = Value::Bit(true),
            GameResult::Player2 => bits[1] = Value::Bit(true),
            GameResult::Draw => bits[2] = Value::Bit(true),
        }

        Value::Array(bits)
    }
}

impl GameResult {
    pub fn from(player1: GameAction, player2: GameAction) -> Self {
        match (player1, player2) {
            (GameAction::Rock, GameAction::Scissors) => GameResult::Player1,
            (GameAction::Rock, GameAction::Lizard) => GameResult::Player1,
            (GameAction::Paper, GameAction::Rock) => GameResult::Player1,
            (GameAction::Paper, GameAction::Spock) => GameResult::Player1,
            (GameAction::Scissors, GameAction::Paper) => GameResult::Player1,
            (GameAction::Scissors, GameAction::Lizard) => GameResult::Player1,
            (GameAction::Lizard, GameAction::Spock) => GameResult::Player1,
            (GameAction::Lizard, GameAction::Paper) => GameResult::Player1,
            (GameAction::Spock, GameAction::Rock) => GameResult::Player1,
            (GameAction::Spock, GameAction::Scissors) => GameResult::Player1,

            (GameAction::Rock, GameAction::Rock) => GameResult::Draw,
            (GameAction::Paper, GameAction::Paper) => GameResult::Draw,
            (GameAction::Scissors, GameAction::Scissors) => GameResult::Draw,
            (GameAction::Lizard, GameAction::Lizard) => GameResult::Draw,
            (GameAction::Spock, GameAction::Spock) => GameResult::Draw,

            _ => GameResult::Player2,
        }
    }
}
