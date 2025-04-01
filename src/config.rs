use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub video_folder: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            video_folder: "~/vídeos".into(),
        }
    }
}

pub fn load() -> Config {
    let config_path = shellexpand::tilde("~/.config/rsplayer.toml").into_owned();

    fs::read_to_string(&config_path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn update_video_folder(new_folder: PathBuf) {
    let config_path = shellexpand::tilde("~/.config/rsplayer.toml").into_owned();
    let mut cfg = load();
    cfg.video_folder = new_folder.to_string_lossy().to_string();

    fs::write(&config_path, toml::to_string_pretty(&cfg).unwrap()).unwrap();
}