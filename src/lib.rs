use std::cmp::Ordering;
use std::convert::From;
use std::iter::Iterator;
use std::mem;
use std::fmt::{self, Debug, Error};

#[derive(Copy, Clone, Eq, Ord, Hash)]
pub enum OptionBool {
	SomeTrue,
	SomeFalse,
	None,
}

pub struct Iter {
	inner: OptionBool
}

impl<'a> Iterator for &'a mut Iter {
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
	
	pub fn iter(&self) -> Iter {
		Iter{ inner: *self } // makes a copy, it's cheap
	}
	
	//TODO: iter_mut
	
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

#[test]
fn it_works() {
	let optionals = [ OptionBool::SomeTrue, OptionBool::SomeFalse, OptionBool::None ];
	
	for o in optionals.iter() {
		let opt : Option<bool> = o.into();
		let o2 : OptionBool = opt.into();
		assert!(o == o2)
	}
}
