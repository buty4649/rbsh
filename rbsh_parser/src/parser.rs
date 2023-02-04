extern crate peg;

use crate::ast::*;
use crate::string::ExpandAsciiCode;
use std::os::unix::io::RawFd;

#[derive(Debug, Clone)]
pub(crate) enum Token {
    Word(Vec<WordKind>),
    Redirect(RedirectKind),
    Parameter(Parameter),
}

mod rubyish {
    #[inline]
    pub(crate) fn to(b: bool) -> usize {
        match b {
            true => 1,
            false => 0,
        }
    }
}

peg::parser! {
    pub(crate) grammar rbsh(rubyish: bool) for str {
        use super::rubyish::to;

        pub(crate) rule statement() -> Vec<Node>
            = __* list:command_list()*
              { list }

        rule command_list() -> Node
          = command:list() terminator:(list_terminator())? __*
            {
              if matches!(terminator, Some('&')) {
                Node::Background { body: Box::new(command) }
              } else {
                command
              }
            }

        rule list_terminator() -> char
          = !(";;&" / ";;" / ";&") t:[';' | '&' | '\n'] { t }

        rule list() -> Node
          = command:pipeline_command()
            next:(
              connector:$("&&" / "||") __* command:list()
              { (connector, command)}
            )?
            {
              if let Some((connector, next)) = next {
                let left = Box::new(command);
                let right = Box::new(next);
                match connector {
                  "&&" => Node::And{ left, right },
                  "||" => Node::Or{ left, right },
                  _ => unreachable!(),
                }
              } else {
                command
              }
            }

        rule pipeline_command() -> Node
          = invert:("!" _* ) pipeline:pipeline()?
            { Node::InvertReturn { body: pipeline.map(Box::new) }}
            / pipeline()

        rule pipeline() -> Node
          = command:simple_command() _*
            next:(
              pipe:$("|&" / "|" ) __* command:pipeline()
              { (matches!(pipe, "|&"), command) }
            )?
            {
              if let Some((both, next)) = next {
                let left = Box::new(command);
                let right = Box::new(next);
                Node::Pipe{ left, right, both }
              } else {
                command
              }
            }

        rule pipeline_terminator() -> char
            = !(";;&" / ";;" / ";&") c:[';' | '&' | '\n'] { c }


        rule simple_command() -> Node
            = compound_command()
              / function_command()
              / variable_assignment_block()
              / command()

        rule compound_command() -> Node
            = if_command()
              / rubyish_unless_command()
              / while_command()
              / until_command()
              / for_command()
              / select_command()
              / case_command()
              / group_command()
              / subshell_command()

        rule group_command() -> Node
            = "{" __* body:command_list()+ __* "}" _* redirect:(redirect()+)?
              { Node::Group{ body, redirect }}

        rule subshell_command() -> Node
            = "(" __* body:command_list()+ __* ")" _* redirect:(redirect()+)?
              { Node::Subshell{ body, redirect }}

        // ----------------------------------------------------------
        // if command
        // ----------------------------------------------------------
        rule if_command() -> Node
            = rubyish_if_command() /
              "if" __ test:command_list()
              "then" __ body:statement()
              elif_body:(elif_command()+)?
              else_body:("else" __ body:statement() { body })?
              "fi" _* redirect:(redirect()+)?
              {
                  let body = Condition { test:Box::new(test), body};
                  Node::If { body, elif_body, else_body, redirect}
              }

        rule elif_command() -> Condition
            = "elif" __ test:command_list()
              "then" __ body:statement()
              { Condition { test: Box::new(test), body } }

        rule rubyish_if_command() -> Node
            = block:(
                rubyish_if_short_command() /
                "if" __ test:command_list()
                "then" __ body:statement()
                elif_body:(rubyish_elsif_command()+)?
                else_body:("else" __ body:statement() { body })?
                "end" _* redirect:(redirect()+)?
                {
                    let body = Condition { test:Box::new(test), body};
                    Node::If { body, elif_body, else_body, redirect}
                }
              )*<{to(rubyish)}>
              {?
                let mut block = block;
                match block.pop() {
                    None => Err("rubysh is disabled"),
                    Some(b) => Ok(b),
                }
              }

        rule rubyish_if_short_command() -> Node
            = "if" __ test_and_body:statement()
               elif_body:(rubyish_elsif_command()+)?
               else_body:("else" __ body:statement() { body })?
              "end" _* redirect:(redirect()+)?
              {
                let (test, body) = test_and_body.split_first().unwrap();
                let body = Condition { test: Box::new(test.clone()), body: body.to_vec() };
                Node::If{ body, elif_body, else_body, redirect }
              }

        rule rubyish_elsif_command() -> Condition
            = "elsif" __ test:command_list()
              "then" __ body:statement()
              { Condition { test: Box::new(test), body } }/
              rubyish_elsif_short_command()

        rule rubyish_elsif_short_command() -> Condition
            = "elsif" __ test_and_body:statement()
              {
                let (test, body) = test_and_body.split_first().unwrap();
                Condition { test: Box::new(test.clone()), body: body.to_vec() }
              }

        // ----------------------------------------------------------
        // unless
        // ----------------------------------------------------------
        rule rubyish_unless_command() -> Node
            = block:(
                rubyish_unless_short_command() /
                "unless" __ test:command_list()
                "then" __ body:statement()
                else_body:("else" __ body:statement() { body })?
                "end" _* redirect:(redirect()+)?
                {
                    let body = Condition { test:Box::new(test), body};
                    Node::Unless { body, else_body, redirect }
                }
              )*<{to(rubyish)}>
              {?
                let mut block = block;
                match block.pop() {
                    None => Err("rubysh is disabled"),
                    Some(b) => Ok(b),
                }
              }

        rule rubyish_unless_short_command() -> Node
            = "unless" __ test_and_body:statement()
              else_body:("else" __ body:statement() { body })?
              "end" _* redirect:(redirect()+)?
              {
                let (test, body) = test_and_body.split_first().unwrap();
                let body = Condition { test: Box::new(test.clone()), body: body.to_vec() };
                Node::Unless { body, else_body, redirect }
              }

        // ----------------------------------------------------------
        // while
        // ----------------------------------------------------------
        rule while_command() -> Node
            = rubyish_while_command() /
              "while" __ test:command_list()
              "do" __ body:statement()
              "done" _* redirect:(redirect()+)?
              {
                let body = Condition { test:Box::new(test), body};
                Node::While { body, redirect }
              }

        rule rubyish_while_command() -> Node
            = block:(rubyish_while_short_block() /
                "while" __ test:command_list()
                "do" __ body:statement()
                "end" _* redirect:(redirect()+)?
                {
                    let body = Condition { test:Box::new(test), body};
                    Node::While { body, redirect }
                }
              )*<{to(rubyish)}>
              {?
                let mut block = block;
                match block.pop() {
                    None => Err("rubysh is disabled"),
                    Some(b) => Ok(b),
                }
              }

        rule rubyish_while_short_block() -> Node
            = "while" __ test_and_body:statement()
              "end" _* redirect:(redirect()+)?
              {
                let (test, body) = test_and_body.split_first().unwrap();
                let body = Condition { test: Box::new(test.clone()), body: body.to_vec() };
                Node::While { body, redirect }
              }

        // ----------------------------------------------------------
        // until
        // ----------------------------------------------------------
        rule until_command() -> Node
            = rubyish_until_command() /
              "until" __ test:command_list()
              "do" __ body:statement()
              "done" _* redirect:(redirect()+)?
              {
                let body = Condition { test:Box::new(test), body};
                Node::Until { body, redirect }
              }

        rule rubyish_until_command() -> Node
            = block:(rubyish_until_short_block() /
                "until" __ test:command_list()
                "do" __ body:statement()
                "end" _* redirect:(redirect()+)?
                {
                    let body = Condition { test:Box::new(test), body};
                    Node::Until { body, redirect }
                }
              )*<{to(rubyish)}>
              {?
                let mut block = block;
                match block.pop() {
                    None => Err("rubysh is disabled"),
                    Some(b) => Ok(b),
                }
              }

        rule rubyish_until_short_block() -> Node
            = "until" __ test_and_body:statement()
              "end" _* redirect:(redirect()+)?
              {
                let (test, body) = test_and_body.split_first().unwrap();
                let body = Condition { test: Box::new(test.clone()), body: body.to_vec() };
                Node::Until { body, redirect }
              }

        // ----------------------------------------------------------
        // for
        // ----------------------------------------------------------
        rule for_command() -> Node
            = rubyish_for_command() /
              "for" _ ident:identifier()
              subject:(__ "in" subject:(_ word:word() { word })* { subject })?
              ("\n" / ";") __*
              "do" __ body:statement()
              "done" _* redirect:(redirect()+)?
              {
                Node::For{ ident, subject, body, redirect }
              }

        rule rubyish_for_command() -> Node
            = block:(
                "for" _ ident:identifier()
                subject:(__ "in" subject:(_ word:word() { word })*  { subject })
                ("\n" / ";") __*
                ("do" __)? body:statement()
                "end" _* redirect:(redirect()+)?
                {
                    Node::For{ ident, subject: Some(subject), body, redirect }
                }
              )*<{to(rubyish)}>
              {?
                let mut block = block;
                match block.pop() {
                    None => Err("rubysh is disabled"),
                    Some(b) => Ok(b),
                }
              }

        // ----------------------------------------------------------
        // select
        // ----------------------------------------------------------
        rule select_command() -> Node
            = rubyish_select_command() /
              "select" _ ident:identifier()
              subject:(__* "in" subject:(_ word:word() { word })* { subject })?
              ("\n" / ";") __*
              "do" __ body:statement()
              "done" _* redirect:(redirect()+)?
              {
                Node::Select{ ident, subject, body, redirect }
              }

        rule rubyish_select_command() -> Node
            = block:(
                "select" _ ident:identifier()
                subject:(__ "in" subject:(_ word:word() { word })* { subject })
                ("\n" / ";") __*
                ("do" __)? body:statement()
                "end" _* redirect:(redirect()+)?
                {
                    Node::Select{ ident, subject:Some(subject), body, redirect }
                }
              )*<{to(rubyish)}>
              {?
                let mut block = block;
                match block.pop() {
                    None => Err("rubysh is disabled"),
                    Some(b) => Ok(b),
                }
              }

        // ----------------------------------------------------------
        // case
        // ----------------------------------------------------------
        rule case_command() -> Node
            =  rubyish_case_command() /
              "case" _ word:word() __ "in" __
              pattern:(case_pattern_command()+)?
              "esac" _* redirect:(redirect()+)?
              { Node::Case { word, pattern, redirect } }

        rule case_pattern_command() -> CasePattern
            = "("? _* pattern:(word()**(_* "|" _*)) _* ")" __*
                body:statement() __* next_action:$(";;&" / ";;" / ";&"/ &"esac") __*
              {
                let next_action = match next_action {
                  ";;&" => CasePatternNextAction::TestNext,
                  ";&" => CasePatternNextAction::FallThrough,
                  ";;" => CasePatternNextAction::End,
                  "" => CasePatternNextAction::End,  // esac
                  _ => unreachable!(),
                };
                CasePattern { pattern, body, next_action }
              }

        rule rubyish_case_command() -> Node
            = block:(
                "case" _ word:word() __
                pattern:(rubyish_case_pattern_command()+)
                "end" _* redirect:(redirect()+)?
                { Node::Case { word, pattern:Some(pattern), redirect } }
              )*<{to(rubyish)}>
              {?
                let mut block = block;
                match block.pop() {
                    None => Err("rubysh is disabled"),
                    Some(b) => Ok(b),
                }
              }

        rule rubyish_case_pattern_command() -> CasePattern
            = "when" _ pattern:(word()**(_* "|" _*)) (__ "then" / "\n") __* body:statement() __*
              { CasePattern {pattern, body, next_action: CasePatternNextAction::End }}

        // ----------------------------------------------------------
        // function
        // ----------------------------------------------------------
        rule function_command() -> Node
            = ident:function_identifier() "()" __* body:compound_command()
              { Node::Function { ident, body: Box::new(body) }}
              / "function" _ ident:function_identifier() ("()" / "\n")? __* body:compound_command()
                { Node::Function { ident, body: Box::new(body) }}

        rule function_identifier() -> String
            = !keyword() i:$((!(spaceNL() / "(" / ")" ) ANY())+)
              { i.to_string() }

        // ----------------------------------------------------------
        // variable assignment
        // ----------------------------------------------------------
        rule variable_assignment_block() -> Node
            = body:parameter()++(space()) &(_* (['\n' | '|' | '&' | ';'] / EOF()))
              { Node::VariableAssignment{ body } }

        rule parameter() -> Parameter
            = name:identifier() "=" value:(word())?
              { Parameter{ name, value } }

        // ----------------------------------------------------------
        // command
        // ----------------------------------------------------------
        rule command() -> Node
            = !['{' | '}' | '!']
              pre:(
                token:(parameter_token() / redirect_token()) _
                { token }
              )*
              !keyword() name:word()
              post:(
                _ token:(redirect_token() / word_token())
                { token }
              )*
              {
                let tokens = [pre, post].concat();

                macro_rules! extract {
                  ($p:path, $ty:ty) => {{
                    let r = tokens.iter()
                                  .filter(|t| matches!(t, $p(..)))
                                  .map(|t| match t { $p(inner) => inner, _ => unreachable!()})
                                  .cloned()
                                  .collect::<$ty>();
                    if r.is_empty() { None } else { Some(r) }
                  }}
                }

                let args = extract!(Token::Word, Vec<Vec<WordKind>>);
                let redirect = extract!(Token::Redirect, Vec<RedirectKind>);
                let parameter = extract!(Token::Parameter, Vec<Parameter>);

                Node::Command{name, args, redirect, parameter}
              }

        rule parameter_token() -> Token = parameter:parameter() { Token::Parameter(parameter) }
        rule redirect_token() -> Token = redirect:redirect() { Token::Redirect(redirect) }
        rule word_token() -> Token = word:word() { Token::Word(word) }

        // ----------------------------------------------------------
        // word
        // ----------------------------------------------------------
        rule word() -> Vec<WordKind>
            = word:(
                bareword()
                / single_quote()
                / double_quote()
                / backquote()
                / command_substitute()
                / ansi_expand()
                / locale_expand()
                / parameter_expand()
            )+

        rule bareword() -> WordKind
            = chars:(
                escapeNL() /
                escape(<$(ANY())>) /
                !(
                    spaceNL() / ['\'' | '"' | '`' | '<' | '>' | '#' | '(' | ')' ]
                    / "$(" / "${" / "$"
                    / ";;" / ";&" / ";;&" / ";"
                    / "||" / "|&" / "|"
                    / "&&" / "&"
                )
                c:$(ANY()) { c }
              )+ { WordKind::bare(String::from_iter(chars)) }

        rule single_quote() -> WordKind
            = rubyish_single_quote() /
              "'" inner:$(([^'\''])*) "'"
              { WordKind::Quote(vec![ WordKind::bare(inner) ]) }

        rule rubyish_single_quote() -> WordKind
            = word:(
                "'" inner:(escape(<['\'']>) / [^'\''])* "'"
                { WordKind::Quote(vec![ WordKind::bare(String::from_iter(inner)) ]) }
              )*<{to(rubyish)}>
              {?
                let mut word = word;
                match word.pop() {
                    None => Err("rubysh is disabled"),
                    Some(b) => Ok(b),
                }
              }

        rule double_quote() -> WordKind
            = "\""
                inner:(
                  backquote() /
                  command_substitute() /
                  parameter_expand() /
                  c:(
                    escapeNL() /
                    escape(<$(['"' | '\\' | '`' | '$'])>) /
                    $([^'"' | '$' | '`'])
                  )+ { WordKind::bare(String::from_iter(c)) }
                )*
              "\""
              { WordKind::Quote(inner) }

        rule backquote() -> WordKind
          = "`" inner:(escape(<$(['`'])>) / $([^'`']))* "`"
            {?
              if let Ok(statement) = statement(&String::from_iter(inner), rubyish) {
                  Ok(WordKind::CommandSubstitute(statement))
              } else {
                  Err("unexpected EOF")
              }
            }

        rule command_substitute() -> WordKind
          = "$(" inner:statement() ")" { WordKind::CommandSubstitute(inner) }

        rule ansi_expand() -> WordKind
            = "$'" inner:$([^'\'']*) "'" { WordKind::Quote(vec![ WordKind::bare(inner.expand_ascii_code()) ]) }

        rule locale_expand() -> WordKind
            = "$\"" inner:$((escape(<['"']>) / [^'"'])*) "\"" {? Err("not implemented ")}

        rule parameter_expand() -> WordKind
          = "${" name:(escape(<['}']>) / ['a'..='z' | 'A'..='Z' | '_'])+ "}"
            { WordKind::Parameter(String::from_iter(name))}
          / "$" escapeNL()* name:(
              n:$(['*' | '@' | '#' | '?' | '-' | '$' | '!' | '_']) { n.to_string() } /
              (
                $(['0']) { String::from("0") } /
                head:$(['1'..='9']) tail:(escapeNL()* t:$(['0'..='9']) { t })*
                { format!("{}{}", head, String::from_iter(tail)) }
              ) /
              n:(escapeNL()* n:$(['a'..='z' | 'A'..='Z' | '_']) { n })+
              { String::from_iter(n) }
            )
            { WordKind::parameter(name) }
          / "$" &([' ' | '`' | ';' | '\n' | '&' | '|'] / EOF()) { WordKind::bare("$") }

        // ----------------------------------------------------------
        // redirect
        // ----------------------------------------------------------
        rule redirect() -> RedirectKind
            = redirect:(
                here_string()
                / fd:fd(Some(0)) "<<" _* word()+ {? Err("Here Document is not implemented.") }
                / redirect_read_copy()
                / redirect_read_close()
                / redirect_write_copy()
                / redirect_write_close()
                / redirect_append_both()
                / redirect_write_both()
                / redirect_read_write()
                / redirect_read_from()
                / redirect_append_to()
                / redirect_write_to()
            )

        rule fd(default: Option<RawFd>) -> RawFd
            = fd:$(['0'..='9']+)?
              {?
                match fd {
                  Some(fd) => Ok(fd.parse::<RawFd>().unwrap()),
                  None => if let Some(default) = default {
                    Ok(fd.map_or(default, |f| f.parse::<RawFd>().unwrap() ))
                  } else {
                    Err("Number")
                  }
                }
              }

        rule redirect_read_from() -> RedirectKind
            = fd:fd(Some(0)) "<" _* word:word()
              { RedirectKind::ReadFrom(fd, word) }

        rule redirect_write_to() -> RedirectKind
            = fd:fd(Some(1)) ">" force:"|"? _* word:word()
              { RedirectKind::WriteTo(fd, word, force.is_some()) }

        rule redirect_write_both() -> RedirectKind
            = /* Error in bash */ ">&" _ "-" {? Err("Bad file descriptor")} /
              ("&>" / ">&") _* !['-'] word:word()
              { RedirectKind::WriteBoth(word) }

        rule redirect_read_copy() -> RedirectKind
            = fd1:fd(Some(0)) "<&" _* fd2:fd(None) close:"-"?
              { RedirectKind::ReadCopy(fd1, fd2, close.is_some()) }

        rule redirect_write_copy() -> RedirectKind
            = fd1:fd(Some(1)) ">&" _* fd2:fd(None) close:"-"?
              { RedirectKind::WriteCopy(fd1, fd2, close.is_some()) }

        rule redirect_read_close() -> RedirectKind
            = fd:fd(Some(0))"<&-" { RedirectKind::ReadClose(fd) }

        rule redirect_write_close() -> RedirectKind
            = fd:fd(Some(1)) ">&-" { RedirectKind::WriteClose(fd) }

        rule redirect_append_to() -> RedirectKind
            = fd:fd(Some(1)) ">>" _* word:word() { RedirectKind::AppendTo(fd, word) }

        rule redirect_append_both() -> RedirectKind
            = "&>>" _* word:word() { RedirectKind::AppendBoth(word) }

        rule redirect_read_write() -> RedirectKind
            = fd:fd(Some(0)) "<>" _* word:word()
              { RedirectKind::ReadWrite(fd, word) }

        rule here_string() -> RedirectKind
            = fd:fd(Some(0)) "<<<" _* word:word()
              { RedirectKind::HereString(fd, word) }

        // ----------------------------------------------------------
        // misc
        // ----------------------------------------------------------
        rule space() =  [' ' | '\t'] / escapeNL()
        rule comment() = "#" [^'\n']* ("\n" / EOF())
        rule spaceNL() = space() / "\n" /comment()
        rule _ = space()+
        rule __ = spaceNL()+

        rule escape<T>(c: rule<T>) -> T = "\\" c:c() { c }
        rule escapeNL() -> &'input str = escape(<['\n']>) { "" }

        rule keyword() = "if" / "elif" / "else" / "fi" / "then"
                         / "while" / "do" / "done" / "until"
                         / "for" / "case" / "esac"
                         / ("unless" / "elsif" / "end")*<{to(rubyish)}>

        rule identifier() -> String
            = s:$(['a'..='z' | 'A'..='Z' | '_'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
              { s.to_string() }

        rule ANY() -> char = [_]
        rule EOF() = ![_]
    }
}
