#[doc = "Reader of register TMR5"]
pub type R = crate::R<u32, super::TMR5>;
#[doc = "Writer for register TMR5"]
pub type W = crate::W<u32, super::TMR5>;
#[doc = "Register TMR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTTMRB5`"]
pub type CTTMRB5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRB5`"]
pub struct CTTMRB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTTMRA5`"]
pub type CTTMRA5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRA5`"]
pub struct CTTMRA5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B5."]
    #[inline(always)]
    pub fn cttmrb5(&self) -> CTTMRB5_R {
        CTTMRB5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A5."]
    #[inline(always)]
    pub fn cttmra5(&self) -> CTTMRA5_R {
        CTTMRA5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B5."]
    #[inline(always)]
    pub fn cttmrb5(&mut self) -> CTTMRB5_W {
        CTTMRB5_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A5."]
    #[inline(always)]
    pub fn cttmra5(&mut self) -> CTTMRA5_W {
        CTTMRA5_W { w: self }
    }
}
