use chrono::Local;
use convert_case::{Case, Casing};
use indoc::indoc;
use rustemort::index::StateVec;
use std::{
    fmt::Debug,
    fs::{self, File},
    io::{self, Write},
    iter::repeat,
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::{
    grammar::{res_symbol, Grammar, ResolvingSymbolIndex},
    rustemo_actions::Recognizer,
    settings::Settings,
    table::{lr_states_for_grammar, LRState},
};

use super::rustemo::RustemoParser;

#[cfg(test)]
mod tests;

macro_rules! geni {
    ($w:expr, $($args:tt)*) => {
        ($w).write_indented(&::std::fmt::format(format_args!($($args)*)))?
    }
}

macro_rules! gen {
    ($w:expr, $($args:tt)*) => {
        ($w).write(&::std::fmt::format(format_args!($($args)*)))?
    }
}

#[derive(Default)]
struct RustWrite<W: Write> {
    write: W,
    indent: usize,
}

const DEFAULT_INDENT: usize = 4;

impl<W: Write> RustWrite<W> {
    pub fn new(w: W) -> RustWrite<W> {
        RustWrite {
            write: w,
            indent: 0,
        }
    }

    pub fn inc_indent(&mut self) {
        self.indent += DEFAULT_INDENT;
    }

    pub fn dec_indent(&mut self) {
        self.indent -= DEFAULT_INDENT;
    }

    fn write_indented(&mut self, out: &str) -> io::Result<()> {
        let mut lines = out.lines().peekable();
        while let Some(line) = lines.next() {
            write!(self.write, "{0:1$}", "", self.indent)?;
            if lines.peek().is_some() {
                writeln!(self.write, "{}", line)?;
            } else if out.ends_with("\n") {
                writeln!(self.write, "{}", line)?;
            } else {
                write!(self.write, "{}", line)?;
            }
        }
        Ok(())
    }

    pub fn write(&mut self, out: &str) -> io::Result<()> {
        write!(self.write, "{}", out)
    }
}

pub(crate) fn generate_parser<F>(grammar_path: F) -> io::Result<()>
where
    F: AsRef<Path> + Debug,
{
    let grammar = RustemoParser::default().parse(
        fs::read_to_string(grammar_path.as_ref())
            .unwrap_or_else(|error| {
                panic!(
                    "Cannot load grammar file {:?}. Error: {:?}",
                    grammar_path, error
                );
            })
            .as_str()
            .into(),
    );

    let states = lr_states_for_grammar(&grammar, &Settings::default());

    let out_file = grammar_path.as_ref().with_extension("rs");
    let mut out_file = File::create(out_file).unwrap();

    generate_parser_tables(&grammar, states, out_file)
}

fn generate_parser_tables<W: Write>(
    grammar: &Grammar,
    states: StateVec<LRState>,
    out: W,
) -> io::Result<()> {
    let mut out = RustWrite::new(out);

    geni!(out, "/// Generated by rustemo on {}", Local::now());

    geni!(
        out,
        indoc! {r#"
        use regex::Regex;
        use std::convert::TryFrom;

        use std::marker::PhantomData;
        use rustemort::lexer::{{Lexer, DefaultLexer, Token, LexerDefinition, RecognizerIterator}};
        use rustemort::lr::{{LRParser, LRContext, ParserDefinition}};
        use rustemort::lr::Action::{{self, Shift, Reduce, Accept, Error}};
        use rustemort::index::{{StateIndex, TermIndex, NonTermIndex, ProdIndex}};
        use rustemort::builder::Builder;
        use rustemort::grammar::{{TerminalInfo, TerminalInfos, TerminalsState}};
        use rustemort::debug::{{log, logn}};
        use super::rustemo_types::{{TermKind, ProdKind, Terminal, NonTerminal, Symbol}};

        use super::rustemo_actions::*;

        const TERMINAL_NO: usize = {term_count};
        const NONTERMINAL_NO: usize = {nonterm_count};
        const STATE_NO: usize = {states_count};
        const MAX_ACTIONS: usize = {max_actions};

        pub struct RustemoParserDefinition {{
            actions: [[Action; TERMINAL_NO]; STATE_NO],
            gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO]
        }}

        pub(in crate) static PARSER_DEFINITION: RustemoParserDefinition = RustemoParserDefinition {{
    "#},
        term_count = grammar.term_len(),
        nonterm_count = grammar.nonterm_len(),
        states_count = states.len(),
        max_actions = states
            .iter()
            .map(|x| x.actions.iter().filter(|x| !x.is_empty()).count())
            .max()
            .unwrap(),
    );

    out.inc_indent();
    geni!(out, "actions: [\n");
    for state in &states {
        geni!(
            out,
            "// State {}:{}\n",
            state.idx,
            grammar.symbol_name(state.symbol)
        );
        geni!(out, "[");
        gen!(
            out,
            "{}",
            state
                .actions
                .iter()
                .map(|action| match action.len() {
                    0 => "Error".into(),
                    1 => format!("{}", action[0]),
                    _ => panic!("Multiple actions for state {}", state.idx),
                })
                .collect::<Vec<_>>()
                .join(", ")
        );
        gen!(out, "],\n");
    }
    out.dec_indent();
    geni!(out, "],\n");

    out.inc_indent();
    geni!(out, "gotos: [\n");
    for state in &states {
        geni!(
            out,
            "// State {}:{}\n",
            state.idx,
            grammar.symbol_name(state.symbol)
        );
        geni!(out, "[");
        gen!(
            out,
            "{}",
            state
                .gotos
                .iter()
                .map(|x| match x {
                    Some(state) => format!("Some(StateIndex({}))", state),
                    None => "None".to_string(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        );
        gen!(out, "],\n");
    }
    out.dec_indent();
    geni!(out, "]}};\n\n");

    geni!(
        out,
        indoc! {r#"
        impl ParserDefinition for RustemoParserDefinition {{
            fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {{
                PARSER_DEFINITION.actions[state_index.0][term_index.0]
            }}
            fn goto(&self, state_index: StateIndex, nonterm_id: NonTermIndex) -> StateIndex {{
                PARSER_DEFINITION.gotos[state_index.0][nonterm_id.0].unwrap()
            }}
        }}

        pub struct RustemoParser<'i>(pub LRParser<&'i str, RustemoParserDefinition>);

        impl<'i> Default for RustemoParser<'i> {{
            fn default() -> Self {{
                Self(LRParser {{
                    context: LRContext {{
                        parse_stack: vec![StateIndex(0)],
                        current_state: StateIndex(0),
                        position: 0,
                        token: None,
                    }},
                    definition: &PARSER_DEFINITION,
                }})
            }}
        }}

        pub struct RustemoLexerDefinition {{
            terminals: TerminalInfos<TERMINAL_NO>,
            terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
            recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO]
        }}

        pub(in crate) static LEXER_DEFINITION: RustemoLexerDefinition = RustemoLexerDefinition {{
    "#}
    );

    out.inc_indent();
    geni!(out, "terminals: [\n");
    for terminal in grammar.terminals() {
        geni!(out, "TerminalInfo {{\n");
        out.inc_indent();
        geni!(out, "id: TermIndex({}),\n", terminal.idx);
        geni!(out, "name: \"{}\",\n", terminal.name);
        geni!(out, "location: None,\n");
        out.dec_indent();
        geni!(out, "}},\n");
    }
    out.dec_indent();
    geni!(out, "],\n");

    out.inc_indent();
    geni!(
        out,
        indoc! {"
             // Expected terminals/tokens indexed by state id.
             // Sorted by priority.\n"
        }
    );

    geni!(out, "terminals_for_state: [\n");
    for state in &states {
        geni!(
            out,
            "// State {}:{}\n",
            state.idx,
            grammar.symbol_name(state.symbol)
        );
        geni!(out, "[");
        gen!(
            out,
            "{}",
            &state
                .sorted_terminals
                .iter()
                .map(|x| format!("Some({})", x))
                .chain(
                    // Fill the rest with "None"
                    repeat("None".to_string()).take(
                        &grammar.term_len() - &state.sorted_terminals.len()
                    )
                )
                .collect::<Vec<_>>()
                .join(", ")
        );
        gen!(out, "],\n");
    }
    out.dec_indent();
    geni!(out, "],\n");

    geni!(out, "recognizers: [\n");
    out.inc_indent();
    for terminal in grammar.terminals() {
        if let Some(recognizer) = &terminal.recognizer {
            geni!(out, "// {}:{}\n", terminal.idx, terminal.name);
            match recognizer {
                Recognizer::StrConst(str_match) => {
                    geni!(
                        out,
                        indoc! {
                           r#"
                            |input: &str| {{
                                logn!("Recognizing <{term_name}> -- ");
                                if input.starts_with("{str_match}"){{
                                    log!("recognized");
                                    Some("{str_match}")
                                }} else {{
                                    log!("not recognized");
                                    None
                                }}
                            }},
                            "#
                        },
                        term_name = terminal.name,
                        str_match = str_match
                    )
                }
                Recognizer::RegExTerm(regex_match) => {
                    geni!(
                        out,
                        indoc! {
                           r###"
                            |input: &str| {{
                                logn!("Recognizing <{term_name}> -- ");
                                let regex = Regex::new(r#"{regex_match}"#).unwrap();
                                let match_str = regex.find(input);
                                match match_str {{
                                    Some(x) => {{
                                        let x_str = x.as_str();
                                        log!("recognized <{{}}>", x_str);
                                        Some(x_str)
                                    }},
                                    None => {{
                                        log!("not recognized");
                                        None
                                    }}
                                }}
                            }}
                            "###
                        },
                        term_name = terminal.name,
                        regex_match = regex_match
                    )
                }
            }
        }
    }
    geni!(out, "],\n");
    out.dec_indent();
    geni!(out, "}};\n");

    geni!(
        out,
        indoc! {r#"
            pub struct RustemoLexer<'i>(DefaultLexer<'i, RustemoLexerDefinition>);

            impl<'i> Lexer for RustemoLexer<'i> {{
                type Input = &'i str;

                fn next_token(
                    &self,
                    context: &mut impl rustemort::parser::Context<Self::Input>,
                ) -> Option<rustemort::lexer::Token<Self::Input>> {{
                    self.0.next_token(context)
                }}
            }}

            // Enables creating a lexer from a reference to an object that can be converted
            // to a string reference.
            impl<'i, T> From<&'i T> for RustemoLexer<'i>
            where
                T: AsRef<str> + ?Sized,
            {{
                fn from(input: &'i T) -> Self {{
                    Self(DefaultLexer::new(input.as_ref(), &LEXER_DEFINITION))
                }}
            }}

            impl LexerDefinition for RustemoLexerDefinition {{
                type Recognizer = for<'i> fn(&'i str) -> Option<&'i str>;

                fn recognizers(&self, state_index: StateIndex) -> RecognizerIterator<Self::Recognizer> {{
                        RecognizerIterator {{
                            terminals: &LEXER_DEFINITION.terminals,
                            terminals_for_state: &LEXER_DEFINITION.terminals_for_state[state_index.0][..],
                            recognizers: &LEXER_DEFINITION.recognizers,
                            index: 0
                        }}
                }}
            }}

            pub struct RustemoBuilder<'i, I: 'i> {{
                res_stack: Vec<Symbol>,
                phantom: PhantomData<&'i I>
            }}

            impl<'i, I> Builder for RustemoBuilder<'i, I>
            {{
                type Output = Symbol;
                type Lexer = RustemoLexer<'i>;

                fn new() -> Self {{
                    RustemoBuilder {{
                        res_stack: vec![],
                        phantom: PhantomData,
                    }}
                }}

                fn shift_action(&mut self, term_idx: TermIndex, token: Token<<Self::Lexer as Lexer>::Input>) {{
                    let termval = match TermKind::try_from(term_idx.0).unwrap() {{
        "#
        }
    );

    out.inc_indent();
    out.inc_indent();
    out.inc_indent();
    for terminal in grammar.terminals() {
        if let Some(Recognizer::RegExTerm(_)) = terminal.recognizer {
            geni!(out,
                    "TermKind::{term_name} => Terminal::{term_name}({action_fun}(token)),\n",
                    term_name=terminal.name,
                    action_fun=terminal.name.to_case(Case::Snake)
            )
        } else {
            geni!(
                out,
                "TermKind::{term_name} => Terminal::{term_name},\n",
                term_name = terminal.name
            );
        }
    }
    out.dec_indent();
    geni!(out, "}};\n");
    geni!(out, "self.res_stack.push(Symbol::Terminal(termval));\n");
    out.dec_indent();
    geni!(out, "}}\n");

    geni!(
        out,
        indoc! {r#"

            fn reduce_action(&mut self, prod_kind: ProdIndex, prod_len: usize, _prod_str: &'static str) {{
                let prod = match ProdKind::try_from(prod_kind.0).unwrap() {{
        "#
        }
    );

    out.inc_indent();
    out.inc_indent();
    for production in &grammar.productions()[1..] {
        let prod_nt_name = &grammar.nonterminals()[production.nonterminal].name;
        geni!(
            out,
            "ProdKind::{}P{} => {{\n",
            prod_nt_name,
            production.ntidx
        );
        let rhs_len = production.rhs.len();
        out.inc_indent();
        geni!(out,
              "let mut i = self.res_stack.split_off(self.res_stack.len()-{rhs_len}).into_iter();\n",
              rhs_len=rhs_len
        );
        geni!(
            out,
            "match {}{}{} {{",
            if rhs_len > 1 { "(" } else { "" },
            repeat("i.next().unwrap()")
                .take(rhs_len)
                .collect::<Vec<_>>()
                .join(", "),
            if rhs_len > 1 { ")" } else { "" },
        );
        geni!(out, "\n");
        out.inc_indent();
        let mut counter = 0;
        let mut lhs = production
            .rhs
            .iter()
            .map(|assign| {
                let symbol = res_symbol(assign);
                if grammar.is_term(symbol) {
                    let t =
                        &grammar.terminals()[grammar.symbol_to_term(symbol)];
                    if t.has_content {
                        counter += 1;
                        format!(
                            "Symbol::Terminal(Terminal::{}(p{}))",
                            t.name,
                            counter - 1
                        )
                    } else {
                        "_".to_string()
                    }
                } else {
                    counter += 1;
                    let nt = &grammar.nonterminals()
                        [grammar.symbol_to_nonterm(symbol)];
                    format!(
                        "Symbol::NonTerminal(NonTerminal::{}(p{}))",
                        nt.name,
                        counter - 1
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        if rhs_len > 1 {
            lhs = format!("({})", lhs);
        }

        geni!(
            out,
            "{} => NonTerminal::{}({}_p{}({}))",
            lhs,
            prod_nt_name,
            prod_nt_name.to_case(Case::Snake),
            production.ntidx,
            (0..counter).map(|x| format!("p{}", x)).collect::<Vec<_>>().join(", ")
        );

        out.dec_indent();
        geni!(out, "}}\n");
        out.dec_indent();
        geni!(out, "}},\n");
    }

    out.dec_indent();
    geni!(out, "}};\n");

    out.dec_indent();
    geni!(out, "}}\n");
    out.dec_indent();
    geni!(out, "}}\n");

    Ok(())
}
