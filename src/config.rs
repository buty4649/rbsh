use crate::APP_NAME;
use dirs::data_dir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    history_file_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut history_file_path = get_data_dir();
        history_file_path.push("history.txt");
        Config { history_file_path }
    }

    pub fn history_file_path(&self) -> &Path {
        create_dir_all(self.history_file_path.parent().unwrap()).unwrap();
        &self.history_file_path
    }
}

fn get_data_dir() -> PathBuf {
    let mut dir = data_dir().unwrap();
    dir.push(APP_NAME);
    dir
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
