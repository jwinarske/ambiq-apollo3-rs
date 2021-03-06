#[doc = "Reader of register TMR3"]
pub type R = crate::R<u32, super::TMR3>;
#[doc = "Writer for register TMR3"]
pub type W = crate::W<u32, super::TMR3>;
#[doc = "Register TMR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTTMRB3`"]
pub type CTTMRB3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRB3`"]
pub struct CTTMRB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTTMRA3`"]
pub type CTTMRA3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRA3`"]
pub struct CTTMRA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B3."]
    #[inline(always)]
    pub fn cttmrb3(&self) -> CTTMRB3_R {
        CTTMRB3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A3."]
    #[inline(always)]
    pub fn cttmra3(&self) -> CTTMRA3_R {
        CTTMRA3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B3."]
    #[inline(always)]
    pub fn cttmrb3(&mut self) -> CTTMRB3_W {
        CTTMRB3_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A3."]
    #[inline(always)]
    pub fn cttmra3(&mut self) -> CTTMRA3_W {
        CTTMRA3_W { w: self }
    }
}
