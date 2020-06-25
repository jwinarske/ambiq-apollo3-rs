#[doc = "Reader of register DMATOTCOUNT"]
pub type R = crate::R<u32, super::DMATOTCOUNT>;
#[doc = "Writer for register DMATOTCOUNT"]
pub type W = crate::W<u32, super::DMATOTCOUNT>;
#[doc = "Register DMATOTCOUNT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATOTCOUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOTCOUNT`"]
pub type TOTCOUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOTCOUNT`"]
pub struct TOTCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Total Transfer Count in bytes."]
    #[inline(always)]
    pub fn totcount(&self) -> TOTCOUNT_R {
        TOTCOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Total Transfer Count in bytes."]
    #[inline(always)]
    pub fn totcount(&mut self) -> TOTCOUNT_W {
        TOTCOUNT_W { w: self }
    }
}
