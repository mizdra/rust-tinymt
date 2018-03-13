# rust-tinymt
The Rust implements of TinyMT for Pokémon RNG.


## :warning: Note :warning:
This library is not designed for common use, but for **Pokémon RNG**.
If you want to use for simulation and numerical analysis and so on,
**I recommend other library**.


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tinymt = { git = "https://github.com/mizdra/rust-tinymt" }
```

and this to your crate root:

```rust
extern crate tinymt;
```


## Usage
```rust
extern crate tinymt;

use tinymt::tinymt32;
use std::iter;

fn main() {
  let param = tinymt32::Param {
    mat1: 0x8F7011EE,
    mat2: 0xFC78FF1F,
    tmat: 0x3793fdff,
  };
  let seed = 1u32;
  let status: [u32; 4] = [0xCCA24D8, 0x11BA5AD5, 0xF2DAD045, 0xD95DD7B2];


  // tinymt32::from_seed(param, seed)
  let mut rng1 = tinymt32::from_seed(param, seed);
  assert_eq!(rng1.gen(), 2545341989);
  assert_eq!(rng1.gen(), 981918433);
  assert_eq!(
    iter::repeat(()).map(|()| rng1.gen()).take(2).collect::<Vec<u32>>(),
    [3715302833, 2387538352],
  );
  

  // tinymt32::from_status(param, status)
  let mut rng2 = tinymt32::from_status(param, status);
  rng2.next_state();
  assert_eq!(rng2.temper(), 2545341989);
  assert_eq!(rng2.gen(), 981918433);
  assert_eq!(rng2.temper(), 981918433);
  for _i in 0..2 { rng2.next_state(); }
  assert_eq!(rng1.status(), rng2.status());
}
```


## Suppoerted Feature of MersenneTwister-Lab/TinyMT
- [x] tinymt32
  - [x] tinymt32_next_state
  - [x] tinymt32_temper
  - [x] tinymt32_generate_uint32
  - [ ] tinymt32_generate_float
  - [ ] tinymt32_generate_float01
  - [ ] tinymt32_generate_float12
  - [ ] tinymt32_generate_floatOC
  - [ ] tinymt32_generate_floatOO
  - [ ] tinymt32_generate_32double
- [ ] tinymt64
  - [ ] tinymt64_next_state
  - [ ] tinymt64_temper
  - [ ] tinymt64_generate_uint64
  - [ ] tinymt64_generate_double
  - [ ] tinymt64_generate_double01
  - [ ] tinymt64_generate_double12
  - [ ] tinymt64_generate_doubleOC
  - [ ] tinymt64_generate_doubleOO

# License
- rust-tinymt: [LICENSE](https://raw.githubusercontent.com/mizdra/rust-tinymt/master/LICENSE)
- [MersenneTwister-Lab/TinyMT](https://github.com/MersenneTwister-Lab/TinyMT): [LICENSE_TINYMT](https://raw.githubusercontent.com/mizdra/rust-tinymt/master/LICENSE_TINYMT) (Original: [LICENSE.txt](https://raw.githubusercontent.com/MersenneTwister-Lab/TinyMT/master/LICENSE.txt))
