// Copyright 2015 Andre Bogus
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not be copied, modified,
// or distributed except according to those terms.

//! Space-efficient optional values
//!
//! Type `OptionBool` represents an optional boolean value, similar to
//! `Option<bool>`. Most function implementations are similar or equal.
//! Note that the `map_bool(..)` `and_bool(..)`, `and_then_bool(..)`,
//!`or_bool(..)` and `or_else_bool(..)` functions are working similar to the
//! methods without the `_bool` suffix, but require and return `OptionBool`
//! instead of `Option<bool>`. This allows people to stay within the type.
//!
//! The `OptionBool` type is expected to require only 1 byte of storage:
//!
//! ```
//! assert!(1 == std::mem::size_of::<optional::OptionBool>());
//! ```
//!
//! Then there is the `Optioned<T>` type which wraps a type `T` as an optional
//! value of `T` where one particular value represents None. `Optioned<T>`
//! requires the exact same space as T:
//!
//! ```
//! assert!(std::mem::size_of::<optional::Optioned<i64>>() ==
//!     std::mem::size_of::<i64>());
//! assert!(std::mem::size_of::<optional::Optioned<f32>>() ==
//!     std::mem::size_of::<f32>());
//! assert!(std::mem::size_of::<optional::Optioned<u8>>() ==
//!     std::mem::size_of::<u8>());
//! ```
//!
//! There are implementations for `u8..64,usize` with `std::u..::MAX`
//! representing None, also for `i8..64,isize` with `std::i..::MIN`
//! representing None, and for `f32, f64` with `std::f..::NAN` representing
//! None.
//!
//! Using Optioned for your own types is as simple as implementing `Noned` for
//! your type, provided that your type is already Copy and Sized.

#![deny(missing_docs)]

#[cfg(feature = "serde")]
extern crate serde;

use std::slice::Iter;
use std::cmp::Ordering;
use std::convert::From;
use std::iter::Iterator;
use std::mem;
use std::ops::{Deref, Index, RangeFull};
use std::fmt::{self, Debug, Error};
use std::hash::{Hash, Hasher};
use self::OptionBool::*;

/// The `OptionBool` type, a space-efficient Option<bool> replacement
#[derive(Copy, Clone, PartialEq, Eq, Ord, Hash)]
pub enum OptionBool {
    /// Some(true)
    SomeTrue,
    /// Some(false)
    SomeFalse,
    /// None
    None,
}

// Deref

// we use this for Deref implementation. As they are constant, we obviously
// cannot implement DerefMut.
const OB_SOME_TRUE: Option<bool> = Option::Some(true);
const OB_SOME_FALSE: Option<bool> = Option::Some(false);
const OB_NONE: Option<bool> = Option::None;

const OB_SOME_TRUE_REF: &'static Option<bool> = &OB_SOME_TRUE;
const OB_SOME_FALSE_REF: &'static Option<bool> = &OB_SOME_FALSE;
const OB_NONE_REF: &'static Option<bool> = &OB_NONE;

/// We can deref-coerce to `Option<bool>`
impl Deref for OptionBool {
    type Target = Option<bool>;

    #[inline]
    fn deref(&self) -> &'static Option<bool> {
        match *self {
            SomeTrue => OB_SOME_TRUE_REF,
            SomeFalse => OB_SOME_FALSE_REF,
            None => OB_NONE_REF,
        }
    }
}

impl<'a> PartialEq<OptionBool> for &'a OptionBool {
    #[inline]
    fn eq(&self, other: &OptionBool) -> bool {
        match (*self, other) {
            (&SomeTrue, &SomeTrue) | (&SomeFalse, &SomeFalse) | (&None, &None) => true,
            _ => false,
        }
    }
}

/// Index for `RangeFull` (to slice)
impl Index<RangeFull> for OptionBool {
    type Output = [bool];

    #[inline]
    fn index<'a>(&'a self, _: RangeFull) -> &'static [bool] {
        match *self {
            SomeTrue => OB_TRUE_SLICE_REF,
            SomeFalse => OB_FALSE_SLICE_REF,
            None => OB_EMPTY_SLICE_REF,
        }
    }
}

/// Some(true) > Some(false) > None
impl PartialOrd for OptionBool {
    #[inline]
    fn partial_cmp(&self, other: &OptionBool) -> Option<Ordering> {
        match (self, other) {
            (&SomeTrue, &SomeTrue) | (&SomeFalse, &SomeFalse) | (&None, &None) => {
                Option::Some(Ordering::Equal)
            }
            (&SomeTrue, &SomeFalse) | (&SomeTrue, &None) | (&SomeFalse, &None) => {
                Option::Some(Ordering::Greater)
            }
            _ => Option::Some(Ordering::Less),
        }
    }
}

static OB_TRUE_SLICE: [bool; 1] = [true];
static OB_FALSE_SLICE: [bool; 1] = [false];
static OB_EMPTY_SLICE: [bool; 0] = [];

