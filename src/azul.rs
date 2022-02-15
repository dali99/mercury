use std::{hash::Hash, ops::{Deref, DerefMut}};
use rand::prelude::*;
use rand::distributions::WeightedIndex;
//use smallvec::{SmallVec, smallvec};

//use alloc_counter::{AllocCounterSystem, no_alloc};
//#[global_allocator]
//static A: AllocCounterSystem = AllocCounterSystem;

extern crate byte_strings;
use ::byte_strings::concat_bytes;

pub fn size_of_stuff() {
    println!("size of azul game: {}", std::mem::size_of::<Game>());
    println!("size of azul tile: {}", std::mem::size_of::<Tile>());
    println!("size of azul bag: {}", std::mem::size_of::<Bag>());
    println!("size of azul market: {}", std::mem::size_of::<Market>());
    println!("size of azul factories: {}", std::mem::size_of::<tinyvec::ArrayVec<[Factory; 9]>>());
    println!("size of azul factory array: {}", std::mem::size_of::<[Factory; 9]>());
    println!("size of azul factory: {}", std::mem::size_of::<Factory>());
    println!("size of azul boards: {}", std::mem::size_of::<tinyvec::ArrayVec<[Board; 4]>>());
    println!("size of azul board array: {}", std::mem::size_of::<[Board; 4]>());
    println!("size of azul board: {}", std::mem::size_of::<Board>());

    println!("size of azul option tile: {}", std::mem::size_of::<Option<Tile>>());

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tile {
//    None,
    Start,
    Blue,
    Yellow,
    Red,
    Black,
    Teal
}

impl Default for Tile {
    fn default() -> Self {
//        Tile::None
        Tile::Blue
    }
}

impl IntoIterator for Tile {
    type Item = Tile;
    type IntoIter = TileIter;

    fn into_iter(self) -> Self::IntoIter {
        TileIter {
            current: self
        }
    }
}

/*impl From<Option<Tile>> for Tile {
    fn from(option: Option<Tile>) -> Tile {
        match option {
            Option::None => Tile::None,
            Some(tile) => tile
        }
    }
}*/

pub struct TileIter {
    current: Tile
}
impl Iterator for TileIter {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile>{
        match self.current {
            Tile::Blue => {
                let next = Tile::Yellow;
                self.current = next;
                Some(next)
            },
            Tile::Yellow => {
                let next = Tile::Red;
                self.current = next;
                Some(next)
            },
            Tile::Red => {
                let next = Tile::Black;
                self.current = next;
                Some(next)
            },
            Tile::Black => {
                let next = Tile::Teal;
                self.current = next;
                Some(next)
            },
            _ => None
        }
    }
}


// factory, color, pattern line
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameMove (pub usize, pub Tile, pub usize);
impl Default for GameMove {
    fn default() -> Self {
        GameMove(0, Tile::Blue, 1)
    }
}
impl IntoIterator for GameMove {
    type Item = GameMove;
    type IntoIter = GameMoveIter;

    fn into_iter(self) -> Self::IntoIter {
        GameMoveIter {
            players: 4,
            current: self
        }
    }
}

#[derive(Debug)]
pub struct GameMoveIter {
    players: u8,
    current: GameMove
}
impl GameMoveIter {
    pub fn new(players: u8) -> Self {
        GameMoveIter {
            players: players,
            current: GameMove::default()
        }
    }
}

fn get_n_factories(players: u8) -> Result<u8, &'static str> {
    return match players {
        2 => Ok(5),
        3 => Ok(7),
        4 => Ok(9),
        _ => Err("Not a valid amount of players")
    };
}

impl Iterator for GameMoveIter {
    type Item = GameMove;

