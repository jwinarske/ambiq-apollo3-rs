#[doc = "Reader of register DMASTAT"]
pub type R = crate::R<u32, super::DMASTAT>;
#[doc = "Writer for register DMASTAT"]
pub type W = crate::W<u32, super::DMASTAT>;
#[doc = "Register DMASTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMASTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCRERR`"]
pub type SCRERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCRERR`"]
pub struct SCRERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRERR_W<'a> {
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
#[doc = "Reader of field `DMAERR`"]
pub type DMAERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAERR`"]
pub struct DMAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAERR_W<'a> {
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
#[doc = "Reader of field `DMACPL`"]
pub type DMACPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACPL`"]
pub struct DMACPL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACPL_W<'a> {
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
#[doc = "Reader of field `DMATIP`"]
pub type DMATIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATIP`"]
pub struct DMATIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATIP_W<'a> {
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
    #[doc = "Bit 3 - Scrambling Access Alignment Error. This active high bit signals that a scrambling operation was specified for a non-word aligned DEVADDR."]
    #[inline(always)]
    pub fn screrr(&self) -> SCRERR_R {
        SCRERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Error. This active high bit signals that an error was encountered during the DMA operation."]
    #[inline(always)]
    pub fn dmaerr(&self) -> DMAERR_R {
        DMAERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Transfer Complete. This signals the end of the DMA operation."]
    #[inline(always)]
    pub fn dmacpl(&self) -> DMACPL_R {
        DMACPL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done."]
    #[inline(always)]
    pub fn dmatip(&self) -> DMATIP_R {
        DMATIP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Scrambling Access Alignment Error. This active high bit signals that a scrambling operation was specified for a non-word aligned DEVADDR."]
    #[inline(always)]
    pub fn screrr(&mut self) -> SCRERR_W {
        SCRERR_W { w: self }
    }
    #[doc = "Bit 2 - DMA Error. This active high bit signals that an error was encountered during the DMA operation."]
    #[inline(always)]
    pub fn dmaerr(&mut self) -> DMAERR_W {
        DMAERR_W { w: self }
    }
    #[doc = "Bit 1 - DMA Transfer Complete. This signals the end of the DMA operation."]
    #[inline(always)]
    pub fn dmacpl(&mut self) -> DMACPL_W {
        DMACPL_W { w: self }
    }
    #[doc = "Bit 0 - DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done."]
    #[inline(always)]
    pub fn dmatip(&mut self) -> DMATIP_W {
        DMATIP_W { w: self }
    }
}
