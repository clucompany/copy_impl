
use std::{error::Error, fmt::Write};
use copy_impl::copy_impl;

struct CustomNum<T>(T);

struct UncheckedCustomNum<T>(T);

copy_impl! {
	impl (CustomNum<i8>),
	impl (CustomNum<i16>),
	impl (CustomNum<i32>),
	impl (UncheckedCustomNum<i8>),
	impl (UncheckedCustomNum<i16>) {
		pub fn write_to(&self, mut w: impl Write) -> Result<(), std::fmt::Error> {
			write!(w, "{}", self.0)
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut tbuff = String::new();
	CustomNum(1i8).write_to(&mut tbuff)?;
	CustomNum(2i16).write_to(&mut tbuff)?;
	CustomNum(3i32).write_to(&mut tbuff)?;
	
	UncheckedCustomNum(4i8).write_to(&mut tbuff)?;
	UncheckedCustomNum(5i16).write_to(&mut tbuff)?;
	// UncheckedCustomNum(6i32).write_to(&mut tbuff)?;
	/*
		no method named `write_to` found for struct `UncheckedCustomNum<i32>` in the current scope
		the method was found for
		- `UncheckedCustomNum<i8>`
		- `UncheckedCustomNum<i16>`
	*/
	
	assert_eq!(tbuff, "12345");
	
	Ok(())
}
