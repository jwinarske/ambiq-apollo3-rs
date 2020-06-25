#[doc = "Reader of register INTSET"]
pub type R = crate::R<u32, super::INTSET>;
#[doc = "Writer for register INTSET"]
pub type W = crate::W<u32, super::INTSET>;
#[doc = "Register INTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DERR`"]
pub type DERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DERR`"]
pub struct DERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DCMP`"]
pub type DCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP`"]
pub struct DCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UNDFL`"]
pub type UNDFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDFL`"]
pub struct UNDFL_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDFL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `OVF`"]
pub type OVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVF`"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
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
#[doc = "Reader of field `THR`"]
pub type THR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THR`"]
pub struct THR_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_W<'a> {
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
    #[doc = "Bit 4 - DMA Error receieved"]
    #[inline(always)]
    pub fn derr(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA completed a transfer"]
    #[inline(always)]
    pub fn dcmp(&self) -> DCMP_R {
        DCMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the FIFO underflow interrupt."]
    #[inline(always)]
    pub fn undfl(&self) -> UNDFL_R {
        UNDFL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This is the FIFO overflow interrupt."]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This is the FIFO threshold interrupt."]
    #[inline(always)]
    pub fn thr(&self) -> THR_R {
        THR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - DMA Error receieved"]
    #[inline(always)]
    pub fn derr(&mut self) -> DERR_W {
        DERR_W { w: self }
    }
    #[doc = "Bit 3 - DMA completed a transfer"]
    #[inline(always)]
    pub fn dcmp(&mut self) -> DCMP_W {
        DCMP_W { w: self }
    }
    #[doc = "Bit 2 - This is the FIFO underflow interrupt."]
    #[inline(always)]
    pub fn undfl(&mut self) -> UNDFL_W {
        UNDFL_W { w: self }
    }
    #[doc = "Bit 1 - This is the FIFO overflow interrupt."]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Bit 0 - This is the FIFO threshold interrupt."]
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W {
        THR_W { w: self }
    }
}
