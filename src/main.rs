mod azul;
use azul::Game;

fn main() -> Result<(), &'static str>{

    let game = Game::new(2)?;
    println!("{:#?}", game);

    Ok(())
}
