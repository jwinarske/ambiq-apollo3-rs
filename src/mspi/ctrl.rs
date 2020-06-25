#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XFERBYTES`"]
pub type XFERBYTES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XFERBYTES`"]
pub struct XFERBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PIOSCRAMBLE`"]
pub type PIOSCRAMBLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIOSCRAMBLE`"]
pub struct PIOSCRAMBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIOSCRAMBLE_W<'a> {
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
#[doc = "Reader of field `TXRX`"]
pub type TXRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRX`"]
pub struct TXRX_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRX_W<'a> {
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
#[doc = "Reader of field `SENDI`"]
pub type SENDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENDI`"]
pub struct SENDI_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDI_W<'a> {
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
#[doc = "Reader of field `SENDA`"]
pub type SENDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENDA`"]
pub struct SENDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDA_W<'a> {
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
#[doc = "Reader of field `ENTURN`"]
pub type ENTURN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTURN`"]
pub struct ENTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTURN_W<'a> {
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
#[doc = "Reader of field `BIGENDIAN`"]
pub type BIGENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIGENDIAN`"]
pub struct BIGENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIGENDIAN_W<'a> {
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
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
#[doc = "Reader of field `QUADCMD`"]
pub type QUADCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QUADCMD`"]
pub struct QUADCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADCMD_W<'a> {
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS`"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Number of bytes to transmit or receive (based on TXRX bit)"]
    #[inline(always)]
    pub fn xferbytes(&self) -> XFERBYTES_R {
        XFERBYTES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 11 - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
    #[inline(always)]
    pub fn pioscramble(&self) -> PIOSCRAMBLE_R {
        PIOSCRAMBLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
    #[inline(always)]
    pub fn txrx(&self) -> TXRX_R {
        TXRX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
    #[inline(always)]
    pub fn sendi(&self) -> SENDI_R {
        SENDI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
    #[inline(always)]
    pub fn senda(&self) -> SENDA_R {
        SENDA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
    #[inline(always)]
    pub fn enturn(&self) -> ENTURN_R {
        ENTURN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
    #[inline(always)]
    pub fn bigendian(&self) -> BIGENDIAN_R {
        BIGENDIAN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Continuation transfer. When 1, indicates that the MSPI will hold CE low after the transaction completes. This is included for compatibility with IOM module since the MSPI transfer module can handle most cases in a single transfer. NOTE: CONT functionality only works with CLKDIV=2 (24 MHz)."]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flag indicating that the operation is a command that should be replicated to both devices in paired QUAD mode. This is typically only used when reading/writing configuration registers in paired flash devices (do not set for memory transfers)."]
    #[inline(always)]
    pub fn quadcmd(&self) -> QUADCMD_R {
        QUADCMD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command status: 1 indicates controller is busy (command in progress)"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - Number of bytes to transmit or receive (based on TXRX bit)"]
    #[inline(always)]
    pub fn xferbytes(&mut self) -> XFERBYTES_W {
        XFERBYTES_W { w: self }
    }
    #[doc = "Bit 11 - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
    #[inline(always)]
    pub fn pioscramble(&mut self) -> PIOSCRAMBLE_W {
        PIOSCRAMBLE_W { w: self }
    }
    #[doc = "Bit 10 - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
    #[inline(always)]
    pub fn txrx(&mut self) -> TXRX_W {
        TXRX_W { w: self }
    }
    #[doc = "Bit 9 - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
    #[inline(always)]
    pub fn sendi(&mut self) -> SENDI_W {
        SENDI_W { w: self }
    }
    #[doc = "Bit 8 - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
    #[inline(always)]
    pub fn senda(&mut self) -> SENDA_W {
        SENDA_W { w: self }
    }
    #[doc = "Bit 7 - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
    #[inline(always)]
    pub fn enturn(&mut self) -> ENTURN_W {
        ENTURN_W { w: self }
    }
    #[doc = "Bit 6 - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
    #[inline(always)]
    pub fn bigendian(&mut self) -> BIGENDIAN_W {
        BIGENDIAN_W { w: self }
    }
    #[doc = "Bit 5 - Continuation transfer. When 1, indicates that the MSPI will hold CE low after the transaction completes. This is included for compatibility with IOM module since the MSPI transfer module can handle most cases in a single transfer. NOTE: CONT functionality only works with CLKDIV=2 (24 MHz)."]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bit 3 - Flag indicating that the operation is a command that should be replicated to both devices in paired QUAD mode. This is typically only used when reading/writing configuration registers in paired flash devices (do not set for memory transfers)."]
    #[inline(always)]
    pub fn quadcmd(&mut self) -> QUADCMD_W {
        QUADCMD_W { w: self }
    }
    #[doc = "Bit 2 - Command status: 1 indicates controller is busy (command in progress)"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 1 - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
    #[doc = "Bit 0 - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
}
