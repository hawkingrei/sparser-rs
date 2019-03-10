extern crate libc;
extern crate rand;
pub mod bitmap;
pub mod decompose_ascii_rawfilters;
pub mod sparser;
pub mod sparser_kernels;
pub mod utils;

#[cfg(target_arch = "x86")]
use std::arch::x86::_rdtsc;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;

#[inline(always)]
pub fn rdtsc() -> i64 {
    unsafe { _rdtsc() }
}
