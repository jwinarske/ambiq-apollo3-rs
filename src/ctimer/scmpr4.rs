#[doc = "Reader of register SCMPR4"]
pub type R = crate::R<u32, super::SCMPR4>;
#[doc = "Writer for register SCMPR4"]
pub type W = crate::W<u32, super::SCMPR4>;
#[doc = "Register SCMPR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCMPR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCMPR4`"]
pub type SCMPR4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCMPR4`"]
pub struct SCMPR4_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_E_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr4(&self) -> SCMPR4_R {
        SCMPR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_E_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr4(&mut self) -> SCMPR4_W {
        SCMPR4_W { w: self }
    }
}
