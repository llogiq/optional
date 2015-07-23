#![feature(test)]


extern crate test;
extern crate optional;

#[bench]
fn bench_iter_opt_u8(bench: &mut test::Bencher) {
	fn or_else_bool_opt() {
		for o in [optional::some(1u8), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(or_else_bool_opt);
}

#[bench]
fn bench_as_slice_iter_opt_u8(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_u8() {
		for o in [optional::some(1u8), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_u8);
}

#[bench]
fn bench_iter_std_u8(bench: &mut test::Bencher) {
	fn or_else_std() {
		for o in [Option::Some(1u8), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(or_else_std);
}
