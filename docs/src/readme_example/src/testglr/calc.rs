/// Generated by rustemo. Do not edit manually!
use std::fmt::Debug;
use std::hash::Hash;
use rustemo::{
    Result, Input as InputT, Lexer, Token, TokenRecognizer as TokenRecognizerT, Parser,
    ParserDefinition, State as StateT, Builder,
};
use regex::Regex;
use once_cell::sync::Lazy;
use rustemo::StringLexer;
use rustemo::LRBuilder;
use super::calc_actions;
use rustemo::{GlrParser, Forest, GssHead};
use rustemo::Action::{self, Shift, Reduce, Accept};
#[allow(unused_imports)]
use rustemo::debug::{log, logn};
#[allow(unused_imports)]
#[cfg(debug_assertions)]
use colored::*;
pub type Input = str;
const STATE_COUNT: usize = 7usize;
const MAX_RECOGNIZERS: usize = 3usize;
#[allow(dead_code)]
const TERMINAL_COUNT: usize = 4usize;
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    #[default]
    STOP,
    Number,
    Add,
    Mul,
}
use TokenKind as TK;
impl From<TokenKind> for usize {
    fn from(t: TokenKind) -> Self {
        t as usize
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq)]
pub enum ProdKind {
    EP1,
    EP2,
    EP3,
}
use ProdKind as PK;
impl std::fmt::Debug for ProdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProdKind::EP1 => "E: E Add E",
            ProdKind::EP2 => "E: E Mul E",
            ProdKind::EP3 => "E: Number",
        };
        write!(f, "{}", name)
    }
}
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum NonTermKind {
    EMPTY,
    AUG,
    E,
}
impl From<ProdKind> for NonTermKind {
    fn from(prod: ProdKind) -> Self {
        match prod {
            ProdKind::EP1 => NonTermKind::E,
            ProdKind::EP2 => NonTermKind::E,
            ProdKind::EP3 => NonTermKind::E,
        }
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    #[default]
    AUGS0,
    NumberS1,
    ES2,
    AddS3,
    MulS4,
    ES5,
    ES6,
}
impl StateT for State {
    fn default_layout() -> Option<Self> {
        None
    }
}
impl From<State> for usize {
    fn from(s: State) -> Self {
        s as usize
    }
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            State::AUGS0 => "0:AUG",
            State::NumberS1 => "1:Number",
            State::ES2 => "2:E",
            State::AddS3 => "3:Add",
            State::MulS4 => "4:Mul",
            State::ES5 => "5:E",
            State::ES6 => "6:E",
        };
        write!(f, "{name}")
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
    Number(calc_actions::Number),
    Add,
    Mul,
}
#[derive(Debug)]
pub enum NonTerminal {
    E(calc_actions::E),
}
type ActionFn = fn(token: TokenKind) -> Vec<Action<State, ProdKind>>;
pub struct CalcParserDefinition {
    actions: [ActionFn; STATE_COUNT],
    gotos: [fn(nonterm: NonTermKind) -> State; STATE_COUNT],
    token_kinds: [[Option<(TokenKind, bool)>; MAX_RECOGNIZERS]; STATE_COUNT],
}
fn action_aug_s0(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::Number => Vec::from(&[Shift(State::NumberS1)]),
        _ => vec![],
    }
}
fn action_number_s1(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::STOP => Vec::from(&[Reduce(PK::EP3, 1usize)]),
        TK::Add => Vec::from(&[Reduce(PK::EP3, 1usize)]),
        TK::Mul => Vec::from(&[Reduce(PK::EP3, 1usize)]),
        _ => vec![],
    }
}
fn action_e_s2(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::STOP => Vec::from(&[Accept]),
        TK::Add => Vec::from(&[Shift(State::AddS3)]),
        TK::Mul => Vec::from(&[Shift(State::MulS4)]),
        _ => vec![],
    }
}
fn action_add_s3(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::Number => Vec::from(&[Shift(State::NumberS1)]),
        _ => vec![],
    }
}
fn action_mul_s4(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::Number => Vec::from(&[Shift(State::NumberS1)]),
        _ => vec![],
    }
}
fn action_e_s5(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::STOP => Vec::from(&[Reduce(PK::EP1, 3usize)]),
        TK::Add => Vec::from(&[Shift(State::AddS3), Reduce(PK::EP1, 3usize)]),
        TK::Mul => Vec::from(&[Shift(State::MulS4), Reduce(PK::EP1, 3usize)]),
        _ => vec![],
    }
}
fn action_e_s6(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::STOP => Vec::from(&[Reduce(PK::EP2, 3usize)]),
        TK::Add => Vec::from(&[Shift(State::AddS3), Reduce(PK::EP2, 3usize)]),
        TK::Mul => Vec::from(&[Shift(State::MulS4), Reduce(PK::EP2, 3usize)]),
        _ => vec![],
    }
}
fn goto_aug_s0(nonterm_kind: NonTermKind) -> State {
    match nonterm_kind {
        NonTermKind::E => State::ES2,
        _ => {
            panic!(
                "Invalid terminal kind ({nonterm_kind:?}) for GOTO state ({:?}).",
                State::AUGS0
            )
        }
    }
}
fn goto_add_s3(nonterm_kind: NonTermKind) -> State {
    match nonterm_kind {
        NonTermKind::E => State::ES5,
        _ => {
            panic!(
                "Invalid terminal kind ({nonterm_kind:?}) for GOTO state ({:?}).",
                State::AddS3
            )
        }
    }
}
fn goto_mul_s4(nonterm_kind: NonTermKind) -> State {
    match nonterm_kind {
        NonTermKind::E => State::ES6,
        _ => {
            panic!(
                "Invalid terminal kind ({nonterm_kind:?}) for GOTO state ({:?}).",
                State::MulS4
            )
        }
    }
}
fn goto_invalid(_nonterm_kind: NonTermKind) -> State {
    panic!("Invalid GOTO entry!");
}
pub(crate) static PARSER_DEFINITION: CalcParserDefinition = CalcParserDefinition {
    actions: [
        action_aug_s0,
        action_number_s1,
        action_e_s2,
        action_add_s3,
        action_mul_s4,
        action_e_s5,
        action_e_s6,
    ],
    gotos: [
        goto_aug_s0,
        goto_invalid,
        goto_invalid,
        goto_add_s3,
        goto_mul_s4,
        goto_invalid,
        goto_invalid,
    ],
    token_kinds: [
        [Some((TK::Number, false)), None, None],
        [Some((TK::STOP, true)), Some((TK::Add, true)), Some((TK::Mul, true))],
        [Some((TK::STOP, true)), Some((TK::Add, true)), Some((TK::Mul, true))],
        [Some((TK::Number, false)), None, None],
        [Some((TK::Number, false)), None, None],
        [Some((TK::STOP, true)), Some((TK::Add, true)), Some((TK::Mul, true))],
        [Some((TK::STOP, true)), Some((TK::Add, true)), Some((TK::Mul, true))],
    ],
};
impl ParserDefinition<State, ProdKind, TokenKind, NonTermKind> for CalcParserDefinition {
    fn actions(&self, state: State, token: TokenKind) -> Vec<Action<State, ProdKind>> {
        PARSER_DEFINITION.actions[state as usize](token)
    }
    fn goto(&self, state: State, nonterm: NonTermKind) -> State {
        PARSER_DEFINITION.gotos[state as usize](nonterm)
    }
    fn expected_token_kinds(&self, state: State) -> Vec<(TokenKind, bool)> {
        PARSER_DEFINITION.token_kinds[state as usize].iter().map_while(|t| *t).collect()
    }
    fn longest_match() -> bool {
        true
    }
    fn grammar_order() -> bool {
        false
    }
}
pub(crate) type Context<'i, I> = GssHead<'i, I, State, TokenKind>;
pub struct CalcParser<
    'i,
    I: InputT + ?Sized,
    L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
    B,
