
#![allow(non_snake_case)]
//Copyright 2021 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 2021
/*!


*/

#![no_std]

#[macro_export]
macro_rules! code_copy {
	[
		$(
			[ // MRESULT
				$($result:tt)*
			]
			[ // MDATA_ARRAY
				[	$($impls_data:tt)*				]
				[	$($code_block:tt)*				]
			]
		)?
		
		impl [$($impls_ident:tt)*]:
		
		$($all:tt)*
	] => {
		$crate::code_copy! {
			[
				$($($result)*)?
			]
			[
				[ // IMPLS DATA
					$($($impls_data)*)?
					[ // impl data
						[ ]
						[ $($impls_ident)* ]
					]
				]
				[	$($($code_block)*)? ]
			]
			
			$($all)*
		}
	};
	[
		$(
			[ // MRESULT
				$($result:tt)*
			]
			[ // MDATA_ARRAY
				[	$($impls_data:tt)*				]
				[	$($code_block:tt)*				]
			]
		)?
		
		impl [$($prefix:tt)*] [$($impls_ident:tt)*]:
		
		$($all:tt)*
	] => {
		$crate::code_copy! {
			[
				$($($result)*)?
			]
			[
				[ // IMPLS DATA
					$($($impls_data)*)?
					
					[ // ADD impl data
						[ $($prefix)* ]
						[ $($impls_ident)* ]
					]
				]
				[	$($($code_block)*)?	]
			]
			
			$($all)*
		}
	};
	
	[
		$(
			[ // MRESULT
				$($result:tt)*
			]
			[ // MDATA_ARRAY
				[	$($impls_data:tt)*				]
				[	$($code_block:tt)*				]
			]
		)?
		
		@code {
			$($code:tt)*
		}
		$($all:tt)*
	] => {
		$crate::code_copy! {
			[ // MRESULT
				$($($result)*)?
				[
					[	$($($impls_data)*)?				]
					[
						$($($code_block)*)?
						$($code)*
					]
				]
			]
			[ // MDATA_ARRAY
				[][]
			]
			
			$($all)*
		}
	};

	
	[ // FULL_END
		[ // MRESULT
			$(
				[
					[	$($all_impls:tt)*				]
					[	$($code_block:tt)*				]
				]
			)*
		]
		[ // MDATA_ARRAY
			[][]
		]
	] => {
		$crate::__code_copy! {
			$([
				[	$($all_impls)*				]
				[	$($code_block)*			]
			])*
		}
	};
	
	[ // FULL_END, EMPTY
		[ // MRESULT
		]
		[ // MDATA_ARRAY
			[][]
		]
	] => {
		
	};
	
	[ // UNKNOWN
		$(
			[ // MRESULT
				$($result:tt)*
			]
			[ // MDATA_ARRAY
				$($all0:tt)*
			]
		)?
		
		$($all:tt)*
	] => {
		compile_error!(
			concat!(
				"Unknown syntax, ",
				stringify!($($all)*)
			)
		);
	};
}

#[macro_export]
macro_rules! __code_copy {
	[
		$([
			[
				[
					[  $($impl_prefix:tt)*	]
					[ $($impl_ident:tt)*		]
				]
				
				$($all_impls:tt)*
			]
			[	$($code_block:tt)*				]
		])*
	] => {
		$(
			impl < $($impl_prefix)* > $($impl_ident)* {
				$($code_block)*
			}
		)*
		
		$crate::__code_copy! {
			$([
				[	$($all_impls)*				]
				[	$($code_block)*			] // copy tree (tt)
			])*
		}
	};
	
	[
		$([
			[]
			[	$($code_block:tt)*				]
		])*
	] => {
		
	};
	
	[
		[]
	] => {};
	[] => {};
}