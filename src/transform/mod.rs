use ast::*;

pub mod join_assoc;

impl BCSAT {
    pub fn transform<F>(&mut self, f: &F)
        where F: Fn(&Formula) -> Formula
    {
        for stm in self.statements.iter_mut() {
            *stm = stm.transform(f)
        }
    }
    pub fn rebuild<F>(&mut self, f: &F) -> Self
        where F: Fn(&Formula) -> Formula
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
    pub fn transform<F>(&self, f: &F) -> Statement
        where F: Fn(&Formula) -> Formula
    {
        use self::Statement::*;
        let mut stm = self.clone();
        match stm {
            Assigned(ref mut form) |
            Defined(_, ref mut form) => *form = Rc::from(f(form)),
            _ => (),
        };
        stm
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
}
