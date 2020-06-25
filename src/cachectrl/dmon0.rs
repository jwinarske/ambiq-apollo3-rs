#[doc = "Reader of register DMON0"]
pub type R = crate::R<u32, super::DMON0>;
#[doc = "Writer for register DMON0"]
pub type W = crate::W<u32, super::DMON0>;
#[doc = "Register DMON0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMON0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DACCESS_COUNT`"]
pub type DACCESS_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DACCESS_COUNT`"]
pub struct DACCESS_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCESS_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
    #[inline(always)]
    pub fn daccess_count(&self) -> DACCESS_COUNT_R {
        DACCESS_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
    #[inline(always)]
    pub fn daccess_count(&mut self) -> DACCESS_COUNT_W {
        DACCESS_COUNT_W { w: self }
    }
}
