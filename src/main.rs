mod azul;
use azul::*;
use rand::prelude::*;

fn main() -> Result<(), &'static str> {
    let mut g_rng = StdRng::seed_from_u64(42);
    for _ in 0..500000 {
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
        while let Some(mut i) = game_move {
            match game.do_move(i) {
                Err(e) => {
                    //println!("{:?}: {}", i, e);
                    game_move = i.next();
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

impl Iterator for Tile {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile>{
        match *self {
            Tile::Start => None,
            Tile::Blue => Some(Tile::Yellow),
            Tile::Yellow => Some(Tile::Red),
            Tile::Red => Some(Tile::Black),
            Tile::Black => Some(Tile::Teal),
            Tile::Teal => None
        }
    }
}

impl Iterator for GameMove {
    type Item = GameMove;

    fn next(&mut self) -> Option<GameMove> {
        let mut factory = self.0 + 1;
        let mut _tile = Some(self.1);
        let mut pattern = self.2;
        if factory > 9 {
            factory = 0;
            _tile = _tile.unwrap().next();
        };
        let tile = match _tile {
            None => {
                match pattern {
                    6 => pattern = 0,
                    0 => return None,
                    _ => pattern = pattern + 1
                };
                Tile::Blue
            },
            Some(x) => x
        };

        Some(GameMove(factory, tile, pattern))
    }
}