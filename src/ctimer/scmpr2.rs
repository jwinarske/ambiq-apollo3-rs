#[doc = "Reader of register SCMPR2"]
pub type R = crate::R<u32, super::SCMPR2>;
#[doc = "Writer for register SCMPR2"]
pub type W = crate::W<u32, super::SCMPR2>;
#[doc = "Register SCMPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCMPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCMPR2`"]
pub type SCMPR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCMPR2`"]
pub struct SCMPR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_C_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr2(&self) -> SCMPR2_R {
        SCMPR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_C_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr2(&mut self) -> SCMPR2_W {
        SCMPR2_W { w: self }
    }
}
