use crate::stats::player::{self, Player};
use colored::Colorize;

pub fn new() -> player::Player {
    println!("Creating new player");
    println!("\nEnter pc details");

    let mut buf = String::new();
    let fname = super::input_prompt(&format!("\tFirst name"), &mut buf);
    let lname = super::input_prompt(&format!("\tLast name"), &mut buf);
    let speed = attribute_prompt(&format!("speed"), &mut buf);
    let strength = attribute_prompt(&format!("strength"), &mut buf);
    let agility = attribute_prompt(&format!("agility"), &mut buf);
    let reaction = attribute_prompt(&format!("reaction"), &mut buf);
    let charisma = attribute_prompt(&format!("charisma"), &mut buf);
    let awareness = attribute_prompt(&format!("awareness"), &mut buf);

    let player = player::Player::new_pc(speed, strength, agility, reaction, charisma, awareness, Some(fname), Some(lname));
    player
}

pub fn load(){
    // get player files
    let files = match Player::list_players(None){
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error {:?}", e);
            return;
        }
    };

    // print playes files
    println!("Available: players");
    for (i, file) in files.iter().enumerate() {
        println!("\t{}) {:?}", i, file.file_name());
    }
    let mut buf= String::new();
    let prompt = &"Select player to load by number".to_string();
    let mut pick_str = super::input_prompt(prompt, &mut buf);
    let mut pick = pick_str.parse::<usize>();
    while !pick.is_ok() {
        println!("{}", "Invailid input".blue());
        pick_str = super::input_prompt(prompt, &mut buf);
        pick = pick_str.parse::<usize>();
    }
    
    let pick_i = pick.unwrap();
    let path = Some(files[pick_i].path().to_string_lossy().to_string());

    let player = Player::load(path); 
    println!("{:?}", player);
}

//attribute_prompt(prompt: &String, buf: &mut String) -> Option<u16>
//params:
//  params:
//  - prompt: name of the attribute
//  - buf: buffer for the user input
//returns: Ok(u32) if valid input or None
fn attribute_prompt(prompt: &String, buf: &mut String) -> Option<u16> {
    let defaults = ["default", "none"];

    let prompt = format!("Enter positive number for {},(or default for to use the default attribute)", prompt);

    loop {
        let res = super::input_prompt(&prompt, buf);

        for default in defaults {
            if &res.to_lowercase() == default{
                return None;
            }
        }

        match res.parse::<u16>() {
            Ok(num) => return Some(num),
            Err(_) => eprintln!("{} is not a valid number. Try agign", res)
        }
    }
}