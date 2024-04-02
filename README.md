<div id="header" align="center">

  <b>[copy_impl]</b>
  
  (Macro for effortlessly duplicating impl block code across various types in Rust.)
  </br></br>

<div id="badges">
  <a href="./LICENSE_MIT">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/mit.png?raw=true" alt="mit"/>
  </a>
  <a href="./LICENSE_APACHE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/copy_impl">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/copy_impl">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>
	
  [![CI](https://github.com/clucompany/copy_impl/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/copy_impl/actions/workflows/CI.yml) 


</div>
</div>

## Usage:

Add this to your Cargo.toml:

```toml
[dependencies]
copy_impl = "0.3.3"
```

and this to your source code:
```rust
use copy_impl::copy_impl;
```

## Example:

```rust
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
```

<a href="./examples">
  See all
</a>

## License:
This project has a dual license according to (LICENSE-MIT) and (LICENSE-APACHE-2-0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2021-2024 #UlinProject</b>
	
  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License:
<div align="left">
  <a href="./LICENSE_APACHE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>
    
  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>

### MIT License:
<div align="left">
  <a href="./LICENSE_MIT">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/mit.png?raw=true" alt="mit"/>
  </a>
  <b>&nbsp;Licensed under the MIT License.</b>
  </br></br></br></br>
</div>
