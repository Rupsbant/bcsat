use ast::*;

pub mod join_assoc;

pub trait FormulaTransform {
    fn exec(&mut self, f: &Formula) -> Formula;
    fn eb(&mut self, f: &Formula) -> F {
        From::from(self.exec(f))
    }
    fn ev(&mut self, f: &Vec<F>) -> Vec<F> {
        f.iter().map(|f| self.eb(f)).collect::<Vec<F>>()
    }
}
impl<F> FormulaTransform for F where
    F: FnMut(&Formula) -> Formula {
    fn exec(&mut self, f: &Formula) -> Formula {
        self(f)
    }
}

impl BCSAT {
    pub fn transform<F>(&mut self, f: &mut F)
        where F: FormulaTransform
    {
        for stm in self.statements.iter_mut() {
            *stm = stm.transform(f)
        }
    }
    pub fn rebuild<F>(&self, f: &mut F) -> Self
        where F: FormulaTransform
    {
        let statements = self.statements
            .iter()
            .map(|stm| {
                     stm.transform(f)
                 })
            .collect::<Vec<_>>();
        BCSAT {
            header: self.header.clone(),
            statements: statements,
        }

    }
}
impl Statement {
    pub fn transform<F>(&self, f: &mut F) -> Statement
        where F: FormulaTransform
    {
        use self::Statement::*;
        let mut stm = self.clone();
        match stm {
            Assigned(ref mut form) |
            Defined(_, ref mut form) => *form = Rc::from(f.exec(form)),
            _ => (),
        };
        stm
    }
}
impl Formula {
    pub fn transform<F>(&mut self, f: &mut F)
        where F: FormulaTransform
    {
        let form = ::std::mem::replace(self, Formula::Constant(Constant::False));
        let form = f.exec(&form);
        ::std::mem::replace(self, form);
    }
    pub fn t<F>(self, f: &F) -> Self
        where F: Fn(Formula) -> Formula
    {
        f(self)
    }
}