static OB_TRUE_SLICE_REF: &'static [bool] = &OB_TRUE_SLICE;
static OB_FALSE_SLICE_REF: &'static [bool] = &OB_FALSE_SLICE;
static OB_EMPTY_SLICE_REF: &'static [bool] = &OB_EMPTY_SLICE;

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OptionBool {
    /// with `feature = "serde"`, (de)serialization support is active.
    ///
    /// ```rust
    ///# extern crate serde_json;
    ///# extern crate optional;
    ///# use optional::OptionBool::SomeTrue;
    ///# fn main() {
    /// assert_eq!("true", serde_json::to_string(&SomeTrue).unwrap());
    ///# }
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<OptionBool, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::<bool>::deserialize(deserializer).map(OptionBool::from)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for OptionBool {
    /// with `feature = "serde"`, (de)serialization support is active.
    ///
    /// ```rust
    ///# extern crate serde_json;
    ///# extern crate optional;
    ///# use optional::OptionBool::SomeTrue;
    ///# fn main() {
    /// assert_eq!(SomeTrue, serde_json::from_str("true").unwrap());
    ///# }
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Option::<bool>::from(*self).serialize(serializer)
    }
}

impl OptionBool {
    /// Create a SomeTrue for true, SomeFalse for false
    #[inline]
    pub fn some(b: bool) -> Self {
        if b {
            SomeTrue
        } else {
            SomeFalse
        }
    }

    /// Create a None value.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(OptionBool::none(), optional::OptionBool::None);
    /// ```
    #[inline]
    pub fn none() -> Self {
        None
    }

    /// Returns true if the option is a Some value.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert!(OptionBool::SomeTrue.is_some());
    /// assert!(OptionBool::SomeFalse.is_some());
    /// assert!(!OptionBool::None.is_some());
    /// ```
    #[inline]
    pub fn is_some(&self) -> bool {
        if let OptionBool::None = *self {
            false
        } else {
            true
        }
    }

    /// Returns true if the option is a Some value.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert!(!OptionBool::SomeTrue.is_none());
    /// assert!(!OptionBool::SomeFalse.is_none());
    /// assert!(OptionBool::None.is_none());
    /// ```
    #[inline]
    pub fn is_none(&self) -> bool {
        if let OptionBool::None = *self {
            true
        } else {
            false
        }
    }

    /// Unwraps the contained bool, panics on None with given message.
    ///
    /// # Panics
    ///
    /// if self is None
    ///
    /// # Examples
    ///
    /// For SomeTrue/SomeFalse, the corresponding bool is returned.
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert!(OptionBool::SomeTrue.expect("FAIL"));
    /// assert!(!OptionBool::SomeFalse.expect("FAIL"));
    /// ```
    ///
    /// On None, it panics with the given message.
    ///
    /// ```should_panic
    ///# use optional::OptionBool;
    /// OptionBool::None.expect("FAIL"); // panics with FAIL
    /// ```
    #[inline]
    pub fn expect(&self, msg: &str) -> bool {
        match *self {
            SomeTrue => true,
            SomeFalse => false,
            None => panic!("{}", msg),
        }
    }

    /// Unwraps the contained bool, panics on None.
    ///
    /// # Panics
    ///
    /// if self is None
    ///
    /// # Examples
    ///
    /// For SomeTrue/SomeFalse, the corresponding bool is returned.
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert!(OptionBool::SomeTrue.unwrap());
    /// assert!(!OptionBool::SomeFalse.unwrap());
    /// ```
    ///
    /// On None, it panics with "unwrap called on None"
    ///
    /// ```should_panic
    ///# use optional::OptionBool;
    /// OptionBool::None.unwrap(); // panics
    /// ```
    #[inline]
    pub fn unwrap(&self) -> bool {
        self.expect("unwrap called on None")
    }

    /// Returns the contained bool or a default.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert!(OptionBool::SomeTrue.unwrap_or(false));
    /// assert!(!OptionBool::SomeFalse.unwrap_or(true));
    /// assert!(OptionBool::None.unwrap_or(true));
    /// assert!(!OptionBool::None.unwrap_or(false));
    /// ```
    #[inline]
    pub fn unwrap_or(&self, def: bool) -> bool {
        match *self {
            SomeTrue => true,
            SomeFalse => false,
            None => def,
        }
    }

