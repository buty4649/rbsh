use crate::APP_NAME;
use std::fs::create_dir_all;
use xdg::BaseDirectories;

#[derive(Debug, Clone)]
pub struct Config {
    dirs: BaseDirectories,
}

impl Config {
    pub fn new() -> Self {
        Config {
            dirs: BaseDirectories::with_prefix(APP_NAME).unwrap(),
        }
    }

    pub fn history_file(&self) -> String {
        let path = self.dirs.place_data_file("history.txt").unwrap();
        create_dir_all(path.parent().unwrap()).unwrap();
        path.to_str().unwrap().to_string()
    }
}
