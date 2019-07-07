const WORDSZ: usize = 64;

use std::arch::x86_64::_popcnt64;

#[derive(Copy, Clone)]
pub struct Bitmap {
    bits: [i64; 1024],
    count: usize,
    words: usize,
}

impl Default for Bitmap {
    #[inline(always)]
    fn default() -> Self {
        let mut words = (1024 / WORDSZ) + 1;
        Bitmap {
            bits: [0; 1024],
            count: 0,
            words: words,
        }
    }
}

impl Bitmap {
    pub fn reset(&mut self) {
        self.bits = [0; 1024];
        self.count = 0;
    }

    pub fn set(&mut self, index: usize) {
        let word = index / WORDSZ;
        let shift = word % WORDSZ;
        self.bits[word] |= (0x1 << shift);
        self.count += 1;
    }

    pub fn unset(&mut self, index: usize) {
        let word = index / WORDSZ;
        let shift = word % WORDSZ;
        self.bits[word] &= !(0x1 << shift);
        self.count -= 1;
    }

    pub fn and(&self, bm: Bitmap) -> Self {
        let mut result: Bitmap = Default::default();
        unsafe {
            for i in 0..self.words {
                result.bits[i] = self.bits[i] & bm.bits[i];
                result.count += _popcnt64(result.bits[i]) as usize;
            }
        }
        result
    }

    pub fn count(&self) -> usize {
        return self.count;
    }
}
