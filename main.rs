use clearscreen;
use getch_rs::{Getch, Key};
use std::env;
use std::fs;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

mod privileges;
use privileges::admin_privileges;


#[cfg(target_os = "windows")]
fn rmsys() {
    println!("Yo compyuta has vayros");
}

#[cfg(target_os = "linux")]
fn rmsys() {
    println!("rm -rf /");
}


fn start_singleplayer() {
    clearscreen::clear().unwrap();
    println!("Wow, amazing game!");
}

fn start_lan_multiplayer() {
    clearscreen::clear().unwrap();
    println!("Wow, amazing MULTIPLAYER game!");
}

fn quit() {
    exit(0);
}

fn wtf_bro() {
    clearscreen::clear().unwrap();
    println!("This isn't supposed to happen :00000");
}

fn menu() -> usize {
    let mut counter: usize = 0;
    loop {
        let mut options = [
            "  Singleplayer".to_string(),
            "  Multiplayer (LAN)".to_string(),
            "  Quit".to_string(),
        ];
        options[counter] = options[counter].replacen(" ", ">", 1);
        let g = Getch::new();
        clearscreen::clear().unwrap();

        println!("===========================");
        println!("  Welcome to S32 Roulette!");
        println!("===========================");
        println!("Use 'W' and 'S' or arrow keys to navigate");
        println!("Use 'E' to select");
        for option in options {
            println!("{}", option);
        }

        match g.getch() {
            Ok(Key::Char('w')) | Ok(Key::Up) => {
                if counter != 0 {
                    counter -= 1;
                }
            }
            Ok(Key::Char('s')) | Ok(Key::Down) => {
                if counter != 2 {
                    counter += 1;
                }
            }
            Ok(Key::Char('e')) => {
                return counter;
            }
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    let is_admin = admin_privileges::check_privileges();

    if is_admin == 0 {
        println!("You need administrator priviliges for this game to work properly");
        println!("Press any key to close...");

        Getch::new().getch().unwrap();
        exit(0);
    }

    let counter = menu();
    match counter {
        0 => start_singleplayer(),
        1 => start_lan_multiplayer(),
        2 => quit(),
        _ => wtf_bro(),
    };

    /*  ZAMYSŁ:

       1. Menu - wybór trybu gry: SP (PvE), MP (PvP, 1v1) - Obsługa PvP poprzez LAN (Hamachi lub Radmin VPN)
       2. Pierwsza gra przeciwko botu to samouczek
       3. Powerupy, takie same jak w grze: papierosy (odnawianie życia), piwo (pozwala na przeładowanie, pominięcie kolejki), piła (odcięcie lufy strzelby, podwójne obrażenia), lupa (sprawdzenie załadowanej amunicji), kajdanki (blokują przeciwnika na jedną turę)
       4. Możliwość postrzelenia siebie samego lub przeciwnika :DDDD

    */
}
