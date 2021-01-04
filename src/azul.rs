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
#[derive(Debug, Clone, Copy)]
pub struct GameMove (pub usize, pub Tile, pub usize);
impl Default for GameMove {
    fn default() -> Self {
        GameMove(0, Tile::Blue, 0)
    }
}

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
impl Board {
    fn wall_index(color: Tile, row: usize) -> Result<usize, &'static str> {
        match row {
            0 => {
                match color {
                    Tile::Blue => Ok(0),
                    Tile::Yellow => Ok(1),
                    Tile::Red => Ok(2),
                    Tile::Black => Ok(3),
                    Tile::Teal => Ok(4),
                    _ => return Err("Not a valid tile on the wall")
                }
            },
            1 => {
                match color {
                    Tile::Blue => Ok(1),
                    Tile::Yellow => Ok(2),
                    Tile::Red => Ok(3),
                    Tile::Black => Ok(4),
                    Tile::Teal => Ok(0),
                    _ => return Err("Not a valid tile on the wall")
                }
            },
            2 => {
                match color {
                    Tile::Blue => Ok(2),
                    Tile::Yellow => Ok(3),
                    Tile::Red => Ok(4),
                    Tile::Black => Ok(0),
                    Tile::Teal => Ok(1),
                    _ => return Err("Not a valid tile on the wall")
                }
            },
            3 => {
                match color {
                    Tile::Blue => Ok(3),
                    Tile::Yellow => Ok(4),
                    Tile::Red => Ok(0),
                    Tile::Black => Ok(1),
                    Tile::Teal => Ok(2),
                    _ => return Err("Not a valid tile on the wall")
                }
            },
            4 => {
                match color {
                    Tile::Blue => Ok(4),
                    Tile::Yellow => Ok(0),
                    Tile::Red => Ok(1),
                    Tile::Black => Ok(2),
                    Tile::Teal => Ok(3),
                    _ => return Err("Not a valid tile on the wall")
                }
            },
            _ => return Err("Not a valid row on the wall")
        }
    }
    fn connected(&self, coordinate: (usize, usize)) -> u8 {
        coz::scope!("connected tiles");
        let wall = self.wall;

        let mut sum = 0;
        let mut active = false;

        let mut count = 0;
        for i in 0..5 {
            if active == true && wall[coordinate.0][i] == false {
                break;
            } else if wall[coordinate.0][i] == false {
                count = 0;
            } else if (coordinate.0, i) == coordinate {
                active = true;
                count += 1;
            } else {
                count += 1;
            }
        }
        sum += count;

        let mut active = false;
        let mut count = 0;
        for i in 0..5 {
            if active == true && wall[i][coordinate.1] == false {
                break;
            } else if wall[i][coordinate.1] == false {
                count = 0;
            } else if (i, coordinate.1) == coordinate {
                active = true;
                count += 1;
            } else {
                count += 1;
            }
        }
        sum += count;

        return sum
    }
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
    turn: u32,
    player: usize,
    box_top: Bag,
    bag: Bag,
    market: Market,
    factories: Vec<Factory>,
    boards: Vec<Board>
}
impl Game {
    pub fn new(players: usize) -> Result<Self, &'static str> {
        coz::scope!("create game");
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
            box_top: Vec::<Tile>::with_capacity(100).into(),
            bag: Bag::default(),
            market: Market::default(),
            factories: factories,
            boards: boards
        };

        Ok(game)
    }
    pub fn fill(&mut self) -> Result<(), &'static str> {
        coz::scope!("fill factories from bag");
        for factory in &self.factories {
            if factory.len() != 0 {
                return Err("Cannot fill, factories are not empty")
            };
        };
        for factory in &mut self.factories {
            for _ in 0..4 {
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
    fn score(&mut self) -> Result<(), &'static str> {
        coz::scope!("score boards");
        for board in &mut self.boards {
            for row in 0..4 {
                if board.patterns[row].len() == (row + 1) {
                    let color = board.patterns[row].remove(0);
                    let index = Board::wall_index(color, row)?;
                    board.wall[row][index] = true;
                    board.score += board.connected((row, index));

                    self.box_top.append(&mut board.patterns[row])
                }
            }
            let negative = match board.floor.len() {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 4,
                4 => 6,
                5 => 8,
                6 => 11,
                _ => 14
            };
            board.score -= negative;
        }
        Ok(())
    }
    pub fn do_move(&self, game_move: GameMove) -> Result<Game, &'static str> {
        coz::scope!("do a move");
        if game_move.1 == Tile::Start {
            return Err("You can't take the start tile alone")
        }

        let mut game = self.clone();

        let board =  &mut game.boards[self.player];

        match game_move {
            GameMove(_, Tile::Start, _) => return Err("You can't take the start tile specifically"),
            GameMove(0, _, 0) => {
                if self.market.contains(&game_move.1) {
                    let mut hand = self.market.deref().clone();
                    hand.retain(|&x| x == Tile::Start || x == game_move.1);
                    game.market.retain(|x| *x != Tile::Start && *x != game_move.1);
                    
                    board.floor.append(&mut hand)
                }
                else {
                    return Err("Market does not contain selected tile")
                }
            },
            GameMove(0, _, 1..=6) => {
                if self.market.len() == 0 {
                    return Err("Market is empty");
                }
                else if self.market.contains(&game_move.1) {
                    let target = &mut board.patterns[game_move.2 - 1];
                    let empty = game_move.2 - target.len();

                    if empty == 0 {
                        return Err("That pattern is full")
                    }

                    let mut hand = self.market.deref().clone();
                    hand.retain(|&x| x == Tile::Start || x == game_move.1);
                    game.market.retain(|x| *x != Tile::Start && *x != game_move.1);

                    for tile in hand.drain(..) {
                        let empty = game_move.2 - &target.len();
                        if tile == Tile::Start {
                            board.floor.push(tile);
                        }
                        else {
                            if empty >= 1 {
                                target.push(tile);
                            }
                            else {
                                board.floor.push(tile)
                            }
                        }
                    }
                }
                else {
                    return Err("Market does not contain selected tile")
                }
            },
            GameMove(1..=9, _, _) => {
                if game_move.0 > self.factories.len() - 1 {
                    return Err("That factory is out of bounds");
                }

                let old_factory = &self.factories[game_move.0 - 1];
                if old_factory.contains(&game_move.1) {
                    
                    let mut new_factory = &mut game.factories[game_move.0 - 1];
    
                    let mut hand = old_factory.deref().clone();
    
                    hand.retain(|&x| x == game_move.1);
                    new_factory.retain(|&x| x != game_move.1);

                    game.market.append(&mut new_factory);

                    match game_move.2 {
                        0 => {
                            board.floor.append(&mut hand)
                        },
                        1..=9 => {
                            let target = &mut board.patterns[game_move.2 - 1];
                            let empty = game_move.2 - target.len();
                            if hand.len() <= empty {
                                target.append(&mut hand);
                            }
                            else if empty != 0 {
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
                            else {
                                return Err("That pattern line is full")
                            }
                        },
                        _ => return Err("Not a valid destination")
                    }
                }
                else {
                    return Err("That tile is not in that factory")
                }
            },
            GameMove(_,_,_) => return Err("Not a valid move")
        }

        let mut empty = true;
        for factory in &game.factories {
            if factory.len() != 0 {
                empty = false;
                break;
            }
        }
        if empty == true {
            if game.market.len() != 0 {
                empty = false;
            }
        }

        if empty == true {
            game.score()?;
        }
        else {
            game.player = (game.player + 1) % self.boards.len();
        }

        game.turn += 1;

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

#[test]
fn connected() -> Result<(), String> {
    let mut board = Board::default();
    board.wall[0] = [false, false, false, false, false];
    board.wall[1] = [true,  false, true,  false, false];
    board.wall[2] = [true,  false, true,  false, false];
    board.wall[3] = [true,  false, false, false, false];
    board.wall[4] = [true,  true,  true,  false, false];

    let coordinate = (4 as usize, Board::wall_index(Tile::Yellow, 4)?);
    assert_eq!(coordinate, (4,0));

    let score = board.connected(coordinate);

    assert_eq!(score, 7);
    Ok(())
}