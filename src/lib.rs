#![feature(ptr_wrapping_offset_from)]
#![feature(repeat_generic_slice)]

pub mod bitmap;
pub mod common;
pub mod decompose_ascii_rawfilters;
pub mod sparser;
pub mod sparser_kernels;

use std::convert::TryInto;

#[cfg(target_arch = "x86")]
use std::arch::x86::_rdtsc;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;

#[inline(always)]
pub fn rdtsc() -> i64 {
    unsafe { _rdtsc().try_into().unwrap() }
}
