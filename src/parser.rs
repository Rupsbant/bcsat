use std::str;
use nom;
use std::string::ToString;
use ast::*;

named!(not, alt!(tag!("!") | tag!("~")));
named!(infix_and, complete!(tag!("&")));
named!(infix_or, complete!(tag!("|")));
named!(infix_equiv, complete!(tag!("==")));
named!(infix_imply, complete!(tag!("=>")));
named!(infix_xor, complete!(tag!("^")));
named!(and, tag_no_case!("AND"));
named!(not_l, tag_no_case!("NOT"));
named!(or, tag_no_case!("OR"));
named!(odd, tag_no_case!("ODD"));
named!(even, tag_no_case!("EVEN"));
named!(equiv, tag_no_case!("EQUIV"));
named!(imply, tag_no_case!("IMPLY"));
named!(ite, tag_no_case!("ITE"));
named!(def, ws!(tag!(":=")));
named!(assign, tag_no_case!("ASSIGN"));
named!(comment_op, tag!("//"));
named!(pub header<Header>, do_parse!(
    tag_no_case!("BC") >>
    major: int >>
    tag!(".") >>
    minor: int >>
    tag!("\n") >>
    (Header::new(major, minor))
));

named!(constant<Constant>, alt!(
    do_parse!(alt!(tag_no_case!("true") | tag_no_case!("t")) >> (Constant::True)) |
    do_parse!(alt!(tag_no_case!("false") | tag_no_case!("f")) >> (Constant::False))
));
named!(brackets<(usize, usize)>,
    delimited!(
        tag!("["),
        separated_pair!(int, tag!(","), int),
        tag!("]")));
named!(pair<(F, F)>,
    do_parse!(
        tag!("(") >>
        x: f >>
        tag!(",") >>
        y: f >>
        tag!(")") >>
        (x,y)
    ));
named!(triple<(F, F, F)>,
    do_parse!(
        tag!("(") >>
        x: f >>
        tag!(",") >>
        y: f >>
        tag!(",") >>
        z: f >>
        tag!(")") >>
        (x,y,z)
    ));
named!(identifier<String>,
    map!(
    map_res!(
            take_while1!(is_identifier_character),
            str::from_utf8
    ),
    ToString::to_string));
named!(int<usize>,
        map_res!(
        map_res!(
                take_while1!(is_identifier_character),
                str::from_utf8
        ),
        str::parse::<usize>));
named!(comment<String>,
    map!(
    map_res!(
            preceded!(comment_op, nom::not_line_ending),
            str::from_utf8
    ),
    ToString::to_string));
named!(pub formula_list<Vec<F>>, delimited!( tag!("("), separated_list!(char!(','), ws!(f)), tag!(")")));
named!(f_0<Formula>, terminated!(
    alt_complete!(
        do_parse!(id: identifier >> (Formula::Named(id)))
        | ws!(delimited!( tag!("("), formula, tag!(")") ))
        | map!(constant, From::from)
    ), opt!(complete!(ws!(comment))))
);
named!(f_1<Formula>, alt!(
    do_parse!(not >> f: f_1 >> (Formula::Not(From::from(f))))
    | f_0
));
// Left associative parsing of conjunction, parse a & b & c to (a & b) & c
named!(f_2<Formula>, do_parse!(
    f1: f_1 >>
    end: fold_many0!(preceded!(ws!(infix_and), f_1), f1, |f1, f2| Formula::And(as_vec(&[&f1, &f2]))) >>
    (end)
));
named!(f_3<Formula>,do_parse!(
    f1: f_2 >>
    end: fold_many0!(preceded!(ws!(infix_or), f_2), f1, |f1, f2| Formula::Or(as_vec(&[&f1, &f2]))) >>
    (end)
));
named!(f_4<Formula>, do_parse!(
    f1: f_3 >>
    end: fold_many0!(preceded!(ws!(infix_xor), f_3), f1, |f1, f2| Formula::Odd(as_vec(&[&f1, &f2]))) >>
    (end)
));
named!(f_5<Formula>, do_parse!(
    f1: f_4 >>
    end: fold_many0!(preceded!(ws!(infix_equiv), f_4), f1, |f1, f2| Formula::Equiv(as_vec(&[&f1, &f2]))) >>
    (end)
));
// Right associative parsing of implication.
// a => b => c is equal to a => (b =>c)
named!(f_6<Formula>, do_parse!(
    f1: f_5 >>
    other: many0!(preceded!(ws!(infix_imply), f_5)) >>
    ({let r = other.into_iter().rev();
    r.fold(f1, |deep, next| Formula::Imply(From::from(next), From::from(deep)))})
));
// Either TLV parsing  (Tag-Length-Value) of formulas or prefix parsing with precedence.
named!(f_7<Formula>, alt!(
    do_parse!(ws!(not_l) >> f: f >> (Formula::Not(f)))
    | do_parse!(ws!(and) >> f_vec: formula_list >> (Formula::And(f_vec)))
    | do_parse!(ws!(or)  >> f_vec: formula_list >> (Formula::Or(f_vec)))
    | do_parse!(ws!(odd) >> f_vec: formula_list >> (Formula::Odd(f_vec)))
    | do_parse!(ws!(even) >> f_vec: formula_list >> (Formula::Even(f_vec)))
    | do_parse!(ws!(equiv) >> f_vec: formula_list >> (Formula::Equiv(f_vec)))
    | do_parse!(ws!(imply) >> p: pair >> (Formula::Imply(p.0, p.1)))
    | do_parse!(ws!(ite) >> tr: triple >> (Formula::ITE(tr.0, tr.1, tr.2)))
    | do_parse!( lu: brackets >> f_vec : formula_list >> (Formula::Between(lu.0, lu.1, f_vec)))
    | f_6
));
named!(pub formula<Formula>,  ws!(f_7));
named!(f<F>, map!(formula, From::from));

