/// Generated by rustemo on 2022-07-21 20:13:52.625719562 +02:00

use num_enum::TryFromPrimitive;
use super::rustemo_actions::*;

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum TermKind {
    STOP = 0,
    Plus = 1,
    Mul = 2,
    LParen = 3,
    RParen = 4,
    Num = 5,
}

#[derive(Debug, Copy, Clone)]
pub enum NonTermKind {
    EMPTY = 0,
    AUG = 1,
    E = 2,
    T = 3,
    F = 4,
}

#[derive(Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal)
}

#[derive(Debug)]
pub enum Terminal {
    STOP,
    Plus,
    Mul,
    LParen,
    RParen,
    Num(Num),
}

#[derive(Debug)]
pub enum NonTerminal {
    E(E),
    T(T),
    F(F),
}

#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum ProdKind {
    EP0 = 1,
    EP1 = 2,
    TP0 = 3,
    TP1 = 4,
    FP0 = 5,
    FP1 = 6,
}
