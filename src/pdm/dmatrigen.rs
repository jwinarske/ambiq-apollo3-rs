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
#[doc = "Reader of field `DTHR90`"]
pub type DTHR90_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTHR90`"]
pub struct DTHR90_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHR90_W<'a> {
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
#[doc = "Reader of field `DTHR`"]
pub type DTHR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTHR`"]
pub struct DTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHR_W<'a> {
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
    #[doc = "Bit 1 - Trigger DMA at FIFO 90 percent full. This signal is also used internally for AUTOHIP function"]
    #[inline(always)]
    pub fn dthr90(&self) -> DTHR90_R {
        DTHR90_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Trigger DMA upon when FIFO iss filled to level indicated by the FIFO THRESHOLD,at granularity of 16 bytes only"]
    #[inline(always)]
    pub fn dthr(&self) -> DTHR_R {
        DTHR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Trigger DMA at FIFO 90 percent full. This signal is also used internally for AUTOHIP function"]
    #[inline(always)]
    pub fn dthr90(&mut self) -> DTHR90_W {
        DTHR90_W { w: self }
    }
    #[doc = "Bit 0 - Trigger DMA upon when FIFO iss filled to level indicated by the FIFO THRESHOLD,at granularity of 16 bytes only"]
    #[inline(always)]
    pub fn dthr(&mut self) -> DTHR_W {
        DTHR_W { w: self }
    }
}
