#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub column: usize,
    pub line: usize,
}

impl Location {
    pub fn new(column: usize, line: usize) -> Self {
        Self { column, line }
    }

    pub fn from(other: &Self) -> Self {
        Self::from_offset(other, 0, 0)
    }

    pub fn from_offset(other: &Self, column_offset: usize, line_offset: usize) -> Self {
        Self::new(other.column + column_offset, other.line + line_offset)
    }

    pub fn next(&mut self) {
        self.column += 1;
    }

    pub fn newline(&mut self) {
        self.column = 1;
        self.line += 1;
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

#[macro_export]
macro_rules! location {
    ($c:expr, $l:expr) => {
        Location::new($c, $l)
    };
    ($c:expr) => {
        location!($c, 1)
    };
    () => {
        location!(1)
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annotate<T: Clone> {
    pub value: T,
    pub location: Location,
}

impl<T: Clone> Annotate<T> {
    pub fn new(value: T, location: Location) -> Self {
        Self { value, location }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn location() {
        let mut location = Location::new(1, 2);
        assert_eq!(location.column, 1);
        assert_eq!(location.line, 2);
        assert_eq!(Location::from(&location), Location::new(1, 2));
        assert_eq!(Location::from_offset(&location, 1, 1), Location::new(2, 3));
        location.newline();
        assert_eq!(location.line, 3);
        location.next();
        assert_eq!(location.column, 2);
        assert_eq!(Location::default(), Location::new(1, 1));
    }

    #[test]
    fn annotate() {
        let annotate: Annotate<i8> = Annotate::new(1, Location::default());
        assert_eq!(annotate.value, 1);
        assert_eq!(annotate.location, Location::default());
    }
}
