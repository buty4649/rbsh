#[cfg(test)]
mod test {
    use super::*;
    use crate::Location;

    macro_rules! word {
        ($e: expr) => {
            Word::new($e.to_string(), WordKind::Normal, Location::new(1, 1))
        };
    }

    macro_rules! wordlist {
        ($($e: expr$(,)?)+) => {{
            let mut wl = WordList::new();
            $(wl.push($e);)+
            wl
        }};
    }

    macro_rules! hashmap {
        ($(($k: expr, $v: expr)$(,)?)+) => {{
            let mut h = HashMap::new();
            $(h.insert($k.to_string(), $v.to_string());)+
            h
        }};
    }

    #[test]
    fn test_split_env_and_commands() {
        let ctx = Context::new();
        assert_eq!(
            (HashMap::new(), vec!["foo".to_string()]),
            split_env_and_commands(vec![wordlist![word!("foo")]], &ctx)
        );

        assert_eq!(
            (
                hashmap![("foo", "bar"), ("baz", "foo")],
                vec!["bar".to_string()]
            ),
            split_env_and_commands(
                vec![
                    wordlist![word!("foo=bar")],
                    wordlist![word!("baz=foo")],
                    wordlist![word!("bar")]
                ],
                &ctx
            )
        );

        assert_eq!(
            (
                hashmap![("foo", "bar")],
                vec!["baz".to_string(), "hoge=fuga".to_string()]
            ),
            split_env_and_commands(
                vec![
                    wordlist![word!("foo=bar")],
                    wordlist![word!("baz")],
                    wordlist![word!("hoge=fuga")]
                ],
                &ctx
            )
        );
    }
}
