#![feature(test)]
use cached::proc_macro::cached;

mod azul;
use azul::*;
mod azul2;
use azul2::*;
use rand::prelude::*;

use thousands::Separable;

//#[global_allocator]
//static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

//#[global_allocator]
//static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<(), &'static str> {

    let program = std::env::args().nth(1).expect("no program given")
        .parse().unwrap_or(1);

    // size_of_stuff();

    return match program {
        1 => {
            let mut game = some_game()?;

            game.do_move(GameMove(2, Tile::Red, 3))?;
            game.do_move(GameMove(5, Tile::Yellow, 2))?;

            game.do_move(GameMove(3, Tile::Blue, 2))?;
            //game.do_move(GameMove(0, Tile::Black, 4))?;

            //game.do_move(GameMove(5, Tile::Black, 1))?;
            //game.do_move(GameMove(0, Tile::Blue, 3))?;

            println!("{}", count_options(game, 1, 2));
            Ok(())
        },
        2 => calculate_options(),
        3 => size_of_bitfields(),
        4 => {
            let mut game = Game2::create();
            game.fill(StdRng::seed_from_u64(42));
            Ok(())
        },
        _ => Err("Not a valid program")
    }
}

fn run(rng: StdRng) -> Result<(), &'static str> {

    let mut game = Game::new(2)?;
    game.fill(StdRng::from_entropy())?;

    let mut all_err = false;
    while !all_err {
        //println!("{:#?}", game);
        all_err = true;
        let mut game_move:Option<GameMove> = Some(GameMove::default());
        while let Some(i) =  game_move {
            match game.do_move(i) {
                Err(e) => {
                    //println!("{:?}: {}", i, e);
                    game_move = i.into_iter().next();
                },
                Ok(_) => {
                    all_err = false;
                    break;
                }
            }
        }
    }
    Ok(())
}

fn calculate_options() -> Result<(), &'static str> {
    let mut game = complicated()?;
    println!("{:#?}", game);

    // We know how many possibilities there are the first round...
    game.do_move(GameMove(1, Tile::Blue, 0))?;
    // We also know how many possiblities there are the second round.
    // NB: THIS STEP INTRODUCES ERROR, THE REAL NUMBER WILL BE SMALLER THAN THIS
    game.do_move(GameMove(0, Tile::Yellow, 0))?;

    let options = count_options(game, 1, 5);
    println!("{}", options * (20 * 6)*(19 * 6));

    Ok(())
}

#[cached(size=45_000_000, key = "State", convert = r#"{ _game.state }"#)]
fn count_options(_game: Game, depth: u8, treshold: u8) -> u128 {
    let before = std::time::Instant::now();

    let mut sum = 0;
    let mut all_failed = true;

    let mut multiplier = 1;

    // Upper bound calculation by skipping first depth and assuming most complicated possible game as multiplier
    let game = match depth {
        0 => {
            let mut new_game = _game.clone();
            for i in GameMoveIter::new(2).next() {
                match new_game.do_move(i) {
                    Ok(_) => break,
                    Err(_) => continue
                }
            }
            multiplier = 20*6;
            new_game
        },
        _ => {
            _game
        }
    };

    let i = GameMoveIter::new(2);

    for game_move in i {
        //println!("{:?}", game_move);
        if game.is_legal(game_move).is_err() {
            continue;
        };
        let mut new_game = game.clone();
        let r = new_game.do_move(game_move);
        match r {
            Ok(_) => sum += {/*println!("{}", depth);*/ all_failed = false; /*coz::progress!("OK");*/ multiplier * count_options(new_game, depth + 1, treshold)},
            Err(_) => continue
        };
    };

    if all_failed {
        if depth <= treshold {
            println!("{}: ALL FAILED!", depth);
        }
        return 1;
    }

    if depth <= treshold {
        let elapsed = before.elapsed();
        let hashrate = sum / std::cmp::max(elapsed.as_millis() as u128, 1u128) * 1000u128;
        println!("{}: sum: {}, took +{:?}: {}/s",
                  depth,   sum,      elapsed, hashrate.separate_with_spaces()
        );
    }
    return sum;

    /*game_move.iter_mut().map(|x| {
        let mut new_game = game.clone();
        let r = new_game.do_move(*x);
        println!("{:#?}", r);
        match r {
            Ok(_) => count_options(new_game),
            Err(_) => 0
        }
    })
    .sum::<u128>()*/

}
