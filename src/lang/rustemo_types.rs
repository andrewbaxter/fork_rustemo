// Generated on 2022-02-04 15:46:18.496388 from bootstrap.py. Do not edit!

use num_enum::TryFromPrimitive;
use super::types::*;

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum TermKind {
	Terminals = 0,
	Import = 1,
	StrConst = 2,
	SemiColon = 3,
	As = 4,
	Name = 5,
	Action = 6,
	Colon = 7,
	OBrace = 8,
	CBrace = 9,
	Bar = 10,
	Left = 11,
	Reduce = 12,
	Right = 13,
	Shift = 14,
	Dynamic = 15,
	Nops = 16,
	Nopse = 17,
	IntConst = 18,
	Comma = 19,
	Prefer = 20,
	Finish = 21,
	Nofinish = 22,
	FloatConst = 23,
	BoolConst = 24,
	Equals = 25,
	QEquals = 26,
	OBracket = 27,
	CBracket = 28,
	Asterisk = 29,
	AsteriskGready = 30,
	Plus = 31,
	PlusGready = 32,
	Question = 33,
	QuestionGready = 34,
	OSquare = 35,
	CSquare = 36,
	RegExTerm = 37,
	WS = 38,
	OComment = 39,
	CComment = 40,
	CommentLine = 41,
	NotComment = 42,
	STOP = 43,
}

#[derive(Debug, Copy, Clone)]
pub enum NonTermKind {
	PGFile = 1,
	Imports = 2,
	Import = 3,
	ProductionRules = 4,
	ProductionRuleWithAction = 5,
	ProductionRule = 6,
	ProductionRuleRHS = 7,
	Production = 8,
	TerminalRules = 9,
	TerminalRuleWithAction = 10,
	TerminalRule = 11,
	ProductionMetaData = 12,
	ProductionMetaDatas = 13,
	TerminalMetaData = 14,
	TerminalMetaDatas = 15,
	UserMetaData = 16,
	Const = 17,
	Assignment = 18,
	Assignments = 19,
	PlainAssignment = 20,
	BoolAssignment = 21,
	ProductionGroup = 22,
	GrammarSymbolReference = 23,
	OptRepeatOperator = 24,
	RepeatOperator = 25,
	OptionalRepeatModifiersExpression = 26,
	OptionalRepeatModifiers = 27,
	OptionalRepeatModifier = 28,
	GrammarSymbol = 29,
	Recognizer = 30,
	LAYOUT = 31,
	LAYOUT_ITEM = 32,
	Comment = 33,
	CORNCS = 34,
	CORNC = 35,
}

#[derive(Debug)]
pub enum Symbol {
	Terminal(Terminal),
	NonTerminal(NonTerminal)
}

#[derive(Debug)]
pub enum Terminal {
	Terminals,
	Import,
	StrConst(StrConst),
	SemiColon,
	As,
	Name(Name),
	Action(Action),
	Colon,
	OBrace,
	CBrace,
	Bar,
	Left,
	Reduce,
	Right,
	Shift,
	Dynamic,
	Nops,
	Nopse,
	IntConst(IntConst),
	Comma,
	Prefer,
	Finish,
	Nofinish,
	FloatConst(FloatConst),
	BoolConst(BoolConst),
	Equals,
	QEquals,
	OBracket,
	CBracket,
	Asterisk,
	AsteriskGready,
	Plus,
	PlusGready,
	Question,
	QuestionGready,
	OSquare,
	CSquare,
	RegExTerm(RegExTerm),
	WS(WS),
	OComment,
	CComment,
	CommentLine(CommentLine),
	NotComment(NotComment),
	STOP(STOP),
}

#[derive(Debug)]
pub enum NonTerminal {
	PGFile(PGFile),
	Imports(Imports),
	Import(Import),
	ProductionRules(ProductionRules),
	ProductionRuleWithAction(ProductionRuleWithAction),
	ProductionRule(ProductionRule),
	ProductionRuleRHS(ProductionRuleRHS),
	Production(Production),
	TerminalRules(TerminalRules),
	TerminalRuleWithAction(TerminalRuleWithAction),
	TerminalRule(TerminalRule),
	ProductionMetaData(ProductionMetaData),
	ProductionMetaDatas(ProductionMetaDatas),
	TerminalMetaData(TerminalMetaData),
	TerminalMetaDatas(TerminalMetaDatas),
	UserMetaData(UserMetaData),
	Const(Const),
	Assignment(Assignment),
	Assignments(Assignments),
	PlainAssignment(PlainAssignment),
	BoolAssignment(BoolAssignment),
	ProductionGroup(ProductionGroup),
	GrammarSymbolReference(GrammarSymbolReference),
	OptRepeatOperator(OptRepeatOperator),
	RepeatOperator(RepeatOperator),
	OptionalRepeatModifiersExpression(OptionalRepeatModifiersExpression),
	OptionalRepeatModifiers(OptionalRepeatModifiers),
	OptionalRepeatModifier(OptionalRepeatModifier),
	GrammarSymbol(GrammarSymbol),
	Recognizer(Recognizer),
	LAYOUT(LAYOUT),
	LAYOUTITEM(LAYOUTITEM),
	Comment(Comment),
	CORNCS(CORNCS),
	CORNC(CORNC),
	Empty
}

