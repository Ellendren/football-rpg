use crate::stats::player::{self, Player};
use colored::Colorize;

pub fn new() {
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

    let player = player::Player::new(speed, strength, agility, reaction, charisma, awareness, Some(fname), Some(lname));
    match player.save() {
        Ok(_) => println!("Save Succesful!"),
        Err(e) => {
            let err_msg = format!("{}: {:?}", "Error saving file".red(), e);
            eprintln!("{}", err_msg)
        }
    };
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
        println!("\t{}) {}", i+1, file);
    }
    let mut buf= String::new();
    let pick = super::input_prompt(&"Select player to load by number".to_string(), &mut buf);
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