#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FHF`"]
pub type FHF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FHF`"]
pub struct FHF_W<'a> {
    w: &'a mut W,
}
impl<'a> FHF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FT2REND`"]
pub type FT2REND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT2REND`"]
pub struct FT2REND_W<'a> {
    w: &'a mut W,
}
impl<'a> FT2REND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
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
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR`"]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
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
#[doc = "Reader of field `FER`"]
pub type FER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FER`"]
pub struct FER_W<'a> {
    w: &'a mut W,
}
impl<'a> FER_W<'a> {
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
#[doc = "Reader of field `TBERBF`"]
pub type TBERBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBERBF`"]
pub struct TBERBF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBERBF_W<'a> {
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
#[doc = "Reader of field `FNE`"]
pub type FNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FNE`"]
pub struct FNE_W<'a> {
    w: &'a mut W,
}
impl<'a> FNE_W<'a> {
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
    #[doc = "Bit 6 - FIFO Half Full."]
    #[inline(always)]
    pub fn fhf(&self) -> FHF_R {
        FHF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX to RX finished."]
    #[inline(always)]
    pub fn ft2rend(&self) -> FT2REND_R {
        FT2REND_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX FIFO overflow."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Framing error."]
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO empty (transmit) or full (receive)."]
    #[inline(always)]
    pub fn tberbf(&self) -> TBERBF_R {
        TBERBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RX FIFO not empty."]
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - FIFO Half Full."]
    #[inline(always)]
    pub fn fhf(&mut self) -> FHF_W {
        FHF_W { w: self }
    }
    #[doc = "Bit 5 - TX to RX finished."]
    #[inline(always)]
    pub fn ft2rend(&mut self) -> FT2REND_W {
        FT2REND_W { w: self }
    }
    #[doc = "Bit 4 - Parity Error."]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO overflow."]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    #[doc = "Bit 2 - Framing error."]
    #[inline(always)]
    pub fn fer(&mut self) -> FER_W {
        FER_W { w: self }
    }
    #[doc = "Bit 1 - FIFO empty (transmit) or full (receive)."]
    #[inline(always)]
    pub fn tberbf(&mut self) -> TBERBF_W {
        TBERBF_W { w: self }
    }
    #[doc = "Bit 0 - RX FIFO not empty."]
    #[inline(always)]
    pub fn fne(&mut self) -> FNE_W {
        FNE_W { w: self }
    }
}
