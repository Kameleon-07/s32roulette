use std::env;
use std::fs;

fn user_os() -> &'static str {
    let users_os = env::consts::OS;

    return users_os;
}

fn rmsys() {
    // Usunięcie system32 lub /
}

fn main() {

    println!("Boom, you System32 is non-existent now. Good Luck!");

    // imo mocny przekaz ^
    /*  ZAMYSŁ:
        1. Menu - wybór trybu gry: SP (PvE), MP (PvP, 1v1) - Obsługa PvP poprzez LAN
        2. Pierwsza gra przeciwko botu to samouczek
        3. Powerupy, takie same jak w grze: papierosy (odnawianie życia), piwo (pozwala na przeładowanie, pominięcie kolejki), piła (odcięcie lufy strzelby, podwójne obrażenia), lupa (sprawdzenie załadowanej amunicji), kajdanki (blokują przeciwnika na jedną turę)
        4. Możliwość postrzelenia siebie samego lub przeciwnika :DDDD
        5. Wersja bezpieczna - bez usuwania katalogu '/' i 'system32'
        6. Wersja niebezpieczna - z usuwaniem
     */

    println!("{}", user_os());
}