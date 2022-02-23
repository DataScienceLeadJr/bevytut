mod invaders;

use clap::{App, arg};

use invaders::invaders_app;

fn main() {
    let matches =  App::new("Bevy tutorials")
                            .version("0.2")
                            .author("Sindre Fjermestad")
                            .about("Bevy engine tutorials followed")
                            .arg(arg!(-g --game <GAME_NAME> "Specify which game tutorial to run.").required(true).ignore_case(true))
                            .get_matches();
    
    match matches.value_of("game").unwrap() {
        "invaders" => invaders_app(),
        "gol" | "game-of-life" | "game_of_life"  => {
            println!("GAME OF LIFE TIME!");
        },
        _ => panic!("WHAT WAS THAT!?"),
    }
}
