use crate::APP_NAME;
use std::{fs::create_dir_all, path::PathBuf};
use xdg::BaseDirectories;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    history_file: String,
}

impl Config {
    pub fn new() -> Self {
        let dirs = BaseDirectories::with_prefix(APP_NAME).unwrap();

        macro_rules! path2string {
            ($e: expr) => {
                dirs.place_data_file($e)
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string()
            };
        }

        let history_file = path2string!("history.txt");
        Config { history_file }
    }

    pub fn history_file(&self) -> String {
        let mut path = PathBuf::new();
        path.push(&*self.history_file);
        create_dir_all(path.parent().unwrap()).unwrap();
        self.history_file.to_string()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