    fn next(&mut self) -> Option<GameMove> {
        let factory = self.current.0;
        let tile = self.current.1;
        let pattern = self.current.2;

        let max_factories = match get_n_factories(self.players) {
            Ok(n) => n,
            Err(_) => return None
        } as usize;

        if factory == max_factories && tile == Tile::Teal && pattern == 0 {
            return None
        }
        else if factory == max_factories && tile == Tile::Teal {
            let next = GameMove(0, Tile::Blue, (pattern + 1) % 6);
            self.current = next;
            return Some(next)
        }
        else if factory == max_factories {
            let next = GameMove(0, tile.into_iter().next().unwrap(), pattern);
            self.current = next;
            return Some(next)
        }
        else {
            let next = GameMove(factory + 1, tile, pattern);
            self.current = next;
            return Some(next)
        } 
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
struct Bag {
    blue: u8,
    yellow: u8,
    red: u8,
    black: u8,
    teal: u8,
}

impl Default for Bag {
    fn default() -> Self {
        Bag {
            blue: 20,
            yellow: 20,
            red: 20,
            black: 20,
            teal: 20, 
        }
    }
}

impl Bag {
    fn len(&self) -> u8 {
        self.blue + self.yellow + self.red + self.black + self.teal
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn hash(&self) -> [u8; 5] {
        [
            self.blue,
            self.yellow,
            self.red,
            self.black,
            self.teal
        ]
    }
}

/*#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
struct Bag (tinyvec::ArrayVec::<[Tile; 128]>);

impl Default for Bag {
    fn default() -> Self {
        let mut bag = tinyvec::ArrayVec::<[Tile; 128]>::new();
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
    type Target = tinyvec::ArrayVec<[Tile; 128]>;

    fn deref(&self) -> &tinyvec::ArrayVec<[Tile; 128]> {
        &self.0
    }
}
impl DerefMut for Bag {
    fn deref_mut(&mut self) -> &mut tinyvec::ArrayVec<[Tile; 128]> {
        &mut self.0
    }
}

impl From<tinyvec::ArrayVec<[Tile; 128]>> for Bag {
    fn from(vector: tinyvec::ArrayVec<[Tile; 128]>) -> Bag {
        Bag(vector)
    }
}*/


#[derive(Default, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Factory (tinyvec::ArrayVec<[Tile; 4]>);
impl Clone for Factory {
    //#[no_alloc]
    fn clone(&self) -> Self {
        Factory(self.0.clone())
    }
}
impl Deref for Factory {
    type Target = tinyvec::ArrayVec<[Tile; 4]>;

    fn deref(&self) -> &tinyvec::ArrayVec<[Tile; 4]> {
        &self.0
    }
}
impl DerefMut for Factory {
    fn deref_mut(&mut self) -> &mut tinyvec::ArrayVec<[Tile; 4]> {
        &mut self.0
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Market (tinyvec::ArrayVec<[Tile; 28]>);
impl Default for Market {
    fn default() -> Self {
        // tiles * factories + start = 3 * 9 + 1 = 28
        let mut market = tinyvec::ArrayVec::<[Tile; 28]>::new();
        market.push(Tile::Start);
        Market(market)
    }
}
impl Deref for Market {
    type Target = tinyvec::ArrayVec<[Tile; 28]>;

    fn deref(&self) -> &tinyvec::ArrayVec<[Tile; 28]> {
        &self.0
    }
}
impl DerefMut for Market {
    fn deref_mut(&mut self) -> &mut tinyvec::ArrayVec<[Tile; 28]> {
        &mut self.0
    }
}

type Patterns = [tinyvec::ArrayVec<[Tile; 5]>; 5];

type Row  = [bool; 5];
type Wall = [Row;  5];

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
struct Board {
    score: u8,
    wall: Wall,
    floor: tinyvec::ArrayVec<[Tile; 20]>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    player: u8,
    box_top: Bag,
    bag: Bag,
    market: Market,
    factories: tinyvec::ArrayVec<[Factory; 5]>,  // TODO set to 9?
    boards: tinyvec::ArrayVec<[Board; 2]>        // TODO set to 4?
}
impl State {
    pub fn new(players: u8) -> Result<State, &'static str> {
        let n_factories = get_n_factories(players)?;
        let mut factories = tinyvec::ArrayVec::<[Factory; 5]>::new();
        for _ in 0..n_factories {
            factories.push(Factory::default())
        }

        let mut boards = tinyvec::ArrayVec::<[Board; 2]>::new();
        for _ in 0..players {
            boards.push(Board::default());
        }

        let game = State {
            player: 0,
            box_top: Bag {
                blue: 0,
                yellow: 0,
                red: 0,
                black: 0,
                teal: 0,
            },
            bag: Bag::default(),
            market: Market::default(),
            factories: factories,
            boards: boards
        };

        Ok(game)
    }
    pub fn fill(&mut self, mut rng: StdRng) -> Result<(), &'static str> {
        for factory in &self.factories {
            if factory.len() != 0 {
                return Err("Cannot fill, factories are not empty")
            };
        };
        for factory in &mut self.factories {
            for _ in 0..4 {
                if self.bag.is_empty() && !self.box_top.is_empty() {
                    self.bag = self.box_top;
                    self.box_top = Bag { blue: 0, yellow: 0, red: 0, black: 0, teal: 0};
                }
                else if self.bag.is_empty() {
                    return Ok(())
                }
                else {
                    let choices = [Tile::Blue, Tile::Yellow, Tile::Red, Tile::Black, Tile::Teal];
                    let weights = [self.bag.blue, self.bag.yellow, self.bag.red, self.bag.black, self.bag.teal];
                    let dist = WeightedIndex::new(&weights).unwrap();
                    let tile = choices[dist.sample(&mut rng)];

                    match tile {
                        Tile::Start => {return Err("This should never happen")}
                        Tile::Blue => {self.bag.blue -= 1}
                        Tile::Yellow => {self.bag.yellow -= 1}
                        Tile::Red => {self.bag.red -= 1}
                        Tile::Black => {self.bag.black -= 1}
                        Tile::Teal => {self.bag.teal -= 1}
                    }
                    factory.push(tile)
                }
            }
            factory.sort_unstable();
        };
        self.factories.sort_unstable();
        Ok(())
    }
    fn score(&mut self) -> Result<(), &'static str> {
        for board in &mut self.boards {
            for row in 0..4 {
                if board.patterns[row].len() == (row + 1) {
                    let color = board.patterns[row].remove(0);
                    let index = Board::wall_index(color, row)?;
                    board.wall[row][index] = true;
                    board.score += board.connected((row, index));

                    match color {
                        Tile::Start => {return Err("This should never happen")}
                        Tile::Blue => {self.box_top.blue += (row as u8 + 1)}
                        Tile::Yellow => {self.box_top.yellow += (row as u8 + 1)}
                        Tile::Red => {self.box_top.red += (row as u8 + 1)}
                        Tile::Black => {self.box_top.black += (row as u8 + 1)}
                        Tile::Teal => {self.box_top.teal += (row as u8 + 1)}
                    }
                    //self.box_top.extend_from_slice(board.patterns[row].as_slice());
                    board.patterns[row].clear();
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
    pub fn do_move(&mut self, game_move: GameMove) -> Result<(), &'static str> {
        let board =  &mut self.boards[self.player as usize];
        match game_move {
            GameMove(_, Tile::Start, _) => return Err("You can't take the start tile specifically"),
            GameMove(0, _, 0) => {
                if self.market.contains(&game_move.1) {
                    let mut hand= self.market.clone();
                    hand.retain(|x| *x == Tile::Start || *x == game_move.1);
                    self.market.retain(|x| *x != Tile::Start && *x != game_move.1);
                    
                    board.floor.extend_from_slice(hand.as_slice());
                    hand.clear();
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
                    if target.first().is_some() && target[0] != game_move.1 {
                        return Err("That pattern line already contains a different color")
                    }
                    let empty = game_move.2 - target.len();

                    if empty == 0 {
                        return Err("That pattern is full")
                    }

                    let mut hand = self.market.deref().clone();
                    hand.retain(|x| *x == Tile::Start || *x == game_move.1);
                    self.market.retain(|x| *x != Tile::Start && *x != game_move.1);

                    for tile in hand.drain(..) {
                        let empty = game_move.2 - target.len();
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
                let board =  &mut self.boards[self.player as usize];
                if game_move.0 > self.factories.len() {
                    return Err("That factory is out of bounds");
                }

                let factory = self.factories[game_move.0 - 1].deref_mut();
                if factory.contains(&game_move.1) {  
                    let mut hand = factory.clone();
                    hand.retain(|x| *x == game_move.1);
                    factory.retain(|x| *x != game_move.1);

                    self.market.extend_from_slice(factory.as_slice());
                    factory.clear();

                    match game_move.2 {
                        0 => {
                            board.floor.extend_from_slice(hand.as_slice());
                        },
                        1..=9 => {
                            let target = &mut board.patterns[game_move.2 - 1];
                            if target.first().is_some() && target[0] != game_move.1 {
                                return Err("That pattern line already contains a different color")
                            }
                            let empty = game_move.2 - target.len();
                            if hand.len() <= empty {
                                target.extend_from_slice(hand.as_slice());
                                hand.clear();
                            }
                            else if empty != 0 {
                                for tile in hand.drain(..) {
                                    let empty = game_move.2 - target.len();
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

        /*
        let mut empty = true;
        for factory in &mut self.factories {
            if factory.len() != 0 {
                empty = false;
                break;
            }
        }
        if empty == true {
            if self.market.len() != 0 {
                empty = false;
            }
        }

        if empty == true {
            self.score()?;
        }
        else {
            self.player = (self.player + 1) % self.boards.len();
        }
        */

        self.factories.sort_unstable();
        self.market.sort();
        for board in &mut self.boards {
            board.floor.sort_unstable();
        }

        self.player = (self.player + 1) % self.boards.len() as u8;
        Ok(())
    }
    pub fn hash(&self) -> [u8; 256]{
        [
            [self.player],
            self.box_top.hash(),
            self.bag.hash()
        ].concat()
    }
}

//#[repr(align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Game {
    pub state: State,
    pub turn: u32,
}
impl Game {
    pub fn new(players: u8) -> Result<Game, &'static str> {
        let game = Game {
            state: State::new(players)?,
            turn: 0
        };
        Ok(game)
    }
    pub fn do_move(&mut self, game_move: GameMove) -> Result<(), &'static str> {
        let result = self.state.do_move(game_move);
        self.turn += 1;
        result
    }
}
impl Deref for Game {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl DerefMut for Game {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}


// Tests

pub fn complicated() -> Result<Game, &'static str> {
    let mut game = Game::new(2)?;

    let mut tiles = Tile::Blue;

    for factory in &mut game.factories {
        for _ in 0..4 {
            factory.push(tiles);
            tiles = tiles.into_iter().next().unwrap_or(Tile::Blue);
        }
    }

    Ok(game)
}

pub fn some_game() -> Result<Game, &'static str> {
    let mut game = Game::new(2)?;

    let factories = &mut game.factories;

    factories[0].push(Tile::Blue);
    factories[0].push(Tile::Blue);
    factories[0].push(Tile::Yellow);
    factories[0].push(Tile::Black);

    factories[1].push(Tile::Blue);
    factories[1].push(Tile::Red);
    factories[1].push(Tile::Red);
    factories[1].push(Tile::Red);

    factories[2].push(Tile::Blue);
    factories[2].push(Tile::Red);
    factories[2].push(Tile::Black);
    factories[2].push(Tile::Teal);

    factories[3].push(Tile::Blue);
    factories[3].push(Tile::Red);
    factories[3].push(Tile::Black);
    factories[3].push(Tile::Teal);

    factories[4].push(Tile::Yellow);
    factories[4].push(Tile::Yellow);
    factories[4].push(Tile::Black);
    factories[4].push(Tile::Teal);

    let bag = &mut game.bag;
    bag.blue -= 5;
    bag.yellow -= 3;
    bag.black -= 4;
    bag.red -= 5;
    bag.teal -= 3;

    Ok(game)
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

#[test]
fn game_move_iter() {
    let i = GameMove::default();
    println!("Original: {:?}", i);
    assert_eq!(i.into_iter().next().unwrap(), GameMove(1, Tile::Blue, 1));

    //assert_eq!(i.into_iter().count(), 5)
}

#[test]
fn sizes() {
    println!("size of azul game: {}", std::mem::size_of::<Game>());
}
