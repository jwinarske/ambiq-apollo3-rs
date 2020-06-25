#[doc = "Reader of register SCAPT1"]
pub type R = crate::R<u32, super::SCAPT1>;
#[doc = "Writer for register SCAPT1"]
pub type W = crate::W<u32, super::SCAPT1>;
#[doc = "Register SCAPT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCAPT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCAPT1`"]
pub type SCAPT1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCAPT1`"]
pub struct SCAPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAPT1_W<'a> {
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
    pub fn scapt1(&self) -> SCAPT1_R {
        SCAPT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt1(&mut self) -> SCAPT1_W {
        SCAPT1_W { w: self }
    }
}
