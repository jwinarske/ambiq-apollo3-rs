#[doc = "Reader of register FIFOPOP"]
pub type R = crate::R<u32, super::FIFOPOP>;
#[doc = "Writer for register FIFOPOP"]
pub type W = crate::W<u32, super::FIFOPOP>;
#[doc = "Register FIFOPOP `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOPOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFODOUT`"]
pub type FIFODOUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FIFODOUT`"]
pub struct FIFODOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFODOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
    #[inline(always)]
    pub fn fifodout(&self) -> FIFODOUT_R {
        FIFODOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
    #[inline(always)]
    pub fn fifodout(&mut self) -> FIFODOUT_W {
        FIFODOUT_W { w: self }
    }
}
