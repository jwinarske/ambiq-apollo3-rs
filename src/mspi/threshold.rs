#[doc = "Reader of register THRESHOLD"]
pub type R = crate::R<u32, super::THRESHOLD>;
#[doc = "Writer for register THRESHOLD"]
pub type W = crate::W<u32, super::THRESHOLD>;
#[doc = "Register THRESHOLD `reset()`'s with value 0"]
impl crate::ResetValue for super::THRESHOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXTHRESH`"]
pub type RXTHRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXTHRESH`"]
pub struct RXTHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXTHRESH`"]
pub type TXTHRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXTHRESH`"]
pub struct TXTHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - Number of entries in TX FIFO that cause RXE interrupt"]
    #[inline(always)]
    pub fn rxthresh(&self) -> RXTHRESH_R {
        RXTHRESH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Number of entries in TX FIFO that cause TXF interrupt"]
    #[inline(always)]
    pub fn txthresh(&self) -> TXTHRESH_R {
        TXTHRESH_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - Number of entries in TX FIFO that cause RXE interrupt"]
    #[inline(always)]
    pub fn rxthresh(&mut self) -> RXTHRESH_W {
        RXTHRESH_W { w: self }
    }
    #[doc = "Bits 0:4 - Number of entries in TX FIFO that cause TXF interrupt"]
    #[inline(always)]
    pub fn txthresh(&mut self) -> TXTHRESH_W {
        TXTHRESH_W { w: self }
    }
}
