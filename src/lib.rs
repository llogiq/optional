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
use std::cmp::Ordering;
use std::convert::From;
use std::iter::Iterator;
use std::mem;
use std::ops::Deref;
use std::fmt::{self, Debug, Error};
use self::OptionBool::*;

/// The `OptionBool` type, a space-efficient Option<bool> replacement
#[derive(Copy, Clone, Eq, Ord, Hash)]
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
const OB_SOME_TRUE : Option<bool> = Option::Some(true);
const OB_SOME_FALSE : Option<bool> = Option::Some(false);
const OB_NONE : Option<bool> = Option::None;

const OB_SOME_TRUE_REF : &'static Option<bool> = &OB_SOME_TRUE;
const OB_SOME_FALSE_REF : &'static Option<bool> = &OB_SOME_FALSE;
const OB_NONE_REF : &'static Option<bool> = &OB_NONE;

/// We can deref-coerce to Option<bool>
impl Deref for OptionBool {
	type Target = Option<bool>;

	#[inline]	
	fn deref(&self) -> &'static Option<bool> {
		match self {
			&SomeTrue => OB_SOME_TRUE_REF,
			&SomeFalse => OB_SOME_FALSE_REF,
			&None => OB_NONE_REF,
		}
	}
}

/// The IterBool iterates over the zero or one values of an OptionBool
pub struct IterBool {
	inner: OptionBool,
}

impl<'a> Iterator for IterBool {
	type Item=bool;
	
	#[inline]
	fn next(&mut self) -> Option<bool> {
		self.inner.take()
	}
}

impl PartialEq for OptionBool {
	#[inline]
	fn eq(&self, other: &OptionBool) -> bool {
		match (self, other) {
			(&SomeTrue, &SomeTrue) |
			(&SomeFalse, &SomeFalse) |
			(&None, &None) => true,
			_ => false,
		}
	}
}

impl<'a> PartialEq<OptionBool> for &'a OptionBool {
	#[inline]
	fn eq(&self, other: &OptionBool) -> bool {
		match (*self, other) {
			(&SomeTrue, &SomeTrue) |
			(&SomeFalse, &SomeFalse) |
			(&None, &None) => true,
			_ => false,
		}
	}
}

/// Some(true) > Some(false) > None
impl PartialOrd for OptionBool {
	#[inline]
	fn partial_cmp(&self, other: &OptionBool) -> Option<Ordering> {
		match (self, other) {
			(&SomeTrue, &SomeTrue) |
			(&SomeFalse, &SomeFalse) |
			(&None, &None) => Option::Some(Ordering::Equal),
			(&SomeTrue, &SomeFalse) |
			(&SomeTrue, &None) |
			(&SomeFalse, &None) => 
				Option::Some(Ordering::Greater),
			_ => Option::Some(Ordering::Less),
		}
	}
}

impl OptionBool {
	/// Create a SomeTrue for true, SomeFalse for false
	#[inline]
	pub fn some(b: bool) -> Self {
		if b { SomeTrue } else { SomeFalse }
	}
	
	/// Create a None value.
	///
	/// # Examples
	///
	/// ```
	/// assert_eq!(optional::OptionBool::none(), optional::OptionBool::None);
	/// ```
	#[inline]
	pub fn none() -> Self { None }
	
	/// Returns true if the option is a Some value.
	///
	/// # Examples
	///
	/// ```
	/// assert!(optional::OptionBool::SomeTrue.is_some());
	/// assert!(optional::OptionBool::SomeFalse.is_some());
	/// assert!(!optional::OptionBool::None.is_some());
	/// ```
	#[inline]
	pub fn is_some(&self) -> bool {
		if let &None = self { false } else { true }
	}
	
