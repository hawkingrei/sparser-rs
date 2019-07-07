#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline]
fn ffs(x: i32) -> i32 {
    let mut r: i32 = 1;
    let mut val = x;
    if (val == 0) {
        return val;
    }
    if ((val & 0xffff) == 0) {
        val >>= 16;
        r += 16;
    }
    if ((val & 0xff) == 0) {
        val >>= 8;
        r += 8;
    }
    if ((val & 0xf) == 0) {
        val >>= 4;
        r += 4;
    }
    if ((val & 3) == 0) {
        val >>= 2;
        r += 2;
    }
    if ((val & 1) == 0) {
        val >>= 1;
        r += 1;
    }
    return r;
}

#[test]
fn test_ffs() {
    assert!(ffs(1) == 1);
    assert!(ffs(16) == 5);
    assert!(ffs(64) == 7);
}

pub unsafe fn search(reg: __m256i, base: Vec<u8>) -> bool {
    match base.len() {
        1 => {
            let base_ptr = _mm256_set1_epi8(*base.get(0).unwrap() as i8);
            let base_req: __m256i = _mm256_loadu_si256(&base_ptr);
            let result = search_epi8(reg, base_req);
        }
        2 => {
            let base_ptr = base.repeat(32).as_ptr() as *const __m256i;
        }
        4 => return true,
        _ => return false,
    }

    return true;
}

pub fn memmem(reg: &Vec<u8>, base: Vec<u8>) -> bool {
    let mut local_reg = reg.clone();
    if local_reg.len() < 32 {
        local_reg.resize_with(32, Default::default);
    } else if local_reg.len() == 32 {

    } else {

    }

    return true;
}

/** Search for an 8-bit search string.
 *
 * @param reg the register filled with the search value
 * @param base the data to search. Should be at least 32 bytes long.
 *
 * @return the number of matches found.
 */
#[inline]
pub fn search_epi8(reg: __m256i, base: __m256i) -> u32 {
    let mut count = 0;
    unsafe {
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(reg, base));
        while (mask != 0) {
            let index = ffs(mask) - 1;
            mask &= !(1 << index);
            count = count + 1;
        }
    }
    return count;
}

/** Search for an 16-bit search string.
 *
 * @param reg the register filled with the search value
 * @param base the data to search. Should be at least 32 bytes long.
 *
 * @return the number of matches found.
 */
#[inline]
pub fn search_epi16(reg: __m256i, base: __m256i) -> u32 {
    let mut count = 0;
    unsafe {
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi16(reg, base));
        mask &= 0x55555555;
        while (mask != 0) {
            let index = ffs(mask) - 1;
            mask &= !(1 << index);
            count = count + 1;
        }
    }
    return count;
}

/** Search for an 32-bit search string.
 *
 * @param reg the register filled with the search value
 * @param base the data to search. Should be at least 32 bytes long.
 *
 * @return the number of matches found.
 */
#[inline]
pub fn search_epi32(reg: __m256i, base: __m256i) -> u32 {
    let mut count = 0;
    unsafe {
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi32(reg, base));
        println!("{:?}", mask);
        mask = mask & 0x11111111;
        while (mask != 0) {
            let index = ffs(mask) - 1;
            mask &= !(1 << index);
            count = count + 1;
        }
    }
    return count;
}

#[cfg(test)]
mod test {
    use crate::sparser_kernels::search_epi16;
    use crate::sparser_kernels::search_epi32;
    use crate::sparser_kernels::search_epi8;
    use std::arch::x86_64::__m256i;
    use std::arch::x86_64::_mm256_loadu_si256;

    #[test]
    fn test_search_epi32() {
        unsafe {
            let mut load_bytes = "an i an interactive reference tool ".as_bytes().to_vec();
            load_bytes.resize_with(32, Default::default);
            let lb_ptr = load_bytes.as_slice().as_ptr();
            let req: __m256i = _mm256_loadu_si256(lb_ptr as *const __m256i);
            let base: &[u8] = "an ian ian ian ian ian ian ian i".as_bytes();
            let base_req: __m256i = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
            let result = search_epi32(req, base_req);
            assert_eq!(result, 1);
        }
    }

    #[test]
    fn test_search_epi8() {
        unsafe {
            let mut load_bytes = "an i an interactive reference tool ".as_bytes().to_vec();
            load_bytes.resize_with(32, Default::default);
            let lb_ptr = load_bytes.as_slice().as_ptr();
            let req: __m256i = _mm256_loadu_si256(lb_ptr as *const __m256i);
            let base: &[u8] = "nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn".as_bytes();
            let base_req: __m256i = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
            let result = search_epi8(req, base_req);
            assert_eq!(result, 4);
        }
    }

    #[test]
    fn test_search_epi16() {
        unsafe {
            let mut load_bytes = "an i an interactive reference tool ".as_bytes().to_vec();
            load_bytes.resize_with(32, Default::default);
            let lb_ptr = load_bytes.as_slice().as_ptr();
            let req: __m256i = _mm256_loadu_si256(lb_ptr as *const __m256i);
            let base: &[u8] = "an".as_bytes();
            let base_req: __m256i = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
            let result = search_epi16(req, base_req);
            assert_eq!(result, 1);
        }
    }
}
