#[doc = "Reader of register CMPRB6"]
pub type R = crate::R<u32, super::CMPRB6>;
#[doc = "Writer for register CMPRB6"]
pub type W = crate::W<u32, super::CMPRB6>;
#[doc = "Register CMPRB6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPRB6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPR1B6`"]
pub type CMPR1B6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR1B6`"]
pub struct CMPR1B6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1B6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMPR0B6`"]
pub type CMPR0B6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR0B6`"]
pub struct CMPR0B6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0B6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B6 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1b6(&self) -> CMPR1B6_R {
        CMPR1B6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer B6 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0b6(&self) -> CMPR0B6_R {
        CMPR0B6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B6 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1b6(&mut self) -> CMPR1B6_W {
        CMPR1B6_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer B6 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0b6(&mut self) -> CMPR0B6_W {
        CMPR0B6_W { w: self }
    }
}
