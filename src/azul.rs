use std::ops::{Deref, DerefMut};
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Start,
    Blue,
    Yellow,
    Red,
    Black,
    Teal
}

// factory, color, pattern line
pub struct GameMove (pub usize, pub Tile, pub usize);

#[derive(Debug, Clone)]
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

impl From<Vec<Tile>> for Bag {
    fn from(vector: Vec<Tile>) -> Bag {
        Bag(vector)
    }
}


#[derive(Default, Debug, Clone)]
struct Factory ([Option<Tile>; 4]);

impl Deref for Factory {
    type Target = [Option<Tile>; 4];

    fn deref(&self) -> &[Option<Tile>; 4] {
        &self.0
    }
}
impl DerefMut for Factory {
    fn deref_mut(&mut self) -> &mut [Option<Tile>; 4] {
        &mut self.0
    }
}


#[derive(Debug, Clone)]
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

type Patterns = (
    [Option<Tile>; 1],
    [Option<Tile>; 2],
    [Option<Tile>; 3],
    [Option<Tile>; 4],
    [Option<Tile>; 5]
);

type Row  = [bool; 5];
type Wall = [Row;  5];

#[derive(Debug, Clone)]
struct Board {
    score: u8,
    wall: Wall,
    floor: Vec<Tile>,
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

#[derive(Default, Debug, Clone)]
pub struct Game {
    turn: u8,
    player: usize,
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

        let mut game = Game {
            turn: 0,
            player: 0,
            box_top: Vec::<Tile>::with_capacity(100).into(),
            bag: Bag::default(),
            market: Market::default(),
            factories: factories,
            boards: boards
        };

        game.fill()?;

        Ok(game)
    }
    fn fill(&mut self) -> Result<(), &'static str> {
        for factory in &self.factories {
            if factory.len() != 0 {
                return Err("Cannot fill, factories are not empty")
            };
        };
        for factory in &mut self.factories {
            for i in 0..4 {
                if self.bag.len() == 0 && self.box_top.len() > 0 {
                    self.bag.append(&mut self.box_top);
                }
                else if self.bag.len() == 0 {
                    return Ok(())
                }
                else {
                    let tile_i = (random::<f32>() * self.bag.len() as f32).floor() as usize;
                    let tile = self.bag.remove(tile_i);
                    factory[i] = Some(tile);
                }
            }
        };
        Ok(())
    }
    pub fn do_move(&self, game_move: GameMove) -> Result<Game, &'static str> {

        if game_move.1 == Tile::Start {
            return Err("You can't take the start tile alone")
        }

        let mut game = self.clone();

        let old_factory = &self.factories[game_move.0];
        let new_factory = &game.factories[game_move.0];

        let sel_tile = game_move.1;
        let mut hand = old_factory.clone();
        hand.to_vec();
        hand.retain(|x| *x == sel_tile);
        if hand.len() == 0 {
            return Err("That tile is not in that factory")
        }

        let target = &mut game.boards[self.player].patterns[game_move.2];
        if target.first().is_some() || *target.first().unwrap() != sel_tile {
            return Err("You cannot place that tile on that pattern-line")
        }

        game.turn += 1;
        game.player = (game.player + 1) % self.boards.len();

        Ok(game)
    }
}