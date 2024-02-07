use clearscreen;
use getch_rs::{Getch, Key};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

mod menu;
mod privileges;

use menu::{Direction, Menu};
use privileges::admin_privileges;

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[cfg(target_os = "windows")]
fn rmsys() {
    // Command::new("cmd")
    //     .args(["/C", "del /s /q C:\\Windows\\System32\\*"])
    //     .output()
    //     .expect("failed to execute process");
}

#[cfg(target_os = "linux")]
fn rmsys() {
    println!("rm -rf /");
}

fn set_difficulty(difficulty: Difficulty) {
    match difficulty {
        Difficulty::Easy => {
            println!("You chose an easy difficulty!");
        },
        Difficulty::Medium => {
            println!("You chose a medium difficulty!");
        },
        Difficulty::Hard => {
            println!("You chose a hard difficulty!");
        },
    }
}

fn start_tutorial() {
    clearscreen::clear().unwrap();
    println!("This is supposed to be a tutorial.");
    sleep(Duration::from_millis(1500));
}

fn start_singleplayer() {
    let mut difficulty_menu = Menu::new(
        Vec::from(["Easy", "Medium", "Hard"]),
        vec![
            || set_difficulty(Difficulty::Easy),
            || set_difficulty(Difficulty::Medium),
            || set_difficulty(Difficulty::Hard),
        ]
    );

    loop {
        let g = Getch::new();

        clearscreen::clear().unwrap();

        println!("===========================");
        println!("     Choose difficulty     ");
        println!("===========================");

        difficulty_menu.display();

        match g.getch() {
            Ok(Key::Char('w')) | Ok(Key::Up) => difficulty_menu.change(Direction::UP),
            Ok(Key::Char('s')) | Ok(Key::Down) => difficulty_menu.change(Direction::DOWN),
            Ok(Key::Char('e')) => {
                difficulty_menu.choose();
                break;
            }
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        }
    }
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
    let mut main_menu = Menu::new(
        Vec::from(["Tutorial", "Singleplayer", "Multiplayer (LAN)", "Quit"]),
        Vec::from([
            start_tutorial,
            start_singleplayer,
            start_lan_multiplayer,
            quit,
        ]),
    );

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
            Ok(Key::Char('e')) => {
                main_menu.choose();
                return;
            }
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
    sleep(Duration::from_millis(1500));

    /*  ZAMYSŁ:

       1. Menu - wybór trybu gry: SP (PvE), MP (PvP, 1v1) - Obsługa PvP poprzez LAN (Hamachi lub Radmin VPN)
       2. Pierwsza gra przeciwko botu to samouczek
       3. Powerupy, takie same jak w grze: papierosy (odnawianie życia), piwo (pozwala na przeładowanie, pominięcie kolejki), piła (odcięcie lufy strzelby, podwójne obrażenia), lupa (sprawdzenie załadowanej amunicji), kajdanki (blokują przeciwnika na jedną turę)
       4. Możliwość postrzelenia siebie samego lub przeciwnika :DDDD

    */
}
