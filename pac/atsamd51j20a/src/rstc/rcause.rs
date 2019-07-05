#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::RCAUSE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PORR {
    bits: bool,
}
impl PORR {
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
#[doc = r" Value of the field"]
pub struct BODCORER {
    bits: bool,
}
impl BODCORER {
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
#[doc = r" Value of the field"]
pub struct BODVDDR {
    bits: bool,
}
impl BODVDDR {
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
#[doc = r" Value of the field"]
pub struct NVMR {
    bits: bool,
}
impl NVMR {
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
#[doc = r" Value of the field"]
pub struct EXTR {
    bits: bool,
}
impl EXTR {
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
#[doc = r" Value of the field"]
pub struct WDTR {
    bits: bool,
}
impl WDTR {
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
#[doc = r" Value of the field"]
pub struct SYSTR {
    bits: bool,
}
impl SYSTR {
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
#[doc = r" Value of the field"]
pub struct BACKUPR {
    bits: bool,
}
impl BACKUPR {
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
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Power On Reset"]
    #[inline]
    pub fn por(&self) -> PORR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        PORR { bits }
    }
    #[doc = "Bit 1 - Brown Out CORE Detector Reset"]
    #[inline]
    pub fn bodcore(&self) -> BODCORER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        BODCORER { bits }
    }
    #[doc = "Bit 2 - Brown Out VDD Detector Reset"]
    #[inline]
    pub fn bodvdd(&self) -> BODVDDR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        BODVDDR { bits }
    }
    #[doc = "Bit 3 - NVM Reset"]
    #[inline]
    pub fn nvm(&self) -> NVMR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        NVMR { bits }
    }
    #[doc = "Bit 4 - External Reset"]
    #[inline]
    pub fn ext(&self) -> EXTR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        EXTR { bits }
    }
    #[doc = "Bit 5 - Watchdog Reset"]
    #[inline]
    pub fn wdt(&self) -> WDTR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        WDTR { bits }
    }
    #[doc = "Bit 6 - System Reset Request"]
    #[inline]
    pub fn syst(&self) -> SYSTR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        SYSTR { bits }
    }
    #[doc = "Bit 7 - Backup Reset"]
    #[inline]
    pub fn backup(&self) -> BACKUPR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        BACKUPR { bits }
    }
}