#[doc = "Reader of register CMPRAUXA2"]
pub type R = crate::R<u32, super::CMPRAUXA2>;
#[doc = "Writer for register CMPRAUXA2"]
pub type W = crate::W<u32, super::CMPRAUXA2>;
#[doc = "Register CMPRAUXA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPRAUXA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPR3A2`"]
pub type CMPR3A2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR3A2`"]
pub struct CMPR3A2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR3A2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMPR2A2`"]
pub type CMPR2A2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR2A2`"]
pub struct CMPR2A2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR2A2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer A2 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr3a2(&self) -> CMPR3A2_R {
        CMPR3A2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A2 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr2a2(&self) -> CMPR2A2_R {
        CMPR2A2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer A2 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr3a2(&mut self) -> CMPR3A2_W {
        CMPR3A2_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A2 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr2a2(&mut self) -> CMPR2A2_W {
        CMPR2A2_W { w: self }
    }
}
