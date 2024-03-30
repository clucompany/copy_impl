<div id="header" align="center">

  <b>[copy_impl]</b>
  
  (Macro for effortlessly duplicating impl block code across various types in Rust.)
  </br></br>

<div id="badges">
  <a href="./LICENSE-MIT">
    <img src="https://github.com/UlinProject/img/blob/main/short_70/mit.png" alt="mit" style="height: 32px; max-width: 100%;"/>
  </a>
  <a href="./LICENSE-APACHE">
    <img src="https://github.com/UlinProject/img/blob/main/short_70/apache2.png" alt="apache2" style="height: 32px; max-width: 100%;"/>
  </a>
  <a href="https://crates.io/copy_impl">
    <img src="https://github.com/UlinProject/img/blob/main/short_70/cratesio.png" alt="cratesio" style="height: 32px; max-width: 100%;"/>
  </a>
  <a href="https://doc.rs/copy_impl">
    <img src="https://github.com/UlinProject/img/blob/main/short_70/docrs.png" alt="docrs" style="height: 32px; max-width: 100%;"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_70/uproject.png" alt="uproject" style="height: 32px; max-width: 100%;"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_70/clulab.png" alt="clulab" style="height: 32px; max-width: 100%;"/>
  </a>
	
  [![CI](https://github.com/clucompany/copy_impl/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/copy_impl/actions/workflows/CI.yml) 


</div>
</div>

## Usage:

Add this to your Cargo.toml:

```toml
[dependencies]
copy_impl = "0.3.0"
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

<a href="https://github.com/clucompany/copy_impl/tree/master/examples">
  See all
</a>

## License:
This project has a dual license according to (LICENSE-MIT) and (LICENSE-APACHE-2-0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_450_220/uproject.png" alt="uproject" style="height: 100px; width: 220px;"/>
  </a>
  
  <b>Copyright (c) 2021-2024 #UlinProject (Denis Kotlyarov).</b>
  </br></br></br></br>
</div>

### Apache License:
<div align="left">
  <a href="https://www.apache.org/licenses/LICENSE-2.0">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_450_220/apache2.png" alt="apache2" style="height: 100px; width: 220px;"/>
  </a>
  
  <b>Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>

### MIT License:
<div align="left">
  <a href="https://mit-license.org/">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_450_220/mit.png" alt="mit" style="height: 100px; width: 220px;"/>
  </a>
  
  <b>Licensed under the MIT License.</b>
  </br></br></br></br>
</div>