    /// Returns the contained bool or a computed default.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert!(OptionBool::SomeTrue.unwrap_or_else(|| false));
    /// assert!(!OptionBool::SomeFalse.unwrap_or_else(|| panic!()));
    /// assert!(OptionBool::None.unwrap_or_else(|| true));
    /// ```
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        match self {
            SomeTrue => true,
            SomeFalse => false,
            None => f(),
        }
    }

    /// Maps an `OptionBool` to an `Option<U>` by applying the function
    /// over the contained bool.
    ///
    /// Note that there is also [`map_bool(..)`](#method.map_bool) which works
    /// similarly, but returns another `OptionBool`.
    ///
    /// # Examples
    ///
    /// Convert the contained bool to a Yes/No message
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(Some("Yes"), OptionBool::SomeTrue.map(
    ///     |b| if b { "Yes" } else { "No" }));
    /// ```
    #[inline]
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(bool) -> U,
    {
        match self {
            SomeTrue => Option::Some(f(true)),
            SomeFalse => Option::Some(f(false)),
            None => Option::None,
        }
    }

    /// Maps an `OptionBool` to another `OptionBool` by applying the
    /// function over the contained bool.
    ///
    /// Note that there is also [`map(..)`](#method.map) which works
    /// similarly, but returns an `Option<bool>`.
    ///
    /// # Examples
    ///
    /// Invert the contained `bool`
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(OptionBool::SomeTrue,
    ///     OptionBool::SomeFalse.map_bool(|b| !b));
    /// ```
    #[inline]
    pub fn map_bool<F>(self, f: F) -> OptionBool
    where
        F: FnOnce(bool) -> bool,
    {
        match self {
            SomeTrue => if f(true) {
                SomeTrue
            } else {
                SomeFalse
            },
            SomeFalse => if f(false) {
                SomeTrue
            } else {
                SomeFalse
            },
            None => None,
        }
    }

    /// Maps a value to a `U` by applying the function or return a
    /// default `U`.
    ///
    /// # Examples
    ///
    /// Map to a string (as per the daily wtf's boolean definition):
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!("True", OptionBool::SomeTrue.map_or("FileNotFound",
    ///     |b| if b { "True" } else { "False" }));
    /// ```
    #[inline]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(bool) -> U,
    {
        match self {
            SomeTrue => f(true),
            SomeFalse => f(false),
            None => default,
        }
    }

    /// Maps a value to a `U` by applying the function or return a
    /// computed default.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!("True", OptionBool::SomeTrue.map_or_else(|| "FileNotFound",
    ///     |b| if b { "True" } else { "False" }));
    /// ```
    #[inline]
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(bool) -> U,
    {
        match self {
            SomeTrue => f(true),
            SomeFalse => f(false),
            None => default(),
        }
    }

    /// Transforms the `OptionBool` into a `Result<bool, E>`, mapping
    /// `Some`X to `Ok(`X`)` and `None` to `Err(err)`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(OptionBool::SomeTrue.ok_or("Ouch"), Ok(true));
    /// assert_eq!(OptionBool::None.ok_or("Ouch"), Err("Ouch"));
    /// ```
    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<bool, E> {
        match self {
            SomeTrue => Ok(true),
            SomeFalse => Ok(false),
            None => Err(err),
        }
    }

    /// Transforms the `OptionBool` into a `Result<bool, E>`, mapping `Some`X to
    /// `Ok(`X`)` and `None` to a calculated `Err(err)`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    ///# fn something_expensive() -> bool { unimplemented!(); }
    /// assert_eq!(OptionBool::SomeTrue.ok_or_else(|| something_expensive()), Ok(true));
    /// assert_eq!(OptionBool::None.ok_or_else(|| "Ouch"), Err("Ouch"));
    /// ```
    #[inline]
    pub fn ok_or_else<E, F>(self, err: F) -> Result<bool, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            SomeTrue => Ok(true),
            SomeFalse => Ok(false),
            None => Err(err()),
        }
    }

    /// Returns `None` if the option is `None`, otherwise returns `optb`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(Some(1), OptionBool::SomeTrue.and(Some(1)));
    /// assert_eq!(None, OptionBool::None.and(Some(1)));
    /// let actual : Option<u8> = None;
    /// assert_eq!(None, OptionBool::SomeTrue.and(actual));
    /// ```
    #[inline]
    pub fn and<U>(self, optb: Option<U>) -> Option<U> {
        match self {
            SomeTrue | SomeFalse => optb,
            None => Option::None,
        }
    }

    /// Returns `None` if the option is `None`, otherwise returns `optb`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(OptionBool::SomeTrue,
    ///     OptionBool::SomeFalse.and_bool(OptionBool::SomeTrue));
    /// assert_eq!(OptionBool::None,
    ///     OptionBool::None.and_bool(OptionBool::SomeFalse));
    /// assert_eq!(OptionBool::None,
    ///     OptionBool::SomeTrue.and_bool(OptionBool::None));
    /// ```
    #[inline]
    pub fn and_bool(self, optb: OptionBool) -> OptionBool {
        match self {
            None => None,
            _ => optb,
        }
    }

    /// returns `None` if the `OptionBool` is `None`, otherwise calls `f` with
    /// the boolean value and returns the result as an `Option<U>`
    ///
    /// Note that there is also [`and_then_bool(..)`](#method.and_then_bool)
    /// which works similarly, but returns another `OptionBool`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(None, OptionBool::SomeFalse.and_then(
    ///     |x| if x { Some(true) } else { None }));
    /// ```
    #[inline]
    pub fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(bool) -> Option<U>,
    {
        match self {
            SomeTrue => f(true),
            SomeFalse => f(false),
            None => Option::None,
        }
    }

    /// returns `None` if the `OptionBool` is `None`, otherwise calls `f` with
    /// the boolean value and returns the result as an `OptionBool`
    ///
    /// Note that there is also [`and_then(..)`](#method.and_then) which works
    /// similarly, but returns an `Option<bool>`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(OptionBool::None, OptionBool::SomeFalse.and_then_bool(
    ///     |x| if x { OptionBool::SomeTrue } else { OptionBool::None }));
    /// ```
    #[inline]
    pub fn and_then_bool<F>(self, f: F) -> OptionBool
    where
        F: FnOnce(bool) -> OptionBool,
    {
        match self {
            SomeTrue => f(true),
            SomeFalse => f(false),
            None => None,
        }
    }

    /// Returns this as Option unless this is `None`, in which case returns
    /// `optb`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(Some(false), OptionBool::SomeFalse.or(Some(true)));
    /// assert_eq!(Some(true), OptionBool::None.or(Some(true)));
    /// assert_eq!(None, OptionBool::None.or(None));
    /// ```
    #[inline]
    pub fn or(self, optb: Option<bool>) -> Option<bool> {
        match self {
            SomeTrue => Some(true),
            SomeFalse => Some(false),
            None => optb,
        }
    }

    /// Returns this as Option unless this is `None`, in which case returns
    /// `optb`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(OptionBool::SomeFalse,
    ///     OptionBool::SomeFalse.or_bool(OptionBool::SomeTrue));
    /// assert_eq!(OptionBool::SomeTrue,
    ///     OptionBool::None.or_bool(OptionBool::SomeTrue));
    /// assert_eq!(OptionBool::None,
    ///     OptionBool::None.or_bool(OptionBool::None));
    /// ```
    #[inline]
    pub fn or_bool(self, optb: OptionBool) -> OptionBool {
        match self {
            None => optb,
            x => x,
        }
    }

    /// Returns this as Option unless this is `None`, in which case use the
    /// supplied function to calculate the result.
    ///
    /// Note that there is also [`or_else_bool(..)`](#method.or_else_bool)
    /// which works similarly, but returns another `OptionBool`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(Some(false), OptionBool::SomeFalse.or_else(|| Some(true)));
    /// assert_eq!(Some(true), OptionBool::None.or_else(|| Some(true)));
    /// assert_eq!(None, OptionBool::None.or_else(|| None));
    /// ```
    #[inline]
    pub fn or_else<F>(self, f: F) -> Option<bool>
    where
        F: FnOnce() -> Option<bool>,
    {
        match self {
            SomeTrue => Option::Some(true),
            SomeFalse => Option::Some(false),
            None => f(),
        }
    }

    /// Returns this as Option unless this is `None`, in which case use the
    /// supplied function to calculate the result.
    ///
    /// Note that there is also [`or_else(..)`](#method.or_else) which works
    /// similarly, but returns an `Option<bool>`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(OptionBool::SomeFalse,
    ///     OptionBool::SomeFalse.or_else_bool(|| OptionBool::SomeTrue));
    /// assert_eq!(OptionBool::SomeTrue,
    ///     OptionBool::None.or_else_bool(|| OptionBool::SomeTrue));
    /// assert_eq!(OptionBool::None,
    ///     OptionBool::None.or_else_bool(|| OptionBool::None));
    /// ```
    #[inline]
    pub fn or_else_bool<F>(self, f: F) -> OptionBool
    where
        F: FnOnce() -> OptionBool,
    {
        match self {
            None => f(),
            x => x,
        }
    }

    /// return an iterator over all contained (that is zero or one) values.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(None, OptionBool::None.iter().next());
    /// assert_eq!(Some(&true), OptionBool::SomeTrue.iter().next());
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<bool> {
        self.as_slice().iter()
    }

    /// return a possibly empty slice with the contained value, if any.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// assert_eq!(&[true], OptionBool::SomeTrue.as_slice());
    /// assert!(OptionBool::None.as_slice().is_empty());
    /// ```
    #[inline]
    pub fn as_slice(self) -> &'static [bool] {
        match self {
            SomeTrue => OB_TRUE_SLICE_REF,
            SomeFalse => OB_FALSE_SLICE_REF,
            None => OB_EMPTY_SLICE_REF,
        }
    }

    /// Takes the value out of the `OptionBool` and returns ist as
    /// `Option<bool>`, changing self to `None`.
    ///
    /// Note that there is also [`take_bool(..)`](#method.take_bool) which
    /// works similarly, but returns an `OptionBool`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// let mut x = OptionBool::some(true);
    /// assert_eq!(Some(true), x.take());
    /// assert_eq!(OptionBool::None, x);
    /// ```
    #[inline]
    pub fn take(&mut self) -> Option<bool> {
        self.take_bool().into()
    }

    /// Takes the value out of the `OptionBool`, changing self to `None`.
    ///
    /// Note that there is also [`take(..)`](#method.take) which works
    /// similarly, but returns an `Option<bool>`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::OptionBool;
    /// let mut x = OptionBool::some(true);
    /// assert_eq!(OptionBool::some(true), x.take_bool());
    /// assert_eq!(OptionBool::None, x);
    /// ```
    #[inline]
    pub fn take_bool(&mut self) -> OptionBool {
        mem::replace(self, None)
    }
}

impl Debug for OptionBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match *self {
                SomeTrue => "Some(true)",
                SomeFalse => "Some(false)",
                None => "None",
            }
        )
    }
}

