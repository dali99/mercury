#![feature(bench_black_box)]

use rand::prelude::*;
use rand::distributions::WeightedIndex;


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

/*            match self.set_factories(self.factories().add_to_firsts(tile)) {
                Err(e) => return Err("Couldn't add tile to factory"),
                Ok(_) => {
                    if tile == Tile2::Blue {
                        self.set_bag(self.bag().with_blue(self.bag().blue() - 1));
                    }
                    else if tile == Tile2::Yellow {
                        self.set_bag(self.bag().with_yellow(self.bag().yellow() - 1));
                    }
                }
            }; */

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

    // let game1 = Game::new(2)?;

    let now = Instant::now();
    for _ in 0..1_000_000_000 {
    //   let mut game1_2 = game1.clone();
    //    game1_2.turn = 1;
    //    std::hint::black_box(game1_2);
    }
    let game_1_time = now.elapsed().as_nanos();

    println!("{} | {}", game_1_time, game_2_time);

    Ok(())


}
