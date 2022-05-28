#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub column: usize,
    pub line: usize,
}

impl Location {
    pub fn new(column: usize, line: usize) -> Self {
        Self { column, line }
    }

    pub fn new_from(other: &Self) -> Self {
        Self::new_from_offset(other, 0, 0)
    }

    pub fn new_from_offset(other: &Self, column_offset: usize, line_offset: usize) -> Self {
        Self::new(other.column + column_offset, other.line + line_offset)
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! loc {
    ($c: expr, $l: expr) => {
        Location::new($c, $l)
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