///iterate over an `OptionBool`
pub struct IterBool {
    o: OptionBool,
}

impl Iterator for IterBool {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        self.o.take()
    }
}

/// `IntoIterator` works as expected
///
/// # Examples
///
/// ```
///# use optional::OptionBool;
/// let mut pass : bool = false;
/// for b in OptionBool::SomeTrue { pass = b; }
/// assert!(pass);
///
/// for b in OptionBool::None { assert!(false); }
/// ```
impl IntoIterator for OptionBool {
    type Item = bool;
    type IntoIter = IterBool;

    #[inline]
    fn into_iter(self) -> IterBool {
        IterBool { o: self }
    }
}

/// `OptionBool` defaults to `None`.
impl Default for OptionBool {
    #[inline]
    fn default() -> OptionBool {
        None
    }
}

impl From<OptionBool> for Option<bool> {
    #[inline]
    fn from(o: OptionBool) -> Option<bool> {
        match o {
            SomeTrue => Option::Some(true),
            SomeFalse => Option::Some(false),
            None => Option::None,
        }
    }
}

impl<'a> From<&'a OptionBool> for Option<bool> {
    #[inline]
    fn from(o: &'a OptionBool) -> Option<bool> {
        match *o {
            SomeTrue => Option::Some(true),
            SomeFalse => Option::Some(false),
            None => Option::None,
        }
    }
}

