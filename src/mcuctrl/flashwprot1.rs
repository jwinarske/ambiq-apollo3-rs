#[doc = "Reader of register FLASHWPROT1"]
pub type R = crate::R<u32, super::FLASHWPROT1>;
#[doc = "Writer for register FLASHWPROT1"]
pub type W = crate::W<u32, super::FLASHWPROT1>;
#[doc = "Register FLASHWPROT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHWPROT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FW1BITS`"]
pub type FW1BITS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FW1BITS`"]
pub struct FW1BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> FW1BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write protect flash 0x00080000 - 0x000FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fw1bits(&self) -> FW1BITS_R {
        FW1BITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protect flash 0x00080000 - 0x000FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fw1bits(&mut self) -> FW1BITS_W {
        FW1BITS_W { w: self }
    }
}
