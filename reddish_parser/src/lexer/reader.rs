use crate::Location;
use std::collections::{vec_deque::Iter, VecDeque};

#[derive(Debug)]
pub(crate) struct Reader {
    input: VecDeque<char>,
    location: Location,
}

impl Reader {
    pub(crate) fn new(input: &str, offset: usize) -> Self {
        Reader {
            input: VecDeque::from_iter(input.chars()),
            location: Location::new_from_offset(&Location::default(), 0, offset),
        }
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.input.is_empty()
    }

    pub(crate) fn peek(&self) -> Option<&char> {
        self.peek_nth(0)
    }

    pub(crate) fn peek_nth(&self, n: usize) -> Option<&char> {
        self.input.get(n)
    }

    pub(crate) fn next(&mut self) -> Option<char> {
        match self.input.pop_front() {
            None => None,
            Some(result) => {
                match result {
                    '\n' => {
                        self.location.column = 1;
                        self.location.line += 1;
                    }
                    _ => {
                        self.location.column += 1;
                    }
                }
                Some(result)
            }
        }
    }

    pub(crate) fn next_if(&mut self, f: impl Fn(&char) -> bool) -> Option<char> {
        match self.peek() {
            Some(c) if f(c) => self.next(),
            _ => None,
        }
    }

    pub(crate) fn skip(&mut self, index: usize) {
        self.location.column += index;
        self.input = self.input.drain(index..).collect::<_>();
    }

    pub(crate) fn iter(&self) -> Iter<char> {
        self.input.iter()
    }

    pub(crate) fn starts_with(&self, s: &str) -> bool {
        String::from_iter(self.input.iter()).starts_with(s)
    }

    pub(crate) fn location(&self) -> Location {
        self.location
    }
}
