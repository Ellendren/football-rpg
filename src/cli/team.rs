use crate::stats::team::Team;
use colored::Colorize;

pub fn new() {
    let mut buf = String::new();
    let team_name = super::input_prompt(&"Enter new team name".to_string(), &mut buf);

    let team = Team::new(team_name);
    match team.save(None) {
        Ok(_) => println!("Team Save succesful!"),
        Err(e) => eprintln!("Team save failed {:?}", e)
    };
}

pub fn load() {
    let list = match  Team::list_teams(None){
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error listing teams: {:?}", e);
            return;
        }
    }; 

    for (i, team) in list.iter().enumerate() {
        println!("\t{}) {:?}", i, team.file_name());
    }

    let mut buf= String::new();
    let prompt = &"Select team to load by number".to_string();
    let mut pick_str = super::input_prompt(prompt, &mut buf);
    let mut pick = pick_str.parse::<usize>();
    while !pick.is_ok() {
        println!("{}", "Invailid input".blue());
        pick_str = super::input_prompt(prompt, &mut buf);
        pick = pick_str.parse::<usize>();
    }

    let pick = pick.unwrap();
    if pick < list.len() {
        let team = Team::load(&list[pick]);
        println!("{:?}", team);
    }
    else {
        eprintln!("{}, is an invalid choice", pick);
        load();
    }
}