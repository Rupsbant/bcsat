extern crate bcsat;
use bcsat::parser::bcsat;
#[test]
fn parse_des_3_1_1() {
    let f = include_bytes!("des_3_1_1.bc");
    let parsed = bcsat(f);
    assert_eq!(parsed.unwrap().1.statements.len(), 1369);
}
