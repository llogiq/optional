/// tests for OptionBool

extern crate optional;

use optional::OptionBool;
use std::convert::Into;

#[test]
fn test_eq_ne() {
    assert!(OptionBool::SomeTrue == OptionBool::SomeTrue);
    assert!(OptionBool::SomeFalse != OptionBool::SomeTrue);
    assert!(OptionBool::None != OptionBool::SomeFalse);
    
    assert!(&OptionBool::SomeTrue != OptionBool::SomeFalse);
    assert!(&OptionBool::None == OptionBool::None);
}

#[test]
fn into_option_bool() {
    let optionals : [OptionBool; 3] = 
        [ OptionBool::some(true), OptionBool::some(false), OptionBool::none() ];

    for o in optionals.iter() {
        let opt : Option<bool> = o.into();
        let o2 : OptionBool = opt.into();
        assert!(o == o2);
    }
}

#[test]
fn test_bool_map() {
    let optionals : [OptionBool; 3] =
        [ OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None ];

    for o in optionals.iter() {
        assert!(o == o.map_bool(|b| b));
        let opt : Option<bool> = **o; // double deref for &
        assert!(opt == o.map(|b| b));
    }

    assert!(OptionBool::SomeTrue == OptionBool::SomeFalse.map_bool(|b| !b))
}

#[test]
fn deref_to_option() {
    assert!(*OptionBool::some(true) == Option::Some(true));
    assert!(*OptionBool::some(false) == Option::Some(false));
    assert!(*OptionBool::none() == Option::None);
}
