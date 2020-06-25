#[doc = "Reader of register TPIURST"]
pub type R = crate::R<u32, super::TPIURST>;
#[doc = "Writer for register TPIURST"]
pub type W = crate::W<u32, super::TPIURST>;
#[doc = "Register TPIURST `reset()`'s with value 0"]
impl crate::ResetValue for super::TPIURST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPIURST`"]
pub type TPIURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPIURST`"]
pub struct TPIURST_W<'a> {
    w: &'a mut W,
}
impl<'a> TPIURST_W<'a> {
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
    #[doc = "Bit 0 - Static reset for the TPIU. Write to '1' to assert reset to TPIU. Write to '0' to clear the reset."]
    #[inline(always)]
    pub fn tpiurst(&self) -> TPIURST_R {
        TPIURST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static reset for the TPIU. Write to '1' to assert reset to TPIU. Write to '0' to clear the reset."]
    #[inline(always)]
    pub fn tpiurst(&mut self) -> TPIURST_W {
        TPIURST_W { w: self }
    }
}
