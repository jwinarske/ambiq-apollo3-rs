#[doc = "Reader of register SCMPR6"]
pub type R = crate::R<u32, super::SCMPR6>;
#[doc = "Writer for register SCMPR6"]
pub type W = crate::W<u32, super::SCMPR6>;
#[doc = "Register SCMPR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCMPR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCMPR6`"]
pub type SCMPR6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCMPR6`"]
pub struct SCMPR6_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_G_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr6(&self) -> SCMPR6_R {
        SCMPR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_G_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr6(&mut self) -> SCMPR6_W {
        SCMPR6_W { w: self }
    }
}
