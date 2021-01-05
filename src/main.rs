mod azul;
use azul::*;
use rand::prelude::*;

//#[global_allocator]
//static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

//#[global_allocator]
//static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;


fn main() -> Result<(), &'static str> {
    
    let rng = StdRng::seed_from_u64(42);
    let mut game = Game::new(2, rng)?;
    game.fill()?;
    println!("{:#?}", game);

    game.do_move(GameMove(1, Tile::Red, 2))?;
    game.do_move(GameMove(4, Tile::Red, 2))?;

    game.do_move(GameMove(2, Tile::Red, 1))?;

    println!("{}", count_options(game, 0, 2));
    

    // calculate_options()?;

    Ok(())
}

fn run(rng: StdRng) -> Result<(), &'static str> {

    let mut game = Game::new(2, rng)?;
    game.fill()?;

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

    let options = count_options(game, 0, 4);
    println!("{}", options * 20 * 6);

    Ok(())
}

fn count_options(game: Game, depth: u8, treshold: u8) -> u128 {
    coz::scope!("count option");
    let mut sum = 0;
    let i = GameMove::default();
    let mut all_failed = true;
    for game_move in i {
        //println!("{:?}", game_move);
        let mut new_game = game.clone();
        let r = new_game.do_move(game_move);
        match r {
            Ok(_) => sum += {/*println!("{}", depth);*/ all_failed = false; coz::progress!("OK"); count_options(new_game, depth + 1, treshold)},
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
        println!("{}: sum: {}", depth, sum);
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