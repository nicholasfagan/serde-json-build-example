// Notice how serde isn't even a runtime dependency!

#[derive(Debug)]
/// A definition of Game
pub struct Game {
    pub name: &'static str,
}

// Copmile-time parsed games.json, converted to rust code and included in the program.
const GAMES: &'static [Game] = &include!(concat!(env!("OUT_DIR"), "/games.rs"));

fn main() {
    // Print the games to prove they are here:
    println!("The constant GAMES = {:#?}", GAMES);
}