impl From<Option<bool>> for OptionBool {
    #[inline]
    fn from(o: Option<bool>) -> Self {
        match o {
            Option::Some(true) => SomeTrue,
            Option::Some(false) => SomeFalse,
            Option::None => None,
        }
    }
}

impl<'a> From<&'a Option<bool>> for OptionBool {
    #[inline]
    fn from(o: &'a Option<bool>) -> Self {
        match *o {
            Option::Some(true) => SomeTrue,
            Option::Some(false) => SomeFalse,
            Option::None => None,
        }
    }
}

/// A trait whose implementation for any type `T` allows the use of
///`Optioned<T>` where `T` is bound by both `Sized` and `Copy`.
pub trait Noned {
    /// Returns `true` if the contained value is the declared `None` for `T`,
    /// `false` otherwise.
    fn is_none(&self) -> bool;
    /// Returns the declared `None` value for `T`.
    fn get_none() -> Self;
}

impl Noned for u8 {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::u8::MAX
    }

    #[inline]
    fn get_none() -> u8 {
        std::u8::MAX
    }
}

impl Noned for u16 {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::u16::MAX
    }

    #[inline]
    fn get_none() -> u16 {
        std::u16::MAX
    }
}

impl Noned for u32 {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::u32::MAX
    }

    #[inline]
    fn get_none() -> u32 {
        std::u32::MAX
    }
}

impl Noned for u64 {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::u64::MAX
    }

    #[inline]
    fn get_none() -> u64 {
        std::u64::MAX
    }
}

impl Noned for usize {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::usize::MAX
    }

    #[inline]
    fn get_none() -> usize {
        std::usize::MAX
    }
}

impl Noned for i8 {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::i8::MIN
    }

    #[inline]
    fn get_none() -> i8 {
        std::i8::MIN
    }
}

impl Noned for i16 {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::i16::MIN
    }

    #[inline]
    fn get_none() -> i16 {
        std::i16::MIN
    }
}

impl Noned for i32 {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::i32::MIN
    }

    #[inline]
    fn get_none() -> i32 {
        std::i32::MIN
    }
}

impl Noned for i64 {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::i64::MIN
    }

    #[inline]
    fn get_none() -> i64 {
        std::i64::MIN
    }
}

impl Noned for isize {
    #[inline]
    fn is_none(&self) -> bool {
        self == &std::isize::MIN
    }

    #[inline]
    fn get_none() -> isize {
        std::isize::MIN
    }
}

impl Noned for f32 {
    #[inline]
    fn is_none(&self) -> bool {
        self.is_nan()
    }

    #[inline]
    fn get_none() -> f32 {
        std::f32::NAN
    }
}

impl Noned for f64 {
    #[inline]
    fn is_none(&self) -> bool {
        self.is_nan()
    }

    #[inline]
    fn get_none() -> f64 {
        std::f64::NAN
    }
}

impl Noned for char {
    #[inline]
    fn is_none(&self) -> bool { *self == '\0' }

    #[inline]
    fn get_none() -> char { '\0' }
}

///Equality within Optioned
pub trait OptEq {
    /// Is the other optioned equal to this one?
    #[inline]
    fn opt_eq(&self, other: &Self) -> bool;
}

impl OptEq for u8 {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl OptEq for u16 {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl OptEq for u32 {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl OptEq for u64 {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl OptEq for usize {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl OptEq for i8 {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl OptEq for i16 {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl OptEq for i32 {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl OptEq for i64 {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl OptEq for isize {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl OptEq for f32 {
    fn opt_eq(&self, other: &Self) -> bool {
        if self.is_nan() {
            other.is_nan()
        } else {
            self == other
        }
    }
}
impl OptEq for f64 {
    fn opt_eq(&self, other: &Self) -> bool {
        if self.is_nan() {
            other.is_nan()
        } else {
            self == other
        }
    }
}

impl OptEq for char {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}

///Ordering within Optioned
pub trait OptOrd {
    /// compare this Optioned with another
    #[inline]
    fn opt_cmp(&self, other: &Self) -> Ordering;
}

#[inline]
fn _opt_cmp<T: Ord + Copy + Noned>(a: &T, b: &T) -> Ordering {
    if a.is_none() {
        if b.is_none() {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    } else if b.is_none() {
        Ordering::Greater
    } else {
        a.cmp(b)
    }
}

#[inline]
fn _opt_cmp_part<T: PartialOrd + Copy + Noned>(a: &T, b: &T) -> Ordering {
    if a.is_none() {
        if b.is_none() {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    } else if b.is_none() {
        Ordering::Greater
    } else {
        a.partial_cmp(b).unwrap()
    }
}

impl OptOrd for u8 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        _opt_cmp(self, other)
    }
}
impl OptOrd for u16 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        _opt_cmp(self, other)
    }
}
impl OptOrd for u32 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        _opt_cmp(self, other)
    }
}
impl OptOrd for u64 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        _opt_cmp(self, other)
    }
}
impl OptOrd for usize {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        _opt_cmp(self, other)
    }
}

impl OptOrd for i8 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
impl OptOrd for i16 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
impl OptOrd for i32 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
impl OptOrd for i64 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
impl OptOrd for isize {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl OptOrd for f32 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        _opt_cmp_part(self, other)
    }
}
impl OptOrd for f64 {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        _opt_cmp_part(self, other)
    }
}

impl OptOrd for char {
    fn opt_cmp(&self, other: &Self) -> Ordering {
        _opt_cmp(self, other)
    }
}


/// An `Option<T>`-like structure that takes only as much space as the enclosed
/// value, at the cost of removing one particular `None` value from the value
/// domain (see `Noned`)
#[derive(Copy, Clone)]
pub struct Optioned<T: Noned + Copy> {
    value: T,
}

/// Equality works as usual.
///
/// # Examples
///
/// ```
///# use ::optional::{some, none};
/// assert_eq!(some(1u8), some(1u8));
/// assert_eq!(none::<u32>(), none::<u32>());
/// ```
impl<T> PartialEq for Optioned<T>
where
    T: OptEq + Noned + Copy,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value.opt_eq(&other.value)
    }
}

