/// Generated by rustemo. Do not edit manually!
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use rustemo::Result;
use rustemo::lexer::{self, Token, AsStr};
use rustemo::parser::Parser;
use rustemo::builder::Builder;
use rustemo::lr::builder::LRBuilder;
use rustemo::lr::parser::{LRParser, ParserDefinition};
use rustemo::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
use rustemo::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
#[allow(unused_imports)]
use rustemo::debug::{log, logn};
const TERMINAL_COUNT: usize = 3usize;
const NONTERMINAL_COUNT: usize = 3usize;
const STATE_COUNT: usize = 5usize;
#[allow(dead_code)]
const MAX_ACTIONS: usize = 1usize;
use regex::Regex;
use once_cell::sync::Lazy;
use rustemo::lexer::StringLexer;
use super::calculator_actions;
pub type Input = str;
pub type Context<'i> = lexer::Context<'i, Input>;
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[default]
    STOP,
    Operand,
    Operator,
}
impl AsStr for TokenKind {
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            TokenKind::STOP => "STOP",
            TokenKind::Operand => "Operand",
            TokenKind::Operator => "Operator",
        }
    }
}
impl From<TermIndex> for TokenKind {
    fn from(term_index: TermIndex) -> Self {
        match term_index.0 {
            0usize => TokenKind::STOP,
            1usize => TokenKind::Operand,
            2usize => TokenKind::Operator,
            _ => unreachable!(),
        }
    }
}
impl From<TokenKind> for TermIndex {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::STOP => TermIndex(0usize),
            TokenKind::Operand => TermIndex(1usize),
            TokenKind::Operator => TermIndex(2usize),
        }
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
pub enum ProdKind {
    ExpressionP1,
}
impl AsStr for ProdKind {
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            ProdKind::ExpressionP1 => "ExpressionP1",
        }
    }
}
impl std::fmt::Display for ProdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProdKind::ExpressionP1 => "Expression: Operand Operator Operand",
        };
        write!(f, "{}", name)
    }
}
impl From<ProdIndex> for ProdKind {
    fn from(prod_index: ProdIndex) -> Self {
        match prod_index.0 {
            1usize => ProdKind::ExpressionP1,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Terminal {
    Operand(calculator_actions::Operand),
    Operator(calculator_actions::Operator),
}
#[derive(Debug)]
pub enum NonTerminal {
    Expression(calculator_actions::Expression),
}
pub struct CalculatorParserDefinition {
    actions: [[Action; TERMINAL_COUNT]; STATE_COUNT],
    gotos: [[Option<StateIndex>; NONTERMINAL_COUNT]; STATE_COUNT],
    token_recognizers: [[Option<TokenRecognizer>; 1usize]; STATE_COUNT],
}
pub(crate) static PARSER_DEFINITION: CalculatorParserDefinition = CalculatorParserDefinition {
    actions: [
        [Error, Shift(StateIndex(1usize)), Error],
        [Error, Error, Shift(StateIndex(3usize))],
        [Accept, Error, Error],
        [Error, Shift(StateIndex(4usize)), Error],
        [Reduce(ProdIndex(1usize), 3usize, NonTermIndex(2usize)), Error, Error],
    ],
    gotos: [
        [None, None, Some(StateIndex(2usize))],
        [None, None, None],
        [None, None, None],
        [None, None, None],
        [None, None, None],
    ],
    token_recognizers: [
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Operand,
                recognizer: Recognizer::RegexMatch(1usize),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Operator,
                recognizer: Recognizer::RegexMatch(2usize),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Operand,
                recognizer: Recognizer::RegexMatch(1usize),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
        ],
    ],
};
impl ParserDefinition<TokenRecognizer> for CalculatorParserDefinition {
    fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
        PARSER_DEFINITION.actions[state_index.0][term_index.0]
    }
    fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {
        PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
    }
    fn recognizers(&self, state_index: StateIndex) -> Vec<&TokenRecognizer> {
        PARSER_DEFINITION
            .token_recognizers[state_index.0]
            .iter()
            .map_while(|tr| tr.as_ref())
            .collect()
    }
}
#[derive(Default)]
pub struct CalculatorParser {
    content: Option<<Input as ToOwned>::Owned>,
}
#[allow(dead_code)]
impl<'i> CalculatorParser {
    pub fn new() -> Self {
        Self { content: None }
    }
    #[allow(clippy::needless_lifetimes)]
    pub fn parse_file<P: AsRef<std::path::Path>>(
        &'i mut self,
        file: P,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        self.content = Some(<Input as rustemo::lexer::Input>::read_file(&file)?);
        let mut context = Context::new(
            file.as_ref().to_string_lossy().to_string(),
            self.content.as_ref().unwrap(),
        );
        self.inner_parse(&mut context)
    }
    #[allow(clippy::needless_lifetimes)]
    pub fn parse(
        &self,
        input: &'i Input,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        let mut context = Context::new("<str>".to_string(), input);
        self.inner_parse(&mut context)
    }
    #[allow(clippy::needless_lifetimes)]
    fn inner_parse(
        &self,
        context: &mut Context<'i>,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        let local_lexer = StringLexer::new(true);
        let lexer = &local_lexer;
        let mut local_builder = DefaultBuilder::new();
        let builder = &mut local_builder;
        let mut parser = LRParser::new(&PARSER_DEFINITION, StateIndex(0), false);
        parser.parse(context, lexer, builder)
    }
}
pub(crate) static RECOGNIZERS: [Option<Lazy<Regex>>; TERMINAL_COUNT] = [
    None,
    Some(Lazy::new(|| { Regex::new(concat!("^", "\\d+(\\.\\d+)?")).unwrap() })),
    Some(Lazy::new(|| { Regex::new(concat!("^", "\\+|-|\\*|/")).unwrap() })),
];
#[allow(dead_code)]
#[derive(Debug)]
pub enum Recognizer {
    Stop,
    StrMatch(&'static str),
    RegexMatch(usize),
}
#[derive(Debug)]
pub struct TokenRecognizer {
    token_kind: TokenKind,
    recognizer: Recognizer,
    finish: bool,
}
impl lexer::TokenRecognizer for TokenRecognizer {
    type TokenKind = TokenKind;
    type Input = str;
    fn recognize<'i>(&self, input: &'i str) -> Option<&'i str> {
        match &self.recognizer {
            Recognizer::StrMatch(s) => {
                logn!("Recognizing <{:?}> -- ", self.token_kind());
                if input.starts_with(s) {
                    log!("recognized");
                    Some(s)
                } else {
                    log!("not recognized");
                    None
                }
            }
            Recognizer::RegexMatch(r) => {
                logn!("Recognizing <{:?}> -- ", self.token_kind());
                let match_str = RECOGNIZERS[*r].as_ref().unwrap().find(input);
                match match_str {
                    Some(x) => {
                        let x_str = x.as_str();
                        log!("recognized <{}>", x_str);
                        Some(x_str)
                    }
                    None => {
                        log!("not recognized");
                        None
                    }
                }
            }
            Recognizer::Stop => {
                logn!("Recognizing <STOP> -- ");
                if input.is_empty() {
                    log!("recognized");
                    Some("")
                } else {
                    log!("not recognized");
                    None
                }
            }
        }
    }
    #[inline]
    fn token_kind(&self) -> TokenKind {
        self.token_kind
    }
    #[inline]
    fn finish(&self) -> bool {
        self.finish
    }
}
impl PartialEq for TokenRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.token_kind == other.token_kind
    }
}
impl Eq for TokenRecognizer {}
impl Hash for TokenRecognizer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_kind.hash(state);
    }
}
pub struct DefaultBuilder {
    res_stack: Vec<Symbol>,
}
impl Builder for DefaultBuilder {
    type Output = calculator_actions::Expression;
    fn new() -> Self {
        Self { res_stack: vec![] }
    }
    fn get_result(&mut self) -> Self::Output {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::Expression(r)) => r,
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}
impl<'i> LRBuilder<'i, Input, TokenKind> for DefaultBuilder {
    #![allow(unused_variables)]
    fn shift_action(
        &mut self,
        context: &mut Context<'i>,
        token: Token<'i, Input, TokenKind>,
    ) {
        let val = match token.kind {
            TokenKind::STOP => panic!("Cannot shift STOP token!"),
            TokenKind::Operand => {
                Terminal::Operand(calculator_actions::operand(context, token))
            }
            TokenKind::Operator => {
                Terminal::Operator(calculator_actions::operator(context, token))
            }
        };
        self.res_stack.push(Symbol::Terminal(val));
    }
    fn reduce_action(
        &mut self,
        context: &mut Context<'i>,
        prod_idx: ProdIndex,
        _prod_len: usize,
    ) {
        let prod = match ProdKind::from(prod_idx) {
            ProdKind::ExpressionP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::Operand(p0)),
                        Symbol::Terminal(Terminal::Operator(p1)),
                        Symbol::Terminal(Terminal::Operand(p2)),
                    ) => {
                        NonTerminal::Expression(
                            calculator_actions::expression_c1(context, p0, p1, p2),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
}
