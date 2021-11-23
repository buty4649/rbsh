use std::collections::HashMap;

pub trait IsPresent {
    fn is_present(&self) -> bool;
}

impl<T> IsPresent for Vec<T> {
    fn is_present(&self) -> bool {
        !self.is_empty()
    }
}

impl<T, U> IsPresent for HashMap<T, U> {
    fn is_present(&self) -> bool {
        !self.is_empty()
    }
}
