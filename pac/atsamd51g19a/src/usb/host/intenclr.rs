#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::INTENCLR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u16 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct HSOFR {
    bits: bool,
}
impl HSOFR {
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
pub struct RSTR {
    bits: bool,
}
impl RSTR {
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
pub struct WAKEUPR {
    bits: bool,
}
impl WAKEUPR {
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
pub struct DNRSMR {
    bits: bool,
}
impl DNRSMR {
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
pub struct UPRSMR {
    bits: bool,
}
impl UPRSMR {
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
pub struct RAMACERR {
    bits: bool,
}
impl RAMACERR {
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
pub struct DCONNR {
    bits: bool,
}
impl DCONNR {
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
pub struct DDISCR {
    bits: bool,
}
impl DDISCR {
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
#[doc = r" Proxy"]
pub struct _HSOFW<'a> {
    w: &'a mut W,
}
impl<'a> _HSOFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u16) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 3);
        self.w.bits |= ((value as u16) & 0x01) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 4);
        self.w.bits |= ((value as u16) & 0x01) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DNRSMW<'a> {
    w: &'a mut W,
}
impl<'a> _DNRSMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 5);
        self.w.bits |= ((value as u16) & 0x01) << 5;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UPRSMW<'a> {
    w: &'a mut W,
}
impl<'a> _UPRSMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 6);
        self.w.bits |= ((value as u16) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RAMACERW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMACERW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u16) & 0x01) << 7;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCONNW<'a> {
    w: &'a mut W,
}
impl<'a> _DCONNW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 8);
        self.w.bits |= ((value as u16) & 0x01) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DDISCW<'a> {
    w: &'a mut W,
}
impl<'a> _DDISCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 9);
        self.w.bits |= ((value as u16) & 0x01) << 9;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 2 - Host Start Of Frame Interrupt Disable"]
    #[inline]
    pub fn hsof(&self) -> HSOFR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        HSOFR { bits }
    }
    #[doc = "Bit 3 - BUS Reset Interrupt Disable"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        RSTR { bits }
    }
    #[doc = "Bit 4 - Wake Up Interrupt Disable"]
    #[inline]
    pub fn wakeup(&self) -> WAKEUPR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        WAKEUPR { bits }
    }
    #[doc = "Bit 5 - DownStream to Device Interrupt Disable"]
    #[inline]
    pub fn dnrsm(&self) -> DNRSMR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        DNRSMR { bits }
    }
    #[doc = "Bit 6 - Upstream Resume from Device Interrupt Disable"]
    #[inline]
    pub fn uprsm(&self) -> UPRSMR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        UPRSMR { bits }
    }
    #[doc = "Bit 7 - Ram Access Interrupt Disable"]
    #[inline]
    pub fn ramacer(&self) -> RAMACERR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        RAMACERR { bits }
    }
    #[doc = "Bit 8 - Device Connection Interrupt Disable"]
    #[inline]
    pub fn dconn(&self) -> DCONNR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        DCONNR { bits }
    }
    #[doc = "Bit 9 - Device Disconnection Interrupt Disable"]
    #[inline]
    pub fn ddisc(&self) -> DDISCR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        DDISCR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Host Start Of Frame Interrupt Disable"]
    #[inline]
    pub fn hsof(&mut self) -> _HSOFW {
        _HSOFW { w: self }
    }
    #[doc = "Bit 3 - BUS Reset Interrupt Disable"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bit 4 - Wake Up Interrupt Disable"]
    #[inline]
    pub fn wakeup(&mut self) -> _WAKEUPW {
        _WAKEUPW { w: self }
    }
    #[doc = "Bit 5 - DownStream to Device Interrupt Disable"]
    #[inline]
    pub fn dnrsm(&mut self) -> _DNRSMW {
        _DNRSMW { w: self }
    }
    #[doc = "Bit 6 - Upstream Resume from Device Interrupt Disable"]
    #[inline]
    pub fn uprsm(&mut self) -> _UPRSMW {
        _UPRSMW { w: self }
    }
    #[doc = "Bit 7 - Ram Access Interrupt Disable"]
    #[inline]
    pub fn ramacer(&mut self) -> _RAMACERW {
        _RAMACERW { w: self }
    }
    #[doc = "Bit 8 - Device Connection Interrupt Disable"]
    #[inline]
    pub fn dconn(&mut self) -> _DCONNW {
        _DCONNW { w: self }
    }
    #[doc = "Bit 9 - Device Disconnection Interrupt Disable"]
    #[inline]
    pub fn ddisc(&mut self) -> _DDISCW {
        _DDISCW { w: self }
    }
}