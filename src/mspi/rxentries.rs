#[doc = "Reader of register RXENTRIES"]
pub type R = crate::R<u32, super::RXENTRIES>;
#[doc = "Writer for register RXENTRIES"]
pub type W = crate::W<u32, super::RXENTRIES>;
#[doc = "Register RXENTRIES `reset()`'s with value 0"]
impl crate::ResetValue for super::RXENTRIES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXENTRIES`"]
pub type RXENTRIES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXENTRIES`"]
pub struct RXENTRIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENTRIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of 32-bit words/entries in RX FIFO"]
    #[inline(always)]
    pub fn rxentries(&self) -> RXENTRIES_R {
        RXENTRIES_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 32-bit words/entries in RX FIFO"]
    #[inline(always)]
    pub fn rxentries(&mut self) -> RXENTRIES_W {
        RXENTRIES_W { w: self }
    }
}
