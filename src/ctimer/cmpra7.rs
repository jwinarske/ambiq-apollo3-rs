#[doc = "Reader of register CMPRA7"]
pub type R = crate::R<u32, super::CMPRA7>;
#[doc = "Writer for register CMPRA7"]
pub type W = crate::W<u32, super::CMPRA7>;
#[doc = "Register CMPRA7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPRA7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPR1A7`"]
pub type CMPR1A7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR1A7`"]
pub struct CMPR1A7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1A7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMPR0A7`"]
pub type CMPR0A7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR0A7`"]
pub struct CMPR0A7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0A7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer A7 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1a7(&self) -> CMPR1A7_R {
        CMPR1A7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A7 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0a7(&self) -> CMPR0A7_R {
        CMPR0A7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer A7 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1a7(&mut self) -> CMPR1A7_W {
        CMPR1A7_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A7 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0a7(&mut self) -> CMPR0A7_W {
        CMPR0A7_W { w: self }
    }
}
