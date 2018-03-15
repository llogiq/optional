#[macro_use]
extern crate bencher;
extern crate optional;

use optional::OptionBool;

fn bench_is_some_opt(bench: &mut bencher::Bencher) {
    fn is_some_optbool() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_optbool);
}

fn bench_is_some_std(bench: &mut bencher::Bencher) {
    fn is_some_stdopt() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.is_some());
        }
    }

    bench.iter(is_some_stdopt);
}

fn invert(b: bool) -> bool {
    !b
}

fn bench_map_invert_opt_bool(bench: &mut bencher::Bencher) {
    fn map_invert_opt_bool() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_bool(invert));
        }
    }

    bench.iter(map_invert_opt_bool);
}

fn bench_map_invert_opt(bench: &mut bencher::Bencher) {
    fn map_invert_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map(invert));
        }
    }

    bench.iter(map_invert_opt);
}

fn bench_map_invert_std(bench: &mut bencher::Bencher) {
    fn map_invert_std() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map(invert));
        }
    }

    bench.iter(map_invert_std);
}

fn bench_unwrap_opt(bench: &mut bencher::Bencher) {
    fn unwrap_opt() {
        for o in [OptionBool::SomeTrue, OptionBool::SomeFalse]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.unwrap());
        }
    }

    bench.iter(unwrap_opt);
}

fn bench_unwrap_std(bench: &mut bencher::Bencher) {
    fn unwrap_std() {
        for o in [Option::Some(true), Option::Some(false)]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.unwrap());
        }
    }

    bench.iter(unwrap_std);
}

fn bench_unwrap_or_opt(bench: &mut bencher::Bencher) {
    fn unwrap_or_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.unwrap_or(false));
        }
    }

    bench.iter(unwrap_or_opt);
}

fn bench_unwrap_or_std(bench: &mut bencher::Bencher) {
    fn unwrap_or_std() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.unwrap_or(false));
        }
    }

    bench.iter(unwrap_or_std);
}

fn oracle() -> bool {
    true
}

fn bench_unwrap_or_else_opt(bench: &mut bencher::Bencher) {
    fn unwrap_or_else_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.unwrap_or_else(oracle));
        }
    }

    bench.iter(unwrap_or_else_opt);
}

fn bench_unwrap_or_else_std(bench: &mut bencher::Bencher) {
    fn unwrap_or_else_std() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.unwrap_or_else(oracle));
        }
    }

    bench.iter(unwrap_or_else_std);
}

fn bench_map_or_opt(bench: &mut bencher::Bencher) {
    fn map_or_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(false, invert));
        }
    }

    bench.iter(map_or_opt);
}

fn bench_map_or_std(bench: &mut bencher::Bencher) {
    fn map_or_std() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or(false, invert));
        }
    }

    bench.iter(map_or_std);
}

fn bench_map_or_else_opt(bench: &mut bencher::Bencher) {
    fn map_or_else_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or_else(oracle, invert));
        }
    }

    bench.iter(map_or_else_opt);
}

fn bench_map_or_else_std(bench: &mut bencher::Bencher) {
    fn map_or_else_std() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.map_or_else(oracle, invert));
        }
    }

    bench.iter(map_or_else_std);
}

fn bench_and_opt(bench: &mut bencher::Bencher) {
    fn and_opt() {
        for o1 in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(33)
        {
            for o2 in [Option::Some(true), Option::Some(false), Option::None]
                .iter()
                .cycle()
                .take(33)
            {
                bencher::black_box(o1.and(*o2));
            }
        }
    }

    bench.iter(and_opt);
}

fn bench_and_bool_opt(bench: &mut bencher::Bencher) {
    fn and_bool_opt() {
        for o1 in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(33)
        {
            for o2 in [
                OptionBool::SomeTrue,
                OptionBool::SomeFalse,
                OptionBool::None,
            ].iter()
                .cycle()
                .take(33)
            {
                bencher::black_box(o1.and_bool(*o2));
            }
        }
    }

    bench.iter(and_bool_opt);
}

fn bench_and_std(bench: &mut bencher::Bencher) {
    fn and_std() {
        for o1 in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(33)
        {
            for o2 in [Option::Some(true), Option::Some(false), Option::None]
                .iter()
                .cycle()
                .take(33)
            {
                bencher::black_box(o1.and(*o2));
            }
        }
    }

    bench.iter(and_std);
}