#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum ProdKind {
	PGFileP0 = 1,
	PGFileP1 = 2,
	PGFileP2 = 3,
	PGFileP3 = 4,
	PGFileP4 = 5,
	ImportsP0 = 6,
	ImportsP1 = 7,
	ImportP0 = 8,
	ImportP1 = 9,
	ProductionRulesP0 = 10,
	ProductionRulesP1 = 11,
	ProductionRuleWithActionP0 = 12,
	ProductionRuleWithActionP1 = 13,
	ProductionRuleP0 = 14,
	ProductionRuleP1 = 15,
	ProductionRuleRHSP0 = 16,
	ProductionRuleRHSP1 = 17,
	ProductionP0 = 18,
	ProductionP1 = 19,
	TerminalRulesP0 = 20,
	TerminalRulesP1 = 21,
	TerminalRuleWithActionP0 = 22,
	TerminalRuleWithActionP1 = 23,
	TerminalRuleP0 = 24,
	TerminalRuleP1 = 25,
	TerminalRuleP2 = 26,
	TerminalRuleP3 = 27,
	ProductionMetaDataP0 = 28,
	ProductionMetaDataP1 = 29,
	ProductionMetaDataP2 = 30,
	ProductionMetaDataP3 = 31,
	ProductionMetaDataP4 = 32,
	ProductionMetaDataP5 = 33,
	ProductionMetaDataP6 = 34,
	ProductionMetaDataP7 = 35,
	ProductionMetaDataP8 = 36,
	ProductionMetaDatasP0 = 37,
	ProductionMetaDatasP1 = 38,
	TerminalMetaDataP0 = 39,
	TerminalMetaDataP1 = 40,
	TerminalMetaDataP2 = 41,
	TerminalMetaDataP3 = 42,
	TerminalMetaDataP4 = 43,
	TerminalMetaDataP5 = 44,
	TerminalMetaDatasP0 = 45,
	TerminalMetaDatasP1 = 46,
	UserMetaDataP0 = 47,
	ConstP0 = 48,
	ConstP1 = 49,
	ConstP2 = 50,
	ConstP3 = 51,
	AssignmentP0 = 52,
	AssignmentP1 = 53,
	AssignmentP2 = 54,
	AssignmentsP0 = 55,
	AssignmentsP1 = 56,
	PlainAssignmentP0 = 57,
	BoolAssignmentP0 = 58,
	ProductionGroupP0 = 59,
	GrammarSymbolReferenceP0 = 60,
	GrammarSymbolReferenceP1 = 61,
	OptRepeatOperatorP0 = 62,
	OptRepeatOperatorP1 = 63,
	RepeatOperatorP0 = 64,
	RepeatOperatorP1 = 65,
	RepeatOperatorP2 = 66,
	RepeatOperatorP3 = 67,
	RepeatOperatorP4 = 68,
	RepeatOperatorP5 = 69,
	OptionalRepeatModifiersExpressionP0 = 70,
	OptionalRepeatModifiersExpressionP1 = 71,
	OptionalRepeatModifiersP0 = 72,
	OptionalRepeatModifiersP1 = 73,
	OptionalRepeatModifierP0 = 74,
	GrammarSymbolP0 = 75,
	GrammarSymbolP1 = 76,
	RecognizerP0 = 77,
	RecognizerP1 = 78,
	LAYOUTP0 = 79,
	LAYOUTP1 = 80,
	LAYOUTP2 = 81,
	LAYOUTITEMP0 = 82,
	LAYOUTITEMP1 = 83,
	CommentP0 = 84,
	CommentP1 = 85,
	CORNCSP0 = 86,
	CORNCSP1 = 87,
	CORNCSP2 = 88,
	CORNCP0 = 89,
	CORNCP1 = 90,
	CORNCP2 = 91,
}

