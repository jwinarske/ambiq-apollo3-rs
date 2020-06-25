#[doc = "Reader of register TMR6"]
pub type R = crate::R<u32, super::TMR6>;
#[doc = "Writer for register TMR6"]
pub type W = crate::W<u32, super::TMR6>;
#[doc = "Register TMR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTTMRB6`"]
pub type CTTMRB6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRB6`"]
pub struct CTTMRB6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTTMRA6`"]
pub type CTTMRA6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRA6`"]
pub struct CTTMRA6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B6."]
    #[inline(always)]
    pub fn cttmrb6(&self) -> CTTMRB6_R {
        CTTMRB6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A6."]
    #[inline(always)]
    pub fn cttmra6(&self) -> CTTMRA6_R {
        CTTMRA6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B6."]
    #[inline(always)]
    pub fn cttmrb6(&mut self) -> CTTMRB6_W {
        CTTMRB6_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A6."]
    #[inline(always)]
    pub fn cttmra6(&mut self) -> CTTMRA6_W {
        CTTMRA6_W { w: self }
    }
}
