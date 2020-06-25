#[doc = "Reader of register DMON1"]
pub type R = crate::R<u32, super::DMON1>;
#[doc = "Writer for register DMON1"]
pub type W = crate::W<u32, super::DMON1>;
#[doc = "Register DMON1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMON1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLOOKUP_COUNT`"]
pub type DLOOKUP_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DLOOKUP_COUNT`"]
pub struct DLOOKUP_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLOOKUP_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Total tag lookups from data cache."]
    #[inline(always)]
    pub fn dlookup_count(&self) -> DLOOKUP_COUNT_R {
        DLOOKUP_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total tag lookups from data cache."]
    #[inline(always)]
    pub fn dlookup_count(&mut self) -> DLOOKUP_COUNT_W {
        DLOOKUP_COUNT_W { w: self }
    }
}
