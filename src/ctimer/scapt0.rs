#[doc = "Reader of register SCAPT0"]
pub type R = crate::R<u32, super::SCAPT0>;
#[doc = "Writer for register SCAPT0"]
pub type W = crate::W<u32, super::SCAPT0>;
#[doc = "Register SCAPT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCAPT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCAPT0`"]
pub type SCAPT0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCAPT0`"]
pub struct SCAPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAPT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt0(&self) -> SCAPT0_R {
        SCAPT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt0(&mut self) -> SCAPT0_W {
        SCAPT0_W { w: self }
    }
}
