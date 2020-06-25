#[doc = "Reader of register DMABCOUNT"]
pub type R = crate::R<u32, super::DMABCOUNT>;
#[doc = "Writer for register DMABCOUNT"]
pub type W = crate::W<u32, super::DMABCOUNT>;
#[doc = "Register DMABCOUNT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMABCOUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCOUNT`"]
pub type BCOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BCOUNT`"]
pub struct BCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended values are 16 or 32."]
    #[inline(always)]
    pub fn bcount(&self) -> BCOUNT_R {
        BCOUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended values are 16 or 32."]
    #[inline(always)]
    pub fn bcount(&mut self) -> BCOUNT_W {
        BCOUNT_W { w: self }
    }
}
