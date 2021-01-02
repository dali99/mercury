#[derive(Debug)]
enum Tile {
    Start,
    Blue,
    Yellow,
    Red,
    Black,
    Teal
}

#[derive(Debug)]
struct Bag {
    blue: u8,
    yellow: u8,
    red: u8,
    black: u8,
    teal: u8
}
impl Default for Bag {
    fn default() -> Self {
        Bag {
            blue: 20,
            yellow: 20,
            red: 20,
            black: 20,
            teal: 20
        }
    }
}

#[derive(Default, Debug)]
struct Factory {
    blue: u8,
    yellow: u8,
    red: u8,
    black: u8,
    teal: u8
}

#[derive(Debug)]
struct Market {
    start: u8,
    blue: u8,
    yellow: u8,
    red: u8,
    black: u8,
    teal: u8
}
impl Default for Market {
    fn default() -> Self {
        Market {
            start: 1,
            blue: 0,
            yellow: 0,
            red: 0,
            black: 0,
            teal: 0 
        }
    }
}

#[derive(Debug, Default)]
struct Patternline (Option<Tile>, u8);
type Patterns = [Patternline; 5];

type Row  = [bool; 5];
type Wall = [Row;  5];

#[derive(Debug)]
struct Board {
    score: u8,
    wall: Wall,
    floor: Vec<Patternline>,
    patterns: Patterns,
}
impl Default for Board {
    fn default() -> Self {
        Board {
            score: 0,
            wall: Wall::default(),
            floor: Vec::default(),
            patterns: Patterns::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct Game {
    turn: u8,
    player: u8,
    bag: Bag,
    market: Market,
    factories: Vec<Factory>,
    boards: Vec<Board>
}
impl Game {
    pub fn new(players: usize) -> Result<Self, &'static str> {
        let n_factories = match players {
            2 => 5,
            3 => 7,
            4 => 9,
            _ => return Err("Not a valid amount of players")
        };
        let mut factories = Vec::<Factory>::with_capacity(n_factories);
        for _ in 0..n_factories {
            factories.push(Factory::default())
        }

        let mut boards = Vec::<Board>::with_capacity(players);
        for _ in 0..players {
            boards.push(Board::default());
        }

        let game = Game {
            turn: 0,
            player: 0,
            bag: Bag::default(),
            market: Market::default(),
            factories: factories,
            boards: boards
        };
        Ok(game)
    }
}