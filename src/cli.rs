use std::{fmt::format, io::Write, option};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
mod new_game;
mod load;
mod player;

struct NewGame;

impl NewGame {
    const NAME: &'static str = "new";
    const DESC: &'static str = "Starts a new game";
}

struct LoadGame;

impl LoadGame {
    const NAME: &'static str = "load";
    const DESC: &'static str = "Load an old game";
}

enum Commands {
    Menu(MenuCommands), //main meun
}

impl Commands {
    fn menu(&self) -> String {
        match self {
            Commands::Menu(m) => m.menu()
        }
    }
}

#[derive(EnumIter)]
enum MenuCommands {
    NewGame,
    LoadGame
}

impl MenuCommands {
    fn menu(&self) -> String{
        match self {
            MenuCommands::NewGame => format!("{}: {}", NewGame::NAME, NewGame::DESC),
            MenuCommands::LoadGame => format!("{}: {}", LoadGame::NAME, LoadGame::DESC),
        }
    }
}

//starts the CLI for football rpg
pub fn start() {
    println!("Welcome to football rpg");

    // user input
    let mut option = String::new();
    while option.to_lowercase() != format!("exit"){
        print_menu();
        input_prompt(&format!("pick option(or 'exit' to quit)"), &mut option);
        check_choice(&option);
    }

    exit();
}

//input_prompt(prompt: &String, buf: &mut String) -> &String
//params:
//  - prompt: the cli prompt fot the user
//  - buf: buffer for the user input
//returns: buf has the user prompt for a single line
pub fn input_prompt(prompt: &String, buf: &mut String) -> String{
    buf.clear();
    print!("{}: ", prompt);
    std::io::stdout().flush().unwrap_or_else(|e| {panic!("{:?}", e)});
    std::io::stdin().read_line(buf).unwrap_or_else(|e| {panic!("{:?}", e)});
    buf.pop();
    buf.clone()
}   

//pritnts a command menu
//params:
//  - command: a generic "Command" enum iter with a menu impl with the defintion menu(&self) -> String
fn print_menu() {
    let mut menu = format!("Command menu:");
   
    for command in MenuCommands::iter() {
        menu.push_str("\n\t");
        menu.push_str(&command.menu());
    }

    println!("{}", menu);
}

fn exit() {
    println!("Goodbye...");
    let dur = core::time::Duration::from_secs_f32(0.2);
    std::thread::sleep(dur);
    std::process::exit(0);
}

//checks users menu choice
fn check_choice(choice: &str) {
    match choice {
        NewGame::NAME => new_game::new_game(),
        LoadGame::NAME => load::load(),
        _ => {}
    }
}