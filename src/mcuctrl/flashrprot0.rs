#[doc = "Reader of register FLASHRPROT0"]
pub type R = crate::R<u32, super::FLASHRPROT0>;
#[doc = "Writer for register FLASHRPROT0"]
pub type W = crate::W<u32, super::FLASHRPROT0>;
#[doc = "Register FLASHRPROT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHRPROT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FR0BITS`"]
pub type FR0BITS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FR0BITS`"]
pub struct FR0BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> FR0BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00000000 - 0x0007FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr0bits(&self) -> FR0BITS_R {
        FR0BITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00000000 - 0x0007FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr0bits(&mut self) -> FR0BITS_W {
        FR0BITS_W { w: self }
    }
}
