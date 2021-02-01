use std::ops::{Deref, DerefMut};
use rand::prelude::*;
use rand::distributions::WeightedIndex;
//use smallvec::{SmallVec, smallvec};

//use alloc_counter::{AllocCounterSystem, no_alloc};
//#[global_allocator]
//static A: AllocCounterSystem = AllocCounterSystem;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Start,
    Blue,
    Yellow,
    Red,
    Black,
    Teal
}

impl Default for Tile {
    fn default() -> Self {
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

#[derive(Clone, Debug, Copy)]
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
}


#[derive(Default, Debug, Copy)]
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



#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Default, Copy)]
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

//#[repr(align(16))]
#[derive(Debug, Clone, Copy)]
pub struct Game {
    turn: u32,
    player: u8,
    box_top: Bag,
    bag: Bag,
    market: Market,
    factories: tinyvec::ArrayVec<[Factory; 5]>,  // TODO set to 9?
    boards: tinyvec::ArrayVec<[Board; 2]>        // TODO set to 4?
}
impl Game {
    pub fn new(players: u8) -> Result<Game, &'static str> {
        let n_factories = get_n_factories(players)?;
        let mut factories = tinyvec::ArrayVec::<[Factory; 5]>::new();
        for _ in 0..n_factories {
            factories.push(Factory::default())
        }

        let mut boards = tinyvec::ArrayVec::<[Board; 2]>::new();
        for _ in 0..players {
            boards.push(Board::default());
        }

        let game = Game {
            turn: 0,
            player: 0,
            box_top: Bag(tinyvec::ArrayVec::<[Tile; 128]>::new()),
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
                if self.bag.len() == 0 && self.box_top.len() > 0 {
                    self.bag.append(&mut self.box_top);
                }
                else if self.bag.len() == 0 {
                    return Ok(())
                }
                else {
                    let tile_i:usize = rng.gen_range(0..self.bag.len());
                    let tile = self.bag.remove(tile_i);
                    factory.push(tile);
                }
            }
        };
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

                    self.box_top.extend_from_slice(board.patterns[row].as_slice());
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
    // #[no_alloc(forbid)]
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
        self.player = (self.player + 1) % self.boards.len() as u8;
        self.turn += 1;
        Ok(())
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

#[test]
fn game_move_iter() {
    let i = GameMove::default();
    println!("Original: {:?}", i);
    assert_eq!(i.into_iter().next().unwrap(), GameMove(1, Tile::Blue, 1));

    assert_eq!(i.into_iter().count(), 5)
}

#[test]
fn sizes() {
    println!("size of azul game: {}", std::mem::size_of::<Game>());
}


use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier)]
#[bits = 3]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile2 {
    Blue,
    Yellow,
    Red,
    Black,
    Teal,
    Start,
    None
}

