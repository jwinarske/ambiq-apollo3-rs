#[doc = "Reader of register CMPRAUXB6"]
pub type R = crate::R<u32, super::CMPRAUXB6>;
#[doc = "Writer for register CMPRAUXB6"]
pub type W = crate::W<u32, super::CMPRAUXB6>;
#[doc = "Register CMPRAUXB6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPRAUXB6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPR3B6`"]
pub type CMPR3B6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR3B6`"]
pub struct CMPR3B6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR3B6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMPR2B6`"]
pub type CMPR2B6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR2B6`"]
pub struct CMPR2B6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR2B6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B6 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub fn cmpr3b6(&self) -> CMPR3B6_R {
        CMPR3B6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer B6 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub fn cmpr2b6(&self) -> CMPR2B6_R {
        CMPR2B6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B6 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub fn cmpr3b6(&mut self) -> CMPR3B6_W {
        CMPR3B6_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer B6 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub fn cmpr2b6(&mut self) -> CMPR2B6_W {
        CMPR2B6_W { w: self }
    }
}
