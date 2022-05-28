use super::Unit;
#[cfg(not(feature = "debug"))]
pub(crate) fn print(_: &[Unit]) {}

#[cfg(feature = "debug")]
pub(crate) fn print(units: &[Unit]) {
    eprintln!("debug(parser): results:");
    pp::pp(units, 0);
}

#[cfg(feature = "debug")]
mod pp {
    use crate::{ConnecterKind, RedirectList, Unit, UnitKind, Word, WordList};

    macro_rules! debug {
        ($indent:expr, $($args:tt)* ) => {{
            let i = " ".repeat($indent);
            eprint!("debug(parser): {}", i);
            eprintln!($($args)*);
        }};
    }

    pub(crate) fn pp(units: &[Unit], indent: usize) {
        for unit in units.iter() {
            let indent = indent + 1;
            pp_unit(unit, indent);
        }
    }

    fn pp_unit(unit: &Unit, indent: usize) {
        let background = unit.background;
        match &unit.kind {
            UnitKind::SimpleCommand { command, redirect } => {
                print_simple_command(indent, command, redirect, background)
            }
            UnitKind::Connecter { left, right, kind } => {
                print_connecter(indent, left, right, kind, background)
            }
            UnitKind::Pipe { left, right, both } => {
                print_pipe(indent, left, right, both, background)
            }
            UnitKind::If {
                condition,
                true_case,
                false_case,
                redirect,
            } => print_if(
                indent, condition, true_case, false_case, redirect, false, background,
            ),
            UnitKind::Unless {
                condition,
                false_case,
                true_case,
                redirect,
            } => print_if(
                indent, condition, false_case, true_case, redirect, true, background,
            ),
            UnitKind::While {
                condition,
                command,
                redirect,
            } => print_while(indent, condition, command, redirect, false, background),
            UnitKind::Until {
                condition,
                command,
                redirect,
            } => print_while(indent, condition, command, redirect, true, background),
            UnitKind::For {
                identifier,
                list,
                command,
                redirect,
            } => print_for(indent, identifier, list, command, redirect, background),
        }
    }

    fn print_simple_command(
        indent: usize,
        command: &[WordList],
        redirect: &RedirectList,
        backgroud: bool,
    ) {
        debug!(indent, "SimpleCommand:");
        debug!(indent + 1, "command: {:?}", command);
        debug!(indent + 1, "redirect: {:?}", redirect);
        debug!(indent + 1, "background: {}", backgroud);
    }

    fn print_connecter(
        indent: usize,
        left: &Unit,
        right: &Unit,
        kind: &ConnecterKind,
        background: bool,
    ) {
        match kind {
            ConnecterKind::And => debug!(indent, "And:"),
            ConnecterKind::Or => debug!(indent, "Or:"),
        }

        debug!(indent + 1, "left:");
        pp_unit(left, indent + 2);

        debug!(indent + 1, "right:");
        pp_unit(right, indent + 2);

        debug!(indent + 1, "background: {}", background);
    }

    fn print_pipe(indent: usize, left: &Unit, right: &Unit, both: &bool, background: bool) {
        debug!(indent, "Pipe:");

        debug!(indent + 1, "left:");
        pp_unit(left, indent + 2);

        debug!(indent + 1, "right:");
        pp_unit(right, indent + 2);

        debug!(indent + 1, "both: {}", both);
        debug!(indent + 1, "background: {}", background);
    }

    fn print_if(
        indent: usize,
        condition: &Unit,
        true_case: &[Unit],
        false_case: &Option<Vec<Unit>>,
        redirect: &RedirectList,
        reverse: bool,
        background: bool,
    ) {
        if reverse {
            debug!(indent, "Unless:")
        } else {
            debug!(indent, "If:")
        }

        debug!(indent + 1, "condition:");
        pp_unit(condition, indent + 2);

        debug!(indent + 1, "true_case:");
        pp(true_case, indent + 2);

        if let Some(units) = false_case {
            debug!(indent + 1, "false_case:");
            pp(units, indent + 2);
        } else {
            debug!(indent + 1, "false_case: null");
        }

        debug!(indent + 1, "redirect: {:?}", redirect);
        debug!(indent + 1, "background: {}", background);
    }

    fn print_while(
        indent: usize,
        condition: &Unit,
        command: &[Unit],
        redirect: &RedirectList,
        reverse: bool,
        background: bool,
    ) {
        if reverse {
            debug!(indent, "Until:");
        } else {
            debug!(indent, "While:");
        }

        debug!(indent + 1, "condition:");
        pp_unit(condition, indent + 2);

        debug!(indent + 1, "command:");
        pp(command, indent + 2);

        debug!(indent + 1, "redirect: {:?}", redirect);
        debug!(indent + 1, "background: {}", background);
    }

    fn print_for(
        indent: usize,
        identifier: &Word,
        list: &Option<Vec<WordList>>,
        command: &[Unit],
        redirect: &RedirectList,
        background: bool,
    ) {
        debug!(indent, "For:");

        debug!(indent + 1, "identifier: {:?}", identifier);

        if let Some(list) = list {
            debug!(indent + 1, "list: {:?}", list);
        } else {
            debug!(indent + 1, "list: null");
        }

        debug!(indent + 1, "command:");
        pp(command, indent + 2);

        debug!(indent + 1, "redirect: {:?}", redirect);
        debug!(indent + 1, "background: {}", background);
    }
}
