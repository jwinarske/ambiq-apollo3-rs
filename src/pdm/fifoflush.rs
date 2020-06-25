#[doc = "Reader of register FIFOFLUSH"]
pub type R = crate::R<u32, super::FIFOFLUSH>;
#[doc = "Writer for register FIFOFLUSH"]
pub type W = crate::W<u32, super::FIFOFLUSH>;
#[doc = "Register FIFOFLUSH `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOFLUSH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOFLUSH`"]
pub type FIFOFLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOFLUSH`"]
pub struct FIFOFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOFLUSH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FIFO FLUSH."]
    #[inline(always)]
    pub fn fifoflush(&self) -> FIFOFLUSH_R {
        FIFOFLUSH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO FLUSH."]
    #[inline(always)]
    pub fn fifoflush(&mut self) -> FIFOFLUSH_W {
        FIFOFLUSH_W { w: self }
    }
}
