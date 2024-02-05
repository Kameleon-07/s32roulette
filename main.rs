use clearscreen;
use getch_rs::{Getch, Key};

fn start_singleplayer() {
    clearscreen::clear().unwrap();
    println!("Wow, amazing game!");
}

fn start_lan_multiplayer() {
    clearscreen::clear().unwrap();
    println!("Wow, amazing MULTIPLAYER game!");
}

fn quit() { 
    std::process::exit(0);
}

fn wtf_bro() {
    clearscreen::clear().unwrap();
    println!("This isn't supposed to happen :00000");
}

fn menu() -> usize {
    let mut counter: usize = 0;
    loop {
        let mut options = ["  Single Player".to_string(), "  LAN Multiplayer".to_string(), "  Quit".to_string()];
        options[counter] = options[counter].replacen(" ", ">", 1);
        let g = Getch::new();
        clearscreen::clear().unwrap();
        
        println!("===========================");
        println!("  Welcome to S32 Roulette!");
        println!("===========================");
        println!("Use 'W' and 'S' to navigate");
        println!("Use 'E' to select");
        for option in options {
            println!("{}", option);
        }

        match g.getch() {
            Ok(Key::Char('w')) => {
                if counter != 0 {
                    counter -= 1;
                }
            },
            Ok(Key::Char('s')) => {
                if counter != 2 {
                    counter += 1;
                }
            },
            Ok(Key::Char('e')) => {
                return counter;
            },
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    let counter = menu();
    match(counter) {
        0 => start_singleplayer(),
        1 => start_lan_multiplayer(),
        2 => quit(),
        _ => wtf_bro(),
    };
}