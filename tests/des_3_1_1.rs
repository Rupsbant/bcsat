extern crate bcsat;
use bcsat::parser::bcsat;
use bcsat::printer::FormulaOptions;
#[test]
fn parse_des_3_1_1() {
    let f = include_bytes!("des_3_1_1.bc");
    let parsed = bcsat(f);
    let parsed = parsed.unwrap().1;
    println!("Parsed");
    println!("{:?}", parsed);
    println!("{}", FormulaOptions::default().display(&parsed));
    assert_eq!(parsed.statements.len(), 1369);

}
