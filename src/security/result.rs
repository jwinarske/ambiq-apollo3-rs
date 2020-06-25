#[doc = "Reader of register RESULT"]
pub type R = crate::R<u32, super::RESULT>;
#[doc = "Writer for register RESULT"]
pub type W = crate::W<u32, super::RESULT>;
#[doc = "Register RESULT `reset()`'s with value 0"]
impl crate::ResetValue for super::RESULT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Seed/Result. Software must seed the CRC with 0xFFFFFFFF before starting a CRC operation (unless the CRC is continued from a previous operation)."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Seed/Result. Software must seed the CRC with 0xFFFFFFFF before starting a CRC operation (unless the CRC is continued from a previous operation)."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
}
