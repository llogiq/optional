use std::cmp::Ordering;
use std::convert::From;
use std::iter::Iterator;
use std::mem;
use std::ops::Deref;
use std::fmt::{self, Debug, Error};

const OB_SOME_TRUE : Option<bool> = Some(true);
const OB_SOME_FALSE : Option<bool> = Some(false);
const OB_NONE : Option<bool> = None;

const OB_SOME_TRUE_REF : &'static Option<bool> = &OB_SOME_TRUE;
const OB_SOME_FALSE_REF : &'static Option<bool> = &OB_SOME_FALSE;
const OB_NONE_REF : &'static Option<bool> = &OB_NONE;

#[derive(Copy, Clone, Eq, Ord, Hash)]
pub enum OptionBool {
	SomeTrue,
	SomeFalse,
	None,
}

impl Deref for OptionBool {
	type Target = Option<bool>;
	
	fn deref(&self) -> &'static Option<bool> {
		match self {
			&OptionBool::SomeTrue => OB_SOME_TRUE_REF,
			&OptionBool::SomeFalse => OB_SOME_FALSE_REF,
			&OptionBool::None => OB_NONE_REF,
		}
	}
}

pub struct IterBool {
	inner: OptionBool,
}

impl<'a> Iterator for IterBool {
	type Item=bool;
	
	fn next(&mut self) -> Option<bool> {
		self.inner.take()
	}
}

impl PartialEq for OptionBool {
	fn eq(&self, other: &OptionBool) -> bool {
		match (self, other) {
			(&OptionBool::SomeTrue, &OptionBool::SomeTrue) |
			(&OptionBool::SomeFalse, &OptionBool::SomeFalse) |
			(&OptionBool::None, &OptionBool::None) => true,
			_ => false,
		}
	}
}

impl<'a> PartialEq<OptionBool> for &'a OptionBool {
	fn eq(&self, other: &OptionBool) -> bool {
		match (*self, other) {
			(&OptionBool::SomeTrue, &OptionBool::SomeTrue) |
			(&OptionBool::SomeFalse, &OptionBool::SomeFalse) |
			(&OptionBool::None, &OptionBool::None) => true,
			_ => false,
		}
	}
}

impl PartialOrd for OptionBool {
	fn partial_cmp(&self, other: &OptionBool) -> Option<Ordering> {
		match (self, other) {
			(&OptionBool::SomeTrue, &OptionBool::SomeTrue) |
			(&OptionBool::SomeFalse, &OptionBool::SomeFalse) |
			(&OptionBool::None, &OptionBool::None) => Some(Ordering::Equal),
			(&OptionBool::SomeTrue, &OptionBool::SomeFalse) |
			(&OptionBool::SomeTrue, &OptionBool::None) |
			(&OptionBool::SomeFalse, &OptionBool::None) => 
				Some(Ordering::Greater),
			_ => Some(Ordering::Less),
		}
	}
}

impl OptionBool {
	pub fn some(b: bool) -> Self {
		if b { OptionBool::SomeTrue } else { OptionBool::SomeFalse }
	}
	
	pub fn none() -> Self { OptionBool::None }
	
	pub fn is_some(&self) -> bool {
		if let &OptionBool::None = self { false } else { true }
	}
	
	pub fn is_none(&self) -> bool {
		if let &OptionBool::None = self { true } else { false }
	}
	
	pub fn expect(&self, msg: &str) -> bool {
		match self {
			&OptionBool::SomeTrue => true,
			&OptionBool::SomeFalse => false,
			&OptionBool::None => panic!("{}", msg)
		}
	}
	
	pub fn unwrap(&self) -> bool {
		self.expect("unwrap called on None")
	}
	
	pub fn unwrap_or(&self, def: bool) -> bool {
		match self {
			&OptionBool::SomeTrue => true,
			&OptionBool::SomeFalse => false,
			&OptionBool::None => def,
		}
	}
	
	pub fn unwrap_or_else<F>(self, f: F) -> bool where F: FnOnce() -> bool {
		match self {
			OptionBool::SomeTrue => true,
			OptionBool::SomeFalse => false,
			OptionBool::None => f(),
		}
	}
	
	pub fn map<U, F>(self, f: F) -> Option<U> 
	where F: FnOnce(bool) -> U {
		match self {
			OptionBool::SomeTrue => Some(f(true)),
			OptionBool::SomeFalse => Some(f(false)),
			OptionBool::None => None,
		}
	}
	
	pub fn map_bool<F>(self, f: F) -> OptionBool
	where F: FnOnce(bool) -> bool {
		match self {
			OptionBool::SomeTrue => if f(true) { 
				OptionBool::SomeTrue } else { OptionBool::SomeFalse },
			OptionBool::SomeFalse => if f(false) { 
				OptionBool::SomeTrue } else { OptionBool::SomeFalse },
			OptionBool::None => OptionBool::None,
		}
	}
	
	pub fn map_or<U, F>(self, default: U, f: F) -> U 
	where F: FnOnce(bool) -> U {
		match self {
			OptionBool::SomeTrue => f(true),
			OptionBool::SomeFalse => f(false),
			OptionBool::None => default,
		}
	}
	
	pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U 
	where D: FnOnce() -> U, F: FnOnce(bool) -> U {
		match self {
			OptionBool::SomeTrue => f(true),
			OptionBool::SomeFalse => f(false),
			OptionBool::None => default(),
		}
	}
	
	pub fn ok_or<E>(self, err: E) -> Result<bool, E> {
		match self {
			OptionBool::SomeTrue => Ok(true),
			OptionBool::SomeFalse => Ok(false),
			OptionBool::None => Err(err),
		}
	}
	
	pub fn ok_or_else<E, F>(self, err: F) -> Result<bool, E> 
	where F: FnOnce() -> E {
		match self {
			OptionBool::SomeTrue => Ok(true),
			OptionBool::SomeFalse => Ok(false),
			OptionBool::None => Err(err()),
		}
	}
	
	pub fn and<U>(self, optb: Option<U>) -> Option<U> {
		match self {
			OptionBool::SomeTrue | OptionBool::SomeFalse => optb,
			OptionBool::None => None,
		}
	}
	
	pub fn and_bool(self, optb: OptionBool) -> OptionBool {
		match self {
			OptionBool::SomeTrue | OptionBool::SomeFalse => optb,
			OptionBool::None => OptionBool::None,
		}
	}
	
	pub fn and_then<U, F>(self, f: F) -> Option<U> 
	where F: FnOnce(bool) -> Option<U> {
		match self {
			OptionBool::SomeTrue => f(true),
			OptionBool::SomeFalse => f(false),
			OptionBool::None => None,
		}
	}
	
	pub fn and_then_bool<F>(self, f: F) -> OptionBool 
	where F: FnOnce(bool) -> OptionBool {
		match self {
			OptionBool::SomeTrue => f(true),
			OptionBool::SomeFalse => f(false),
			OptionBool::None => OptionBool::None,
		}
	}
	
	pub fn or(self, optb: Option<bool>) -> Option<bool> {
		match self {
			OptionBool::SomeTrue => Some(true),
			OptionBool::SomeFalse => Some(false),
			OptionBool::None => optb,
		}
	}
	
	pub fn or_bool(self, optb: OptionBool) -> OptionBool {
		match self {
			OptionBool::None => optb,
			x => x,
		}
	}
	
	pub fn or_else<F>(self, f: F) -> Option<bool> 
	where F: FnOnce() -> Option<bool> {
		match self {
			OptionBool::SomeTrue => Some(true),
			OptionBool::SomeFalse => Some(false),
			OptionBool::None => f(),
		}
	}
	
	pub fn or_else_bool<F>(self, f: F) -> OptionBool
	where F: FnOnce() -> OptionBool {
		match self {
			OptionBool::None => f(),
			x => x,
		}
	}
	
	pub fn iter(&self) -> IterBool {
		IterBool{ inner: *self } // makes a copy, it's cheap
	}
	
	pub fn take(&mut self) -> Option<bool> {
		mem::replace(self, OptionBool::None).into()
	}
}

impl Debug for OptionBool {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
		write!(f, "{}", match self {
			&OptionBool::SomeTrue => "Some(true)",
			&OptionBool::SomeFalse => "Some(false)",
			&OptionBool::None => "None",
		})
	}
}

impl Default for OptionBool {
	fn default() -> OptionBool { OptionBool::None }
}

impl From<OptionBool> for Option<bool> {
	fn from(o: OptionBool) -> Option<bool> {
		match o {
			OptionBool::SomeTrue => Some(true),
			OptionBool::SomeFalse => Some(false),
			OptionBool::None => None,
		}
	}
}

impl<'a> From<&'a OptionBool> for Option<bool> {
	fn from(o: &'a OptionBool) -> Option<bool> {
		match o {
			&OptionBool::SomeTrue => Some(true),
			&OptionBool::SomeFalse => Some(false),
			&OptionBool::None => None,
		}
	}
}

impl From<Option<bool>> for OptionBool {
	fn from(o: Option<bool>) -> Self {
		match o {
			Option::Some(true) => OptionBool::SomeTrue,
			Option::Some(false) => OptionBool::SomeFalse,
			Option::None => OptionBool::None,
		}
	}
}

pub trait Noned: Sized {
	fn is_none(&self) -> bool;
	fn get_none() -> Self;
}

impl Noned for u8 { 
	fn is_none(&self) -> bool { self == &std::u8::MAX } 
	fn get_none() -> u8 { std::u8::MAX }
}

impl Noned for u16 { 
	fn is_none(&self) -> bool { self == &std::u16::MAX } 
	fn get_none() -> u16 { std::u16::MAX }
}

impl Noned for u32 { 
	fn is_none(&self) -> bool { self == &std::u32::MAX } 
	fn get_none() -> u32 { std::u32::MAX }
}

impl Noned for u64 { 
	fn is_none(&self) -> bool { self == &std::u64::MAX } 
	fn get_none() -> u64 { std::u64::MAX }
}

