use ast::*;
use printer::operators::*;
use super::*;


pub fn join_full_assoc(f: Formula) -> Formula {
    use self::Formula::*;
    let red_assoc = &join_full_assoc;
    match f {
        Odd(v) => accumulate(Op::Odd, v),
        Even(v) => accumulate(Op::Even, v),
        And(v) => accumulate(Op::And, v),
        Or(v) => accumulate(Op::Or, v),
        Comment(form, com) => Comment(form.tb(red_assoc), com.clone()),
        Equiv(mut v) => {
            for e in v.iter_mut() {
                transform_box(e, red_assoc);
            }
            Equiv(v)
        }
        Imply(l, r) => Imply(l.tb(red_assoc), r.tb(red_assoc)),
        ITE(i, t, e) => ITE(i.tb(red_assoc), t.tb(red_assoc), e.tb(red_assoc)),
        Not(n) => Not(n.tb(red_assoc)),
        Between(l, u, mut v) => {
            for e in v.iter_mut() {
                transform_box(e, red_assoc);
            }
            Between(l, u, v)
        }
        f => f.clone(),
    }
}
pub fn accumulate(mut op: Op, mut check: Vec<F>) -> Formula {
    use self::Formula::*;
    let mut rejected = vec![];
    while let Some(f) = check.pop() {
        match (op, f) {
            (Op::Or, box Or(v)) |
            (Op::And, box And(v)) |
            (Op::Equiv, box Equiv(v)) |
            (Op::Even, box Odd(v)) |
            (Op::Odd, box Odd(v)) => check.extend(v.into_iter()),
            (Op::Odd, box Even(v)) => {
                op = Op::Even;
                check.extend(v.into_iter())
            }
            (Op::Even, box Even(v)) => {
                op = Op::Odd;
                check.extend(v.into_iter())
            }
            (_, f) => rejected.push(f.tb(&join_full_assoc)),
        }
    }
    rejected.reverse();
    op.build(rejected)
}

impl Op {
    fn build(&self, with: Vec<F>) -> Formula {
        use self::Op::*;
        match *self {
            And => Formula::And(with),
            Or => Formula::Or(with),
            Not => {
                assert!(with.len() == 1);
                let n = with.into_iter().next().unwrap();
                Formula::Not(n)
            }
            Imply => {
                assert!(with.len() == 2);
                let mut it = with.into_iter();
                let l = it.next().unwrap();
                let r = it.next().unwrap();
                Formula::Imply(l, r)
            }
            Equiv => Formula::Equiv(with),
            Even => Formula::Even(with),
            Odd => Formula::Odd(with),
            ITE => {
                assert!(with.len() == 3);
                let mut it = with.into_iter();
                let i = it.next().unwrap();
                let t = it.next().unwrap();
                let e = it.next().unwrap();
                Formula::ITE(i, t, e)
            }
            Between(l, u) => Formula::Between(l, u, with),
        }
    }
}
