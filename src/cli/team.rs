use crate::stats::team::Team;

pub fn new() {
    let mut buf = String::new();
    let team_name = super::input_prompt(&"Enter new team name".to_string(), &mut buf);

    let team = Team::new(team_name);
    match team.save(None) {
        Ok(_) => println!("Team Save succesful!"),
        Err(e) => eprintln!("Team save failed {:?}", e)
    };
}