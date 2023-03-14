use std::{
    collections::HashMap,
    env::var,
    fs::{self, create_dir_all, read_to_string, File},
    io::{self, ErrorKind},
    net::IpAddr,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::PRODUCT_NAME;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub ip: IpAddr,
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    // The path to this file
    #[serde(skip, default = "State::default_path")]
    _path: PathBuf,

    /// Device indexed by mac address
    pub devices: HashMap<String, Device>,
    pub default_data_directory: PathBuf,
}

impl State {
    pub fn default_path() -> PathBuf {
        let config_home = var("XDG_CONFIG_HOME")
            .or_else(|_| var("HOME").map(|home| format!("{home}/.config")))
            .or_else(|_| var("USER").map(|user| format!("/home/{user}/.config")))
            .expect("Failed to get config directory");
        return PathBuf::from(config_home).join(format!("{PRODUCT_NAME}/state.json"));
    }

    pub fn default_data_directory() -> PathBuf {
        let data_home = var("XDG_DATA_HOME")
            .or_else(|_| var("HOME").map(|home| format!("{home}/.local/share")))
            .or_else(|_| var("USER").map(|user| format!("/home/{user}/.local/share")))
            .expect("Failed to get config directory");
        return PathBuf::from(data_home).join(PRODUCT_NAME);
    }

    pub fn load() -> State {
        let path = State::default_path();

        if let Err(e) = File::open(&path) {
            if e.kind() == ErrorKind::NotFound {
                create_dir_all(&path.parent().unwrap()).expect("Failed to create config directory");
                let state = State::default();
                state.save().unwrap();
                return state;
            } else {
                panic!("Failed to load state: {}", e.to_string())
            }
        }
        let state: State =
            serde_json::from_str(&read_to_string(path).unwrap()).expect("Invalid State");
        return state;
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let state = State::default();
        fs::write(&self._path, serde_json::to_string(&state).unwrap())
            .expect("Failed to save state");
        return Ok(());
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            _path: State::default_path(),
            default_data_directory: State::default_data_directory(),
            devices: Default::default(),
        }
    }
}