	/// Returns true if the option is a Some value.
	///
	/// # Examples
	/// 
	/// ```
	/// assert!(!optional::OptionBool::SomeTrue.is_none());
	/// assert!(!optional::OptionBool::SomeFalse.is_none());
	/// assert!(optional::OptionBool::None.is_none());
	/// ```
	#[inline]
	pub fn is_none(&self) -> bool {
		if let &None = self { true } else { false }
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
	/// assert!(optional::OptionBool::SomeTrue.expect("FAIL"));
	/// assert!(!optional::OptionBool::SomeFalse.expect("FAIL"));
	/// ```
	/// 
	/// On None, it panics with the given message.
	///
	/// ```should_panic
	/// optional::OptionBool::None.expect("FAIL"); // panics with FAIL
	/// ```
	#[inline]
	pub fn expect(&self, msg: &str) -> bool {
		match self {
			&SomeTrue => true,
			&SomeFalse => false,
			&None => panic!("{}", msg)
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
	/// assert!(optional::OptionBool::SomeTrue.unwrap());
	/// assert!(!optional::OptionBool::SomeFalse.unwrap());
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
		match self {
			&SomeTrue => true,
			&SomeFalse => false,
			&None => def,
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
	pub fn unwrap_or_else<F>(self, f: F) -> bool where F: FnOnce() -> bool {
		match self {
			SomeTrue => true,
			SomeFalse => false,
			None => f(),
		}
	}
	
	/// Maps an OptionBool to an Option<U> by applying the function over the
	/// contained bool.
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
	where F: FnOnce(bool) -> U {
		match self {
			SomeTrue => Option::Some(f(true)),
			SomeFalse => Option::Some(f(false)),
			None => Option::None,
		}
	}
	
	/// Maps an OptionBool to another OptionBool by applying the function over
	/// the contained bool.
	///
	/// # Examples
	///
	/// Invert the contained bool
	///
	/// ```
	///# use optional::OptionBool;
	/// assert_eq!(OptionBool::SomeTrue, 
	///     OptionBool::SomeFalse.map_bool(|b| !b));
	/// ```
	#[inline]
	pub fn map_bool<F>(self, f: F) -> OptionBool
	where F: FnOnce(bool) -> bool {
		match self {
			SomeTrue => if f(true) { 
				SomeTrue } else { SomeFalse },
			SomeFalse => if f(false) { 
				SomeTrue } else { SomeFalse },
			None => None,
		}
	}
	
	/// Maps a value to a U by applying the function or return a default U.
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
	where F: FnOnce(bool) -> U {
		match self {
			SomeTrue => f(true),
			SomeFalse => f(false),
			None => default,
		}
	}
	
	#[inline]
	pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U 
	where D: FnOnce() -> U, F: FnOnce(bool) -> U {
		match self {
			SomeTrue => f(true),
			SomeFalse => f(false),
			None => default(),
		}
	}
	
	#[inline]
	pub fn ok_or<E>(self, err: E) -> Result<bool, E> {
		match self {
			SomeTrue => Ok(true),
			SomeFalse => Ok(false),
			None => Err(err),
		}
	}
	
	#[inline]
	pub fn ok_or_else<E, F>(self, err: F) -> Result<bool, E> 
	where F: FnOnce() -> E {
		match self {
			SomeTrue => Ok(true),
			SomeFalse => Ok(false),
			None => Err(err()),
		}
	}
	
	#[inline]
	pub fn and<U>(self, optb: Option<U>) -> Option<U> {
		match self {
			SomeTrue | SomeFalse => optb,
			None => Option::None,
		}
	}
	
	#[inline]
	pub fn and_bool(self, optb: OptionBool) -> OptionBool {
		match self {
			SomeTrue | SomeFalse => optb,
			None => None,
		}
	}
	
	#[inline]
	pub fn and_then<U, F>(self, f: F) -> Option<U> 
	where F: FnOnce(bool) -> Option<U> {
		match self {
			SomeTrue => f(true),
			SomeFalse => f(false),
			None => Option::None,
		}
	}
	
	#[inline]
	pub fn and_then_bool<F>(self, f: F) -> OptionBool 
	where F: FnOnce(bool) -> OptionBool {
		match self {
			SomeTrue => f(true),
			SomeFalse => f(false),
			None => None,
		}
	}
	
	#[inline]
	pub fn or(self, optb: Option<bool>) -> Option<bool> {
		match self {
			SomeTrue => Some(true),
			SomeFalse => Some(false),
			None => optb,
		}
	}
	
	#[inline]
	pub fn or_bool(self, optb: OptionBool) -> OptionBool {
		match self {
			None => optb,
			x => x,
		}
	}
	
	#[inline]
	pub fn or_else<F>(self, f: F) -> Option<bool> 
	where F: FnOnce() -> Option<bool> {
		match self {
			SomeTrue => Option::Some(true),
			SomeFalse => Option::Some(false),
			None => f(),
		}
	}
	
	#[inline]
	pub fn or_else_bool<F>(self, f: F) -> OptionBool
	where F: FnOnce() -> OptionBool {
		match self {
			None => f(),
			x => x,
		}
	}
	
	#[inline]
	pub fn iter(&self) -> IterBool {
		IterBool{ inner: *self } // makes a copy, it's cheap
	}
	
	#[inline]
	pub fn take(&mut self) -> Option<bool> {
		mem::replace(self, None).into()
	}
}

impl Debug for OptionBool {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
		write!(f, "{}", match self {
			&SomeTrue => "Some(true)",
			&SomeFalse => "Some(false)",
			&None => "None",
		})
	}
}

impl Default for OptionBool {
	#[inline]
	fn default() -> OptionBool { None }
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
		match o {
			&SomeTrue => Option::Some(true),
			&SomeFalse => Option::Some(false),
			&None => Option::None,
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

/// A trait whose implementation for any type `T` allows the use of 
///`Optioned<T>`
pub trait Noned: Sized + Copy {
	/// Gibt `true` zurück, wenn der Wert dem None-Wert entspricht, sonst 
	/// `false`
	fn is_none(&self) -> bool;
	/// Ermittelt den None-Wert
	fn get_none() -> Self;
}

impl Noned for u8 { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::u8::MAX } 

	#[inline]
	fn get_none() -> u8 { std::u8::MAX }
}

impl Noned for u16 { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::u16::MAX } 

	#[inline]
	fn get_none() -> u16 { std::u16::MAX }
}

impl Noned for u32 { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::u32::MAX } 

	#[inline]
	fn get_none() -> u32 { std::u32::MAX }
}

impl Noned for u64 { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::u64::MAX } 

	#[inline]
	fn get_none() -> u64 { std::u64::MAX }
}

impl Noned for usize { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::usize::MAX } 

