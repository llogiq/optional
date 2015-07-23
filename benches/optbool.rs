#![feature(test, as_slice)]

extern crate test;
extern crate optional;

use optional::OptionBool;

#[bench]
fn bench_is_some_opt(bench: &mut test::Bencher) {
	fn is_some_optbool() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}

	bench.iter(is_some_optbool);
}

#[bench]
fn bench_is_some_std(bench: &mut test::Bencher) {
	fn is_some_stdopt() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}

	bench.iter(is_some_stdopt);
}

fn invert(b: bool) -> bool { !b }

#[bench]
fn bench_map_invert_opt_bool(bench: &mut test::Bencher) {
	fn map_invert_opt_bool() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map_bool(invert));
		}
	}
	
	bench.iter(map_invert_opt_bool);
}

#[bench]
fn bench_map_invert_opt(bench: &mut test::Bencher) {
	fn map_invert_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map(invert));
		}
	}
	
	bench.iter(map_invert_opt);
}

#[bench]
fn bench_map_invert_std(bench: &mut test::Bencher) {
	fn map_invert_std() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map(invert));
		}
	}
	
	bench.iter(map_invert_std);
}


#[bench]
fn bench_unwrap_opt(bench: &mut test::Bencher) {
	fn unwrap_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse]
				.iter().cycle().take(1200) {
			test::black_box(o.unwrap());
		}
	}

	bench.iter(unwrap_opt);
}

#[bench]
fn bench_unwrap_std(bench: &mut test::Bencher) {
	fn unwrap_std() {
		for o in [Option::Some(true), Option::Some(false)]
				.iter().cycle().take(1200) {
			test::black_box(o.unwrap());
		}
	}

	bench.iter(unwrap_std);
}

#[bench]
fn bench_unwrap_or_opt(bench: &mut test::Bencher) {
	fn unwrap_or_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.unwrap_or(false));
		}
	}
	
	bench.iter(unwrap_or_opt);
}

#[bench]
fn bench_unwrap_or_std(bench: &mut test::Bencher) {
	fn unwrap_or_std() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.unwrap_or(false));
		}
	}
	
	bench.iter(unwrap_or_std);
}

fn oracle() -> bool { true }

#[bench]
fn bench_unwrap_or_else_opt(bench: &mut test::Bencher) {
	fn unwrap_or_else_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.unwrap_or_else(oracle));
		}
	}
	
	bench.iter(unwrap_or_else_opt);
}

#[bench]
fn bench_unwrap_or_else_std(bench: &mut test::Bencher) {
	fn unwrap_or_else_std() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.unwrap_or_else(oracle));
		}
	}
	
	bench.iter(unwrap_or_else_std);
}

#[bench]
fn bench_map_or_opt(bench: &mut test::Bencher) {
	fn map_or_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map_or(false, invert));
		}
	}
	
	bench.iter(map_or_opt);
}

#[bench]
fn bench_map_or_std(bench: &mut test::Bencher) {
	fn map_or_std() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map_or(false, invert));
		}
	}
	
	bench.iter(map_or_std);
}

#[bench]
fn bench_map_or_else_opt(bench: &mut test::Bencher) {
	fn map_or_else_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map_or_else(oracle, invert));
		}
	}
	
	bench.iter(map_or_else_opt);
}

#[bench]
fn bench_map_or_else_std(bench: &mut test::Bencher) {
	fn map_or_else_std() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map_or_else(oracle, invert));
		}
	}
	
	bench.iter(map_or_else_std);
}

#[bench]
fn bench_and_opt(bench: &mut test::Bencher) {
	fn and_opt() {
		for o1 in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(33) {
			for o2 in [Option::Some(true), Option::Some(false), Option::None]
					.iter().cycle().take(33) {
				test::black_box(o1.and(*o2));
			}
		}
	}
	
	bench.iter(and_opt);
}

