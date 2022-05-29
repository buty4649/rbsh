pub(crate) fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

pub(crate) fn is_newline(c: char) -> bool {
    c == '\n'
}

pub(crate) fn is_termination(c: char) -> bool {
    c == ';'
}

pub(crate) fn is_symbol(c: char) -> bool {
    ['&', '|', '<', '>', ';', '{', '}'].contains(&c)
}

pub(crate) fn is_number(c: char) -> bool {
    c.is_ascii_digit()
}

pub(crate) fn is_single_quote(c: char) -> bool {
    c == '\''
}

pub(crate) fn is_double_quote(c: char) -> bool {
    c == '"'
}

pub(crate) fn is_normal_word_delimiter(c: char) -> bool {
    is_space(c)
        || is_newline(c)
        || is_single_quote(c)
        || is_double_quote(c)
        || is_symbol(c)
        || c == '$'
}
