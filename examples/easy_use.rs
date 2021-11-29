
use core::marker::PhantomData;
use code_copy::code_copy;

#[derive(Debug)]
#[repr(transparent)]
pub struct PlatformData<P, N> {
	data: [u8; 256],
	
	p: PhantomData<P>,
	p2: PhantomData<N>
}

enum NumFlagMap {}
enum StringFlagMap {}

impl<P, N> PlatformData<P, N> {
	pub fn zeroed() -> Self {
		Self {
			data: unsafe { std::mem::zeroed() },
			
			p: PhantomData,
			p2: PhantomData,
		}
	}
}

code_copy! {
	impl[PlatformData<usize, NumFlagMap>]:
	impl[PlatformData<u64, NumFlagMap>]:
	impl[PlatformData<u32, NumFlagMap>]:
	impl[PlatformData<u16, NumFlagMap>]:
	impl[PlatformData<u8, NumFlagMap>]:
	@code {
		#[inline(always)]
		pub const fn is_num(&self) -> bool {
			true
		}
		
		#[inline(always)]
		pub const fn into_num(self) -> usize {
			self.data[0] as _ // todo, test code
		}
	}
	
	impl[PlatformData<usize, StringFlagMap>]:
	impl[PlatformData<u64, StringFlagMap>]:
	impl[PlatformData<u32, StringFlagMap>]:
	impl[PlatformData<u16, StringFlagMap>]:
	impl[PlatformData<u8, StringFlagMap>]:
	@code {
		#[inline(always)]
		pub const fn is_num(&self) -> bool {
			false
		}
		
		#[inline(always)]
		pub const fn into_num(self) -> usize {
			0
		}
	}
}

fn main() {
	let a_num: PlatformData<u64, NumFlagMap> = PlatformData::zeroed();
	let a_num2: PlatformData<u8, NumFlagMap> = PlatformData::zeroed();
	let a_string: PlatformData<u64, StringFlagMap> = PlatformData::zeroed();
	let a_string2: PlatformData<u8, StringFlagMap> = PlatformData::zeroed();
	
	assert_eq!(a_num.is_num(), true);
	assert_eq!(a_num2.is_num(), true);
	
	assert_eq!(a_string.is_num(), false);
	assert_eq!(a_string2.is_num(), false);
}
