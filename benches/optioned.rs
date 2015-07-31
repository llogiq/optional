#![feature(test)]


extern crate test;
extern crate optional;

#[bench]
fn bench_iter_opt_u8(bench: &mut test::Bencher) {
	fn iter_opt_u8() {
		for o in [optional::some(1u8), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_u8);
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
	fn iter_std_u8() {
		for o in [Option::Some(1u8), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_u8);
}

#[bench]
fn bench_iter_opt_u16(bench: &mut test::Bencher) {
	fn iter_opt_u16() {
		for o in [optional::some(1u16), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_u16);
}

#[bench]
fn bench_as_slice_iter_opt_u16(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_u16() {
		for o in [optional::some(1u16), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_u16);
}

#[bench]
fn bench_iter_std_u16(bench: &mut test::Bencher) {
	fn iter_std_u16() {
		for o in [Option::Some(1u16), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_u16);
}

#[bench]
fn bench_iter_opt_u32(bench: &mut test::Bencher) {
	fn iter_opt_u32() {
		for o in [optional::some(1u32), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_u32);
}

#[bench]
fn bench_as_slice_iter_opt_u32(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_u32() {
		for o in [optional::some(1u32), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_u32);
}

#[bench]
fn bench_iter_std_u32(bench: &mut test::Bencher) {
	fn iter_std_u32() {
		for o in [Option::Some(1u32), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_u32);
}

#[bench]
fn bench_iter_opt_u64(bench: &mut test::Bencher) {
	fn iter_opt_u64() {
		for o in [optional::some(1u64), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_u64);
}

#[bench]
fn bench_as_slice_iter_opt_u64(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_u64() {
		for o in [optional::some(1u64), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_u64);
}

#[bench]
fn bench_iter_std_u64(bench: &mut test::Bencher) {
	fn iter_std_u64() {
		for o in [Option::Some(1u64), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_u64);
}

#[bench]
fn bench_iter_opt_i8(bench: &mut test::Bencher) {
	fn iter_opt_i8() {
		for o in [optional::some(1i8), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_i8);
}

#[bench]
fn bench_as_slice_iter_opt_i8(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_i8() {
		for o in [optional::some(1i8), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_i8);
}

#[bench]
fn bench_iter_std_i8(bench: &mut test::Bencher) {
	fn iter_std_i8() {
		for o in [Option::Some(1i8), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_i8);
}

#[bench]
fn bench_iter_opt_i16(bench: &mut test::Bencher) {
	fn iter_opt_i16() {
		for o in [optional::some(1i16), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_i16);
}

#[bench]
fn bench_as_slice_iter_opt_i16(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_i16() {
		for o in [optional::some(1i16), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_i16);
}

#[bench]
fn bench_iter_std_i16(bench: &mut test::Bencher) {
	fn iter_std_i16() {
		for o in [Option::Some(1i16), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_i16);
}

#[bench]
fn bench_iter_opt_i32(bench: &mut test::Bencher) {
	fn iter_opt_i32() {
		for o in [optional::some(1i32), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_i32);
}

#[bench]
fn bench_as_slice_iter_opt_i32(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_i32() {
		for o in [optional::some(1i32), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_i32);
}

#[bench]
fn bench_iter_std_i32(bench: &mut test::Bencher) {
	fn iter_std_i32() {
		for o in [Option::Some(1i32), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_i32);
}

#[bench]
fn bench_iter_opt_i64(bench: &mut test::Bencher) {
	fn iter_opt_i64() {
		for o in [optional::some(1i64), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_i64);
}

#[bench]
fn bench_as_slice_iter_opt_i64(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_i64() {
		for o in [optional::some(1i64), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_i64);
}

#[bench]
fn bench_iter_std_i64(bench: &mut test::Bencher) {
	fn iter_std_i64() {
		for o in [Option::Some(1i64), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_i64);
}

#[bench]
fn bench_iter_opt_f32(bench: &mut test::Bencher) {
	fn iter_opt_f32() {
		for o in [optional::some(1f32), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_f32);
}

#[bench]
fn bench_as_slice_iter_opt_f32(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_f32() {
		for o in [optional::some(1f32), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_f32);
}

#[bench]
fn bench_iter_std_f32(bench: &mut test::Bencher) {
	fn iter_std_f32() {
		for o in [Option::Some(1f32), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_f32);
}

#[bench]
fn bench_iter_opt_f64(bench: &mut test::Bencher) {
	fn iter_opt_f64() {
		for o in [optional::some(1f64), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_opt_f64);
}

#[bench]
fn bench_as_slice_iter_opt_f64(bench: &mut test::Bencher) {
	fn as_slice_iter_opt_f64() {
		for o in [optional::some(1f64), optional::none()]
				.iter().cycle().take(1200) {
			for b in o.as_slice().iter() { test::black_box(b); }
		}
	}
	
	bench.iter(as_slice_iter_opt_f64);
}

#[bench]
fn bench_iter_std_f64(bench: &mut test::Bencher) {
	fn iter_std_f64() {
		for o in [Option::Some(1f64), Option::None]
				.iter().cycle().take(1200) {
			for b in o.iter() { test::black_box(b); }
		}
	}
	
	bench.iter(iter_std_f64);
}

#[bench]
fn bench_is_some_opt_u8(bench: &mut test::Bencher) {
	fn is_some_std_u8() {
		for o in [optional::some(1u8), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_u8);
}

#[bench]
fn bench_is_some_std_u8(bench: &mut test::Bencher) {
	fn is_some_std_u8() {
		for o in [Option::Some(1u8), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_u8);
}

#[bench]
fn bench_is_some_opt_u16(bench: &mut test::Bencher) {
	fn is_some_std_u16() {
		for o in [optional::some(1u16), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_u16);
}

#[bench]
fn bench_is_some_std_u16(bench: &mut test::Bencher) {
	fn is_some_std_u16() {
		for o in [Option::Some(1u16), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_u16);
}

#[bench]
fn bench_is_some_opt_u32(bench: &mut test::Bencher) {
	fn is_some_std_u32() {
		for o in [optional::some(1u32), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_u32);
}

#[bench]
fn bench_is_some_std_u32(bench: &mut test::Bencher) {
	fn is_some_std_u32() {
		for o in [Option::Some(1u32), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_u32);
}

#[bench]
fn bench_is_some_opt_u64(bench: &mut test::Bencher) {
	fn is_some_std_u64() {
		for o in [optional::some(1u64), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_u64);
}

#[bench]
fn bench_is_some_std_u64(bench: &mut test::Bencher) {
	fn is_some_std_u64() {
		for o in [Option::Some(1u64), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_u64);
}

#[bench]
fn bench_is_some_opt_f32(bench: &mut test::Bencher) {
	fn is_some_std_f32() {
		for o in [optional::some(1f32), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_f32);
}

#[bench]
fn bench_is_some_std_f32(bench: &mut test::Bencher) {
	fn is_some_std_f32() {
		for o in [Option::Some(1f32), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_f32);
}

#[bench]
fn bench_is_some_opt_f64(bench: &mut test::Bencher) {
	fn is_some_std_f64() {
		for o in [optional::some(1f64), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_f64);
}

#[bench]
fn bench_is_some_std_f64(bench: &mut test::Bencher) {
	fn is_some_std_f64() {
		for o in [Option::Some(1f64), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.is_some());
		}
	}
	
	bench.iter(is_some_std_f64);
}

#[bench]
fn bench_map_opt_u8(bench: &mut test::Bencher) {
	fn map_std_u8() {
		for o in [optional::some(1u8), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1));
		}
	}
	
	bench.iter(map_std_u8);
}

#[bench]
fn bench_map_std_u8(bench: &mut test::Bencher) {
	fn map_std_u8() {
		for o in [Option::Some(1u8), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1));
		}
	}
	
	bench.iter(map_std_u8);
}

#[bench]
fn bench_map_opt_u16(bench: &mut test::Bencher) {
	fn map_std_u16() {
		for o in [optional::some(1u16), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1));
		}
	}
	
	bench.iter(map_std_u16);
}

#[bench]
fn bench_map_std_u16(bench: &mut test::Bencher) {
	fn map_std_u16() {
		for o in [Option::Some(1u16), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1));
		}
	}
	
	bench.iter(map_std_u16);
}

#[bench]
fn bench_map_opt_u32(bench: &mut test::Bencher) {
	fn map_std_u32() {
		for o in [optional::some(1u32), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1));
		}
	}
	
	bench.iter(map_std_u32);
}


#[bench]
fn bench_map_std_u32(bench: &mut test::Bencher) {
	fn map_std_u32() {
		for o in [Option::Some(1u32), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1));
		}
	}
	
	bench.iter(map_std_u32);
}

#[bench]
fn bench_map_opt_u64(bench: &mut test::Bencher) {
	fn map_std_u64() {
		for o in [optional::some(1u64), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1));
		}
	}
	
	bench.iter(map_std_u64);
}

#[bench]
fn bench_map_std_u64(bench: &mut test::Bencher) {
	fn map_std_u64() {
		for o in [Option::Some(1u64), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1));
		}
	}
	
	bench.iter(map_std_u64);
}

#[bench]
fn bench_map_opt_f32(bench: &mut test::Bencher) {
	fn map_std_f32() {
		for o in [optional::some(1f32), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1.0));
		}
	}
	
	bench.iter(map_std_f32);
}

#[bench]
fn bench_map_std_f32(bench: &mut test::Bencher) {
	fn map_std_f32() {
		for o in [Option::Some(1f32), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1.0));
		}
	}
	
	bench.iter(map_std_f32);
}

#[bench]
fn bench_map_opt_f64(bench: &mut test::Bencher) {
	fn map_std_f64() {
		for o in [optional::some(1f64), optional::none()]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1.0));
		}
	}
	
	bench.iter(map_std_f64);
}

#[bench]
fn bench_map_std_f64(bench: &mut test::Bencher) {
	fn map_std_f64() {
		for o in [Option::Some(1f64), Option::None]
				.iter().cycle().take(1200) {
			test::black_box(o.map(|x| x + 1.0));
		}
	}
	
	bench.iter(map_std_f64);
}
