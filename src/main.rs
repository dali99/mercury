mod azul;
use azul::*;

fn main() -> Result<(), &'static str>{

    let mut game = Game::new(2)?;
    game.fill()?;
    println!("{:#?}", game);
    let game2 = game.do_move(GameMove(0, Tile::Red, 0))?;
    println!("{:#?}", game2);
    Ok(())
}
