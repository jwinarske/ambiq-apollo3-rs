#[doc = "Reader of register DEBUGDATA"]
pub type R = crate::R<u32, super::DEBUGDATA>;
#[doc = "Writer for register DEBUGDATA"]
pub type W = crate::W<u32, super::DEBUGDATA>;
#[doc = "Register DEBUGDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUGDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEBUGDATA`"]
pub type DEBUGDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DEBUGDATA`"]
pub struct DEBUGDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Debug Data"]
    #[inline(always)]
    pub fn debugdata(&self) -> DEBUGDATA_R {
        DEBUGDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug Data"]
    #[inline(always)]
    pub fn debugdata(&mut self) -> DEBUGDATA_W {
        DEBUGDATA_W { w: self }
    }
}
