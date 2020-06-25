#[doc = "Reader of register SCMPR3"]
pub type R = crate::R<u32, super::SCMPR3>;
#[doc = "Writer for register SCMPR3"]
pub type W = crate::W<u32, super::SCMPR3>;
#[doc = "Register SCMPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCMPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCMPR3`"]
pub type SCMPR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCMPR3`"]
pub struct SCMPR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_D_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr3(&self) -> SCMPR3_R {
        SCMPR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_D_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr3(&mut self) -> SCMPR3_W {
        SCMPR3_W { w: self }
    }
}
