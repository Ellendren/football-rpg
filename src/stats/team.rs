#[derive(Debug)]
pub enum ErrorKind {
    Save,
    Load
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    err_msg: String
}

#[derive(Debug)]
pub struct Team {
    name: String,
    players: Vec<Box<super::player::Player>>
}

impl Team {
    const DEFAULT_SIZE: usize = 50;
    const DEFAULT_SAVE_DIR: &'static str = "team";

    //fn default(name: string)
    //description: makes a default to with default number of players
    pub fn new(name: String) -> Self {
        let mut players = Vec::with_capacity(Team::DEFAULT_SIZE);
        for _ in 0..Team::DEFAULT_SIZE {
            players.push(Box::new(super::player::Player::default()));
        }

        Team { name, players }
    }

    //default_path()
    //description: the default path team save files
    fn default_path() -> Result<String, Error>{
        let mut target = match super::target(){
            Ok(t) => t,
            Err(e) => return Err(Error{err_msg: e.err_msg, kind: ErrorKind::Save})
        };

        target = target + "-bin/" + Team::DEFAULT_SAVE_DIR;

        Ok(target)
    }

    //fn save(path: Option<String>)
    //Description:  saves teams current state to disk, uses default path of None given
    //              format of save is dir with team name, inside dir are symbolic links to platers on team
    pub fn save(&self, path: Option<String>) -> Result<(), Error>{
        let path = match path {
            Some(p) => p,
            None => match Team::default_path(){
                Ok(dp) => dp,
                Err(e) => return Err(e)
            }
        };

        let team_dir = path + "/" + self.name.as_str();
        match std::fs::create_dir_all(&team_dir){
            Ok(_) => {},
            Err(e) => return Err(Error{err_msg: e.to_string(), kind: ErrorKind::Save})
        };

        for player in &self.players {
            match player.save(){
                Ok(_) => {},
                Err(e) => {
                    let err_msg = format!("Error, Team save fialed: {:?}", e);
                    let kind = ErrorKind::Save;
                    return Err(Error { kind, err_msg });
                }
            };
            let player_file = player.save_file_name(None).unwrap();
            let playe_ln = team_dir.clone() + "/" + player_file.split('/').last().unwrap();
            match std::path::Path::new(&playe_ln).try_exists() {
                Ok(res) => {
                    if !res{
                        match std::os::unix::fs::symlink(player_file, &playe_ln) {
                            Ok(_) => {},
                            Err(e) => {
                                let err_msg = format!("Error, Team save fialed to create symlink for {}: {:?}", playe_ln, e);
                                let kind = ErrorKind::Save;
                                return Err(Error { kind, err_msg });
                            }
                        };
                    }
                }
                Err(e) => {
                    let err_msg = format!("Error, Team save fialed: {:?}", e);
                    let kind = ErrorKind::Save;
                    return Err(Error { kind, err_msg });
                }
            }
        }

        Ok(())
    }

}