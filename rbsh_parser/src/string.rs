use peg;

trait ParseDigit {
    fn parse_digit(&self, radix: u32) -> Option<char>;
}

impl ParseDigit for &str {
    fn parse_digit(&self, radix: u32) -> Option<char> {
        self.chars()
            .try_fold(0, |sum, c| c.to_digit(radix).map(|c| sum * radix + c))
            .and_then(char::from_u32)
    }
}

peg::parser! {
    grammar escape_character() for str {
        rule traced<T>(e: rule<T>) -> T =
            &(input:$(any()*) {
                #[cfg(feature = "trace")]
                println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
            })
            e:e()? {?
                #[cfg(feature = "trace")]
                println!("[PEG_TRACE_STOP]");
                e.ok_or("")
            }

        pub(crate) rule expand_ascii_code() -> String
        = traced(<
            chars:(ascii_code() / any())*
            { String::from_iter(chars) }
          >)

        pub(crate) rule expand() -> String
        = traced(<
            chars:(ascii_code() / unicode_chars() / backslash() / any())*
            { String::from_iter(chars) }
          >)

        rule any() -> String = c:$([_]) { c.to_string() }
        rule hex() -> char = ['0'..='9' | 'a'..='f' | 'A'..='F']

        rule ascii_code() -> String
        = ['\\'] code:(
            "a" { '\x07' } / "b" { '\x08' } / ['e' | 'E'] { '\x1b' } /
            "f" { '\x0c' } / "n" { '\n' } / "r" { '\r' } / "t" { '\t' } /
            "v" { '\x0b' } / ['\\' | '\'' | '"'] /

            // \nnn
            n:$(['0'..='7']*<1, 3>) { n.parse_digit(8).unwrap() } /

            // \xHH / \uHHHH / \UHHHHHHHH
            h:(
                ("x" h:$(hex()*<1,2>) { h }) /
                ("u" h:$(hex()*<1,4>) { h }) /
                ("U" h:$(hex()*<1,8>) { h })
            ) { h.parse_digit(16).unwrap() } /

            // \cX
            ("c" c:control_character() { c })
        ) { code.to_string() }

        rule control_character() -> char
        = ['?'] { '\x7f' } /
          // 0x20-0x3e
          c:[' '..='>'] { unsafe { char::from_u32_unchecked(c as u32 - 0x20) } } /
          // 0x40-0x5f
          c:['@'..='_'] { unsafe { char::from_u32_unchecked(c as u32 - 0x40) } } /
          // 0x60-0x7e
          c:['`'..='~'] { unsafe { char::from_u32_unchecked(c as u32 - 0x60) } }

        rule backslash() -> String
        = ['\\'] s:(
            ['\n'] { "".to_string() } /
            code:(
                "s" { '\x20' } /

                // \C-x
                ("C-" c:control_character() { c }) /

                // \M-x / \M-\C-x
                ("M-" c:(
                        "\\C-" c:control_character() { unsafe{ char::from_u32_unchecked((c as u32) | 0x80) } } /
                        c:['\x00'..='\x7f'] { unsafe { char::from_u32_unchecked((c as u32) | 0x80) } }
                    ) { c }
                )
            ) { code.to_string() } /
            s:$([_]) { s.to_string() }
          ) { s }

          // \u{nnnn}
          rule unicode_chars() -> String
          = "\\u{" u:($(hex()*<1,6>)**" ") "}" {
            String::from_iter(u.iter().map(|v| v.parse_digit(16).unwrap() ))
          }
    }
}

pub trait ExpandAsciiCode {
    fn expand_ascii_code(&self) -> String;
}

impl ExpandAsciiCode for &str {
    fn expand_ascii_code(&self) -> String {
        escape_character::expand_ascii_code(self).unwrap()
    }
}

pub trait ExpandEscapeCharacter {
    fn expand_escape_character(&self) -> String;
}

impl ExpandEscapeCharacter for &str {
    fn expand_escape_character(&self) -> String {
        escape_character::expand(self).unwrap()
    }
}
