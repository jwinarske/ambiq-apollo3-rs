#[doc = "Reader of register SCWLIM"]
pub type R = crate::R<u32, super::SCWLIM>;
#[doc = "Writer for register SCWLIM"]
pub type W = crate::W<u32, super::SCWLIM>;
#[doc = "Register SCWLIM `reset()`'s with value 0"]
impl crate::ResetValue for super::SCWLIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCWLIMEN`"]
pub type SCWLIMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCWLIMEN`"]
pub struct SCWLIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCWLIMEN_W<'a> {
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
    #[doc = "Bit 0 - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
    #[inline(always)]
    pub fn scwlimen(&self) -> SCWLIMEN_R {
        SCWLIMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
    #[inline(always)]
    pub fn scwlimen(&mut self) -> SCWLIMEN_W {
        SCWLIMEN_W { w: self }
    }
}
