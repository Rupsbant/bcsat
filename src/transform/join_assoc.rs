use ast::*;
use printer::operators::*;
use super::*;

pub fn join_assoc_box(f: &Formula) -> F {
    From::from(join_full_assoc(f))
}
pub fn join_full_assoc(f: &Formula) -> Formula {
    use self::Formula::*;
    match *f {
        Odd(ref v) => accumulate(Op::Odd, v),
        Even(ref v) => accumulate(Op::Even, v),
        And(ref v) => accumulate(Op::And, v),
        Or(ref v) => accumulate(Op::Or, v),
        Comment(ref form, ref com) => Comment(join_assoc_box(form), com.clone()),
        Equiv(ref v) => Equiv(v.iter().map(|e| join_assoc_box(e)).collect::<Vec<_>>()),
        Imply(ref l, ref r) => Imply(join_assoc_box(&l), join_assoc_box(&r)),
        ITE(ref i, ref t, ref e) => ITE(join_assoc_box(&i), join_assoc_box(&t), join_assoc_box(&e)),
        Not(ref n) => Not(join_assoc_box(&n)),
        Between(ref l, ref u, ref v) => {
            Between(l.clone(),
                    u.clone(),
                    v.iter().map(|e| join_assoc_box(e)).collect::<Vec<_>>())
        }
        ref f => f.clone(),
    }
}
fn accumulate(mut op: Op, check: &Vec<F>) -> Formula {
    use self::Formula::*;
    let mut rejected = vec![];
    let mut check = check.clone();
    while let Some(f) = check.pop() {
        let f = (*f).clone();
        match (op, f) {
            (Op::Or, Or(v)) |
            (Op::And, And(v)) |
            (Op::Equiv, Equiv(v)) |
            (Op::Even, Odd(v)) |
            (Op::Odd, Odd(v)) => check.extend(v.into_iter()),
            (Op::Odd, Even(v)) => {
                op = Op::Even;
                check.extend(v.into_iter())
            }
            (Op::Even, Even(v)) => {
                op = Op::Odd;
                check.extend(v.into_iter())
            }
            (_, f) => rejected.push(join_assoc_box(&f)),
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
