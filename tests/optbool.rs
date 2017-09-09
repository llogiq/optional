/// tests for `OptionBool`

extern crate optional;

use optional::OptionBool;
use std::convert::Into;

#[test]
fn test_eq_ne() {
    assert_eq!(OptionBool::SomeTrue, OptionBool::SomeTrue);
    assert_ne!(OptionBool::SomeFalse, OptionBool::SomeTrue);
    assert_ne!(OptionBool::None, OptionBool::SomeFalse);

    assert_ne!(&OptionBool::SomeTrue, OptionBool::SomeFalse);
    assert_eq!(&OptionBool::None, OptionBool::None);
}

#[test]
fn into_option_bool() {
    let optionals : [OptionBool; 3] =
        [ OptionBool::some(true), OptionBool::some(false), OptionBool::none() ];

    for o in &optionals {
        let opt : Option<bool> = o.into();
        let o2 : OptionBool = opt.into();
        assert_eq!(o, o2);
    }
}

#[test]
fn test_bool_map() {
    let optionals : [OptionBool; 3] =
        [ OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None ];

    for o in &optionals {
        assert_eq!(o, o.map_bool(|b| b));
        let opt : Option<bool> = **o; // double deref for &
        assert_eq!(opt, o.map(|b| b));
    }

    assert_eq!(OptionBool::SomeTrue, OptionBool::SomeFalse.map_bool(|b| !b))
}

#[test]
fn deref_to_option() {
    assert_eq!(*OptionBool::some(true), Option::Some(true));
    assert_eq!(*OptionBool::some(false), Option::Some(false));
    assert_eq!(*OptionBool::none(), Option::None);
}