impl<T> Eq for Optioned<T>
where
    T: OptEq + Noned + Copy,
{
}

impl<T> PartialOrd for Optioned<T>
where
    T: PartialEq + OptEq + OptOrd + Noned + Copy,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.opt_cmp(&other.value))
    }
}

impl<T> Ord for Optioned<T>
where
    T: Eq + OptEq + OptOrd + Noned + Copy,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.opt_cmp(&other.value)
    }
}

impl<T> Hash for Optioned<T>
where
    T: Noned + Copy + Hash,
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.value.hash(state)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Optioned<T>
where
    T: Noned + Copy + serde::Deserialize<'de>,
{
    /// with `feature = "serde"`, (de)serialization support is active.
    ///
    /// ```rust
    ///# extern crate serde_json;
    ///# extern crate optional;
    ///# use optional::{Optioned, Noned, some};
    ///# fn main() {
    /// assert_eq!("1.0", serde_json::to_string(&some(1f32)).unwrap());
    ///# }
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<Optioned<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(Optioned::from)
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Optioned<T>
where
    T: Noned + Copy + serde::Serialize,
{
    /// with `feature = "serde"`, (de)serialization support is active.
    ///
    /// ```rust
    ///# extern crate serde_json;
    ///# extern crate optional;
    ///# use optional::{Optioned, Noned, some};
    ///# fn main() {
    /// assert_eq!(some(1f32), serde_json::from_str("1.0").unwrap());
    ///# }
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_option().serialize(serializer)
    }
}

mod slice_of_up_to_one {
    /// Get a slice of zero or one elements from a ref to the single value and a bool whether
    /// the value should be included
    ///
    /// note: This is safe because:
    ///
    /// ```rust
    /// assert_eq!(0, false as usize); // empty slice
    /// assert_eq!(1, true as usize); // slice of one element
    /// ```
    #[inline]
    pub fn slice_of<T>(value: &T, one: bool) -> &[T] {
        unsafe { ::std::slice::from_raw_parts(value, one as usize) }
    }
}

impl<T: Noned + Copy> Optioned<T> {
    /// Create an `Optioned<T>` that is `some(t)`.
    ///
    /// # Panics
    ///
    /// panics if the supplied value is the None value
    ///
    /// # Examples
    ///
    /// ```
    ///# use ::optional::Optioned;
    /// Optioned::<i32>::some(1); // Optioned(1)
    /// ```
    ///
    /// ```should_panic
    ///# use ::optional::Optioned;
    /// Optioned::<f64>::some(std::f64::NAN); // panic!s
    /// ```
    #[inline]
    pub fn some(t: T) -> Self {
        assert!(!t.is_none());
        Optioned::<T> { value: t }
    }

    /// Create an `Optioned<T>` that is `none()`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use ::optional::Optioned;
    /// Optioned::<u16>::none(); // Optioned(std::u16::MAX)
    /// ```
    #[inline]
    pub fn none() -> Self {
        Optioned::<T> {
            value: <T as Noned>::get_none(),
        }
    }

    #[inline]
    fn as_option(&self) -> Option<T> {
        if self.value.is_none() {
            Option::None
        } else {
            Option::Some(self.value)
        }
    }

