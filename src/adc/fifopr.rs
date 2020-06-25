#[doc = "Reader of register FIFOPR"]
pub type R = crate::R<u32, super::FIFOPR>;
#[doc = "Writer for register FIFOPR"]
pub type W = crate::W<u32, super::FIFOPR>;
#[doc = "Register FIFOPR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSVDPR`"]
pub type RSVDPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSVDPR`"]
pub struct RSVDPR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVDPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SLOTNUMPR`"]
pub type SLOTNUMPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLOTNUMPR`"]
pub struct SLOTNUMPR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTNUMPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - RESERVED."]
    #[inline(always)]
    pub fn rsvdpr(&self) -> RSVDPR_R {
        RSVDPR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Slot number associated with this FIFO data."]
    #[inline(always)]
    pub fn slotnumpr(&self) -> SLOTNUMPR_R {
        SLOTNUMPR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 20:27 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 0:19 - Oldest data in the FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - RESERVED."]
    #[inline(always)]
    pub fn rsvdpr(&mut self) -> RSVDPR_W {
        RSVDPR_W { w: self }
    }
    #[doc = "Bits 28:30 - Slot number associated with this FIFO data."]
    #[inline(always)]
    pub fn slotnumpr(&mut self) -> SLOTNUMPR_W {
        SLOTNUMPR_W { w: self }
    }
    #[doc = "Bits 20:27 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bits 0:19 - Oldest data in the FIFO."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
