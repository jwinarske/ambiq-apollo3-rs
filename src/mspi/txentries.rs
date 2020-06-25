#[doc = "Reader of register TXENTRIES"]
pub type R = crate::R<u32, super::TXENTRIES>;
#[doc = "Writer for register TXENTRIES"]
pub type W = crate::W<u32, super::TXENTRIES>;
#[doc = "Register TXENTRIES `reset()`'s with value 0"]
impl crate::ResetValue for super::TXENTRIES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXENTRIES`"]
pub type TXENTRIES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXENTRIES`"]
pub struct TXENTRIES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENTRIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of 32-bit words/entries in TX FIFO"]
    #[inline(always)]
    pub fn txentries(&self) -> TXENTRIES_R {
        TXENTRIES_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 32-bit words/entries in TX FIFO"]
    #[inline(always)]
    pub fn txentries(&mut self) -> TXENTRIES_W {
        TXENTRIES_W { w: self }
    }
}
