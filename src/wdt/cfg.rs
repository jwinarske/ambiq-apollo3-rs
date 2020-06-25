#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x00ff_ff00"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_ff00
    }
}
#[doc = "Select the frequency for the WDT. All values not enumerated below are undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Low Power Mode.  This setting disables the watch dog timer. value."]
    OFF = 0,
    #[doc = "1: 128 Hz LFRC clock. value."]
    _128HZ = 1,
    #[doc = "2: 16 Hz LFRC clock. value."]
    _16HZ = 2,
    #[doc = "3: 1 Hz LFRC clock. value."]
    _1HZ = 3,
    #[doc = "4: 1/16th Hz LFRC clock. value."]
    _1_16HZ = 4,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSEL_A::OFF),
            1 => Val(CLKSEL_A::_128HZ),
            2 => Val(CLKSEL_A::_16HZ),
            3 => Val(CLKSEL_A::_1HZ),
            4 => Val(CLKSEL_A::_1_16HZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLKSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `_128HZ`"]
    #[inline(always)]
    pub fn is_128hz(&self) -> bool {
        *self == CLKSEL_A::_128HZ
    }
    #[doc = "Checks if the value of the field is `_16HZ`"]
    #[inline(always)]
    pub fn is_16hz(&self) -> bool {
        *self == CLKSEL_A::_16HZ
    }
    #[doc = "Checks if the value of the field is `_1HZ`"]
    #[inline(always)]
    pub fn is_1hz(&self) -> bool {
        *self == CLKSEL_A::_1HZ
    }
    #[doc = "Checks if the value of the field is `_1_16HZ`"]
    #[inline(always)]
    pub fn is_1_16hz(&self) -> bool {
        *self == CLKSEL_A::_1_16HZ
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low Power Mode. This setting disables the watch dog timer. value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLKSEL_A::OFF)
    }
    #[doc = "128 Hz LFRC clock. value."]
    #[inline(always)]
    pub fn _128hz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_128HZ)
    }
    #[doc = "16 Hz LFRC clock. value."]
    #[inline(always)]
    pub fn _16hz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_16HZ)
    }
    #[doc = "1 Hz LFRC clock. value."]
    #[inline(always)]
    pub fn _1hz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_1HZ)
    }
    #[doc = "1/16th Hz LFRC clock. value."]
    #[inline(always)]
    pub fn _1_16hz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_1_16HZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `INTVAL`"]
pub type INTVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTVAL`"]
pub struct INTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESVAL`"]
pub type RESVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESVAL`"]
pub struct RESVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESEN`"]
pub type RESEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESEN`"]
pub struct RESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN`"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `WDTEN`"]
pub type WDTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTEN`"]
pub struct WDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    pub fn intval(&self) -> INTVAL_R {
        INTVAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline(always)]
    pub fn resval(&self) -> RESVAL_R {
        RESVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 2 - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline(always)]
    pub fn resen(&self) -> RESEN_R {
        RESEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bitfield enables the WDT."]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    pub fn intval(&mut self) -> INTVAL_W {
        INTVAL_W { w: self }
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline(always)]
    pub fn resval(&mut self) -> RESVAL_W {
        RESVAL_W { w: self }
    }
    #[doc = "Bit 2 - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline(always)]
    pub fn resen(&mut self) -> RESEN_W {
        RESEN_W { w: self }
    }
    #[doc = "Bit 1 - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bit 0 - This bitfield enables the WDT."]
    #[inline(always)]
    pub fn wdten(&mut self) -> WDTEN_W {
        WDTEN_W { w: self }
    }
}
