mod azul;
use azul::Game;

fn main() -> Result<(), &'static str>{

    let mut game = Game::new(2)?;
    println!("{:#?}", game);
    game.fill()?;
    Ok(())
}
