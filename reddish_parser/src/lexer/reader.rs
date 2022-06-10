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

#[cfg(test)]
mod test {
    use super::*;
    use crate::location;

    #[test]
    fn is_eof() {
        assert!(Reader::new("", 0).is_eof());
        assert!(!Reader::new("a", 0).is_eof());
    }

    #[test]
    fn peek() {
        assert_eq!(Reader::new("abc", 0).peek(), Some(&'a'));
        assert_eq!(Reader::new("", 0).peek(), None);
    }

    #[test]
    fn peek_nth() {
        let reader = Reader::new("abc", 0);
        assert_eq!(reader.peek_nth(1), Some(&'b'));
        assert_eq!(reader.peek_nth(2), Some(&'c'));
        assert_eq!(reader.peek_nth(3), None);
        assert_eq!(Reader::new("", 0).peek_nth(4), None);
    }

    #[test]
    fn next() {
        let mut reader = Reader::new("abc", 0);
        assert_eq!(reader.next(), Some('a'));
        assert_eq!(reader.location, location!(2, 1));
        assert_eq!(reader.next(), Some('b'));
        assert_eq!(reader.location, location!(3, 1));
        assert_eq!(reader.next(), Some('c'));
        assert_eq!(reader.location, location!(4, 1));
        assert_eq!(reader.next(), None);
        assert_eq!(reader.location, location!(4, 1));
    }

    #[test]
    fn next_if() {
        let mut reader = Reader::new("abc", 0);
        assert_eq!(reader.next_if(|c| c == &'a'), Some('a'));
        assert_eq!(reader.location, location!(2, 1));
        assert_eq!(reader.next_if(|c| c == &'c'), None);
        assert_eq!(reader.location, location!(2, 1));
    }

    #[test]
    fn skip() {
        let mut reader = Reader::new("abc", 0);
        reader.skip(2);
        assert_eq!(reader.peek(), Some(&'c'));
        assert_eq!(reader.location, location!(3, 1));
    }

    #[test]
    fn iter() {
        assert!(Reader::new("abc", 0).iter().eq([&'a', &'b', &'c']));
    }

    #[test]
    fn starts_with() {
        let reader = Reader::new("abc", 0);
        assert!(reader.starts_with("ab"));
        assert!(!reader.starts_with("bc"));
    }

    #[test]
    fn location() {
        assert_eq!(Reader::new("abc", 0).location, Location::default());
    }
}
