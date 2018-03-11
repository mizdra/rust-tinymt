extern crate tinymt;

use tinymt::tinymt32;

// #[test]
// fn tinymt32() {
//   let mut param = tinymt32::Param {
//     mat1: 0x8F7011EE,
//     mat2: 0xFC78FF1F,
//     tmat: 0x3793fdff
//   };
//   let rng = tinymt32::init(param, seed);
//   // assert_eq!(rng, tinymt32::Rng { status: [0, 0, 0, 0], param: param });

//   let v: Vec<u32> = iter::repeat(()).map(|()| rng.gen()).take(8).collect();
//   assert_eq!(v, []);
// }
