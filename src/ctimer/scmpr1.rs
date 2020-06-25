#[doc = "Reader of register SCMPR1"]
pub type R = crate::R<u32, super::SCMPR1>;
#[doc = "Writer for register SCMPR1"]
pub type W = crate::W<u32, super::SCMPR1>;
#[doc = "Register SCMPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCMPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCMPR1`"]
pub type SCMPR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCMPR1`"]
pub struct SCMPR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_B_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr1(&self) -> SCMPR1_R {
        SCMPR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_B_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr1(&mut self) -> SCMPR1_W {
        SCMPR1_W { w: self }
    }
}
