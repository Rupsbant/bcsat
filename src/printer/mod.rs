pub mod options;
pub mod operators;
use std::fmt;
use ast::*;

pub use self::options::*;
use self::operators::*;

pub struct PrintBCSAT<'a, T>(&'a FormulaOptions, T);
type PF<'a, 'b> = PrintBCSAT<'a, &'b Formula>;

impl FormulaOptions {
    pub fn display<T>(&self, t: T) -> PrintBCSAT<T> {
        PrintBCSAT(self, t)
    }
    fn print_formula(&self, f: &mut fmt::Formatter, formula: &Formula) -> fmt::Result {
        use self::Formula::*;
        match *formula {
            Named(ref x) => write!(f, "{}", x),
            Constant(ref c) => write!(f, "{}", c),
            Comment(ref form, ref com) => self.print_comments.fmt(f, self.display(form), com),
            Imply(ref l, ref r) => {
                self.imply
                    .binary(f, Op::Imply, self.display(l), self.display(r))
            }
            ITE(ref i, ref t, ref e) => write!(f, "ITE{}", self.display(vec![i, t, e])),
            Not(ref n) => self.not.unary(f, Op::Not, self.display(n)),
            Equiv(ref v) => self.equiv.nary(f, Op::Equiv, self.display(v)),
            Odd(ref v) => self.odd.nary(f, Op::Odd, self.display(v)),
            Even(ref v) => self.even.nary(f, Op::Even, self.display(v)),
            And(ref v) => self.and.nary(f, Op::And, self.display(v)),
            Or(ref v) => self.or.nary(f, Op::Or, self.display(v)),
            Between(l, u, ref v) => write!(f, "[{}, {}]{}", l, u, self.display(v)),
        }
    }
}

impl InfixOptions {
    fn unary(&self, f: &mut fmt::Formatter, op: Op, n: PF) -> fmt::Result {
        use self::InfixOptions::*;
        match *self {
            InfixBinary(Parens::Yes) => write!(f, "{}({})", op.short(), n,),
            InfixBinary(_) => panic!("Not yet implemented: removing parentheses"),
            Long => write!(f, "{}({})", op, n),
        }
    }
    fn binary(&self, f: &mut fmt::Formatter, op: Op, l: PF, r: PF) -> fmt::Result {
        use self::InfixOptions::*;
        match *self {
            InfixBinary(Parens::Yes) => write!(f, "({}) {} ({})", l, op.short(), r),
            InfixBinary(_) => panic!("Not yet implemented: removing parentheses"),
            Long => write!(f, "{}({}, {})", op, l, r),
        }
    }
    fn nary(&self, f: &mut fmt::Formatter, op: Op, l: PrintBCSAT<&Vec<F>>) -> fmt::Result {
        match l.1.len() {
            2 => self.binary(f, op, l.0.display(&l.1[0]), l.0.display(&l.1[1])),
            _ => write!(f, "{}{}", op, l),
        }
    }
}
impl InfixChainOptions {
    fn nary(&self, f: &mut fmt::Formatter, op: Op, l: PrintBCSAT<&Vec<F>>) -> fmt::Result {
        use self::InfixChainOptions::*;
        match *self {
            InfixChain(Parens::Yes) => {
                let mut it = l.1.iter();
                if let Some(elem) = it.next() {
                    try!(write!(f, "({})", l.0.display(elem)));
                    for elem in it {
                        try!(write!(f, " {} ({})", op.short(), l.0.display(elem)));
                    }
                    Ok(())
                } else {
                    InfixOptions::Long.nary(f, op, l)
                }
            }
            InfixChain(_) => panic!("Not yet implemented: removing parentheses"),
            NoChain(ref x) => x.nary(f, op, l),
        }
    }
}
impl PrintComments {
    fn fmt<T>(&self, f: &mut fmt::Formatter, t: T, c: &str) -> fmt::Result
        where T: fmt::Display
    {
        use self::PrintComments::*;
        match *self {
            Yes => write!(f, "{} //{}\n", t, c),
            No => write!(f, "{}", t),
        }
    }
}

impl<'a, 'b> fmt::Display for PrintBCSAT<'a, &'b Formula> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.print_formula(f, self.1)
    }
}
impl<'a, 'b, T> fmt::Display for PrintBCSAT<'a, &'b T>
    where for<'c> PrintBCSAT<'c, T>: fmt::Display,
          T: Clone
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.display(self.1.clone()).fmt(f)
    }
}
impl<'a, T> fmt::Display for PrintBCSAT<'a, Box<T>>
    where for<'b> PrintBCSAT<'b, &'b T>: fmt::Display
{
    fn fmt<'x>(&'x self, f: &mut fmt::Formatter) -> fmt::Result {
        let d: PrintBCSAT<'a, &'x T> = self.0.display(&self.1);
        d.fmt(f)
    }
}
use std::rc::Rc;
impl<'a, T> fmt::Display for PrintBCSAT<'a, Rc<T>>
    where for<'b> PrintBCSAT<'b, &'b T>: fmt::Display
{
    fn fmt<'x>(&'x self, f: &mut fmt::Formatter) -> fmt::Result {
        let d: PrintBCSAT<'a, &'x T> = self.0.display(&self.1);
        d.fmt(f)
    }
}
impl<'a, T> fmt::Display for PrintBCSAT<'a, Vec<T>>
    where for<'b, 'c> PrintBCSAT<'b, &'c T>: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = self.1.iter();
        if let Some(elem) = it.next() {
            try!(write!(f, "({}", self.0.display(elem)));
            for elem in it {
                try!(write!(f, ", {}", self.0.display(elem)));
            }
            write!(f, ")")
        } else {
            write!(f, "()")
        }
    }
}
impl<'a, 'b> fmt::Display for PrintBCSAT<'a, &'b Statement> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Statement::*;
        match *self.1 {
            Name(ref n) => write!(f, "{};", n),
            Defined(ref n, ref form) => write!(f, "{} := {};", n, self.0.display(form)),
            Assigned(ref form) => write!(f, "ASSIGN {};", self.0.display(form)),
            Comment(ref c) => write!(f, "//{}\n", c),
        }
    }
}
impl<'a, 'b> fmt::Display for PrintBCSAT<'a, &'b BCSAT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "BC1.1\n"));
        for statement in &self.1.statements {
            try!(write!(f, "{}\n", self.0.display(statement)));
        }
        Ok(())
    }
}
