/// Binding power levels for operators in the expression parser
///
/// This enum represents the precedence hierarchy of operators in C.
/// Higher numeric values indicate higher precedence (tighter binding).
///
/// The precedence levels follow the C operator precedence table,
/// with each level supporting both left and right binding power
/// for Pratt parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum BindingPower {
    /// Assignment operators (=, +=, -=, etc.)
    /// Right-associative: a = b = c parses as a = (b = c)
    Assignment = 0,

    /// Equality operators (==, !=)
    Equality = 5,

    /// Relational operators (<, >, <=, >=)
    Relational = 7,

    /// Additive operators (+, -)
    Additive = 9,

    /// Multiplicative operators (*, /, %)
    Multiplicative = 11,

    /// Unary operators (-, !, ~, etc.)
    Unary = 13,

    /// Postfix operators (function calls, array subscript, member access)
    /// Highest precedence
    Postfix = 19,
}

impl BindingPower {
    /// Returns the left and right binding power for infix operators
    ///
    /// For left-associative operators: (n, n+1)
    /// For right-associative operators: (n+1, n)
    #[inline]
    pub const fn infix(self, right_associative: bool) -> (u8, u8) {
        let base = self as u8;
        if right_associative {
            // Right-associative: higher left BP, lower right BP
            // This causes the parser to recurse on the right side first
            (base + 1, base)
        } else {
            // Left-associative: lower left BP, higher right BP
            // This causes the parser to reduce left side before continuing
            (base, base + 1)
        }
    }

    /// Returns the left binding power for postfix operators
    #[inline]
    pub const fn postfix(self) -> u8 {
        self as u8
    }

    /// Returns the right binding power for prefix operators
    #[inline]
    pub const fn prefix(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_associative_binding_power() {
        // Left-associative: (n, n+1)
        assert_eq!(BindingPower::Additive.infix(false), (9, 10));
        assert_eq!(BindingPower::Multiplicative.infix(false), (11, 12));
    }

    #[test]
    fn test_right_associative_binding_power() {
        // Right-associative: (n+1, n)
        assert_eq!(BindingPower::Assignment.infix(true), (1, 0));
    }

    #[test]
    fn test_postfix_binding_power() {
        assert_eq!(BindingPower::Postfix.postfix(), 19);
    }

    #[test]
    fn test_precedence_ordering() {
        // Higher precedence should have higher numeric values
        assert!(BindingPower::Postfix > BindingPower::Unary);
        assert!(BindingPower::Unary > BindingPower::Multiplicative);
        assert!(BindingPower::Multiplicative > BindingPower::Additive);
        assert!(BindingPower::Additive > BindingPower::Relational);
        assert!(BindingPower::Relational > BindingPower::Equality);
        assert!(BindingPower::Equality > BindingPower::Assignment);
    }
}
