pub struct Bit;

impl Bit {
    #[inline(always)]
    pub fn get(value: u64, index: u64) -> u64 {
        if value & (1_u64 << index) > 0 {
            1
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn set(value: &mut u64, index: u64) {
        *value |= 1_u64 << index;
    }

    #[inline(always)]
    pub fn clear(value: &mut u64, index: u64) {
        *value &= !(1_u64 << index)
    }

    #[inline(always)]
    pub fn lsb(value: u64) -> u64 {
        value & (1_u64 << value.trailing_zeros())
    }

    pub fn pop_lsb(value: &mut u64) -> u64 {
        let lsb = Self::lsb(*value);
        *value &= *value - 1;
        lsb
    }
}
