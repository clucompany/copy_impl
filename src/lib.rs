
//Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 2022
/*!


*/

#![allow(non_snake_case)]
#![no_std]


#[macro_export]
macro_rules! copy_impl {
	[ /* START */
		impl $(<$($p_impl:tt)*>)? ($($impl:tt)*) $(where ($($where:tt)*))?
		
		$($all:tt)*
	] => {
		$crate::copy_impl! {
			[
				[
					/* IMPL_ARRAY */
					[ [$($($p_impl)*)?][$($impl)*][$($($where)*)?] ]
					/* all */
				]
			]
			
			$($all)*
		}
	};
	
	[ /* CONTINUE */
		[
			[
				/* IMPL_ARRAY */
				$($all_impl_array:tt)+
			]
		]
		
		+ impl $(<$($p_impl:tt)*>)? ($($impl:tt)*) $(where ($($where:tt)*))?
		
		$($all:tt)*
	] => {
		$crate::copy_impl! {
			[
				[
					/* IMPL_ARRAY */
					$($all_impl_array)+
					
					[ [$($($p_impl)*)?][$($impl)*][$($($where)*)?] ]
					/* all */
				]
			]
			
			$($all)*
		}
	};
	
	[
		[
			[ /* IMPL_ARRAY */
				$([
					[$($p_impl:tt)*][$($impl:tt)*][$($where:tt)*]
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
				[$($p_impl)*][$($impl)*][$($where)*]
			])+
		}
		
		$(
			$crate::copy_impl! {
				$($all)*
			}
		)?
	};
	
	[] => {};
	
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
				[ [$($p_impl)*][$($impl)*][$($where)*][$($code)*] ]
			]
			[$($code)*] ->
			
			$($unk)*
		}
	};
	[
		[
			$([ [$($p_impl:tt)*][$($impl:tt)*][$($where:tt)*][$($code:tt)*] ])*
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
