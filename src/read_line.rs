use super::exec::syscall::{SysCallWrapper, Wrapper};
use rustyline::{config::Configurer, error::ReadlineError, Editor};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error as IoError},
    os::unix::io::{AsRawFd, FromRawFd},
    path::Path,
};

pub trait ReadLine {
    fn readline(&mut self, prompt: &str) -> Result<String, ReadLineError>;
    fn keep_linenumer(&self) -> bool;

    fn load_history(&mut self, _: &Path) -> Result<(), ReadLineError> {
        Ok(())
    }

    fn save_history(&mut self, _: &Path) -> Result<(), ReadLineError> {
        Ok(())
    }

    fn add_history_entry(&mut self, _: &str) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum ReadLineError {
    Io(IoError),
    Eof,
    Interrupted,
    Utf8Error,
}

pub struct ReadFromFile {
    reader: BufReader<File>,
}

impl ReadFromFile {
    pub fn new(wrapper: &Wrapper, path: Option<&Path>) -> Result<Self, IoError> {
        let fd = match path {
            Some(path) => {
                let file = File::open(path)?;
                wrapper
                    .dup_fd(file.as_raw_fd(), 255)
                    .map_err(|e| IoError::from_raw_os_error(e.errno() as i32))?
            }
            None => 0,
        };
        let file = unsafe { File::from_raw_fd(fd) };
        Ok(Self {
            reader: BufReader::new(file),
        })
    }
}

impl ReadLine for ReadFromFile {
    fn readline(&mut self, _prompt: &str) -> Result<String, ReadLineError> {
        let mut result = String::new();

        match self.reader.read_line(&mut result) {
            Ok(size) if size == 0 => Err(ReadLineError::Eof),
            Ok(_) => Ok(result.trim_end_matches('\n').to_string()),
            Err(e) => Err(ReadLineError::Io(e)),
        }
    }

    fn keep_linenumer(&self) -> bool {
        true
    }
}

pub struct ReadFromTTY {
    editor: Editor<()>,
}

impl ReadFromTTY {
    pub fn new() -> Self {
        let mut editor = Editor::new();

        editor.set_check_cursor_position(true);
        Self { editor }
    }
}

impl ReadLine for ReadFromTTY {
    fn readline(&mut self, prompt: &str) -> Result<String, ReadLineError> {
        match self.editor.readline(prompt) {
            Ok(line) => Ok(line),
            Err(e) => Err(ReadLineError::from(e)),
        }
    }

    fn keep_linenumer(&self) -> bool {
        false
    }

    fn load_history(&mut self, path: &Path) -> Result<(), ReadLineError> {
        self.editor.load_history(path)?;
        Ok(())
    }

    fn save_history(&mut self, path: &Path) -> Result<(), ReadLineError> {
        self.editor.save_history(path)?;
        Ok(())
    }

    fn add_history_entry(&mut self, line: &str) -> bool {
        self.editor.add_history_entry(line)
    }
}

impl From<ReadlineError> for ReadLineError {
    fn from(e: ReadlineError) -> Self {
        match e {
            ReadlineError::Io(e) => Self::Io(e),
            ReadlineError::Eof => Self::Eof,
            ReadlineError::Interrupted => Self::Interrupted,
            ReadlineError::Utf8Error => Self::Utf8Error,
            ReadlineError::Errno(e) => Self::Io(IoError::from_raw_os_error(e as i32)),
            _ => unreachable![], // for windows error
        }
    }
}
