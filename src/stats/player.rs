use std::io::{Read, Write};
use std::fmt;

#[derive(Debug)]
pub enum  Error {
    Save{err_msg: String},
    Load{err_msg: String}
}

#[derive(Debug)]
struct HP {
    max: u32,
    curr: i64
}

impl HP {
    //stream()
    //description: creates HPSTream
    fn stream(&self) -> HPStream {
        let stream;

        unsafe {
            let max = super::util::u32_to_u8array(self.max);
            let curr = super::util::i64_to_u8array(self.curr);

            stream = HPStream{max, curr};
        }
        stream
    }
}

#[derive(Debug,Clone, Copy)]
struct HPStream {
    max: [u8; 4],
    curr: [u8; 8]
}

impl HPStream {
    // bytes in the stream
    const SIZE: usize = 12;

    //from_Stream(stream: [u8]) -> Self
    //description: creates a new HPStream from a file stream
    //params:
    //  - stream: a byte stream formated in the HPStream::raw() style
    fn from_u8array(stream: [u8; HPStream::SIZE]) -> Self {
        let max = stream[..4].try_into().unwrap();
        let curr = stream[4..HPStream::SIZE].try_into().unwrap();

        HPStream { max, curr }
    }

    //raw(&self)
    //description: creates a structured byte stream of all the datavaules in HPStream
    //returns: array structure - 12 bytes; max b1-b4, curr b5-b12 
    fn raw(&self) -> Vec<u8> {
        let mut raw = self.max.to_vec();
        let mut curr = self.curr.to_vec();
        raw.append(&mut curr);
        raw
    }

    //to_hp(&self)
    //description: converts the values of self into a HP struct
    fn to_hp(&self) -> HP {
        let mut hp = HP{max: 0, curr: 0};

        unsafe {
            hp.max = super::util::u8array_to_u32(&self.max);
            hp.curr = super::util::u8array_to_i64(&self.curr);
        }

        hp
    }
}

#[derive(Debug)]
enum IDType {
    PC(u64),
    NPC(u64)
}

impl fmt::Display for IDType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IDType::PC(id) => write!(f, "PC-{}", id),
            IDType::NPC(id) => write!(f, "NPC-{}", id)
        }
    }
}

impl IDType {
    //from_string(id: String)
    //param:
    // - id: a sting in form NPC-<num> or PC-<num>
    fn from_string(id: String) -> Self {
        let id: Vec<&str> = id.split('-').collect();
        if id.len() != 2 {
            return IDType::NPC(0);
        }
        
        match id[0] {
            "NPC" => {
                let num = id[1].parse::<u64>().unwrap_or(0);
                return IDType::NPC(num);
            },
            "PC" => {
                let num = id[1].parse::<u64>().unwrap_or(0);
                return IDType::PC(num);
            },
            _ => return IDType::NPC(0)
        }
    }
}

#[derive(Debug)]
pub struct Player {
    speed: u16,
    strength: u16,
    agility: u16,
    reaction: u16,
    charisma: u16,
    awarness: u16,
    hp: HP,
    lname: String,
    fname: String,
    id: IDType
}

impl Player {
    const BASE_SVFILE: &'static str = "player";

    pub fn default() -> Self{
        return Player::new(None, None, None, None, None, None, None, None)
    }

    pub fn new(
        speed: Option<u16>,
        strength: Option<u16>,
        agility: Option<u16>,
        reaction: Option<u16>,
        charisma: Option<u16>,
        awarness: Option<u16>,
        fname: Option<String>,
        lname: Option<String>
    ) -> Self{
        let hp = HP {curr: 1, max: 1};
        Player{
            speed: speed.unwrap_or(1),
            strength: strength.unwrap_or(1),
            agility: agility.unwrap_or(1),
            reaction: reaction.unwrap_or(1),
            charisma: charisma.unwrap_or(1),
            awarness: awarness.unwrap_or(1),
            hp,
            fname: fname.unwrap_or("No".to_string()),
            lname: lname.unwrap_or("name".to_string()),
            id: IDType::NPC(0)
        }
    }

