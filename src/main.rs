mod azul;
use azul::*;
use rand::prelude::*;

fn main() -> Result<(), &'static str> {
    let mut g_rng = StdRng::seed_from_u64(42);
    for _ in 0..10000 {
        let rng = match StdRng::from_rng(&mut g_rng) {
            Ok(r) => r,
            Err(e) => {
                println!("error! {:?}", e);
                break;
            }
        };
        run(rng)?;
    }
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

#[test]
fn calculate_options() -> Result<(), String> {
    let game = complicated()?;
    println!("{:#?}", game);

    println!("{}", count_options(game, 0));

    Ok(())
}

fn count_options(game: Game, depth: u8) -> u128 {
    let mut sum = 0;
    let i = GameMove::default();
    let mut all_failed = true;
    for game_move in i {
        //println!("{:?}", game_move);
        let mut new_game = game.clone();
        let r = new_game.do_move(game_move);
        match r {
            Ok(_) => sum += {/*println!("{}", depth);*/ all_failed = false; count_options(new_game, depth + 1)},
            Err(_) => continue
        };
    };

    if all_failed {
        //println!("{}: ALL FAILED!", depth);
        return 1;
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