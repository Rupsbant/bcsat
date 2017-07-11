use ast::*;

pub mod join_assoc;

impl BCSAT {
    pub fn transform<F>(&mut self, f: &F)
        where F: Fn(Formula) -> Formula
    {
        use self::Statement::*;
        for stm in self.statements.iter_mut() {
            match *stm {
                Assigned(ref mut form) |
                Defined(_, ref mut form) => transform_box(form, f),
                _ => (),
            }
        }
    }
}
impl Formula {
    pub fn transform<F>(&mut self, f: &F)
        where F: Fn(Formula) -> Formula
    {
        let form = ::std::mem::replace(self, Formula::Constant(Constant::False));
        let form = f(form);
        ::std::mem::replace(self, form);
    }
    pub fn t<F>(self, f: &F) -> Self
        where F: Fn(Formula) -> Formula
    {
        f(self)
    }
    pub fn tb<F>(mut self: Box<Self>, f: &F) -> Box<Self>
        where F: Fn(Formula) -> Formula
    {
        self.transform(f);
        self
    }
}

pub fn transform_box<F>(b: &mut Box<Formula>, f: &F)
    where F: Fn(Formula) -> Formula
{
    b.transform(f)
}
