use crate::{builder::Builder, lexer::Lexer};
use core::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct StateIndex(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct ProdIndex(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct TermIndex(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct NonTermIndex(pub usize);
impl NonTermIndex {
    pub(crate) fn to_symbol_index(&self, len: usize) -> SymbolIndex {
        SymbolIndex(self.0 + len)
    }
}

// Symbol index for non-terminal is <max term index> + NonTermIndex.
// For terminals symbol index is the same as TermIndex
#[derive(Debug, Copy, Clone)]
pub struct SymbolIndex(pub usize);

impl Default for SymbolIndex {
    fn default() -> Self {
        Self(usize::MAX)    // invalid value by default
    }
}

impl From<TermIndex> for SymbolIndex {
    fn from(index: TermIndex) -> Self {
        Self(index.0)
    }
}

pub trait Parser<L, B>
where
    L: Lexer,
    B: Builder<Lexer = L>,
{
    fn parse(&mut self, lexer: L) -> B::Output;
}

pub trait ParserDefinition {
    fn action(&self, state: StateIndex, term_index: TermIndex) -> Action;
    fn goto(&self, state: StateIndex, nonterm_id: NonTermIndex) -> StateIndex;
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Shift(StateIndex, TermIndex),
    Reduce(ProdIndex, usize, NonTermIndex, &'static str),
    Accept,
    Error,
}

/// Parser context provides necessary information to lexers and actions.
pub trait Context {
    fn position(&self) -> usize;
    fn set_position(&mut self, position: usize);
    fn state(&self) -> StateIndex;
}
