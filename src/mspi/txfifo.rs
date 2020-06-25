#[doc = "Reader of register TXFIFO"]
pub type R = crate::R<u32, super::TXFIFO>;
#[doc = "Writer for register TXFIFO"]
pub type W = crate::W<u32, super::TXFIFO>;
#[doc = "Register TXFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXFIFO`"]
pub type TXFIFO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TXFIFO`"]
pub struct TXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data to be transmitted. Data should normall be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn txfifo(&self) -> TXFIFO_R {
        TXFIFO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to be transmitted. Data should normall be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn txfifo(&mut self) -> TXFIFO_W {
        TXFIFO_W { w: self }
    }
}
