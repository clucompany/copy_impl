//Copyright (c) 2022-2024 #UlinProject (Denis Kotlyarov)


//-----------------------------------------------------------------------------
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

// or

//-----------------------------------------------------------------------------
//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

#![doc = include_str!("../README.md")]

/*!


*/

#![allow(non_snake_case)]
#![no_std]

///
/// Macro for easily copying impl block code for different types.
/// ```rust
/// copy_impl! {
///	impl (CustomNum<i8>),
///	impl (CustomNum<i16>),
///	impl (CustomNum<i32>),
///	impl (UncheckedCustomNum<i8>),
///	impl (UncheckedCustomNum<i16>) {
///		pub fn write_to(&self, mut w: impl Write) -> Result<(), std::fmt::Error> {
///			write!(w, "{}", self.0)
///		}
///	}
///}
/// ```
#[macro_export]
macro_rules! copy_impl {
	[ /* COLD_START */
		impl $(<$($p_impl:tt)*>)? ($($impl:tt)*) $(where ($($where:tt)*))?
		
		$($all:tt)*
	] => {
		$crate::copy_impl! {
			[ // the COLD_START!
				[
					/* COPY_IMPL_DATA */
					[ 
						[ $($($p_impl)*)? ] // <T>
						[ $($impl)* ] // impldata
						[ $($($where)*)? ]
					]
				]
			]
			
			$($all)*
		}
	};
	
	[ /* HOT_CONTINUE */
		[
			[
				/* COPY_IMPL_DATA */
				$($all_copy_impl_data:tt)+
			]
		]
		
		, impl $(<$($p_impl:tt)*>)? ($($impl:tt)*) $(where ($($where:tt)*))?
		
		$($all:tt)*
	] => {
		$crate::copy_impl! {
			[
				[
					/* COPY_IMPL_DATA */
					$($all_copy_impl_data)+
					
					[ 
						[ $($($p_impl)*)? ] // <T>
						[ $($impl)* ] // impldata
						[ $($($where)*)? ]
					]
					/* all */
				]
			]
			
			$($all)*
		}
	};
	
	[ /* skip (,) */
		[
			[
				/* COPY_IMPL_DATA */
				$($all_copy_impl_data:tt)*
			]
		]
		,
		
		$($all:tt)*
	] => {
		$crate::copy_impl! {
			[
				[
					/* COPY_IMPL_DATA */
					$($all_copy_impl_data)*
				]
			]
			
			$($all)*
		}
	};
	
	[ // END HEADERS(IMPL), the CODE!
		[
			[ /* COPY_IMPL_DATA */
				$([
					[ $($p_impl:tt)* ]
					[ $($impl:tt)* ]
					[ $($where:tt)* ]
				])+
			]
		]
		{ $($code:tt)* }
		
		$(
			; $($all:tt)*
		)?
	] => {
		$crate::__internal_make_copy_impl! {
			[]
			[$($code)*] ->
			
			$([
				[$($p_impl)*] // <T>
				[$($impl)*] // impldata
				[$($where)*]
			])+
		}
		
		$(
			$crate::copy_impl! {
				$($all)*
			}
		)?
	};
	
	[ $(;)? ] => {};
	
	[ /* UNK */ $($all:tt)+ ] => {
		compile_error!(stringify!(
			$($all)+
		));
	}
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_make_copy_impl {
	[
		[
			/* RARRAY */
			$($rarray:tt)*
		]
		[$($code:tt)*] ->
		
		[ [$($p_impl:tt)*][$($impl:tt)*][$($where:tt)*] ]
		$($unk:tt)*
	] => {
		$crate::__internal_make_copy_impl! {
			[
				$($rarray)*
				[
					[$($p_impl)*]
					[$($impl)*]
					[$($where)*]
					[$($code)*]
				]
			]
			[$($code)*] ->
			
			$($unk)*
		}
	};
	[
		[
			$([
				[$($p_impl:tt)*]
				[$($impl:tt)*]
				[$($where:tt)*]
				[$($code:tt)*]
			])*
		]
		[$($_code:tt)*] ->
	] => {
		$(
			impl $(<$($p_impl)*>)? $($impl)* $(where $($where)*)? {
				$($code)*
			}
		)*
	}
}