    /// Returns `true` if this `Optioned` is `None`, `false` otherwise.
    #[inline]
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }

    /// Returns `true` if this `Optioned` contains a value, `false` otherwise.
    #[inline]
    pub fn is_some(&self) -> bool {
        !self.value.is_none()
    }

    /// Unwraps the value, if any, else panics with the given message.
    ///
    /// # Panics
    ///
    /// if self is None
    ///
    /// # Examples
    ///
    /// For Some(_), the corresponding value is returned.
    ///
    /// ```
    ///# use optional::Optioned;
    /// assert_eq!(42u8, Optioned::some(42u8).expect("FAIL"));
    /// ```
    ///
    /// On None, it panics with the given message.
    ///
    /// ```should_panic
    ///# use optional::Optioned;
    ///Optioned::<u8>::none().expect("FAIL"); // panics with FAIL
    /// ```
    #[inline]
    pub fn expect(&self, msg: &str) -> T {
        if self.is_none() {
            panic!("{}", msg)
        }
        self.value
    }

    /// Unwraps the value, if any, else panics with "unwrap called on None".
    ///
    /// # Panics
    ///
    /// if self is `None`
    ///
    /// # Examples
    ///
    /// For `Some(_)`, the corresponding value is returned.
    ///
    /// ```
    ///# use optional::Optioned;
    /// assert_eq!(42u8, Optioned::some(42u8).unwrap());
    /// ```
    ///
    /// On `None`, it panics with the given message.
    ///
    /// ```should_panic
    ///# use optional::Optioned;
    ///Optioned::<u8>::none().unwrap(); // panics
    /// ```
    #[inline]
    pub fn unwrap(&self) -> T {
        self.expect("unwrap called on None")
    }

    /// Returns the contained value, even if None.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!(-128i8, none().unpack());
    /// assert_eq!(1u32, some(1).unpack());
    /// ```
    #[inline]
    pub fn unpack(&self) -> T {
        self.value
    }

    /// Returns the contained value or a default.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!(-1i8, some(-1i8).unwrap_or(127i8));
    /// assert_eq!(42u16, none().unwrap_or(42u16));
    /// ```
    #[inline]
    pub fn unwrap_or(&self, def: T) -> T {
        if self.is_none() {
            def
        } else {
            self.value
        }
    }

    /// Returns the contained value or a calculated default.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!(-1i8, some(-1i8).unwrap_or_else(|| panic!()));
    /// assert_eq!(42u16, none().unwrap_or_else(|| 42u16));
    /// ```
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        if self.is_none() {
            f()
        } else {
            self.value
        }
    }

    /// Maps the `Optioned` to an `Option<U>` by applying the function over the
    /// contained value, if any.
    ///
    /// # Examples
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!(Some(-42), some(42i8).map(|x| -x));
    /// assert_eq!(None, none::<i8>().map(|x| -x));
    /// ```
    #[inline]
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        if self.is_none() {
            Option::None
        } else {
            Option::Some(f(self.value))
        }
    }

    /// Maps the `Optioned<T>` to an `Optioned<U>` by applying the function over
    /// the contained value, if any. Requires that the result type of the
    /// function be `Noned + Copy`, as other types aren't compatible with
    /// Optioned.
    ///
    /// # Examples
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!(some(-42), some(42i8).map_t(|x| -x));
    /// assert_eq!(none::<i8>(), none::<i8>().map_t(|x| -x));
    /// ```
    #[inline]
    pub fn map_t<U, F>(self, f: F) -> Optioned<U>
    where
        F: FnOnce(T) -> U,
        U: Noned + Copy,
    {
        if self.is_none() {
            none()
        } else {
            some(f(self.value))
        }
    }

    /// Maps the contained value to a `U` by applying the function or return a
    /// default.
    ///
    /// # Examples
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!("1", some(1usize).map_or("Unknown".to_string(), |b| b.to_string()));
    /// assert_eq!("Unknown", none::<usize>().map_or("Unknown".to_string(), |b| b.to_string()));
    /// ```
    #[inline]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        if self.is_none() {
            default
        } else {
            f(self.value)
        }
    }

    /// Maps a value to a `U` by applying the function or return a computed
    /// default.
    ///
    /// # Examples
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!("1", some(1usize).map_or_else(|| "Unknown".to_string(),
    ///     |b| b.to_string()));
    /// assert_eq!("Unknown", none::<usize>().map_or_else(
    ///     || "Unknown".to_string(), |b| b.to_string()));
    /// ```
    #[inline]
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        if self.is_none() {
            default()
        } else {
            f(self.value)
        }
    }

    /// Returns this option if it contains a value, otherwise returns the other.
    ///
    /// Arguments passed to `or` are eagerly evaluated;
    /// if you are passing the result of a function call,
    /// it is recommended to use `or_else`, which is lazily evaluated.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional::{Optioned, some, none};
    /// let x = some(2);
    /// let y = none();
    /// assert_eq!(x.or(y), some(2));
    ///
    /// let x = none();
    /// let y = some(100);
    /// assert_eq!(x.or(y), some(100));
    ///
    /// let x = some(2);
    /// let y = some(100);
    /// assert_eq!(x.or(y), some(2));
    ///
    /// let x: Optioned<u32> = none();
    /// let y = none();
    /// assert_eq!(x.or(y), none());
    /// ```
    #[inline]
    pub fn or(self, other: Optioned<T>) -> Optioned<T> {
        if self.is_some() {
            self
        } else {
            other
        }
    }

    /// Returns this option if it contains a value, otherwise calls `f` and returns the result.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional::{Optioned, some, none};
    /// fn nothing() -> Optioned<u32> { none() }
    /// fn something() -> Optioned<u32> { some(1) }
    ///
    /// assert_eq!(some(2).or_else(something), some(2));
    /// assert_eq!(none().or_else(something), some(1));
    /// assert_eq!(none().or_else(nothing), none());
    /// ```
    pub fn or_else<F>(self, f: F) -> Optioned<T>
    where
        F: FnOnce() -> Optioned<T>,
    {
        if self.is_some() {
            self
        } else {
            f()
        }
    }

    /// Returns the `None` value for type `U` if this value or `other` contains their respective
    /// `None` values. Otherwise returns the `other` `Optioned` struct. 
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional::{Optioned, some, none};
    /// let the_other = some::<u32>(42);
    ///
    /// assert_eq!(some('a').and(the_other), some(42));
    /// assert_eq!(none::<char>().and(the_other), none::<u32>());
    /// assert_eq!(some('a').and(none::<u32>()), none::<u32>());
    /// assert_eq!(none::<char>().and(none::<u32>()), none::<u32>());
    /// ```
    pub fn and<U>(self, other: Optioned<U>) -> Optioned<U>
    where
        U: Noned + Copy
    {
        if self.is_some() {
           other
        } else {
            none::<U>()
        }
    }

    /// Returns this `Optioned` if it contains the the `None` value, otherwise calls `f` with
    /// the contained value and returns the result as an `Optioned<U>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional::{Optioned, some, none, wrap};
    /// fn nothing() -> Optioned<u32> { none() }
    /// fn something() -> Optioned<u32> { some(1) }
    /// fn add_two(val: u32) -> Optioned<u32> {
    ///   wrap( val + 2)
    /// }
    /// 
    /// fn failed_function(val: u32) -> Optioned<u32> {
    ///   none()
    /// }
    ///
    /// assert_eq!(some(2).and_then(add_two), some(4));
    /// assert_eq!(none().and_then(add_two), none());
    /// assert_eq!(some(2).and_then(failed_function), none());
    /// assert_eq!(none().and_then(failed_function), none());
    /// ```
    pub fn and_then<F,U>(self, f: F) -> Optioned<U>
    where
        F: FnOnce(T) -> Optioned<U>,
        U: Noned + Copy
    {
        if self.is_some() {
            f(self.value)
        } else {
            none()
        }
    }

    /// Takes the value out of the `Optioned` and returns ist as
    /// `Option<T>`, changing self to `None`.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::{some, none};
    /// let mut x = some(1u8);
    /// assert_eq!(Some(1u8), x.take());
    /// assert!(x.is_none());
    /// ```
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        mem::replace(self, Self::none()).as_option()
    }

    /// Return a possibly empty slice over the contained value, if any.
    ///
    /// # Examples
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!(&[42], some(42u8).as_slice());
    /// assert!(none::<i16>().as_slice().is_empty());
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        slice_of_up_to_one::slice_of(&self.value, self.is_some())
    }

    /// return an iterator over all contained (that is zero or one) values.
    ///
    /// # Examples
    ///
    /// ```
    ///# use optional::{some, none};
    /// assert_eq!(None, none::<u64>().iter().next());
    /// assert_eq!(Some(42u64), some(42u64).iter().next());
    /// ```
    #[inline]
    pub fn iter(&self) -> OptionedIter<T> {
        OptionedIter { o: *self } // make a copy
    }
}

