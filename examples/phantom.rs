
use core::fmt::Debug;
use core::marker::PhantomData;
use copy_impl::copy_impl;

#[derive(Debug)]
pub struct PlatformData<D, T>(D, PhantomData<T>);

enum Num {}
enum String2 {}

copy_impl! {
	impl (PlatformData<usize, Num>),
	impl (PlatformData<u64, Num>),
	impl (PlatformData<u32, Num>),
	impl (PlatformData<u16, Num>),
	impl (PlatformData<u8, Num>) {
		pub const fn is_num(&self) -> bool {
			true
		}
	};
	
	impl (PlatformData<usize, String2>),
	impl (PlatformData<u64, String2>),
	impl (PlatformData<u32, String2>),
	impl (PlatformData<u16, String2>),
	impl (PlatformData<u8, String2>) {
		pub const fn is_num(&self) -> bool {
			false
		}
		
		pub const fn get_staticstr(&self) -> &'static str {
			"test"
		}
	}
}

fn main() {
	let a_num: PlatformData<_, Num> = PlatformData(255u8, PhantomData);
	let a_num2: PlatformData<_, Num> = PlatformData(255u16, PhantomData);
	let a_string: PlatformData<_, String2> = PlatformData(255u8, PhantomData);
	let a_string2: PlatformData<_, String2> = PlatformData(255u64, PhantomData);
	
	assert_eq!(a_num.is_num(), true);
	assert_eq!(a_num2.is_num(), true);
	assert_eq!(a_string.is_num(), false);
	assert_eq!(a_string2.is_num(), false);
	
	let astr = a_string.get_staticstr();
	assert_eq!(astr, "test");
}
