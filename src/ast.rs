pub type F = Box<Formula>;
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Header {
    major: usize,
    minor: usize,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Constant {
    True,
    False,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Formula {
    Comment(F, String),
    Equiv(Vec<F>),
    Imply(Vec<F>),
    Or(Vec<F>),
    And(Vec<F>),
    Odd(Vec<F>),
    Even(Vec<F>),
    ITE(F, F, F),
    Between(usize, usize, Vec<F>),
    Not(F),
    Named(String),
    Constant(Constant),
}
impl<'a> From<&'a str> for Formula {
    fn from(f: &'a str) -> Self {
        Formula::from(f.to_string())
    }
}
impl From<String> for Formula {
    fn from(f: String) -> Self {
        Formula::Named(f)
    }
}
impl From<Constant> for Formula {
    fn from(c: Constant) -> Self {
        Formula::Constant(c)
    }
}
impl<'a> From<&'a Formula> for Formula {
    fn from(f: &'a Formula) -> Self {
        f.clone()
    }
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    Name(String),
    Defined(String, F),
    Assigned(F),
    Comment(String),
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BCSAT {
    pub header: Header,
    pub statements: Vec<Statement>,
}

impl Header {
    pub fn new(major: usize, minor: usize) -> Header {
        Header {
            major: major,
            minor: minor,
        }
    }
}
