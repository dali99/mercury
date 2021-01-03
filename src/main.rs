mod azul;
use azul::*;

fn main() -> Result<(), &'static str>{

    let mut game = Game::new(2)?;
    game.fill()?;
    println!("{:#?}", game);
    let game2 = game.do_move(GameMove(1, Tile::Red, 1))?;
    let game2 = game2.do_move(GameMove(0, Tile::Blue, 1))?;
    println!("{:#?}", game2);
    Ok(())
}
