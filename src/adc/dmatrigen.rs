#[doc = "Reader of register DMATRIGEN"]
pub type R = crate::R<u32, super::DMATRIGEN>;
#[doc = "Writer for register DMATRIGEN"]
pub type W = crate::W<u32, super::DMATRIGEN>;
#[doc = "Register DMATRIGEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATRIGEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DFIFOFULL`"]
pub type DFIFOFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFIFOFULL`"]
pub struct DFIFOFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFIFOFULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DFIFO75`"]
pub type DFIFO75_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFIFO75`"]
pub struct DFIFO75_W<'a> {
    w: &'a mut W,
}
impl<'a> DFIFO75_W<'a> {
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
    #[doc = "Bit 1 - Trigger DMA upon FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfifofull(&self) -> DFIFOFULL_R {
        DFIFOFULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Trigger DMA upon FIFO 75 percent Full"]
    #[inline(always)]
    pub fn dfifo75(&self) -> DFIFO75_R {
        DFIFO75_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Trigger DMA upon FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfifofull(&mut self) -> DFIFOFULL_W {
        DFIFOFULL_W { w: self }
    }
    #[doc = "Bit 0 - Trigger DMA upon FIFO 75 percent Full"]
    #[inline(always)]
    pub fn dfifo75(&mut self) -> DFIFO75_W {
        DFIFO75_W { w: self }
    }
}
