use std::collections::HashSet;
use make_up_binary::{display_expr, Maker};

fn main() {
	let mut maker: Maker<u8, u8> = Maker::new(HashSet::from([
		0b01010101,
		0b00110011,
		0b00001111,
	]));
	let target = 0b00000101;
	if !maker.check(target) {
		for num in maker.check_detail(target) {
			println!("{:b}", num);
		}
		return;
	}
	// maker.test_or(target);
	// return;
	let expr = maker.get_expr(target);
	println!("{:#?}", maker.make(target));
	println!("{}", display_expr(&expr));
}
