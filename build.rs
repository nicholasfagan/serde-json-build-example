// a description of the 'Game' type is needed, to parse it from json.
// It's kinda bad practice to refernce the crates code from it's build script,
// so in a real program the 'Game' type should be moved to another crate that 
// both the build script and this crate can depend on.
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// A definition of Game that can be serialized
pub struct Game {
    pub name: &'static str,
}


fn main() {
    // Here you could `std::fs::read_to_string()`, but then remember to
    // println!("cargo:rerun-on-changed=games.json") as well.
    // Would save from recompiling the build script each time games.json changes.
    //
    // Parse the json into games
    let games: Vec<Game> = serde_json::from_str(include_str!("games.json")).unwrap();
    // get the output path like "$OUT_DIR/games.rs"
    let mut out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    out.push("games.rs");
    // write the Debug output of games to 'games.rs'.
    // A more complicated struct may require something like the `uneval`
    // crate to serialize this to rust code, but for this simple example Debug works.
    std::fs::write(out, format!("{:?}", games)).unwrap();
}
