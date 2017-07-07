use std::fmt;
use ast::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Op {
    And,
    Or,
    Not,
    Imply,
    Equiv,
    Even,
    Odd,
    ITE,
    Between(usize, usize),
}
pub struct ShortOp {
    op: Op,
}
impl Op {
    pub fn short(&self) -> ShortOp {
        ShortOp{op: *self}
    }
}
impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Constant::True => write!(f, "T"),
            Constant::False => write!(f, "F"),
        }
    }
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Op::*;
        match *self {
            And => write!(f, "AND"),
            Or => write!(f, "OR"),
            Not => write!(f, "NOT"),
            Imply => write!(f, "IMPLY"),
            Equiv => write!(f, "EQUIV"),
            Even => write!(f, "EVEN"),
            Odd => write!(f, "ODD"),
            ITE => write!(f, "ITE"),
            Between(ref x, ref y) => write!(f, "[{}, {}]", x, y),
        }
    }
}
impl fmt::Display for ShortOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Op::*;
        match *(&self.op) {
            And => write!(f, "&"),
            Or => write!(f, "|"),
            Not => write!(f, "~"),
            Imply => write!(f, "=>"),
            Equiv => write!(f, "=="),
            Even => write!(f, "=="),
            Odd => write!(f, "^"),
            _ => panic!("Op does not have a short version"),
        }
    }
}
