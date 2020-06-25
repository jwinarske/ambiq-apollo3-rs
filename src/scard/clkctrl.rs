#[doc = "Reader of register CLKCTRL"]
pub type R = crate::R<u32, super::CLKCTRL>;
#[doc = "Writer for register CLKCTRL"]
pub type W = crate::W<u32, super::CLKCTRL>;
#[doc = "Register CLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APBCLKEN`"]
pub type APBCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APBCLKEN`"]
pub struct APBCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APBCLKEN_W<'a> {
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
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
    #[doc = "Bit 1 - Enable the SCARD APB clock to run continuously."]
    #[inline(always)]
    pub fn apbclken(&self) -> APBCLKEN_R {
        APBCLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable the serial source clock for SCARD."]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable the SCARD APB clock to run continuously."]
    #[inline(always)]
    pub fn apbclken(&mut self) -> APBCLKEN_W {
        APBCLKEN_W { w: self }
    }
    #[doc = "Bit 0 - Enable the serial source clock for SCARD."]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
}
