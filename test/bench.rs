#![cfg(unstable)]
#![feature(test)]

extern crate test;

#[bench]
fn bench_is_some_optbool(bench: &mut test::Bencher) {	
	fn is_some_optbool() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}

	bench.iter(is_some_optbool);
}

#[bench]
fn bench_is_some_stdopt(bench: &mut test::Bencher) {
	fn is_some_stdopt() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}

	bench.iter(is_some_stdopt);
}