	#[inline]
	fn get_none() -> usize { std::usize::MAX }
}

impl Noned for i8 { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::i8::MIN } 

	#[inline]
	fn get_none() -> i8 { std::i8::MIN }
}

impl Noned for i16 { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::i16::MIN } 

	#[inline]
	fn get_none() -> i16 { std::i16::MIN }
}

impl Noned for i32 { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::i32::MIN } 

	#[inline]
	fn get_none() -> i32 { std::i32::MIN }
}

impl Noned for i64 { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::i64::MIN } 

	#[inline]
	fn get_none() -> i64 { std::i64::MIN }
}

impl Noned for isize { 
	#[inline]
	fn is_none(&self) -> bool { self == &std::isize::MIN } 

	#[inline]
	fn get_none() -> isize { std::isize::MIN }
}

impl Noned for f32 {
	#[inline]
	fn is_none(&self) -> bool { self.is_nan() }

	#[inline]
	fn get_none() -> f32 { std::f32::NAN }
}

impl Noned for f64 {
	#[inline]
	fn is_none(&self) -> bool { self.is_nan() }

	#[inline]
	fn get_none() -> f64 { std::f64::NAN }
}

/// An Option<T>-like structure that takes only as much space as the enclosed
/// value, at the cost of removing one particular None value from the value 
/// domain (see Noned)
#[derive(Copy, Clone)]
pub struct Optioned<T: Noned + Sized + Copy> { value: T }

impl<T> PartialEq for Optioned<T> where T: PartialEq + Noned + Sized + Copy {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		&self.value == &other.value
	}
}

impl<T> Eq for Optioned<T> where T: PartialEq + Noned + Sized + Copy + Eq {}

