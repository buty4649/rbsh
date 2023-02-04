extern crate rbsh_parser;

mod test_string {
    use std::vec;

    use rbsh_parser::string::{ExpandAsciiCode, ExpandEscapeCharacter};

    fn fixture() -> Vec<(&'static str, &'static str)> {
        vec![
            ("\\a", "\x07"),
            ("\\b", "\x08"),
            ("\\e", "\x1b"),
            ("\\E", "\x1b"),
            ("\\f", "\x0c"),
            ("\\n", "\n"),
            ("\\r", "\r"),
            ("\\t", "\t"),
            ("\\v", "\x0b"),
            ("\\\\", "\\"),
            ("\\'", "'"),
            ("\\\"", "\""),
            ("\\0", "\x00"),
            ("\\10", "\x08"),
            ("\\141", "\x61"),
            ("\\x0", "\x00"),
            ("\\x61", "\x61"),
            ("\\u0", "\x00"),
            ("\\u61", "\x61"),
            ("\\u430", "\u{430}"),
            ("\\u3042", "\u{3042}"),
            ("\\U0", "\x00"),
            ("\\U61", "\x61"),
            ("\\U430", "\u{430}"),
            ("\\U3042", "\u{3042}"),
            ("\\U1F363", "\u{1F363}"),
            ("\\U10FFFF", "\u{10FFFF}"),
            ("\\c!", "\x01"),
            ("\\c?", "\x7f"),
            ("\\cA", "\x01"),
            ("\\c_", "\x1f"),
            ("\\ca", "\x01"),
            ("\\c~", "\x1e"),
        ]
    }

    #[test]
    fn expand_ascii_code() {
        fixture().into_iter().for_each(|(input, expect)| {
            assert_eq!(expect, input.expand_ascii_code(), "input: {input}")
        });
        assert_eq!("abc", "abc".expand_ascii_code(), "input: abc");
        assert_eq!("ab\\c", "ab\\c".expand_ascii_code(), "input: ab\\c");
    }

    #[test]
    fn expand_escape_character() {
        fixture().into_iter().for_each(|(input, expect)| {
            assert_eq!(expect, input.expand_escape_character(), "input: {input}")
        });

        vec![
            ("\\s", "\x20"),
            ("\\C-!", "\x01"),
            ("\\C-?", "\x7f"),
            ("\\C-A", "\x01"),
            ("\\C-_", "\x1f"),
            ("\\C-a", "\x01"),
            ("\\C-~", "\x1e"),
            ("\\M-a", "\u{e1}"),
            ("\\M-\\C-a", "\u{81}"),
            ("\\M-\\C-?", "\u{ff}"),
            ("\\u{30eb 30d3 30fc 1F363 a}", "\u{30eb}\u{30d3}\u{30fc}\u{1F363}\n"),
            ("\\@", "@"),
            ("abc\\\ndef", "abcdef"),
        ]
        .into_iter()
        .for_each(|(input, expect)| {
            assert_eq!(expect, input.expand_escape_character(), "input: {input}")
        });
    }
}
