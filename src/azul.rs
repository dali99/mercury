use std::ops::{Deref, DerefMut};

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
struct Bag (Vec<Tile>);
impl Default for Bag {
    fn default() -> Self {
        let mut bag = Vec::<Tile>::with_capacity(100);
        for _ in 0..20 {
            bag.push(Tile::Blue);
        };
        for _ in 0..20 {
            bag.push(Tile::Yellow);
        };
        for _ in 0..20 {
            bag.push(Tile::Red);
        };
        for _ in 0..20 {
            bag.push(Tile::Black);
        };
        for _ in 0..20 {
            bag.push(Tile::Teal);
        };

        Bag(bag)
    }
}

impl Deref for Bag {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Vec<Tile> {
        &self.0
    }
}
impl DerefMut for Bag {
    fn deref_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.0
    }
}


#[derive(Default, Debug)]
struct Factory (Vec<Tile>);

impl Deref for Factory {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Vec<Tile> {
        &self.0
    }
}
impl DerefMut for Factory {
    fn deref_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.0
    }
}


#[derive(Debug)]
struct Market (Vec<Tile>);

impl Default for Market {
    fn default() -> Self {
        let mut market = Vec::<Tile>::with_capacity(20);
        market.push(Tile::Start);
        Market(market)
    }
}
impl Deref for Market {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Vec<Tile> {
        &self.0
    }
}
impl DerefMut for Market {
    fn deref_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.0
    }
}


#[derive(Debug, Default)]
struct PatternLine (Option<Tile>, u8);
type Patterns = [PatternLine; 5];

type Row  = [bool; 5];
type Wall = [Row;  5];

#[derive(Debug)]
struct Board {
    score: u8,
    wall: Wall,
    floor: Vec<PatternLine>,
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
    box_top: Bag,
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
            box_top: Bag::default(),
            bag: Bag::default(),
            market: Market::default(),
            factories: factories,
            boards: boards
        };
        Ok(game)
    }
    pub fn fill(&mut self) -> Result<(), &'static str> {
        for factory in &self.factories {
            if factory.len() != 0 {
                return Err("Cannot fill, factories are not empty")
            };
        };
        Ok(())
    }
    // pub fn shop(&mut self, )
}