impl<T: Noned + Sized + Copy> Optioned<T> {
	/// Create an Optioned<T> that is some(t)
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
		Optioned::<T>{ value: t }
	}
	
	#[inline]
	pub fn none() -> Self {
		Optioned::<T>{ value: <T as Noned>::get_none() }
	}
	
	#[inline]
	fn as_option(&self) -> Option<T> {
		if self.value.is_none() { Option::None } else { Option::Some(self.value) }
	}
	
	#[inline]
	pub fn is_none(&self) -> bool {
		self.value.is_none()
	}
	
	#[inline]
	pub fn is_some(&self) -> bool {
		!self.value.is_none()
	}
	
	#[inline]
	pub fn expect(&self, msg: &str) -> T {
		if self.is_none() { panic!("{}", msg) }
		self.value
	}
	
	#[inline]
	pub fn unwrap(&self) -> T {
		self.expect("unwrap called on None")
	}
	
	#[inline]
	pub fn unwrap_or(&self, def: T) -> T {
		if self.is_none() { def } else { self.value }
	}
	
	#[inline]
	pub fn unwrap_or_else<F>(self, f: F) -> T where F: FnOnce() -> T {
		if self.is_none() { f() } else { self.value }
	}
	
	#[inline]
	pub fn map<U, F>(self, f: F) -> Option<U>
	where F: FnOnce(T) -> U {
		if self.is_none() { Option::None } else { Option::Some(f(self.value)) }
	}
	
	#[inline]
	pub fn map_t<U, F>(self, f: F) -> Self
	where F: FnOnce(T) -> T {
		if self.is_none() { self } else { Self::some(f(self.value)) }
	}
	
	#[inline]
	pub fn map_or<U, F>(self, default: U, f: F) -> U where F: FnOnce(T) -> U {
		if self.is_none() { default } else { f(self.value) }
	}
	
	#[inline]
	pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U 
	where D: FnOnce() -> U, F: FnOnce(T) -> U {
		if self.is_none() { default() } else { f(self.value) }
	}
	
	#[inline]
	pub fn take(&mut self) -> Option<T> {
		mem::replace(self, Self::none()).as_option()
	}
	
	#[inline]
	pub fn iter(&self) -> OptionedIter<T> {
		OptionedIter { o: *self } // make a copy
	}
}

impl<T: Noned + Sized + Copy + Debug> Debug for Optioned<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
		if self.is_none() {
			write!(f, "None")
		} else {
			write!(f, "Some({:?})", &self.value)
		}
	}
}

#[derive(Copy, Clone)]
pub struct OptionedIter<T: Noned + Sized + Copy> { o: Optioned<T> }

impl<T: Noned + Sized + Copy> Iterator for OptionedIter<T> {
	type Item=T;
	
	#[inline]
	fn next(&mut self) -> Option<T> {
		self.o.take()
	}
}

impl<'a, T: Noned + Sized + Copy> From<&'a Option<T>> for Optioned<T> {
	#[inline]
	fn from(o: &Option<T>) -> Optioned<T> {
		o.map_or_else(Self::none, Self::some)
	}
}

impl<T: Noned + Sized + Copy> From<Option<T>> for Optioned<T> {
	#[inline]
	fn from(o: Option<T>) -> Optioned<T> {
		o.map_or_else(Self::none, Self::some)
	}
}

impl<T: Noned + Sized + Copy> Into<Option<T>> for Optioned<T> {
	#[inline]
	fn into(self) -> Option<T> { self.as_option() }
}

#[test]
fn into_option_bool() {
	let optionals = [ OptionBool::some(true), OptionBool::some(false), OptionBool::none() ];
	
	for o in optionals.iter() {
		let opt : Option<bool> = o.into();
		let o2 : OptionBool = opt.into();
		assert!(o == o2);
	}
}

#[test]
fn test_bool_map() {
	let optionals = [ OptionBool::some(true), OptionBool::some(false), OptionBool::none() ];
	
	for o in optionals.iter() {
		assert!(o == o.map_bool(|b| b));
		let opt : Option<bool> = **o; // double deref for &
		assert!(opt == o.map(|b| b));
	}
	
	assert!(SomeTrue == SomeFalse.map_bool(|b| !b))
}

#[test]
fn deref_to_option() {
	assert!(*OptionBool::some(true) == Option::Some(true));
	assert!(*OptionBool::some(false) == Option::Some(false));
	assert!(*OptionBool::none() == Option::None);
}

#[test]
fn optioned_is_some_or_none() {
	let opt_u32 : Optioned<u32> = Optioned::some(32);
	assert!(opt_u32.is_some());
	
	let opt_u32_none : Optioned<u32> = Optioned::none();
	assert!(opt_u32_none.is_none());
}