impl Tile2 {
    fn is_none(&self) -> bool {
        *self == Tile2::None
    }
    fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[derive(BitfieldSpecifier)]
#[derive(Debug)]
enum Player2 {
    One,
    Two
}

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Bag2 {
    blue: B5,
    yellow: B5,
    red: B5,
    black: B5,
    teal: B5,
}

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Factory2 {
    one: Tile2,
    two: Tile2,
    three: Tile2,
    four: Tile2
}
impl Factory2 {
    fn is_empty(&self) -> bool {
        self.one().is_none()
            && self.two().is_none()
            && self.three().is_none()
            && self.four().is_none()
    }
    fn is_full(&self) -> bool {
        self.one().is_some()
            && self.two().is_some()
            && self.three().is_some()
            && self.four().is_some()
    }
    fn add_to_first(&mut self, tile: Tile2) -> Result<(), Tile2> {
        if self.one().is_none() {
            self.set_one(tile);
            return Ok(());
        }
        else if self.two().is_none() {
            self.set_two(tile);
            return Ok(());
        }
        else if self.three().is_none() {
            self.set_three(tile);
            return Ok(());
        }
        else if self.four().is_none() {
            self.set_four(tile);
            return Ok(());
        }
        Err(tile)
    }
}

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Factories2 {
    one: Factory2,
    two: Factory2,
    three: Factory2,
    four: Factory2,
    five: Factory2,
}
impl Factories2 {
    fn is_empty(&self) -> bool {
        self.one().is_empty()
            && self.two().is_empty()
            && self.three().is_empty()
            && self.four().is_empty()
            && self.five().is_empty()
    }
    fn add_to_firsts(&mut self, tile: Tile2) -> Result<(), Tile2>{
        if !self.one().is_full() {
            self.one().add_to_first(tile);
            return Ok(())
        }
        else if !self.two().is_full() {
            self.two().add_to_first(tile);
            return Ok(())
        }
        else if !self.three().is_full() {
            self.three().add_to_first(tile);
            return Ok(())
        }
        else if !self.four().is_full() {
            self.four().add_to_first(tile);
            return Ok(())
        }
        else if !self.five().is_full() {
            self.five().add_to_first(tile);
            return Ok(())
        }
        Err(tile)
    }
}

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Row2 {
    one: bool,
    two: bool,
    three: bool,
    four: bool,
    five: bool
}

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct  Wall2 {
    one: Row2,
    two: Row2,
    three: Row2,
    four: Row2,
    five: Row2
}
#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Market2 {
    blue: B5,
    yellow: B5,
    red: B5,
    black: B5,
    teal: B5,
    start: bool
}

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Pattern2_1 (Tile2, B1);
#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Pattern2_2 (Tile2, B2);
#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Pattern2_3 (Tile2, B2);
#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Pattern2_4 (Tile2, B3);
#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Pattern2_5 (Tile2, B3);

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Patterns2 {
    one: Pattern2_1,
    two: Pattern2_2,
    three: Pattern2_3,
    four: Pattern2_4,
    five: Pattern2_5,
}

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
struct Board2 {
    score: B7,
    wall: Wall2,
    patterns: Patterns2,
    floor: Market2
}

/*#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
struct Boards2 {
    one: Board2,
    two: Board2
}*/

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Game2 {
    player: Player2,
    box_top: Bag2,
    bag: Bag2,
    market: Market2,
    factories: Factories2,
    board_1: Board2,
    board_2: Board2,
    #[skip]
    unused: B7
}

