use crate::location::Location;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WordKind {
    Normal,    // word or "word"
    Quote,     // 'word'
    Command,   // `word` or $(word)
    Variable,  // $word
    Parameter, // ${word}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    pub string: String,
    pub kind: WordKind,
    pub location: Location,
}

impl Word {
    pub fn new(string: String, kind: WordKind, location: Location) -> Self {
        Word {
            string,
            kind,
            location,
        }
    }

    #[cfg(test)]
    pub(crate) fn normal(str: &str, location: Location) -> Self {
        Self::new(str.to_string(), WordKind::Normal, location)
    }

    //#[cfg(test)]
    //pub(crate) fn quote(str: &str, location: Location) -> Self {
    //    Self::new(str.to_string(), WordKind::Quote, location)
    //}

    //#[cfg(test)]
    //pub(crate) fn command(str: &str, location: Location) -> Self {
    //    Self::new(str.to_string(), WordKind::Command, location)
    //}

    //#[cfg(test)]
    //pub(crate) fn variable(str: &str, location: Location) -> Self {
    //    Self::new(str.to_string(), WordKind::Variable, location)
    //}

    //#[cfg(test)]
    //pub(crate) fn parameter(str: &str, location: Location) -> Self {
    //    Self::new(str.to_string(), WordKind::Parameter, location)
    //}
}