>(
    GlrParser<'i, State, L, ProdKind, TokenKind, NonTermKind, CalcParserDefinition, I, B>,
);
#[allow(dead_code)]
impl<
    'i,
> CalcParser<
    'i,
    Input,
    StringLexer<Context<'i, Input>, State, TokenKind, TokenRecognizer, TERMINAL_COUNT>,
    DefaultBuilder,
> {
    pub fn new() -> Self {
        Self(
            GlrParser::new(
                &PARSER_DEFINITION,
                false,
                false,
                StringLexer::new(true, &RECOGNIZERS),
            ),
        )
    }
}
#[allow(dead_code)]
impl<'i, I, L, B> Parser<'i, I, Context<'i, I>, State, TokenKind>
for CalcParser<'i, I, L, B>
where
    I: InputT + ?Sized + Debug,
    L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
    B: LRBuilder<'i, I, Context<'i, I>, State, ProdKind, TokenKind>,
{
    type Output = Forest<'i, I, ProdKind, TokenKind>;
    fn parse(&self, input: &'i I) -> Result<Self::Output> {
        self.0.parse(input)
    }
    fn parse_with_context(
        &self,
        context: &mut Context<'i, I>,
        input: &'i I,
    ) -> Result<Self::Output> {
        self.0.parse_with_context(context, input)
    }
    fn parse_file<'a, F: AsRef<std::path::Path>>(
        &'a mut self,
        file: F,
    ) -> Result<Self::Output>
    where
        'a: 'i,
    {
        self.0.parse_file(file)
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum Recognizer {
    Stop,
    StrMatch(&'static str),
    RegexMatch(Lazy<Regex>),
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct TokenRecognizer(TokenKind, Recognizer);
impl<'i> TokenRecognizerT<'i> for TokenRecognizer {
    fn recognize(&self, input: &'i str) -> Option<&'i str> {
        match &self {
            #[allow(unused_variables)]
            TokenRecognizer(token_kind, Recognizer::StrMatch(s)) => {
                logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                if input.starts_with(s) {
                    log!("{}", "recognized".bold().green());
                    Some(s)
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
            #[allow(unused_variables)]
            TokenRecognizer(token_kind, Recognizer::RegexMatch(r)) => {
                logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                let match_str = r.find(input);
                match match_str {
                    Some(x) => {
                        let x_str = x.as_str();
                        log!("{} '{}'", "recognized".bold().green(), x_str);
                        Some(x_str)
                    }
                    None => {
                        log!("{}", "not recognized".red());
                        None
                    }
                }
            }
            TokenRecognizer(_, Recognizer::Stop) => {
                logn!("{} STOP -- ", "    Recognizing".green());
                if input.is_empty() {
                    log!("{}", "recognized".bold().green());
                    Some("")
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
        }
    }
}
pub(crate) static RECOGNIZERS: [TokenRecognizer; TERMINAL_COUNT] = [
    TokenRecognizer(TokenKind::STOP, Recognizer::Stop),
    TokenRecognizer(
        TokenKind::Number,
        Recognizer::RegexMatch(
            Lazy::new(|| { Regex::new(concat!("^(", "\\d+", ")")).unwrap() }),
        ),
    ),
    TokenRecognizer(TokenKind::Add, Recognizer::StrMatch("+")),
    TokenRecognizer(TokenKind::Mul, Recognizer::StrMatch("*")),
];
pub struct DefaultBuilder {
    res_stack: Vec<Symbol>,
}
impl DefaultBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { res_stack: vec![] }
    }
}
impl Builder for DefaultBuilder {
    type Output = calc_actions::E;
    fn get_result(&mut self) -> Self::Output {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::E(r)) => r,
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}
impl<'i> LRBuilder<'i, Input, Context<'i, Input>, State, ProdKind, TokenKind>
for DefaultBuilder {
    #![allow(unused_variables)]
    fn shift_action(
        &mut self,
        context: &mut Context<'i, Input>,
        token: Token<'i, Input, TokenKind>,
    ) {
        let val = match token.kind {
            TokenKind::STOP => panic!("Cannot shift STOP token!"),
            TokenKind::Number => Terminal::Number(calc_actions::number(&*context, token)),
            TokenKind::Add => Terminal::Add,
            TokenKind::Mul => Terminal::Mul,
        };
        self.res_stack.push(Symbol::Terminal(val));
    }
    fn reduce_action(
        &mut self,
        context: &mut Context<'i, Input>,
        prod: ProdKind,
        _prod_len: usize,
    ) {
        let prod = match prod {
            ProdKind::EP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calc_actions::e_c1(&*context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EP2 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calc_actions::e_c2(&*context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EP3 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::Number(p0)) => {
                        NonTerminal::E(calc_actions::e_number(&*context, p0))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
}