#[bench]
fn bench_and_bool_opt(bench: &mut test::Bencher) {
	fn and_bool_opt() {
		for o1 in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(33) {
			for o2 in [OptionBool::SomeTrue, OptionBool::SomeFalse, 
					OptionBool::None].iter().cycle().take(33) {
				test::black_box(o1.and_bool(*o2));
			}
		}
	}
	
	bench.iter(and_bool_opt);
}

#[bench]
fn bench_and_std(bench: &mut test::Bencher) {
	fn and_std() {
		for o1 in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(33) {
			for o2 in [Option::Some(true), Option::Some(false), Option::None]
					.iter().cycle().take(33) {
				test::black_box(o1.and(*o2));
			}
		}
	}
	
	bench.iter(and_std);
}

#[bench]
fn bench_or_opt(bench: &mut test::Bencher) {
	fn or_opt() {
		for o1 in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(33) {
			for o2 in [Option::Some(true), Option::Some(false), Option::None]
					.iter().cycle().take(33) {
				test::black_box(o1.or(*o2));
			}
		}
	}
	
	bench.iter(or_opt);
}

#[bench]
fn bench_or_bool_opt(bench: &mut test::Bencher) {
	fn or_bool_opt() {
		for o1 in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(33) {
			for o2 in [OptionBool::SomeTrue, OptionBool::SomeFalse, 
					OptionBool::None].iter().cycle().take(33) {
				test::black_box(o1.or_bool(*o2));
			}
		}
	}
	
	bench.iter(or_bool_opt);
}

#[bench]
fn bench_or_std(bench: &mut test::Bencher) {
	fn or_std() {
		for o1 in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(33) {
			for o2 in [Option::Some(true), Option::Some(false), Option::None]
					.iter().cycle().take(33) {
				test::black_box(o1.or(*o2));
			}
		}
	}
	
	bench.iter(or_std);
}

fn create_std() -> Option<bool> {
	Some(false)
}

fn create_opt() -> OptionBool {
	OptionBool::SomeFalse
}

#[bench]
fn bench_or_else_opt(bench: &mut test::Bencher) {
	fn or_else_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.or_else(create_std));
		}
	}
	
	bench.iter(or_else_opt);
}

#[bench]
fn bench_or_else_bool_opt(bench: &mut test::Bencher) {
	fn or_else_bool_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.or_else_bool(create_opt));
		}
	}
	
	bench.iter(or_else_bool_opt);
}

#[bench]
fn bench_or_else_std(bench: &mut test::Bencher) {
	fn or_else_std() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.or_else(create_std));
		}
	}
	
	bench.iter(or_else_std);
}

#[bench]
fn bench_iter_opt(bench: &mut test::Bencher) {
	fn iter_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt);
}

#[bench]
fn bench_as_slice_iter_opt(bench: &mut test::Bencher) {
	fn as_slice_iter_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt);
}

#[bench]
fn bench_iter_std(bench: &mut test::Bencher) {
	fn iter_std() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std);
}

#[bench]
fn bench_as_slice_opt(bench: &mut test::Bencher) {
	fn as_slice_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			test::black_box(o.as_slice());
		}
	}
	
	bench.iter(as_slice_opt);
}

#[bench]
fn bench_as_slice_std(bench: &mut test::Bencher) {
	fn or_else_std() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.as_slice());
		}
	}
	
	bench.iter(or_else_std);
}


#[bench]
fn bench_from_opt(bench: &mut test::Bencher) {
	fn from_opt() {
		for o in [Option::Some(true), Option::Some(false), Option::None]
				.iter().cycle().take(1200) {
			let o2 : OptionBool = o.into();
			test::black_box(o2);
		}
	}
	
	bench.iter(from_opt);
}

#[bench]
fn bench_to_opt(bench: &mut test::Bencher) {
	fn to_opt() {
		for o in [OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None]
				.iter().cycle().take(1200) {
			let o2 : Option<bool> = o.into();
			test::black_box(o2);
		}
	}
	
	bench.iter(to_opt);
}
