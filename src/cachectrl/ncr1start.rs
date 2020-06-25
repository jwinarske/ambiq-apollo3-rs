#[doc = "Reader of register NCR1START"]
pub type R = crate::R<u32, super::NCR1START>;
#[doc = "Writer for register NCR1START"]
pub type W = crate::W<u32, super::NCR1START>;
#[doc = "Register NCR1START `reset()`'s with value 0"]
impl crate::ResetValue for super::NCR1START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 4)) | (((value as u32) & 0x007f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:26 - Start address for non-cacheable region 1"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 4) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:26 - Start address for non-cacheable region 1"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
