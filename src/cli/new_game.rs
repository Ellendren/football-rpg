pub fn new_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Starting new game");
    super::team::new();
}