named!(statement<Statement>,alt!(
    do_parse!(id: identifier >> tag!(";") >> (Statement::Name(id)) )
    | do_parse!(id: ws!(identifier) >> def >> f: ws!(f) >> tag!(";") >> (Statement::Defined(id, f)))
    | do_parse!(assign >> f: ws!(f) >> tag!(";") >> (Statement::Assigned(f)))
    | do_parse!(c: comment >> (Statement::Comment(c)))
));
named!(circuit<Vec<Statement> >, many1!(ws!(statement)));
named!(pub bcsat<BCSAT>, do_parse!(
    h: header >>
    c: circuit >> eof!() >>
    (BCSAT{header: h, statements: c})
));

fn is_identifier_character(chr:u8) -> bool {
    nom::is_alphanumeric(chr) || chr == '_' as u8
}

fn as_vec<'a, T: ?Sized>(x: &'a [&T]) -> Vec<F> where Formula: From<&'a T> {
    x.iter()
     .map(|x| From::from(Formula::from(x)))
     .collect::<Vec<F>>()
}

#[cfg(test)]
mod tests {
use super::*;
#[test]
fn basic() {
    use nom::IResult;
    assert_eq!(header(b"BC1.1\n def"), IResult::Done(&b" def"[..], Header::new(1,1)));
    assert_eq!(identifier(b"_abc_1 def"), IResult::Done(&b" def"[..], "_abc_1".to_string()));
    let circuit = bcsat(BC_EXAMPLE).unwrap().1;
    assert_eq!(circuit.statements.len(), 4);
    let parse_list = formula_list(LIST_EXAMPLE).unwrap();
    assert_eq!(parse_list.1, as_vec(&["x", "y"]));
    let xny = Formula::And(as_vec(&["x", "y"]));
    let parse_and = formula(AND_INFIX).unwrap();
    assert_eq!(parse_and.1, xny.clone());
    let parse_and = formula(AND_EXAMPLE).unwrap();
    assert_eq!(parse_and.1, xny);
}
#[test]
fn precedence() {
    let xny = Formula::And(as_vec(&["x", "y"]));
    let bz = Box::new(Formula::Named("z".to_string()));
    let parse_prec = formula(PRECEDENCE_1).unwrap();
    assert_eq!(parse_prec.1, Formula::Or(vec![Box::new(xny.clone()), bz.clone()]));
    let parse_prec = formula(PRECEDENCE_2).unwrap();
    assert_eq!(parse_prec.1, Formula::Or(vec![bz.clone(), Box::new(xny.clone())]));
    let parse_comment = formula(COMMENT).unwrap();
    assert_eq!(parse_comment.1, Formula::Or(vec![Box::new(xny.clone()), bz.clone()]));
}
static BC_EXAMPLE: &'static [u8] = b"BC1.1
a;
b;
// comment
c := a;";
static LIST_EXAMPLE: &'static [u8] = b"( x, y)";
static AND_EXAMPLE: &'static [u8] = b"AnD(x, y)";
static AND_INFIX: &'static [u8] = b"x & y";
static PRECEDENCE_1: &'static [u8] = b"x & y | z";
static PRECEDENCE_2: &'static [u8] = b"z | x & y";
static COMMENT: &'static [u8] = b"x & y //conjunction
| z // disjunction";
}