impl Game2 {
    pub fn create() -> Self {
        let game = Game2::new()
            .with_player(Player2::One)
            .with_box_top(Bag2::new()
                .with_blue(0)
                .with_yellow(0)
                .with_red(0)
                .with_black(0)
                .with_teal(0)
            )
            .with_bag(Bag2::new()
                .with_blue(20)
                .with_yellow(20)
                .with_red(20)
                .with_black(20)
                .with_teal(20)
            )
            .with_market(Market2::new()
                .with_start(true)
                .with_blue(0)
                .with_yellow(0)
                .with_red(0)
                .with_black(0)
                .with_teal(0)
            )
            .with_factories(Factories2::new()
                .with_one(Factory2::new()
                    .with_one(Tile2::None)
                    .with_two(Tile2::None)
                    .with_three(Tile2::None)
                    .with_four(Tile2::None)
                )
                .with_two(Factory2::new()
                    .with_one(Tile2::None)
                    .with_two(Tile2::None)
                    .with_three(Tile2::None)
                    .with_four(Tile2::None)
                )
                .with_three(Factory2::new()
                    .with_one(Tile2::None)
                    .with_two(Tile2::None)
                    .with_three(Tile2::None)
                    .with_four(Tile2::None)
                )
                .with_four(Factory2::new()
                    .with_one(Tile2::None)
                    .with_two(Tile2::None)
                    .with_three(Tile2::None)
                    .with_four(Tile2::None)
                )
                .with_five(Factory2::new()
                    .with_one(Tile2::None)
                    .with_two(Tile2::None)
                    .with_three(Tile2::None)
                    .with_four(Tile2::None)
                )
            )
            .with_board_1(Board2::new()
                .with_score(0)
                .with_wall(Wall2::new()
                    .with_one(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                    .with_two(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                    .with_three(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                    .with_four(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                    .with_five(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                )
                .with_patterns(Patterns2::new()
                    .with_one(Pattern2_1::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                    .with_two(Pattern2_2::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                    .with_three(Pattern2_3::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                    .with_four(Pattern2_4::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                    .with_five(Pattern2_5::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                )
                .with_floor(Market2::new()
                    .with_start(false)
                    .with_blue(0)
                    .with_yellow(0)
                    .with_red(0)
                    .with_black(0)
                    .with_teal(0)
                )
            )
            .with_board_2(Board2::new()
                .with_score(0)
                .with_wall(Wall2::new()
                    .with_one(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                    .with_two(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                    .with_three(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                    .with_four(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                    .with_five(Row2::new()
                        .with_one(false)
                        .with_two(false)
                        .with_three(false)
                        .with_four(false)
                        .with_five(false)
                    )
                )
                .with_patterns(Patterns2::new()
                    .with_one(Pattern2_1::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                    .with_two(Pattern2_2::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                    .with_three(Pattern2_3::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                    .with_four(Pattern2_4::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                    .with_five(Pattern2_5::new()
                        .with_0(Tile2::None)
                        .with_1(0)
                    )
                )
                .with_floor(Market2::new()
                    .with_start(false)
                    .with_blue(0)
                    .with_yellow(0)
                    .with_red(0)
                    .with_black(0)
                    .with_teal(0)
                )
            );
        game
    }
    pub fn fill(&mut self, mut rng: StdRng) -> Result<(), &'static str> {    
        if !self.factories().is_empty() {
            return Err("Factories are not empty");
        }

        for _ in 0..(4*5) {
            let choices = [Tile2::Blue, Tile2::Yellow, Tile2::Red, Tile2::Black, Tile2::Teal];
            let weights = [self.bag().blue(), self.bag().yellow(), self.bag().red(), self.bag().black(), self.bag().teal()];
            let dist = WeightedIndex::new(&weights).unwrap();

            let tile = choices[dist.sample(&mut rng)];

            match self.set_factories(self.factories().add_to_firsts(tile)) {
                Err(e) => return Err("Couldn't add tile to factory"),
                Ok(_) => {
                    if tile == Tile2::Blue {
                        self.set_bag(self.bag().with_blue(self.bag().blue() - 1));
                    }
                    else if tile == Tile2::Yellow {
                        self.set_bag(self.bag().with_yellow(self.bag().yellow() - 1));
                    }
                }
            };

            println!("{:?}", tile);
            println!("{:#?}", self.bag());
            //println!("{:#?}", self.factories());
        }

        Ok(())
/*        for factory in &mut self.factories {
            for _ in 0..4 {
                if self.bag.len() == 0 && self.box_top.len() > 0 {
                    self.bag.append(&mut self.box_top);
                }
                else if self.bag.len() == 0 {
                    return Ok(())
                }
                else {
                    let tile_i:usize = rng.gen_range(0..self.bag.len());
                    let tile = self.bag.remove(tile_i);
                    factory.push(tile);
                }
            }
        };
*/
    }
    /*pub fn do_move(&mut self, game_move: GameMove) -> Result<(), &'static str> {
        match game_move {
            () => 
        }
    }*/
}

use std::time::{Instant, Duration};

pub fn size_of_bitfields() -> Result<(), &'static str> {
    //println!("size of bitfield game: {}", std::mem::size_of::<Game2>());

    let game2 = Game2::create();
    //println!("debug: {:#?}", game2);
    //println!("{:?}", game2.into_bytes());

    let now = Instant::now();
    for _ in 0..1_000_000_000 {
        let mut game2_2 = game2.clone();
        game2_2.set_player(Player2::Two);
        std::hint::black_box(game2_2);
    }
    let game_2_time = now.elapsed().as_nanos();

    let game1 = Game::new(2)?;

    let now = Instant::now();
    for _ in 0..1_000_000_000 {
        let mut game1_2 = game1.clone();
        game1_2.turn = 1;
        std::hint::black_box(game1_2);
    }
    let game_1_time = now.elapsed().as_nanos();

    println!("{} | {}", game_1_time, game_2_time);

    Ok(())


}