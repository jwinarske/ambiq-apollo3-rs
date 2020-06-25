#[doc = "Reader of register FIFOPUSH"]
pub type R = crate::R<u32, super::FIFOPUSH>;
#[doc = "Writer for register FIFOPUSH"]
pub type W = crate::W<u32, super::FIFOPUSH>;
#[doc = "Register FIFOPUSH `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOPUSH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFODIN`"]
pub type FIFODIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FIFODIN`"]
pub struct FIFODIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFODIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
    #[inline(always)]
    pub fn fifodin(&self) -> FIFODIN_R {
        FIFODIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
    #[inline(always)]
    pub fn fifodin(&mut self) -> FIFODIN_W {
        FIFODIN_W { w: self }
    }
}
