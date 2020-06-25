#[doc = "Reader of register OFFSETHI"]
pub type R = crate::R<u32, super::OFFSETHI>;
#[doc = "Writer for register OFFSETHI"]
pub type W = crate::W<u32, super::OFFSETHI>;
#[doc = "Register OFFSETHI `reset()`'s with value 0"]
impl crate::ResetValue for super::OFFSETHI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSETHI`"]
pub type OFFSETHI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFSETHI`"]
pub struct OFFSETHI_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Holds the high order 2 bytes of the 3 byte addressing/offset field to use with IO commands. The number of offset bytes to use is specified in the command register"]
    #[inline(always)]
    pub fn offsethi(&self) -> OFFSETHI_R {
        OFFSETHI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Holds the high order 2 bytes of the 3 byte addressing/offset field to use with IO commands. The number of offset bytes to use is specified in the command register"]
    #[inline(always)]
    pub fn offsethi(&mut self) -> OFFSETHI_W {
        OFFSETHI_W { w: self }
    }
}
