#![allow(unused_macros)]
extern crate rbsh_parser;

macro_rules! assert_parse {
    ($input:expr => $expect:expr) => {
        assert_eq!(
            $expect,
            parse($input, true),
            "\n{}\n{}\n{}",
            "<-- input ---------------------",
            $input,
            ">------------------------------",
        )
    };
}

macro_rules! assert_parse_error {
    ($input:expr) => {
        assert!(parse($input, true).is_err())
    };
}

macro_rules! command {
    (name: $name:expr) => {
        Node::Command {
            name: $name,
            args: None,
            redirect: None,
            parameter: None,
        }
    };
    (name: $name:expr, args: $args:expr) => {
        Node::Command {
            name: $name,
            args: Some($args),
            redirect: None,
            parameter: None,
        }
    };
    (name: $name:expr, args: $args:expr, redirect: $redirect:expr) => {
        Node::Command {
            name: $name,
            args: Some($args),
            redirect: Some($redirect),
            parameter: None,
        }
    };
    (name: $name:expr, redirect: $redirect:expr) => {
        Node::Command {
            name: $name,
            args: None,
            redirect: Some($redirect),
            parameter: None,
        }
    };
    (name: $name:expr, parameter: $parameter:expr) => {
        Node::Command {
            name: $name,
            args: None,
            redirect: None,
            parameter: Some($parameter),
        }
    };
}

macro_rules! variable_assignment {
    ($e1:expr $(, $e2:expr)* $(,)*) => {
        Node::VariableAssignment {
            body: vec![$e1 $(, $e2)* ],
        }
    };
}

macro_rules! parameter {
    ($name:tt) => {
        Parameter {
            name: stringify!($name).to_string(),
            value: None,
        }
    };

    ($name:tt, $value:expr) => {
        Parameter {
            name: stringify!($name).to_string(),
            value: Some($value),
        }
    };
}

macro_rules! bare {
    ($inner:tt) => {{
        let inner = stringify!($inner)
            .trim_start_matches("r#")
            .trim_start_matches("\"");
        let inner = inner.trim_end_matches("#").trim_end_matches("\"");
        WordKind::bare(inner)
    }};
    (@ $inner:expr) => {
        WordKind::bare($inner)
    };
}

macro_rules! quote {
    ($e1:expr $(, $e2:expr)*) => {
        WordKind::Quote(vec![$e1, $($e2),* ])
    };
}

macro_rules! command_sub {
    () => { WordKind::CommandSubstitute(vec![]) };

    ($e1:expr $(, $e2:expr)*) => {
        WordKind::CommandSubstitute(vec![$e1, $($e2),* ])
    };
}

macro_rules! param_sub {
    ($inner:tt) => {
        WordKind::parameter(stringify!($inner))
    };
    (@ $inner:expr) => {
        WordKind::parameter($inner)
    };
}

macro_rules! redirect_read_from {
    ($fd:expr, $word:expr) => {
        RedirectKind::ReadFrom($fd, $word)
    };
}

macro_rules! redirect_write_to {
    ($fd:expr, $word:expr, $force:expr) => {
        RedirectKind::WriteTo($fd, $word, $force)
    };
}

macro_rules! redirect_write_both {
    ($word:expr) => {
        RedirectKind::WriteBoth($word)
    };
}

macro_rules! redirect_read_copy {
    ($fd1:expr, $fd2:expr, $close:expr) => {
        RedirectKind::ReadCopy($fd1, $fd2, $close)
    };
}

macro_rules! redirect_write_copy {
    ($fd1:expr, $fd2:expr, $close:expr) => {
        RedirectKind::WriteCopy($fd1, $fd2, $close)
    };
}

macro_rules! redirect_read_close {
    ($fd:expr) => {
        RedirectKind::ReadClose($fd)
    };
}

macro_rules! redirect_write_close {
    ($fd:expr) => {
        RedirectKind::WriteClose($fd)
    };
}

macro_rules! redirect_append_to {
    ($fd:expr, $word:expr) => {
        RedirectKind::AppendTo($fd, $word)
    };
}

macro_rules! redirect_append_both {
    ($word:expr) => {
        RedirectKind::AppendBoth($word)
    };
}

macro_rules! redirect_read_write {
    ($fd:expr, $word:expr) => {
        RedirectKind::ReadWrite($fd, $word)
    };
}

macro_rules! redirect_here_string {
    ($fd:expr, $word:expr) => {
        RedirectKind::HereString($fd, $word)
    };
}
