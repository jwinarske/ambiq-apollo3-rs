#[doc = "Reader of register IMON0"]
pub type R = crate::R<u32, super::IMON0>;
#[doc = "Writer for register IMON0"]
pub type W = crate::W<u32, super::IMON0>;
#[doc = "Register IMON0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMON0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IACCESS_COUNT`"]
pub type IACCESS_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IACCESS_COUNT`"]
pub struct IACCESS_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IACCESS_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Total accesses to Instruction cache"]
    #[inline(always)]
    pub fn iaccess_count(&self) -> IACCESS_COUNT_R {
        IACCESS_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total accesses to Instruction cache"]
    #[inline(always)]
    pub fn iaccess_count(&mut self) -> IACCESS_COUNT_W {
        IACCESS_COUNT_W { w: self }
    }
}
