#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTFLAGB {
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
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct USB_R {
    bits: bool,
}
impl USB_R {
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
pub struct DSU_R {
    bits: bool,
}
impl DSU_R {
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
pub struct NVMCTRL_R {
    bits: bool,
}
impl NVMCTRL_R {
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
pub struct CMCC_R {
    bits: bool,
}
impl CMCC_R {
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
pub struct PORT_R {
    bits: bool,
}
impl PORT_R {
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
pub struct DMAC_R {
    bits: bool,
}
impl DMAC_R {
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
pub struct HMATRIX_R {
    bits: bool,
}
impl HMATRIX_R {
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
pub struct EVSYS_R {
    bits: bool,
}
impl EVSYS_R {
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
pub struct SERCOM2_R {
    bits: bool,
}
impl SERCOM2_R {
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
pub struct SERCOM3_R {
    bits: bool,
}
impl SERCOM3_R {
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
pub struct TCC0_R {
    bits: bool,
}
impl TCC0_R {
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
pub struct TCC1_R {
    bits: bool,
}
impl TCC1_R {
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
pub struct TC2_R {
    bits: bool,
}
impl TC2_R {
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
pub struct TC3_R {
    bits: bool,
}
impl TC3_R {
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
pub struct RAMECC_R {
    bits: bool,
}
impl RAMECC_R {
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
pub struct _USB_W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_W<'a> {
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u32) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSU_W<'a> {
    w: &'a mut W,
}
impl<'a> _DSU_W<'a> {
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
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u32) & 0x01) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NVMCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> _NVMCTRL_W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMCC_W<'a> {
    w: &'a mut W,
}
impl<'a> _CMCC_W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> _PORT_W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAC_W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 5;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HMATRIX_W<'a> {
    w: &'a mut W,
}
impl<'a> _HMATRIX_W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EVSYS_W<'a> {
    w: &'a mut W,
}
impl<'a> _EVSYS_W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 7;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SERCOM2_W<'a> {
    w: &'a mut W,
}
impl<'a> _SERCOM2_W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 9;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SERCOM3_W<'a> {
    w: &'a mut W,
}
impl<'a> _SERCOM3_W<'a> {
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
        self.w.bits &= !(0x01 << 10);
        self.w.bits |= ((value as u32) & 0x01) << 10;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCC0_W<'a> {
    w: &'a mut W,
}
impl<'a> _TCC0_W<'a> {
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
        self.w.bits &= !(0x01 << 11);
        self.w.bits |= ((value as u32) & 0x01) << 11;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCC1_W<'a> {
    w: &'a mut W,
}
impl<'a> _TCC1_W<'a> {
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
        self.w.bits &= !(0x01 << 12);
        self.w.bits |= ((value as u32) & 0x01) << 12;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TC2_W<'a> {
    w: &'a mut W,
}
impl<'a> _TC2_W<'a> {
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
        self.w.bits &= !(0x01 << 13);
        self.w.bits |= ((value as u32) & 0x01) << 13;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TC3_W<'a> {
    w: &'a mut W,
}
impl<'a> _TC3_W<'a> {
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
        self.w.bits &= !(0x01 << 14);
        self.w.bits |= ((value as u32) & 0x01) << 14;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RAMECC_W<'a> {
    w: &'a mut W,
}
impl<'a> _RAMECC_W<'a> {
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
        self.w.bits &= !(0x01 << 16);
        self.w.bits |= ((value as u32) & 0x01) << 16;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USB"]
    #[inline]
    pub fn usb_(&self) -> USB_R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        USB_R { bits }
    }
    #[doc = "Bit 1 - DSU"]
    #[inline]
    pub fn dsu_(&self) -> DSU_R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        DSU_R { bits }
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline]
    pub fn nvmctrl_(&self) -> NVMCTRL_R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        NVMCTRL_R { bits }
    }
    #[doc = "Bit 3 - CMCC"]
    #[inline]
    pub fn cmcc_(&self) -> CMCC_R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        CMCC_R { bits }
    }
    #[doc = "Bit 4 - PORT"]
    #[inline]
    pub fn port_(&self) -> PORT_R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        PORT_R { bits }
    }
    #[doc = "Bit 5 - DMAC"]
    #[inline]
    pub fn dmac_(&self) -> DMAC_R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        DMAC_R { bits }
    }
    #[doc = "Bit 6 - HMATRIX"]
    #[inline]
    pub fn hmatrix_(&self) -> HMATRIX_R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        HMATRIX_R { bits }
    }
    #[doc = "Bit 7 - EVSYS"]
    #[inline]
    pub fn evsys_(&self) -> EVSYS_R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        EVSYS_R { bits }
    }
    #[doc = "Bit 9 - SERCOM2"]
    #[inline]
    pub fn sercom2_(&self) -> SERCOM2_R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        SERCOM2_R { bits }
    }
    #[doc = "Bit 10 - SERCOM3"]
    #[inline]
    pub fn sercom3_(&self) -> SERCOM3_R {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        SERCOM3_R { bits }
    }
    #[doc = "Bit 11 - TCC0"]
    #[inline]
    pub fn tcc0_(&self) -> TCC0_R {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        TCC0_R { bits }
    }
    #[doc = "Bit 12 - TCC1"]
    #[inline]
    pub fn tcc1_(&self) -> TCC1_R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        TCC1_R { bits }
    }
    #[doc = "Bit 13 - TC2"]
    #[inline]
    pub fn tc2_(&self) -> TC2_R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        TC2_R { bits }
    }
    #[doc = "Bit 14 - TC3"]
    #[inline]
    pub fn tc3_(&self) -> TC3_R {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        TC3_R { bits }
    }
    #[doc = "Bit 16 - RAMECC"]
    #[inline]
    pub fn ramecc_(&self) -> RAMECC_R {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        RAMECC_R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - USB"]
    #[inline]
    pub fn usb_(&mut self) -> _USB_W {
        _USB_W { w: self }
    }
    #[doc = "Bit 1 - DSU"]
    #[inline]
    pub fn dsu_(&mut self) -> _DSU_W {
        _DSU_W { w: self }
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline]
    pub fn nvmctrl_(&mut self) -> _NVMCTRL_W {
        _NVMCTRL_W { w: self }
    }
    #[doc = "Bit 3 - CMCC"]
    #[inline]
    pub fn cmcc_(&mut self) -> _CMCC_W {
        _CMCC_W { w: self }
    }
    #[doc = "Bit 4 - PORT"]
    #[inline]
    pub fn port_(&mut self) -> _PORT_W {
        _PORT_W { w: self }
    }
    #[doc = "Bit 5 - DMAC"]
    #[inline]
    pub fn dmac_(&mut self) -> _DMAC_W {
        _DMAC_W { w: self }
    }
    #[doc = "Bit 6 - HMATRIX"]
    #[inline]
    pub fn hmatrix_(&mut self) -> _HMATRIX_W {
        _HMATRIX_W { w: self }
    }
    #[doc = "Bit 7 - EVSYS"]
    #[inline]
    pub fn evsys_(&mut self) -> _EVSYS_W {
        _EVSYS_W { w: self }
    }
    #[doc = "Bit 9 - SERCOM2"]
    #[inline]
    pub fn sercom2_(&mut self) -> _SERCOM2_W {
        _SERCOM2_W { w: self }
    }
    #[doc = "Bit 10 - SERCOM3"]
    #[inline]
    pub fn sercom3_(&mut self) -> _SERCOM3_W {
        _SERCOM3_W { w: self }
    }
    #[doc = "Bit 11 - TCC0"]
    #[inline]
    pub fn tcc0_(&mut self) -> _TCC0_W {
        _TCC0_W { w: self }
    }
    #[doc = "Bit 12 - TCC1"]
    #[inline]
    pub fn tcc1_(&mut self) -> _TCC1_W {
        _TCC1_W { w: self }
    }
    #[doc = "Bit 13 - TC2"]
    #[inline]
    pub fn tc2_(&mut self) -> _TC2_W {
        _TC2_W { w: self }
    }
    #[doc = "Bit 14 - TC3"]
    #[inline]
    pub fn tc3_(&mut self) -> _TC3_W {
        _TC3_W { w: self }
    }
    #[doc = "Bit 16 - RAMECC"]
    #[inline]
    pub fn ramecc_(&mut self) -> _RAMECC_W {
        _RAMECC_W { w: self }
    }
}