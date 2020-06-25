#[doc = "Reader of register DMON2"]
pub type R = crate::R<u32, super::DMON2>;
#[doc = "Writer for register DMON2"]
pub type W = crate::W<u32, super::DMON2>;
#[doc = "Register DMON2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMON2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DHIT_COUNT`"]
pub type DHIT_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DHIT_COUNT`"]
pub struct DHIT_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DHIT_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cache hits from lookup operations."]
    #[inline(always)]
    pub fn dhit_count(&self) -> DHIT_COUNT_R {
        DHIT_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache hits from lookup operations."]
    #[inline(always)]
    pub fn dhit_count(&mut self) -> DHIT_COUNT_W {
        DHIT_COUNT_W { w: self }
    }
}
