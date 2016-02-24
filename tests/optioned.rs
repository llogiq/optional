extern crate optional;

use optional::Optioned;

#[test]
fn optioned_is_some_or_none() {
    let opt_u32 : Optioned<u32> = Optioned::some(32);
    assert!(opt_u32.is_some());

    let opt_u32_none : Optioned<u32> = Optioned::none();
    assert!(opt_u32_none.is_none());
}
