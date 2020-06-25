#[doc = "Reader of register SCMPR5"]
pub type R = crate::R<u32, super::SCMPR5>;
#[doc = "Writer for register SCMPR5"]
pub type W = crate::W<u32, super::SCMPR5>;
#[doc = "Register SCMPR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCMPR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCMPR5`"]
pub type SCMPR5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCMPR5`"]
pub struct SCMPR5_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_F_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr5(&self) -> SCMPR5_R {
        SCMPR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_F_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr5(&mut self) -> SCMPR5_W {
        SCMPR5_W { w: self }
    }
}
