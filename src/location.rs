
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    column: usize,
    line: usize,
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
    value: T,
    loc: Location,
}

impl<T: Clone> Annotate<T> {
    pub fn new(value: T, loc: Location) -> Self {
        Self { value, loc }
    }

    pub fn take(&self) -> (T, Location) {
        (self.value.clone(), self.loc)
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn location(&self) -> Location {
        self.loc
    }
}