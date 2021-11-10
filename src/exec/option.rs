use nix::unistd::Pid;
use std::os::unix::io::RawFd;

#[derive(Debug, Copy, Clone)]
pub struct ExecOption {
    pgid: Option<Pid>,
    piping: bool,
    input: Option<RawFd>,
    output: Option<(RawFd, bool)>,
    leak_fd: Option<RawFd>,
    quiet: bool,
}

impl ExecOption {
    pub fn pgid(&self) -> Option<Pid> {
        self.pgid
    }

    pub fn piping(&self) -> bool {
        self.piping
    }

    pub fn input(&self) -> Option<RawFd> {
        self.input
    }

    pub fn output(&self) -> Option<(RawFd, bool)> {
        self.output
    }

    pub fn leak_fd(&self) -> Option<RawFd> {
        self.leak_fd
    }

    pub fn quiet(&self) -> bool {
        self.quiet
    }

    pub fn verbose(&self) -> bool {
        !self.quiet()
    }
}

pub struct ExecOptionBuilder {
    pgid: Option<Pid>,
    piping: bool,
    input: Option<RawFd>,
    output: Option<RawFd>,
    both_output: bool,
    leak_fd: Option<RawFd>,
    quiet: bool,
}

impl ExecOptionBuilder {
    pub fn new() -> Self {
        ExecOptionBuilder {
            pgid: None,
            piping: false,
            input: None,
            output: None,
            both_output: false,
            leak_fd: None,
            quiet: false,
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

    pub fn piping(mut self, b: bool) -> Self {
        self.piping = b;
        self
    }

    pub fn input(mut self, input: Option<RawFd>) -> Self {
        self.input = input;
        self
    }

    pub fn output(mut self, output: Option<RawFd>) -> Self {
        self.output = output;
        self
    }

    pub fn both_output(mut self) -> Self {
        self.both_output = true;
        self
    }

    pub fn leak_fd(mut self, fd: Option<RawFd>) -> Self {
        self.leak_fd = fd;
        self
    }

    pub fn quiet(mut self, b: bool) -> Self {
        self.quiet = b;
        self
    }

    pub fn build(self) -> ExecOption {
        let output = match self.output {
            None => None,
            Some(o) => Some((o, self.both_output)),
        };

        ExecOption {
            pgid: self.pgid,
            piping: self.piping,
            input: self.input,
            output,
            leak_fd: self.leak_fd,
            quiet: self.quiet,
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

impl From<ExecOption> for ExecOptionBuilder {
    fn from(option: ExecOption) -> Self {
        let (output, both_output) = match option.output {
            None => (None, false),
            Some((o, b)) => (Some(o), b),
        };

        ExecOptionBuilder {
            pgid: option.pgid,
            piping: option.piping,
            input: option.input,
            output,
            both_output,
            leak_fd: option.leak_fd,
            quiet: option.quiet,
        }
    }
}
