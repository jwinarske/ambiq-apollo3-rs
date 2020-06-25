#[doc = "Reader of register SCAPT2"]
pub type R = crate::R<u32, super::SCAPT2>;
#[doc = "Writer for register SCAPT2"]
pub type W = crate::W<u32, super::SCAPT2>;
#[doc = "Register SCAPT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCAPT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCAPT2`"]
pub type SCAPT2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCAPT2`"]
pub struct SCAPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAPT2_W<'a> {
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
    pub fn scapt2(&self) -> SCAPT2_R {
        SCAPT2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt2(&mut self) -> SCAPT2_W {
        SCAPT2_W { w: self }
    }
}
