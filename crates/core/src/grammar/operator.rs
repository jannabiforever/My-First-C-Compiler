use crate::{grammar::TokenType, t};

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,

    // relational
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl BinaryOp {
    pub const fn binding_power(&self) -> BindingPower {
        match self {
            // Multiplicative operators (highest precedence)
            BinaryOp::Multiply | BinaryOp::Divide => BindingPower::Multiplicative,
            // Additive operators
            BinaryOp::Add | BinaryOp::Subtract => BindingPower::Additive,
            // Relational operators
            BinaryOp::LessThan
            | BinaryOp::GreaterThan
            | BinaryOp::LessThanOrEqual
            | BinaryOp::GreaterThanOrEqual => BindingPower::Relational,
            // Equality operators
            BinaryOp::Equal | BinaryOp::NotEqual => BindingPower::Equality,
        }
    }

    pub const fn from_token_type(token: &TokenType) -> Option<Self> {
        match token {
            t!("+") => Some(BinaryOp::Add),
            t!("-") => Some(BinaryOp::Subtract),
            t!("*") => Some(BinaryOp::Multiply),
            t!("/") => Some(BinaryOp::Divide),
            t!("<") => Some(BinaryOp::LessThan),
            t!(">") => Some(BinaryOp::GreaterThan),
            t!("<=") => Some(BinaryOp::LessThanOrEqual),
            t!(">=") => Some(BinaryOp::GreaterThanOrEqual),
            t!("==") => Some(BinaryOp::Equal),
            t!("!=") => Some(BinaryOp::NotEqual),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

impl UnaryOp {
    pub const fn from_token_type(token: &TokenType) -> Option<Self> {
        match token {
            t!("!") => Some(UnaryOp::Not),
            t!("-") => Some(UnaryOp::Negate),
            _ => None,
        }
    }

    pub const fn binding_power(&self) -> BindingPower {
        BindingPower::Unary
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp {
    Assign,       // =
    PlusAssign,   // +=
    MinusAssign,  // -=
    MulAssign,    // *=
    DivAssign,    // /=
    ModAssign,    // %=
    AndAssign,    // &=
    OrAssign,     // |=
    XorAssign,    // ^=
    LShiftAssign, // <<=
    RShiftAssign, // >>=
}

impl AssignOp {
    #[inline]
    pub const fn binding_power(&self) -> BindingPower {
        BindingPower::Assignment
    }

    pub const fn from_token_type(token: &TokenType) -> Option<Self> {
        match token {
            t!("=") => Some(AssignOp::Assign),
            t!("+=") => Some(AssignOp::PlusAssign),
            t!("-=") => Some(AssignOp::MinusAssign),
            t!("*=") => Some(AssignOp::MulAssign),
            t!("/=") => Some(AssignOp::DivAssign),
            t!("%=") => Some(AssignOp::ModAssign),
            t!("&=") => Some(AssignOp::AndAssign),
            t!("|=") => Some(AssignOp::OrAssign),
            t!("^=") => Some(AssignOp::XorAssign),
            t!("<<=") => Some(AssignOp::LShiftAssign),
            t!(">>=") => Some(AssignOp::RShiftAssign),
            _ => None,
        }
    }
}

/// Binding power levels for operators in the expression parser.
///
/// Higher numeric values mean tighter binding. Both left and right
/// binding powers are provided so that Pratt parsing can distinguish
/// associativity and prefix/postfix operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingPower {
    /// Sentinel / non-operator
    Base,

    /// Comma operator `,`
    /// Lowest precedence, left-associative.
    Comma,

    /// Assignment operators `=, +=, -=, ...`
    /// Right-associative.
    Assignment,

    /// Conditional operator `?:`
    /// Right-associative.
    Conditional,

    /// Logical OR `||` (left-associative)
    LogicalOr,

    /// Logical AND `&&` (left-associative)
    LogicalAnd,

    /// Bitwise OR `|` (left-associative)
    BitwiseOr,

    /// Bitwise XOR `^` (left-associative)
    BitwiseXor,

    /// Bitwise AND `&` (left-associative)
    BitwiseAnd,

    /// Equality `==, !=` (left-associative)
    Equality,

    /// Relational `<, >, <=, >=` (left-associative)
    Relational,

    /// Shift `<<, >>` (left-associative)
    Shift,

    /// Additive `+ , -` (left-associative)
    Additive,

    /// Multiplicative `* , / , %` (left-associative)
    Multiplicative,

    /// Prefix unary operators `-x, !x, ~x, &x, *x, ++x, --x (prefix)...`
    /// Only binds to the right.
    Unary,

    /// Postfix operators `x(), x[], x.y, x->y, x++, x--, ...`
    /// Only binds to the left.
    Postfix,
}

impl BindingPower {
    /// Left binding power.
    pub const fn left_associative(self) -> u8 {
        match self {
            BindingPower::Base => 0,

            // comma: lowest, left-assoc
            BindingPower::Comma => 1,

            // assignment: right-assoc
            BindingPower::Assignment => 2,

            // conditional ?: right-assoc
            BindingPower::Conditional => 3,

            // logical OR / AND: left-assoc
            BindingPower::LogicalOr => 4,
            BindingPower::LogicalAnd => 5,

            // bitwise OR / XOR / AND: left-assoc
            BindingPower::BitwiseOr => 6,
            BindingPower::BitwiseXor => 7,
            BindingPower::BitwiseAnd => 8,

            // equality / relational / shift / additive / multiplicative: left-assoc
            BindingPower::Equality => 9,
            BindingPower::Relational => 10,
            BindingPower::Shift => 11,
            BindingPower::Additive => 12,
            BindingPower::Multiplicative => 13,

            // prefix: no left side
            BindingPower::Unary => 0,

            // postfix: only left side, highest precedence
            BindingPower::Postfix => 17,
        }
    }

    /// Right binding power.
    pub const fn right_associative(self) -> u8 {
        match self {
            BindingPower::Base => 0,

            // comma: left-assoc → rbp > lbp
            BindingPower::Comma => 2, // (1, 2)

            // assignment: right-assoc → rbp <= lbp
            BindingPower::Assignment => 2, // (2, 2)

            // conditional ?: right-assoc
            BindingPower::Conditional => 3, // (3, 3)

            // logical OR / AND: left-assoc
            BindingPower::LogicalOr => 5,  // (4, 5)
            BindingPower::LogicalAnd => 6, // (5, 6)

            // bitwise OR / XOR / AND: left-assoc
            BindingPower::BitwiseOr => 7,  // (6, 7)
            BindingPower::BitwiseXor => 8, // (7, 8)
            BindingPower::BitwiseAnd => 9, // (8, 9)

            // equality / relational / shift / additive / multiplicative: left-assoc
            BindingPower::Equality => 10,       // (9, 10)
            BindingPower::Relational => 11,     // (10, 11)
            BindingPower::Shift => 12,          // (11, 12)
            BindingPower::Additive => 13,       // (12, 13)
            BindingPower::Multiplicative => 14, // (13, 14)

            // prefix: only right side matters
            BindingPower::Unary => 16,

            // postfix: no right side
            BindingPower::Postfix => 0,
        }
    }

    pub const fn as_tuple(&self) -> (u8, u8) {
        (self.left_associative(), self.right_associative())
    }

    pub fn infer_from_token_type(token_type: &TokenType) -> Self {
        if let Some(op) = AssignOp::from_token_type(token_type) {
            op.binding_power()
        } else if let Some(op) = BinaryOp::from_token_type(token_type) {
            op.binding_power()
        } else if matches!(token_type, t!("[") | t!("(") | t!(".")) {
            BindingPower::Postfix
        } else {
            BindingPower::Base
        }
    }
}
