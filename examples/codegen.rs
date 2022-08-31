
use core::fmt::Debug;
use core::marker::PhantomData;
use copy_impl::copy_impl;

#[derive(Debug)]
pub struct PlatformData<D, CODE> {
	#[allow(dead_code)]
	data: D,
	
	p2: PhantomData<CODE>
}

enum NumT {}
enum StringT {}

impl<D, CODE> PlatformData<D, CODE> {
	#[inline]
	pub const fn new(data: D) -> Self {
		Self {
			data,
			p2: PhantomData,
		}
	}
}

copy_impl! {
	impl (PlatformData<usize, NumT>) +
	impl (PlatformData<u64, NumT>) +
	impl (PlatformData<u32, NumT>) +
	impl (PlatformData<u16, NumT>) +
	impl (PlatformData<u8, NumT>) {
		#[inline]
		pub const fn is_num(&self) -> bool {
			true
		}
		
		#[inline]
		pub fn make_debug(&self) -> &impl Debug {
			&self.data
		}
	};
	
	impl (PlatformData<usize, StringT>) +
	impl (PlatformData<u64, StringT>) +
	impl (PlatformData<u32, StringT>) +
	impl (PlatformData<u16, StringT>) +
	impl (PlatformData<u8, StringT>) {
		#[inline]
		pub const fn is_num(&self) -> bool {
			false
		}
		
		#[inline]
		pub fn make_debug(&self) -> &impl Debug {
			&self.data as _
		}
		
		#[inline]
		pub fn get_staticstr(&self) -> &'static str {
			"test"
		}
	}
}

fn main() {
	let a_num: PlatformData<_, NumT> = PlatformData::new(255u8);
	let a_num2: PlatformData<_, NumT> = PlatformData::new(255u8);
	let a_string: PlatformData<_, StringT> = PlatformData::new(255u8);
	let a_string2: PlatformData<_, StringT> = PlatformData::new(255u8);
	
	assert_eq!(a_num.is_num(), true);
	assert_eq!(a_num2.is_num(), true);
	
	// let astr = a_num.get_staticstr(); // <-- no method named `get_staticstr` found for struct `PlatformData<u8, NumT>` in the current scope
	
	assert_eq!(a_string.is_num(), false);
	assert_eq!(a_string2.is_num(), false);
	
	let astr = a_string.get_staticstr();
	assert_eq!(astr, "test");
}