    pub fn new_pc(
        speed: Option<u16>,
        strength: Option<u16>,
        agility: Option<u16>,
        reaction: Option<u16>,
        charisma: Option<u16>,
        awarness: Option<u16>,
        fname: Option<String>,
        lname: Option<String>
    ) -> Self{
        let mut player = Player::new(speed, strength, agility, reaction, charisma, awarness, fname, lname);
        player.id = IDType::PC(0);
        player
    }

    //pub fn save(&self) -> Result<(), Error>
    //description: saves the player to a file with the name <lname>_<fname>_<ID>.player
    //returns: Ok if the save was succesful;
    pub fn save(&self) -> Result<(), Error> {
        let path = match self.save_file_name(None) {
            Ok(path) => path,
            Err(e) => return Err(e)
        };
        let mut file = match std::fs::File::create(&path) {
            Ok(f) => f,
            Err(e) => {
                let err_msg= format!("Error creating file {}, {}", path, e );
                return Err(Error::Save{err_msg})
            }
        };

        //make the byte stream and write it to the file
        let stream = PlayerByteStream::from(&self);
        let raw_stream = stream.raw();
        match file.write_all(raw_stream.as_slice()){
            Ok(_) => {},
            Err(e) => {
                let err_msg = format!("Error writing to file {}, {}", path, e);
                return Err(Error::Save{err_msg})
            }
        }

        Ok(())
    }

    //list_players()
    //description: show save player names
    //params:
    //  - path: path to the player save path 
    //returns: Vecor of DirEntrys with player names in dir given by the path param
    pub fn list_players(path: Option<String>) -> Result<Vec<std::fs::DirEntry>, Error>{
        let path = match Player::dafault_save_name(None) {
            Ok(path) => path,
            Err(_) => return Err(Error::Load { err_msg: format!("Failed to get load file path") })
        };

        let dir = match std::fs::read_dir(path){
            Ok(d) => d,
            Err(e) => return Err(Error::Load { err_msg: format!("Cant read dir: {}", e.to_string()) })
        };

        let files: Vec<std::fs::DirEntry> = dir
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
            .collect();

        Ok(files)
    }

    //fn load() -> Result<Player, Error>
    //description: reads player from default location
    //params:
    // - path: path to player file in form <lname>_<fname>_<ID>.player
    //returns: the loaded player
    pub fn load(path: Option<String>) -> Result<Player, Error> {
        let mut player = Player::default();
        let path = match path {
            Some(p) => p,
            None => return Err(Error::Load { err_msg: format!("Failed to get load file path") })
        };

        let file_name = path.split('/').last();
        let mut file = match std::fs::File::open(&path) {
            Ok(f) => f,
            Err(e) => return Err(Error::Load { err_msg: format!("failed to open file, {}", e) })
        };
        let mut stream = [0; PlayerByteStream::SIZE];
        match file.read_exact(&mut stream) {
            Ok(_) => {},
            Err(e) => {
                let err_msg = format!("Error reding file {}: {}", path, e.to_string());
                return Err(Error::Load { err_msg });
            }
        };
        player = PlayerByteStream::from_u8array(stream).to_player();
        //get name and id from file name
        match file_name {
            Some(f) => {
                //get rid of file extension
                let name = f.split('.').nth(0).unwrap_or("");

                //split lname, fname, id
                let vals: Vec<&str> = name.split('_').collect();
                if vals.len() != 3 {
                    return Err(Error::Load { err_msg: "Invalid player file name, most be in form '<lname>_<fname>_<ID>.player'".to_string() });
                }
                player.lname = vals[0].to_string();
                player.fname = vals[1].to_string();
                player.id = IDType::from_string(vals[2].to_string());
            },
            None => {}
        };

        Ok(player)
    }

    //dafault_save_name()
    //returns the default name for a save file for the player class
    fn dafault_save_name(save_path: Option<String>) -> Result<String, super::Error>{
        match save_path {
            Some(path) => {
                Ok(path)
            },
            None => {
                match super::target(){
                    Ok(mut tar) => {
                        tar.push_str(&format!("-bin/{}", Player::BASE_SVFILE));
                        Ok(tar)
                    },
                    Err(e) => return Err(e)
                }
            }
        }
    }

