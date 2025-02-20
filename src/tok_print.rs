use crate::*;
use std::fmt;

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Newline => write!(f, "newline"),
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::Tab => write!(f, "tab"),
            TokenKind::LeftParen => write!(f, "left parenthesis"),
            TokenKind::RightParen => write!(f, "right parenthesis"),
            TokenKind::LeftBrace => write!(f, "left brace"),
            TokenKind::RightBrace => write!(f, "right brace"),
            TokenKind::Comma => write!(f, "comma"),
            TokenKind::Tilde => write!(f, "tilde"),
            TokenKind::Grave => write!(f, "grave accent"),
            TokenKind::Pound => write!(f, "pound sign"),
            TokenKind::Plus => write!(f, "plus sign"),
            TokenKind::PlusPlus => write!(f, "plus plus"),
            TokenKind::Minus => write!(f, "minus sign"),
            TokenKind::MinusMinus => write!(f, "minus minus"),
            TokenKind::Star => write!(f, "asterisk"),
            TokenKind::Slash => write!(f, "slash"),
            TokenKind::Mod => write!(f, "modulus"),
            TokenKind::Bang => write!(f, "exclamation mark"),
            TokenKind::Equal => write!(f, "equal sign"),
            TokenKind::Greater => write!(f, "greater than"),
            TokenKind::GreaterGreater => write!(f, "greater greater"),
            TokenKind::Less => write!(f, "less than"),
            TokenKind::LessLess => write!(f, "less less"),
            TokenKind::Amp => write!(f, "ampersand"),
            TokenKind::AmpAmp => write!(f, "ampersand ampersand"),
            TokenKind::Pipe => write!(f, "pipe"),
            TokenKind::PipePipe => write!(f, "pipe pipe"),
            TokenKind::Xor => write!(f, "caret"),
            TokenKind::Colon => write!(f, "colon"),
            TokenKind::Register(value) => write!(f, "register({})", value),
            TokenKind::CharLit(value) => write!(f, "character literal('{}')", value),
            TokenKind::StringLit(value) => write!(f, "string literal(\"{}\")", value),
            TokenKind::IntLit(value) => write!(f, "integer literal({})", value),
            TokenKind::MacroDef(value) => write!(f, "macro definition({})", value),
            TokenKind::Constant(value) => write!(f, "constant({})", value),
            TokenKind::Ident(value) => write!(f, "identifier({})", value),
            TokenKind::Directive(value) => write!(f, "directive({})", value),
            TokenKind::MacroIdent(value) => write!(f, "macro identifier({})", value),
            TokenKind::MacroLabel(value) => write!(f, "macro label({})", value),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::Macro(content) => write!(f, "macro({:?})", content),
            TokenKind::Instruction(data) => write!(f, "instruction({:?})", data),
            TokenKind::Label(value) => write!(f, "label({})", value),
            TokenKind::Mem(token) => write!(f, "memory({})", token),
            TokenKind::IMem(token) => write!(f, "immediate memory({})", token),
            TokenKind::IIdent(value) => write!(f, "immediate identifier({})", value),
            TokenKind::IReg(value) => write!(f, "immediate register({})", value),
            TokenKind::Imm(value) => write!(f, "immediate value({})", value),
            TokenKind::Expr(value) => write!(f, "expression value({})", value),
        }
    }
}
