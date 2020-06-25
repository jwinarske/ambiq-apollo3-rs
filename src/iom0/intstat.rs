#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQERR`"]
pub type CQERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQERR`"]
pub struct CQERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CQUPD`"]
pub type CQUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQUPD`"]
pub struct CQUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CQUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CQPAUSED`"]
pub type CQPAUSED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQPAUSED`"]
pub struct CQPAUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPAUSED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ARB`"]
pub type ARB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARB`"]
pub struct ARB_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ICMD`"]
pub type ICMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICMD`"]
pub struct ICMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ICMD_W<'a> {
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
#[doc = "Reader of field `IACC`"]
pub type IACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IACC`"]
pub struct IACC_W<'a> {
    w: &'a mut W,
}
impl<'a> IACC_W<'a> {
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
#[doc = "Reader of field `NAK`"]
pub type NAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAK`"]
pub struct NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_W<'a> {
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
#[doc = "Reader of field `FOVFL`"]
pub type FOVFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOVFL`"]
pub struct FOVFL_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVFL_W<'a> {
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
#[doc = "Reader of field `FUNDFL`"]
pub type FUNDFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUNDFL`"]
pub struct FUNDFL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNDFL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CMDCMP`"]
pub type CMDCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCMP`"]
pub struct CMDCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCMP_W<'a> {
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
    #[doc = "Bit 14 - Error during command queue operations"]
    #[inline(always)]
    pub fn cqerr(&self) -> CQERR_R {
        CQERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub fn cqupd(&self) -> CQUPD_R {
        CQUPD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CQPAUSED_R {
        CQPAUSED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub fn derr(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
    #[inline(always)]
    pub fn dcmp(&self) -> DCMP_R {
        DCMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub fn icmd(&self) -> ICMD_R {
        ICMD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
    #[inline(always)]
    pub fn iacc(&self) -> IACC_R {
        IACC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub fn fovfl(&self) -> FOVFL_R {
        FOVFL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[inline(always)]
    pub fn fundfl(&self) -> FUNDFL_R {
        FUNDFL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub fn thr(&self) -> THR_R {
        THR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub fn cmdcmp(&self) -> CMDCMP_R {
        CMDCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Error during command queue operations"]
    #[inline(always)]
    pub fn cqerr(&mut self) -> CQERR_W {
        CQERR_W { w: self }
    }
    #[doc = "Bit 13 - CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub fn cqupd(&mut self) -> CQUPD_W {
        CQUPD_W { w: self }
    }
    #[doc = "Bit 12 - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub fn cqpaused(&mut self) -> CQPAUSED_W {
        CQPAUSED_W { w: self }
    }
    #[doc = "Bit 11 - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub fn derr(&mut self) -> DERR_W {
        DERR_W { w: self }
    }
    #[doc = "Bit 10 - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
    #[inline(always)]
    pub fn dcmp(&mut self) -> DCMP_W {
        DCMP_W { w: self }
    }
    #[doc = "Bit 9 - Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[inline(always)]
    pub fn arb(&mut self) -> ARB_W {
        ARB_W { w: self }
    }
    #[doc = "Bit 8 - STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 7 - START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 6 - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub fn icmd(&mut self) -> ICMD_W {
        ICMD_W { w: self }
    }
    #[doc = "Bit 5 - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
    #[inline(always)]
    pub fn iacc(&mut self) -> IACC_W {
        IACC_W { w: self }
    }
    #[doc = "Bit 4 - I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W {
        NAK_W { w: self }
    }
    #[doc = "Bit 3 - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub fn fovfl(&mut self) -> FOVFL_W {
        FOVFL_W { w: self }
    }
    #[doc = "Bit 2 - Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[inline(always)]
    pub fn fundfl(&mut self) -> FUNDFL_W {
        FUNDFL_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W {
        THR_W { w: self }
    }
    #[doc = "Bit 0 - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub fn cmdcmp(&mut self) -> CMDCMP_W {
        CMDCMP_W { w: self }
    }
}
