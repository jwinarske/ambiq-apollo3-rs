#[doc = "Reader of register SCDET"]
pub type R = crate::R<u32, super::SCDET>;
#[doc = "Writer for register SCDET"]
pub type W = crate::W<u32, super::SCDET>;
#[doc = "Register SCDET `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::SCDET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `SCDET`"]
pub type SCDET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCDET`"]
pub struct SCDET_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - SCARD card detect pad select."]
    #[inline(always)]
    pub fn scdet(&self) -> SCDET_R {
        SCDET_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SCARD card detect pad select."]
    #[inline(always)]
    pub fn scdet(&mut self) -> SCDET_W {
        SCDET_W { w: self }
    }
}