impl Noned for usize { 
	fn is_none(&self) -> bool { self == &std::usize::MAX } 
	fn get_none() -> usize { std::usize::MAX }
}

impl Noned for i8 { 
	fn is_none(&self) -> bool { self == &std::i8::MIN } 
	fn get_none() -> i8 { std::i8::MIN }
}

impl Noned for i16 { 
	fn is_none(&self) -> bool { self == &std::i16::MIN } 
	fn get_none() -> i16 { std::i16::MIN }
}

impl Noned for i32 { 
	fn is_none(&self) -> bool { self == &std::i32::MIN } 
	fn get_none() -> i32 { std::i32::MIN }
}

impl Noned for i64 { 
	fn is_none(&self) -> bool { self == &std::i64::MIN } 
	fn get_none() -> i64 { std::i64::MIN }
}

impl Noned for isize { 
	fn is_none(&self) -> bool { self == &std::isize::MIN } 
	fn get_none() -> isize { std::isize::MIN }
}

impl Noned for f32 {
	fn is_none(&self) -> bool { self.is_nan() }
	fn get_none() -> f32 { std::f32::NAN }
}

impl Noned for f64 {
	fn is_none(&self) -> bool { self.is_nan() }
	fn get_none() -> f64 { std::f64::NAN }
}

#[derive(Copy, Clone)]
pub struct Optioned<T: Noned + Sized + Copy> { value: T }

impl<T> PartialEq for Optioned<T> where T: PartialEq + Noned + Sized + Copy {
	fn eq(&self, other: &Self) -> bool {
		&self.value == &other.value
	}
}

impl<T> Eq for Optioned<T> where T: PartialEq + Noned + Sized + Copy + Eq {}

impl<T: Noned + Sized + Copy> Optioned<T> {
	pub fn some(t: T) -> Self {
		debug_assert!(!t.is_none());
		Optioned{ value: t }
	}
	
	pub fn none() -> Self {
		Optioned{ value: <T as Noned>::get_none() }
	}
	
	fn as_option(&self) -> Option<T> {
		if self.value.is_none() { None } else { Some(self.value) }
	}
	
	pub fn is_none(&self) -> bool {
		self.value.is_none()
	}
	
	pub fn is_some(&self) -> bool {
		!self.value.is_none()
	}
	
	pub fn expect(&self, msg: &str) -> T {
		if self.is_none() { panic!("{}", msg) }
		self.value
	}
	
	pub fn unwrap(&self) -> T {
		self.expect("unwrap called on None")
	}
	
	pub fn unwrap_or(&self, def: T) -> T {
		if self.is_none() { def } else { self.value }
	}
	
	pub fn unwrap_or_else<F>(self, f: F) -> T where F: FnOnce() -> T {
		if self.is_none() { f() } else { self.value }
	}
	
	pub fn map<U, F>(self, f: F) -> Option<U>
	where F: FnOnce(T) -> U {
		if self.is_none() { None } else { Some(f(self.value)) }
	}
	
	pub fn map_t<U, F>(self, f: F) -> Self
	where F: FnOnce(T) -> T {
		if self.is_none() { self } else { Self::some(f(self.value)) }
	}
	
	pub fn map_or<U, F>(self, default: U, f: F) -> U where F: FnOnce(T) -> U {
		if self.is_none() { default } else { f(self.value) }
	}
	
	pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U 
	where D: FnOnce() -> U, F: FnOnce(T) -> U {
		if self.is_none() { default() } else { f(self.value) }
	}
	
	pub fn take(&mut self) -> Option<T> {
		mem::replace(self, Self::none()).as_option()
	}
	
	pub fn iter(&self) -> Optioned<T> {
		*self // make a copy
	}
}

impl<T: Noned + Sized + Copy> Iterator for Optioned<T> {
	type Item=T;
	
	fn next(&mut self) -> Option<T> {
		self.take()
	}
}

//impl<T: Noned + Sized + Copy> Into<Option<T>> for Optioned<T> {
//	fn into(self) -> Option<T> { self.as_option() }
//}

impl<'a, T: Noned + Sized + Copy> From<&'a Option<T>> for Optioned<T> {
	fn from(o: &Option<T>) -> Optioned<T> {
		o.map_or_else(Self::none, Self::some)
	}
}

impl<T: Noned + Sized + Copy> From<Option<T>> for Optioned<T> {
	fn from(o: Option<T>) -> Optioned<T> {
		o.map_or_else(Self::none, Self::some)
	}
}

impl<T: Noned + Sized + Copy> Into<Option<T>> for Optioned<T> {
	fn into(self) -> Option<T> { self.as_option() }
}

#[test]
fn it_works() {
	let optionals = [ 
		OptionBool::some(true), OptionBool::some(false), OptionBool::none() ];
	
	for o in optionals.iter() {
		let opt : Option<bool> = o.into();
		let o2 : OptionBool = opt.into();
		assert!(o == o2);
	}
	
	let opt_u32 : Optioned<u32> = Optioned::some(32);
	assert!(opt_u32.is_some());
	
	let opt_u32_none : Optioned<u32> = Optioned::none();
	assert!(opt_u32_none.is_none());
}
