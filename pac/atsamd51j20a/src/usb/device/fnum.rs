#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::FNUM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MFNUMR {
    bits: u8,
}
impl MFNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FNUMR {
    bits: u16,
}
impl FNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FNCERRR {
    bits: bool,
}
impl FNCERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline]
    pub fn mfnum(&self) -> MFNUMR {
        let bits = ((self.bits >> 0) & 0x07) as u8;
        MFNUMR { bits }
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline]
    pub fn fnum(&self) -> FNUMR {
        let bits = ((self.bits >> 3) & 0x07ff) as u16;
        FNUMR { bits }
    }
    #[doc = "Bit 15 - Frame Number CRC Error"]
    #[inline]
    pub fn fncerr(&self) -> FNCERRR {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        FNCERRR { bits }
    }
}