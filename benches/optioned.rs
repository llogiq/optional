#[macro_use]
extern crate bencher;
extern crate optional;

fn bench_iter_opt_u8(bench: &mut bencher::Bencher) {
    fn iter_opt_u8() {
        for o in [optional::some(1u8), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_u8);
}

fn bench_as_slice_iter_opt_u8(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_u8() {
        for o in [optional::some(1u8), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_u8);
}

fn bench_iter_std_u8(bench: &mut bencher::Bencher) {
    fn iter_std_u8() {
        for o in [Option::Some(1u8), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_u8);
}

fn bench_iter_opt_u16(bench: &mut bencher::Bencher) {
    fn iter_opt_u16() {
        for o in [optional::some(1u16), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_u16);
}

fn bench_as_slice_iter_opt_u16(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_u16() {
        for o in [optional::some(1u16), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_u16);
}

fn bench_iter_std_u16(bench: &mut bencher::Bencher) {
    fn iter_std_u16() {
        for o in [Option::Some(1u16), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_u16);
}

fn bench_iter_opt_u32(bench: &mut bencher::Bencher) {
    fn iter_opt_u32() {
        for o in [optional::some(1u32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_u32);
}

fn bench_as_slice_iter_opt_u32(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_u32() {
        for o in [optional::some(1u32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_u32);
}

fn bench_iter_std_u32(bench: &mut bencher::Bencher) {
    fn iter_std_u32() {
        for o in [Option::Some(1u32), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_u32);
}

fn bench_iter_opt_u64(bench: &mut bencher::Bencher) {
    fn iter_opt_u64() {
        for o in [optional::some(1u64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_u64);
}

fn bench_as_slice_iter_opt_u64(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_u64() {
        for o in [optional::some(1u64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_u64);
}

fn bench_iter_std_u64(bench: &mut bencher::Bencher) {
    fn iter_std_u64() {
        for o in [Option::Some(1u64), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_u64);
}

fn bench_iter_opt_i8(bench: &mut bencher::Bencher) {
    fn iter_opt_i8() {
        for o in [optional::some(1i8), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_i8);
}

fn bench_as_slice_iter_opt_i8(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_i8() {
        for o in [optional::some(1i8), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_i8);
}

fn bench_iter_std_i8(bench: &mut bencher::Bencher) {
    fn iter_std_i8() {
        for o in [Option::Some(1i8), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_i8);
}

fn bench_iter_opt_i16(bench: &mut bencher::Bencher) {
    fn iter_opt_i16() {
        for o in [optional::some(1i16), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_i16);
}

fn bench_as_slice_iter_opt_i16(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_i16() {
        for o in [optional::some(1i16), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_i16);
}

fn bench_iter_std_i16(bench: &mut bencher::Bencher) {
    fn iter_std_i16() {
        for o in [Option::Some(1i16), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_i16);
}

fn bench_iter_opt_i32(bench: &mut bencher::Bencher) {
    fn iter_opt_i32() {
        for o in [optional::some(1i32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_i32);
}

fn bench_as_slice_iter_opt_i32(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_i32() {
        for o in [optional::some(1i32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_i32);
}

fn bench_iter_std_i32(bench: &mut bencher::Bencher) {
    fn iter_std_i32() {
        for o in [Option::Some(1i32), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_i32);
}

fn bench_iter_opt_i64(bench: &mut bencher::Bencher) {
    fn iter_opt_i64() {
        for o in [optional::some(1i64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_i64);
}

fn bench_as_slice_iter_opt_i64(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_i64() {
        for o in [optional::some(1i64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_i64);
}

fn bench_iter_std_i64(bench: &mut bencher::Bencher) {
    fn iter_std_i64() {
        for o in [Option::Some(1i64), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_i64);
}

fn bench_iter_opt_f32(bench: &mut bencher::Bencher) {
    fn iter_opt_f32() {
        for o in [optional::some(1f32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_f32);
}

fn bench_as_slice_iter_opt_f32(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_f32() {
        for o in [optional::some(1f32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_f32);
}

fn bench_iter_std_f32(bench: &mut bencher::Bencher) {
    fn iter_std_f32() {
        for o in [Option::Some(1f32), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_f32);
}

fn bench_iter_opt_f64(bench: &mut bencher::Bencher) {
    fn iter_opt_f64() {
        for o in [optional::some(1f64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt_f64);
}

fn bench_as_slice_iter_opt_f64(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt_f64() {
        for o in [optional::some(1f64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.as_slice().iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt_f64);
}

fn bench_iter_std_f64(bench: &mut bencher::Bencher) {
    fn iter_std_f64() {
        for o in [Option::Some(1f64), Option::None].iter().cycle().take(1200) {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std_f64);
}

fn bench_is_some_opt_u8(bench: &mut bencher::Bencher) {
    fn is_some_std_u8() {
        for o in [optional::some(1u8), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_u8);
}

fn bench_is_some_std_u8(bench: &mut bencher::Bencher) {
    fn is_some_std_u8() {
        for o in [Option::Some(1u8), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_u8);
}

fn bench_is_some_opt_u16(bench: &mut bencher::Bencher) {
    fn is_some_std_u16() {
        for o in [optional::some(1u16), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_u16);
}

fn bench_is_some_std_u16(bench: &mut bencher::Bencher) {
    fn is_some_std_u16() {
        for o in [Option::Some(1u16), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_u16);
}

fn bench_is_some_opt_u32(bench: &mut bencher::Bencher) {
    fn is_some_std_u32() {
        for o in [optional::some(1u32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_u32);
}

fn bench_is_some_std_u32(bench: &mut bencher::Bencher) {
    fn is_some_std_u32() {
        for o in [Option::Some(1u32), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_u32);
}

fn bench_is_some_opt_u64(bench: &mut bencher::Bencher) {
    fn is_some_std_u64() {
        for o in [optional::some(1u64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_u64);
}

fn bench_is_some_std_u64(bench: &mut bencher::Bencher) {
    fn is_some_std_u64() {
        for o in [Option::Some(1u64), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_u64);
}

fn bench_is_some_opt_f32(bench: &mut bencher::Bencher) {
    fn is_some_std_f32() {
        for o in [optional::some(1f32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_f32);
}

fn bench_is_some_std_f32(bench: &mut bencher::Bencher) {
    fn is_some_std_f32() {
        for o in [Option::Some(1f32), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_f32);
}

fn bench_is_some_opt_f64(bench: &mut bencher::Bencher) {
    fn is_some_std_f64() {
        for o in [optional::some(1f64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_f64);
}

fn bench_is_some_std_f64(bench: &mut bencher::Bencher) {
    fn is_some_std_f64() {
        for o in [Option::Some(1f64), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_std_f64);
}

fn bench_map_opt_u8(bench: &mut bencher::Bencher) {
    fn map_std_u8() {
        for o in [optional::some(1u8), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map(|x| x + 1));
        }
    }

    bench.iter(map_std_u8);
}

fn bench_map_std_u8(bench: &mut bencher::Bencher) {
    fn map_std_u8() {
        for o in [Option::Some(1u8), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map(|x| x + 1));
        }
    }

    bench.iter(map_std_u8);
}

fn bench_map_opt_u16(bench: &mut bencher::Bencher) {
    fn map_std_u16() {
        for o in [optional::some(1u16), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map(|x| x + 1));
        }
    }

    bench.iter(map_std_u16);
}

fn bench_map_std_u16(bench: &mut bencher::Bencher) {
    fn map_std_u16() {
        for o in [Option::Some(1u16), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map(|x| x + 1));
        }
    }

    bench.iter(map_std_u16);
}

fn bench_map_opt_u32(bench: &mut bencher::Bencher) {
    fn map_std_u32() {
        for o in [optional::some(1u32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map(|x| x + 1));
        }
    }

    bench.iter(map_std_u32);
}

fn bench_map_std_u32(bench: &mut bencher::Bencher) {
    fn map_std_u32() {
        for o in [Option::Some(1u32), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map(|x| x + 1));
        }
    }

    bench.iter(map_std_u32);
}

fn bench_map_opt_u64(bench: &mut bencher::Bencher) {
    fn map_std_u64() {
        for o in [optional::some(1u64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map(|x| x + 1));
        }
    }

    bench.iter(map_std_u64);
}

fn bench_map_std_u64(bench: &mut bencher::Bencher) {
    fn map_std_u64() {
        for o in [Option::Some(1u64), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map(|x| x + 1));
        }
    }

    bench.iter(map_std_u64);
}

fn bench_map_opt_f32(bench: &mut bencher::Bencher) {
    fn map_std_f32() {
        for o in [optional::some(1f32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map(|x| x + 1.0));
        }
    }

    bench.iter(map_std_f32);
}

fn bench_map_std_f32(bench: &mut bencher::Bencher) {
    fn map_std_f32() {
        for o in [Option::Some(1f32), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map(|x| x + 1.0));
        }
    }

    bench.iter(map_std_f32);
}

fn bench_map_opt_f64(bench: &mut bencher::Bencher) {
    fn map_std_f64() {
        for o in [optional::some(1f64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map(|x| x + 1.0));
        }
    }

    bench.iter(map_std_f64);
}

fn bench_map_std_f64(bench: &mut bencher::Bencher) {
    fn map_std_f64() {
        for o in [Option::Some(1f64), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map(|x| x + 1.0));
        }
    }

    bench.iter(map_std_f64);
}

fn bench_map_or_std_u8(bench: &mut bencher::Bencher) {
    fn map_or_std_u8() {
        for o in [Option::Some(1u8), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench.iter(map_or_std_u8);
}

fn bench_map_or_std_u16(bench: &mut bencher::Bencher) {
    fn map_or_std_u16() {
        for o in [Option::Some(1u16), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench.iter(map_or_std_u16);
}

fn bench_map_or_std_u32(bench: &mut bencher::Bencher) {
    fn map_or_std_u32() {
        for o in [Option::Some(1u32), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench.iter(map_or_std_u32);
}

fn bench_map_or_std_u64(bench: &mut bencher::Bencher) {
    fn map_or_std_u64() {
        for o in [Option::Some(1u64), Option::None].iter().cycle().take(1200) {
            bencher::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench.iter(map_or_std_u64);
}

fn bench_map_or_std_f32(bench: &mut bencher::Bencher) {
    fn map_or_std_f32() {
        for o in [Option::Some(1.0f32), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(0.0, |i| i + 1.0));
        }
    }

    bench.iter(map_or_std_f32);
}

fn bench_map_or_std_f64(bench: &mut bencher::Bencher) {
    fn map_or_std_f64() {
        for o in [Option::Some(1.0f64), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(0.0, |i| i + 1.0));
        }
    }

    bench.iter(map_or_std_f64);
}

fn bench_map_or_opt_u8(bench: &mut bencher::Bencher) {
    fn map_or_opt_u8() {
        for o in [optional::some(1u8), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench.iter(map_or_opt_u8);
}

fn bench_map_or_opt_u16(bench: &mut bencher::Bencher) {
    fn map_or_opt_u16() {
        for o in [optional::some(1u16), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench.iter(map_or_opt_u16);
}

fn bench_map_or_opt_u32(bench: &mut bencher::Bencher) {
    fn map_or_opt_u32() {
        for o in [optional::some(1u32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench.iter(map_or_opt_u32);
}

fn bench_map_or_opt_u64(bench: &mut bencher::Bencher) {
    fn map_or_opt_u64() {
        for o in [optional::some(1u64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench.iter(map_or_opt_u64);
}

fn bench_map_or_opt_f32(bench: &mut bencher::Bencher) {
    fn map_or_opt_f32() {
        for o in [optional::some(1.0f32), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(0.0, |i| i + 1.0));
        }
    }

    bench.iter(map_or_opt_f32);
}

fn bench_map_or_opt_f64(bench: &mut bencher::Bencher) {
    fn map_or_opt_f64() {
        for o in [optional::some(1.0f64), optional::none()]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(0.0, |i| i + 1.0));
        }
    }

    bench.iter(map_or_opt_f64);
}

benchmark_group!(
    bench,
    bench_iter_opt_u8,
    bench_as_slice_iter_opt_u8,
    bench_iter_std_u8,
    bench_iter_opt_u16,
    bench_as_slice_iter_opt_u16,
    bench_iter_std_u16,
    bench_iter_opt_u32,
    bench_as_slice_iter_opt_u32,
    bench_iter_std_u32,
    bench_iter_opt_u64,
    bench_as_slice_iter_opt_u64,
    bench_iter_std_u64,
    bench_iter_opt_i8,
    bench_as_slice_iter_opt_i8,
    bench_iter_std_i8,
    bench_iter_opt_i16,
    bench_as_slice_iter_opt_i16,
    bench_iter_std_i16,
    bench_iter_opt_i32,
    bench_as_slice_iter_opt_i32,
    bench_iter_std_i32,
    bench_iter_opt_i64,
    bench_as_slice_iter_opt_i64,
    bench_iter_std_i64,
    bench_iter_opt_f32,
    bench_as_slice_iter_opt_f32,
    bench_iter_std_f32,
    bench_iter_opt_f64,
    bench_as_slice_iter_opt_f64,
    bench_iter_std_f64,
    bench_is_some_opt_u8,
    bench_is_some_std_u8,
    bench_is_some_opt_u16,
    bench_is_some_std_u16,
    bench_is_some_opt_u32,
    bench_is_some_std_u32,
    bench_is_some_opt_u64,
    bench_is_some_std_u64,
    bench_is_some_opt_f32,
    bench_is_some_std_f32,
    bench_is_some_opt_f64,
    bench_is_some_std_f64,
    bench_map_opt_u8,
    bench_map_std_u8,
    bench_map_opt_u16,
    bench_map_std_u16,
    bench_map_opt_u32,
    bench_map_std_u32,
    bench_map_opt_u64,
    bench_map_std_u64,
    bench_map_opt_f32,
    bench_map_std_f32,
    bench_map_opt_f64,
    bench_map_std_f64,
    bench_map_or_std_u8,
    bench_map_or_std_u16,
    bench_map_or_std_u32,
    bench_map_or_std_u64,
    bench_map_or_std_f32,
    bench_map_or_std_f64,
    bench_map_or_opt_u8,
    bench_map_or_opt_u16,
    bench_map_or_opt_u32,
    bench_map_or_opt_u64,
    bench_map_or_opt_f32,
    bench_map_or_opt_f64
);

benchmark_main!(bench);
