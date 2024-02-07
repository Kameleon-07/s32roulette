use clearscreen;
use getch_rs::{Getch, Key};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

mod menu;
mod privileges;

use privileges::admin_privileges;
use menu::{Direction, Menu};

#[cfg(target_os = "windows")]
fn rmsys() {
    println!("Yo compyuta has vayros");
}

#[cfg(target_os = "linux")]
fn rmsys() {
    println!("rm -rf /");
}

fn start_tutorial() {
    clearscreen::clear().unwrap();
    println!("This is supposed to be a tutorial.");
    sleep(Duration::from_millis(1500));
}

fn start_singleplayer() {
    clearscreen::clear().unwrap();
    println!("Wow, amazing game!");
    sleep(Duration::from_millis(1500));
}

fn start_lan_multiplayer() {
    clearscreen::clear().unwrap();
    println!("Wow, amazing MULTIPLAYER game!");
    sleep(Duration::from_millis(1500));
}

fn quit() {
    exit(0);
}

fn menu() {
    let mut main_menu = Menu::new(Vec::from(["Tutorial".to_string(), "Singleplayer".to_string(), "Multiplayer (LAN)".to_string(), "Quit".to_string()]), Vec::from([start_tutorial, start_singleplayer, start_lan_multiplayer, quit]));

    loop {
        let g = Getch::new();
        clearscreen::clear().unwrap();

        println!("===========================");
        println!("  Welcome to S32 Roulette!");
        println!("===========================");
        println!("Use 'W' and 'S' or arrow keys to navigate");
        println!("Use 'E' to select");

        main_menu.display();

        match g.getch() {
            Ok(Key::Char('w')) | Ok(Key::Up) => main_menu.change(Direction::UP),
            Ok(Key::Char('s')) | Ok(Key::Down) => main_menu.change(Direction::DOWN),
            Ok(Key::Char('e')) => {main_menu.choose(); return},
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    let is_admin = admin_privileges::is_launched_as_admin();

    if !is_admin {
        println!("You need administrator priviliges for this game to work properly");
        println!("Press any key to close...");

        Getch::new().getch().unwrap();
        exit(0);
    }

    menu();

    /*  ZAMYSŁ:

       1. Menu - wybór trybu gry: SP (PvE), MP (PvP, 1v1) - Obsługa PvP poprzez LAN (Hamachi lub Radmin VPN)
       2. Pierwsza gra przeciwko botu to samouczek
       3. Powerupy, takie same jak w grze: papierosy (odnawianie życia), piwo (pozwala na przeładowanie, pominięcie kolejki), piła (odcięcie lufy strzelby, podwójne obrażenia), lupa (sprawdzenie załadowanej amunicji), kajdanki (blokują przeciwnika na jedną turę)
       4. Możliwość postrzelenia siebie samego lub przeciwnika :DDDD

    */
}
