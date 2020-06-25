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
#[doc = "Reader of field `DTHREN`"]
pub type DTHREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTHREN`"]
pub struct DTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHREN_W<'a> {
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
#[doc = "Reader of field `DCMDCMPEN`"]
pub type DCMDCMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMDCMPEN`"]
pub struct DCMDCMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMDCMPEN_W<'a> {
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
    #[doc = "Bit 1 - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline(always)]
    pub fn dthren(&self) -> DTHREN_R {
        DTHREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
    #[inline(always)]
    pub fn dcmdcmpen(&self) -> DCMDCMPEN_R {
        DCMDCMPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline(always)]
    pub fn dthren(&mut self) -> DTHREN_W {
        DTHREN_W { w: self }
    }
    #[doc = "Bit 0 - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
    #[inline(always)]
    pub fn dcmdcmpen(&mut self) -> DCMDCMPEN_W {
        DCMDCMPEN_W { w: self }
    }
}
