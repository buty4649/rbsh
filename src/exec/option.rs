use nix::unistd::Pid;
use std::os::unix::io::RawFd;

#[derive(Debug, Copy, Clone)]
pub struct ExecOption {
    pgid: Option<Pid>,
    background: bool,
    input: Option<RawFd>,
    output: Option<(RawFd, bool)>,
}

impl ExecOption {
    pub fn pgid(&self) -> Option<Pid> {
        self.pgid
    }

    pub fn background(&self) -> bool {
        self.background
    }

    pub fn input(&self) -> Option<RawFd> {
        self.input
    }

    pub fn output(&self) -> Option<(RawFd, bool)> {
        self.output
    }
}

pub struct ExecOptionBuilder {
    pgid: Option<Pid>,
    background: bool,
    input: Option<RawFd>,
    output: Option<RawFd>,
    both_output: bool,
}

impl ExecOptionBuilder {
    pub fn new() -> Self {
        ExecOptionBuilder {
            pgid: None,
            background: false,
            input: None,
            output: None,
            both_output: false,
        }
    }

    pub fn pgid(mut self, pgid: Pid) -> Self {
        self.pgid = Some(pgid);
        self
    }

    pub fn default_pgid(mut self, pgid: Pid) -> Self {
        self.pgid = Some(self.pgid.unwrap_or(pgid));
        self
    }

    pub fn foreground(mut self) -> Self {
        self.background = false;
        self
    }

    pub fn background(mut self) -> Self {
        self.background = true;
        self
    }

    pub fn input(mut self, input: RawFd) -> Self {
        self.input = Some(input);
        self
    }

    pub fn output(mut self, output: RawFd) -> Self {
        self.output = Some(output);
        self
    }

    pub fn both_output(mut self) -> Self {
        self.both_output = true;
        self
    }

    pub fn build(self) -> ExecOption {
        let output = match self.output {
            None => None,
            Some(o) => Some((o, self.both_output)),
        };

        ExecOption {
            pgid: self.pgid,
            background: self.background,
            input: self.input,
            output,
        }
    }

    pub fn if_then<F>(self, condition: bool, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if condition {
            f(self)
        } else {
            self
        }
    }
}

impl From<Option<ExecOption>> for ExecOptionBuilder {
    fn from(option: Option<ExecOption>) -> Self {
        match option {
            None => Self::new(),
            Some(option) => {
                let (output, both_output) = match option.output {
                    None => (None, false),
                    Some((o, b)) => (Some(o), b),
                };

                ExecOptionBuilder {
                    pgid: option.pgid,
                    background: option.background,
                    input: option.input,
                    output,
                    both_output,
                }
            }
        }
    }
}