/// Create an `Optioned<T>` that is `some(t)`.
///
/// # Panics
///
/// panics if the supplied value is the None value
///
/// # Examples
///
/// ```
///# use ::optional::some;
/// some(1i32); // Optioned(1i32)
/// ```
///
/// ```should_panic
///# use ::optional::some;
/// some(std::f64::NAN); // panic!s
/// ```
pub fn some<T: Noned + Copy>(value: T) -> Optioned<T> {
    Optioned::<T>::some(value)
}

/// Create a `None Optioned<T>`. Note that the type must be inferrible
/// from the context, or you'd need to call with `::<T>` where `T` is
/// the specific type.
///
/// # Examples
///
/// ```
///# use ::optional::{none, Optioned};
/// let x : Optioned<i16> = none();
/// none::<f32>();
/// ```
pub fn none<T: Noned + Copy>() -> Optioned<T> {
    Optioned::<T>::none()
}

/// Wrap a `T` into an `Optioned<T>`, regardless of its None-ness.
///
/// # Examples
///
/// ```
///# use optional::wrap;
/// assert!(wrap(1u8).is_some());
/// assert!(wrap(255u8).is_none());
/// ```
pub fn wrap<T: Noned + Copy>(v: T) -> Optioned<T> {
    Optioned { value: v }
}

impl<T: Noned + Copy + Debug> Debug for Optioned<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
        if self.is_none() {
            write!(f, "None")
        } else {
            write!(f, "Some({:?})", &self.value)
        }
    }
}

impl<T: Noned + Copy> Default for Optioned<T> {
    #[inline]
    fn default() -> Optioned<T> {
        none()
    }
}

/// iterate over an Optioned<T>
#[derive(Copy, Clone)]
pub struct OptionedIter<T: Noned + Copy> {
    o: Optioned<T>,
}

impl<T: Noned + Copy> Iterator for OptionedIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.o.take()
    }
}

impl<'a, T: Noned + Copy> From<&'a Option<T>> for Optioned<T> {
    #[inline]
    fn from(o: &Option<T>) -> Optioned<T> {
        o.map_or_else(Self::none, Self::some)
    }
}

impl<T: Noned + Copy> From<Option<T>> for Optioned<T> {
    #[inline]
    fn from(o: Option<T>) -> Optioned<T> {
        o.map_or_else(Self::none, Self::some)
    }
}

impl<T: Noned + Copy> Into<Option<T>> for Optioned<T> {
    #[inline]
    fn into(self) -> Option<T> {
        self.as_option()
    }
}

impl<T: Noned + Copy> From<T> for Optioned<T> {
    #[inline]
    fn from(o: T) -> Optioned<T> {
        wrap(o)
    }
}
