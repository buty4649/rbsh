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
    ['&', '|', '<', '>', '{', '}'].contains(&c)
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
        || is_termination(c)
        || is_single_quote(c)
        || is_double_quote(c)
        || is_symbol(c)
        || c == '$'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_space() {
        assert!(is_space(' '));
        assert!(is_space('\t'));
        assert!(!is_space('a'));
    }

    #[test]
    fn test_is_newline() {
        assert!(is_newline('\n'));
        assert!(!is_newline('a'));
    }

    #[test]
    fn test_is_termination() {
        assert!(is_termination(';'));
        assert!(!is_termination('a'));
    }

    #[test]
    fn test_is_symbol() {
        assert!(is_symbol('&'));
        assert!(is_symbol('|'));
        assert!(is_symbol('<'));
        assert!(is_symbol('>'));
        assert!(is_symbol('{'));
        assert!(is_symbol('}'));
        assert!(!is_symbol('a'));
    }

    #[test]
    fn test_is_number() {
        for n in '0'..='9' {
            assert!(is_number(n));
        }
        assert!(!is_number('a'));
    }

    #[test]
    fn test_is_single_quote() {
        assert!(is_single_quote('\''));
        assert!(!is_single_quote('a'));
    }

    #[test]
    fn test_is_double_quote() {
        assert!(is_double_quote('"'));
        assert!(!is_double_quote('a'));
    }

    #[test]
    fn test_is_normal_word_delimiter() {
        for c in " \t\n;&|<>{}'\"".chars() {
            assert!(is_normal_word_delimiter(c));
        }
        assert!(!is_normal_word_delimiter('a'));
    }
}
