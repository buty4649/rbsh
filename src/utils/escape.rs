pub trait Escape {
    fn escape(&self) -> String;
}

impl Escape for &str {
    fn escape(&self) -> String {
        let mut result = String::new();
        let mut iter = self.split('\\');

        result.push_str(iter.next().unwrap());
        for s in iter {
            let mut c = s.chars().collect::<Vec<_>>();
            match c[0] {
                'a' => c[0] = '\x07',
                'b' => c[0] = '\x08',
                'c' => {
                    if let Some(cc) = contorl_char(c[1]) {
                        c.remove(0);
                        c[0] = cc;
                    }
                }
                'C' if c[1] == '-' => {
                    if let Some(cc) = contorl_char(c[2]) {
                        c.remove(0);
                        c.remove(0);
                        c[2] = cc;
                    }
                }
                'e' | 'E' => c[0] = '\x1b',
                'f' => c[0] = '\x0c',
                'n' => c[0] = '\n',
                'r' => c[0] = '\r',
                't' => c[0] = '\t',
                'v' => c[0] = '\x0b',

                // \nnn: octal
                n if n.is_digit(8) => {
                    let mut code = 0;
                    for _ in 1..=3 {
                        if c.is_empty() || !c[0].is_digit(8) {
                            break;
                        }
                        code *= 8;
                        code += c.remove(0) as u32;
                    }
                    c.insert(0, char::from_u32(code).unwrap());
                }

                // \xHH: Hex
                // \uHHHH: Unicode(16bit)
                // \UHHHHHHHH: Unicode(32bit)
                'x' | 'u' | 'U' if c[1].is_ascii_hexdigit() => {
                    let max = match c.remove(0) {
                        'x' => 2,
                        'u' => 4,
                        'U' => 4,
                        _ => unreachable![],
                    };
                    let mut code = 0;
                    for _ in 1..=max {
                        if c.is_empty() || !c[0].is_ascii_hexdigit() {
                            break;
                        }
                        code *= 16;
                        code += c.remove(0).to_digit(16).unwrap();
                    }
                    c.insert(0, char::from_u32(code).unwrap());
                }

                // ruby compatible
                's' => c[0] = ' ',
                _ => (),
            }
            result.push_str(&*c.into_iter().collect::<String>())
        }

        result
    }
}

fn contorl_char(c: char) -> Option<char> {
    match c {
        '?' => Some('\x7f'),

        // 0x20-0x3e
        ' '..='>' => char::from_u32(c as u32 - 0x20),

        // 0x40-0x5f
        '@'..='_' => char::from_u32(c as u32 - 0x40),

        // 0x60-0x7e
        '`'..='~' => char::from_u32(c as u32 - 0x60),

        _ => None,
    }
}
