#[doc = "Reader of register FIFOREAD"]
pub type R = crate::R<u32, super::FIFOREAD>;
#[doc = "Writer for register FIFOREAD"]
pub type W = crate::W<u32, super::FIFOREAD>;
#[doc = "Register FIFOREAD `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOREAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOREAD`"]
pub type FIFOREAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FIFOREAD`"]
pub struct FIFOREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOREAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FIFO read data."]
    #[inline(always)]
    pub fn fiforead(&self) -> FIFOREAD_R {
        FIFOREAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO read data."]
    #[inline(always)]
    pub fn fiforead(&mut self) -> FIFOREAD_W {
        FIFOREAD_W { w: self }
    }
}