    //save_file_name()
    //description: returns the file to the save file
    //params:
    //  save_path: path to the directory for the save file, None for defaulr path
    //returns: the path to the save file
    pub fn save_file_name(&self, save_path: Option<String>) -> Result<String, Error> {
        let mut save_path = match Player::dafault_save_name(save_path) {
            Ok(path) => path,
            Err(e) => return Err(Error::Save { err_msg: format!("failed to get defaut path for player {}: {}", self.id, e.err_msg) })
        };

        match std::fs::create_dir_all(&save_path) {
            Ok(_) => {},
            Err(e) => {
                let err_msg = format!("Error creating directory {}, {}", save_path, e);
                return Err(Error::Save {err_msg});
            }
        }

        let file_name = format!("/{}_{}_{}.{}", self.lname, self.fname, self.id, Player::BASE_SVFILE);
        save_path = save_path + &file_name;

        Ok(save_path)
    }
}

//a data structure for saveing and loading a player object
#[derive(Debug, Clone, Copy)]
struct PlayerByteStream{
    speed: [u8; 2],
    strength: [u8; 2],
    agility: [u8; 2],
    reaction: [u8; 2],
    charisma: [u8; 2],
    awarness: [u8; 2],
    hp: HPStream
}

impl PlayerByteStream {
    // bytes in the stream
    const SIZE: usize = 12 + HPStream::SIZE;

    fn from(player: &Player) -> Self {
        let speed;
        let agility;
        let awarness;
        let charisma;
        let reaction;
        let strength;
        let hp = player.hp.stream();

        unsafe {
            speed = super::util::u16_to_u8array(player.speed);
            agility = super::util::u16_to_u8array(player.agility);
            awarness = super::util::u16_to_u8array(player.awarness);
            charisma = super::util::u16_to_u8array(player.charisma);
            reaction = super::util::u16_to_u8array(player.reaction);
            strength = super::util::u16_to_u8array(player.strength);
        }

        PlayerByteStream { 
            speed, 
            strength, 
            agility, 
            reaction, 
            charisma, 
            awarness,
            hp
        }
    }

    //from_Stream(stream: [u8]) -> Self
    //description: creates a new PlayerByteStream from a file stream
    //params:
    //  - stream: a byte stream formated in the PlayerByteStream style
    fn from_u8array(stream: [u8; PlayerByteStream::SIZE]) -> Self{
        //read stats:
        let speed = stream[..2].try_into().unwrap();
        let strength = stream[2..4].try_into().unwrap();
        let agility = stream[4..6].try_into().unwrap();
        let reaction = stream[6..8].try_into().unwrap();
        let charisma = stream[8..10].try_into().unwrap();
        let awarness = stream[10..12].try_into().unwrap();

        let hp_stream = stream[12..].try_into().unwrap();
        let hp = HPStream::from_u8array(hp_stream);

        PlayerByteStream {
            speed,
            strength,
            agility,
            reaction,
            charisma,
            awarness,
            hp
        }
    }

    //to_player(&self)
    //description: converts the values of self into a Player struct
    fn to_player(&self) -> Player{
        let mut player = Player::default();

        unsafe {
            player.speed = super::util::u8array_to_u16(&self.speed);
            player.strength = super::util::u8array_to_u16(&self.strength);
            player.agility = super::util::u8array_to_u16(&self.agility);
            player.reaction = super::util::u8array_to_u16(&self.reaction);
            player.charisma = super::util::u8array_to_u16(&self.charisma);
            player.awarness = super::util::u8array_to_u16(&self.awarness);
        }
        player.hp = self.hp.to_hp();

        player
    }

    //raw(&self)
    //description: createds a structured byte stream of all the datavaules in PlayerByteStream
    // returns: array structure - 24 bytes int the order; speed b1-b2, strength b3-b4, agility b5-b6
    //          reaction b7-b8, charisma b9-b10, awarness b11-b12, HPStream::raw() b13-24 
    fn raw(&self) -> Vec<u8> {
        let mut raw = [
            self.speed, 
            self.strength, 
            self.agility, 
            self.reaction, 
            self.charisma, 
            self.awarness
        ].concat();
        let hp = self.hp;
        raw.append(&mut hp.raw());

        raw
    }
}