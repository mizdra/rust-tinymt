extern crate tinymt;

use tinymt::tinymt32;
use tinymt::tinymt32::*;
use std::iter;

const PARAM: Param = Param {
  mat1: 0x8F7011EE,
  mat2: 0xFC78FF1F,
  tmat: 0x3793fdff,
};
const STATUS: [u32; 4] = [0x63B07A71, 0x5740A11A, 0x3CFE1DE3, 0x08A80987];

#[test]
fn tinymt32_should_transfer_a_next_status() {
  let mut rng = from_status(PARAM, STATUS);

  for _i in 0..100 {
    rng.next_state();
  }

  assert_eq!(rng.status(), [0xB2A870B4, 0x497FA995, 0xC8514463, 0x87BC580F]);
}

#[test]
fn tinymt32_should_generate_a_random_number() {
  let mut rng = from_status(PARAM, STATUS);

  for _i in 0..100 {
    rng.next_state();
  }

  assert_eq!(rng.temper(), 0x34CC99F7);
}

#[test]
fn tinymt32_should_not_overwrite_the_internal_status() {
  let mut status = STATUS;
  let rng = from_status(PARAM, status);

  status[0] = 0;
  assert_eq!(rng.status(), [0x63B07A71, 0x5740A11A, 0x3CFE1DE3, 0x08A80987]);

  let mut internal_status = rng.status();
  internal_status[0] = 1;

  assert_eq!(rng.status(), [0x63B07A71, 0x5740A11A, 0x3CFE1DE3, 0x08A80987]);
}

#[test]
fn tinymt32_should_pass_check32_test() {
  let param = Param {
    mat1: 0x8F7011EE,
    mat2: 0xFC78FF1F,
    tmat: 0x3793fdff,
  };
  let seed = 1;
  let mut rng = from_seed(param, seed);

  let mut actual: Vec<u32> = Vec::new();
  for _i in 0..50 {
    actual.push(rng.gen());
  }

  let expected: Vec<u32> = "\
2545341989  981918433 3715302833 2387538352 3591001365 
3820442102 2114400566 2196103051 2783359912  764534509 
 643179475 1822416315  881558334 4207026366 3690273640 
3240535687 2921447122 3984931427 4092394160   44209675 
2188315343 2908663843 1834519336 3774670961 3019990707 
4065554902 1239765502 4035716197 3412127188  552822483 
 161364450  353727785  140085994  149132008 2547770827 
4064042525 4078297538 2057335507  622384752 2041665899 
2193913817 1080849512   33160901  662956935  642999063 
3384709977 1723175122 3866752252  521822317 2292524454
"
    .split(|c| c == ' ' || c == '\n')
    .filter(|num_str| num_str != &"")
    .map(|num_str| num_str.parse::<u32>().unwrap())
    .collect();
  
  assert_eq!(actual, expected);
}

#[test]
fn readme_example() {
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