fn bench_or_opt(bench: &mut bencher::Bencher) {
    fn or_opt() {
        for o1 in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(33)
        {
            for o2 in [Option::Some(true), Option::Some(false), Option::None]
                .iter()
                .cycle()
                .take(33)
            {
                bencher::black_box(o1.or(*o2));
            }
        }
    }

    bench.iter(or_opt);
}

fn bench_or_bool_opt(bench: &mut bencher::Bencher) {
    fn or_bool_opt() {
        for o1 in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(33)
        {
            for o2 in [
                OptionBool::SomeTrue,
                OptionBool::SomeFalse,
                OptionBool::None,
            ].iter()
                .cycle()
                .take(33)
            {
                bencher::black_box(o1.or_bool(*o2));
            }
        }
    }

    bench.iter(or_bool_opt);
}

fn bench_or_std(bench: &mut bencher::Bencher) {
    fn or_std() {
        for o1 in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(33)
        {
            for o2 in [Option::Some(true), Option::Some(false), Option::None]
                .iter()
                .cycle()
                .take(33)
            {
                bencher::black_box(o1.or(*o2));
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

fn bench_or_else_opt(bench: &mut bencher::Bencher) {
    fn or_else_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.or_else(create_std));
        }
    }

    bench.iter(or_else_opt);
}

fn bench_or_else_bool_opt(bench: &mut bencher::Bencher) {
    fn or_else_bool_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.or_else_bool(create_opt));
        }
    }

    bench.iter(or_else_bool_opt);
}

fn bench_or_else_std(bench: &mut bencher::Bencher) {
    fn or_else_std() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o.or_else(create_std));
        }
    }

    bench.iter(or_else_std);
}

fn bench_iter_opt(bench: &mut bencher::Bencher) {
    fn iter_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_opt);
}

fn bench_as_slice_iter_opt(bench: &mut bencher::Bencher) {
    fn as_slice_iter_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            for b in o[..].iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(as_slice_iter_opt);
}

fn bench_iter_std(bench: &mut bencher::Bencher) {
    fn iter_std() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            for b in o.iter() {
                bencher::black_box(b);
            }
        }
    }

    bench.iter(iter_std);
}

fn bench_as_slice_opt(bench: &mut bencher::Bencher) {
    fn as_slice_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            bencher::black_box(o);
        }
    }

    bench.iter(as_slice_opt);
}

fn bench_from_opt(bench: &mut bencher::Bencher) {
    fn from_opt() {
        for o in [Option::Some(true), Option::Some(false), Option::None]
            .iter()
            .cycle()
            .take(1200)
        {
            let o2: OptionBool = o.into();
            bencher::black_box(o2);
        }
    }

    bench.iter(from_opt);
}

fn bench_to_opt(bench: &mut bencher::Bencher) {
    fn to_opt() {
        for o in [
            OptionBool::SomeTrue,
            OptionBool::SomeFalse,
            OptionBool::None,
        ].iter()
            .cycle()
            .take(1200)
        {
            let o2: Option<bool> = o.into();
            bencher::black_box(o2);
        }
    }

    bench.iter(to_opt);
}

benchmark_group!(
    bench,
    bench_is_some_opt,
    bench_is_some_std,
    bench_map_invert_opt_bool,
    bench_map_invert_opt,
    bench_map_invert_std,
    bench_unwrap_opt,
    bench_unwrap_std,
    bench_unwrap_or_opt,
    bench_unwrap_or_std,
    bench_unwrap_or_else_opt,
    bench_unwrap_or_else_std,
    bench_map_or_opt,
    bench_map_or_std,
    bench_map_or_else_opt,
    bench_map_or_else_std,
    bench_and_opt,
    bench_and_bool_opt,
    bench_and_std,
    bench_or_opt,
    bench_or_bool_opt,
    bench_or_std,
    bench_or_else_opt,
    bench_or_else_bool_opt,
    bench_or_else_std,
    bench_iter_opt,
    bench_as_slice_iter_opt,
    bench_iter_std,
    bench_as_slice_opt,
    bench_from_opt,
    bench_to_opt
);

benchmark_main!(bench);
