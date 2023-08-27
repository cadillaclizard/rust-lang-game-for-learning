mod screen;

use std::io;

fn main() -> io::Result<()> {
    crate::screen::Screen::new();
    // let game = TheGame::new("The Game".to_string());
    // game.run();

    // let cli: Cli = argh::from_env();
    // let tick_rate = Duration::from_millis(25); //(cli.tick_rate);
    // crate::crossterm::run(tick_rate, cli.enhanced_graphics)?;

    Ok(())
}
