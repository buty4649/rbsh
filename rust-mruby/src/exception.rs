use super::{api::*, MRuby, MRubyArray, MRubyValue};
use std::{fmt, ptr::NonNull};

#[derive(Debug)]
pub struct Exception {
    #[allow(dead_code)]
    kind: ExceptionKind,
    message: String,
    backtrace: Option<Vec<String>>,
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.backtrace {
            Some(backtrace) => {
                if backtrace.len() >= 2 {
                    writeln!(f, "trace (most recent call last):")?;

                    let mut bt = backtrace.iter().enumerate().collect::<Vec<_>>();
                    bt.remove(0);
                    bt.reverse();
                    for (index, b) in bt {
                        writeln!(f, "\t[{}] {}", index, b)?;
                    }
                }
                writeln!(f, "{}: {}", backtrace[0], self.message)
            }
            None => writeln!(f, "{}", self.message),
        }
    }
}

#[derive(Debug)]
pub enum ExceptionKind {
    SyntaxError,
    Unknwon(String),
}

impl<T: AsRef<str>> From<T> for ExceptionKind {
    fn from(s: T) -> ExceptionKind {
        match s.as_ref() {
            "SyntaxError" => ExceptionKind::SyntaxError,
            s => ExceptionKind::Unknwon(s.to_string()),
        }
    }
}

pub trait MRubyException {
    fn exception(&self) -> Option<Exception>;
}

impl MRubyException for MRuby {
    fn exception(&self) -> Option<Exception> {
        let m: &mrb_state = unsafe { NonNull::new(self.mrb).unwrap().as_mut() };
        match m.exc as usize == 0 {
            true => None,
            false => {
                let exc = MRubyValue::from_ptr(self.mrb, m.exc);
                let class = exc.call("class", None).to_s();
                let kind = ExceptionKind::from(class);
                let message = exc.call("inspect", None).to_s();

                let array: MRubyArray = exc.call("backtrace", None).into();
                let backtrace = match array.is_empty() {
                    true => None,
                    false => {
                        let mut backtrace = vec![];
                        for a in array {
                            backtrace.push(a.to_s());
                        }
                        Some(backtrace)
                    }
                };

                Some(Exception {
                    kind,
                    message,
                    backtrace,
                })
            }
        }
    }
}
