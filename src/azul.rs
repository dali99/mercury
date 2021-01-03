use std::ops::{Deref, DerefMut};
use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
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

type Patterns = [Vec<Tile>; 5];

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

        Ok(game)
    }
    pub fn fill(&mut self) -> Result<(), &'static str> {
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
                    factory.push(tile);
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

        let board =  &mut game.boards[self.player];

        let old_factory = &self.factories[game_move.0];
        let new_factory = &mut game.factories[game_move.0];

        let sel_tile = game_move.1;
        let mut hand = old_factory.to_vec();
        hand.retain(|x| *x == sel_tile);
        if hand.len() == 0 {
            return Err("That tile is not in that factory")
        }

        new_factory.retain(|x| *x != sel_tile);
        game.market.append(new_factory);

        if game_move.2 == 0 {
            board.floor.append(&mut hand);
            return Ok(game)
        };

        // This is a workaround for borrowing _only_that board pattern
        let target: &mut Vec<Tile> = match game_move.2 {
            1..=5 => &mut board.patterns[game_move.2 - 1],
            _ => return Err("That's not a valid pattern line")
        };

        println!("{:#?}", target);

        if target.first() != None && target.first() != Some(&sel_tile) {
            return Err("You cannot place that tile on that pattern-line, because there are already other tiles with a different color there")
        }

        if target.len() == game_move.2 {
            return Err("That line is full!")
        }

        let empty = game_move.2 - target.len();
        if hand.len() <= empty {
            target.append(&mut hand);
        }
        else {
            for tile in hand.drain(..) {
                let empty = game_move.2 - &target.len();
                if empty >= 1 {
                    target.push(tile);
                }
                else {
                    board.floor.push(tile)
                }
            }
        }

        game.turn += 1;
        game.player = (game.player + 1) % self.boards.len();

        Ok(game)
    }
}


// Tests

#[test]
fn bag() {
    let game = Game::new(2).unwrap();
    let bag = game.bag;
    assert_eq!(bag.len(), 100);

    let mut reds = bag.clone();
    reds.retain(|x| *x == Tile::Red);
    assert_eq!(reds.len(), 20);
}