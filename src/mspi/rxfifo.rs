#[doc = "Reader of register RXFIFO"]
pub type R = crate::R<u32, super::RXFIFO>;
#[doc = "Writer for register RXFIFO"]
pub type W = crate::W<u32, super::RXFIFO>;
#[doc = "Register RXFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXFIFO`"]
pub type RXFIFO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RXFIFO`"]
pub struct RXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn rxfifo(&self) -> RXFIFO_R {
        RXFIFO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn rxfifo(&mut self) -> RXFIFO_W {
        RXFIFO_W { w: self }
    }
}
