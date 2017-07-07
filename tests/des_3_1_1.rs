extern crate bcsat;
use bcsat::parser::bcsat;
use bcsat::printer::*;
#[test]
fn parse_des_3_1_1() {
    let f = include_bytes!("des_3_1_1.bc");
    let parsed = bcsat(f);
    let mut parsed = parsed.unwrap().1;
    let mut opt = FormulaOptions::default();
    opt.odd = InfixChainOptions::NoChain(InfixOptions::Long);
    let printed = format!("{}", opt.display(&parsed));
    let p2 = bcsat(printed.as_bytes());
    let p2 = p2.unwrap().1;
    assert_eq!(parsed.statements, p2.statements);
}
