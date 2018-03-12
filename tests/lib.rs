extern crate tinymt;

use tinymt::tinymt32;

const PARAM: tinymt32::Param = tinymt32::Param {
  mat1: 0x8F7011EE,
  mat2: 0xFC78FF1F,
  tmat: 0x3793fdff,
};
const STATUS: [u32; 4] = [0x63B07A71, 0x5740A11A, 0x3CFE1DE3, 0x08A80987];

#[test]
fn tinymt32_should_transfer_a_next_status() {
  let mut rng = tinymt32::from_status(PARAM, STATUS);

  for _i in 0..100 {
    rng.next_state();
  }

  assert_eq!(rng.status(), [0xB2A870B4, 0x497FA995, 0xC8514463, 0x87BC580F]);
}

#[test]
fn tinymt32_should_generate_a_random_number() {
  let mut rng = tinymt32::from_status(PARAM, STATUS);

  for _i in 0..100 {
    rng.next_state();
  }

  assert_eq!(rng.temper(), 0x34CC99F7);
}

#[test]
fn tinymt32_should_not_overwrite_the_internal_status() {
  let mut status = STATUS;
  let rng = tinymt32::from_status(PARAM, status);

  status[0] = 0;
  assert_eq!(rng.status(), [0x63B07A71, 0x5740A11A, 0x3CFE1DE3, 0x08A80987]);

  let mut internal_status = rng.status();
  internal_status[0] = 1;

  assert_eq!(rng.status(), [0x63B07A71, 0x5740A11A, 0x3CFE1DE3, 0x08A80987]);
}
