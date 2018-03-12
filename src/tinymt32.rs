const MIN_LOOP: i32 = 8;
const PRE_LOOP: i32 = 8;
const TINYMT32_MASK: u32 = 0x7fffffff;
const TINYMT32_SH0: u32 = 1;
const TINYMT32_SH1: u32 = 10;
const TINYMT32_SH8: u32 = 8;

pub struct Param {
  pub mat1: u32,
  pub mat2: u32,
  pub tmat: u32,
}

pub struct Rng {
  param: Param,
  status: [u32; 4],
}

impl Rng {
  pub fn status(&self) -> [u32; 4] {
    // Copy a array and return
    // ref: https://doc.rust-lang.org/std/primitive.array.html
    self.status
  }

  pub fn next_state(&mut self) {
    let mut x: u32;
    let mut y: u32;

    y = self.status[3];
    x = (self.status[0] & TINYMT32_MASK)
	    ^ self.status[1]
	    ^ self.status[2];
    x ^= x.wrapping_shl(TINYMT32_SH0);
    y ^= y.wrapping_shr(TINYMT32_SH0) ^ x;
    self.status[0] = self.status[1];
    self.status[1] = self.status[2];
    self.status[2] = x ^ y.wrapping_shl(TINYMT32_SH1);
    self.status[3] = y;
    self.status[1] ^= (-((y & 1) as i32) as u32) & self.param.mat1;
    self.status[2] ^= (-((y & 1) as i32) as u32) & self.param.mat2;
  }

  pub fn temper(&self) -> u32 {
    let mut t0: u32;
    let t1: u32;
    t0 = self.status[3];
    t1 = self.status[0].wrapping_add(self.status[2].wrapping_shr(TINYMT32_SH8));
    t0 ^= t1;
    t0 ^= (-((t1 & 1) as i32) as u32) & self.param.tmat;
    t0
  }

  pub fn gen(&mut self) -> u32 {
    self.next_state();
    self.temper()
  }
}

pub fn from_seed(param: Param, seed: u32) -> Rng {
  let mut status: [u32; 4] = [
    seed,
    param.mat1,
    param.mat2,
    param.tmat,
  ];
  for i in 1..MIN_LOOP {
    let a = (i as usize) & 3;
    let b = ((i as usize) - 1) & 3;
    status[a] ^= (i as u32).wrapping_add(
      1812433253u32.wrapping_mul(
        status[b] ^ status[b].wrapping_shr(30)
      )
    );
  }
  let status: [u32; 4] = period_certification(status);
  let mut rng = Rng { param: param, status: status };

  for _i in 0..PRE_LOOP {
	  rng.next_state();
  }

  rng
}

pub fn from_status(param: Param, status: [u32; 4]) -> Rng {
  Rng { param: param, status: status }
}

fn period_certification(status: [u32; 4]) -> [u32; 4] {
  if (status[0] & TINYMT32_MASK) == 0 &&
	  status[1] == 0 &&
	  status[2] == 0 &&
	  status[3] == 0
  {
      ['T' as u32, 'I' as u32, 'N' as u32, 'Y' as u32]
  } else {
    status
  }
}
