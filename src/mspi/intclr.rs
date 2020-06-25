#[doc = "Reader of register INTCLR"]
pub type R = crate::R<u32, super::INTCLR>;
#[doc = "Writer for register INTCLR"]
pub type W = crate::W<u32, super::INTCLR>;
#[doc = "Register INTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCLR {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CQCMP`"]
pub type CQCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQCMP`"]
pub struct CQCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CQCMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RXF`"]
pub type RXF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXF`"]
pub struct RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXF_W<'a> {
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
#[doc = "Reader of field `RXO`"]
pub type RXO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXO`"]
pub struct RXO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXO_W<'a> {
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
#[doc = "Reader of field `RXU`"]
pub type RXU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXU`"]
pub struct RXU_W<'a> {
    w: &'a mut W,
}
impl<'a> RXU_W<'a> {
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
#[doc = "Reader of field `TXO`"]
pub type TXO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXO`"]
pub struct TXO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXO_W<'a> {
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
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
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
    #[doc = "Bit 12 - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub fn screrr(&self) -> SCRERR_R {
        SCRERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command Queue Error Interrupt"]
    #[inline(always)]
    pub fn cqerr(&self) -> CQERR_R {
        CQERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Command Queue is Paused."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CQPAUSED_R {
        CQPAUSED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub fn cqupd(&self) -> CQUPD_R {
        CQUPD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Command Queue Complete Interrupt"]
    #[inline(always)]
    pub fn cqcmp(&self) -> CQCMP_R {
        CQCMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA Error Interrupt"]
    #[inline(always)]
    pub fn derr(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA Complete Interrupt"]
    #[inline(always)]
    pub fn dcmp(&self) -> DCMP_R {
        DCMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
    #[inline(always)]
    pub fn rxu(&self) -> RXU_R {
        RXU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously"]
    #[inline(always)]
    pub fn cmdcmp(&self) -> CMDCMP_R {
        CMDCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub fn screrr(&mut self) -> SCRERR_W {
        SCRERR_W { w: self }
    }
    #[doc = "Bit 11 - Command Queue Error Interrupt"]
    #[inline(always)]
    pub fn cqerr(&mut self) -> CQERR_W {
        CQERR_W { w: self }
    }
    #[doc = "Bit 10 - Command Queue is Paused."]
    #[inline(always)]
    pub fn cqpaused(&mut self) -> CQPAUSED_W {
        CQPAUSED_W { w: self }
    }
    #[doc = "Bit 9 - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub fn cqupd(&mut self) -> CQUPD_W {
        CQUPD_W { w: self }
    }
    #[doc = "Bit 8 - Command Queue Complete Interrupt"]
    #[inline(always)]
    pub fn cqcmp(&mut self) -> CQCMP_W {
        CQCMP_W { w: self }
    }
    #[doc = "Bit 7 - DMA Error Interrupt"]
    #[inline(always)]
    pub fn derr(&mut self) -> DERR_W {
        DERR_W { w: self }
    }
    #[doc = "Bit 6 - DMA Complete Interrupt"]
    #[inline(always)]
    pub fn dcmp(&mut self) -> DCMP_W {
        DCMP_W { w: self }
    }
    #[doc = "Bit 5 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
    #[inline(always)]
    pub fn rxo(&mut self) -> RXO_W {
        RXO_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
    #[inline(always)]
    pub fn rxu(&mut self) -> RXU_W {
        RXU_W { w: self }
    }
    #[doc = "Bit 2 - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub fn txo(&mut self) -> TXO_W {
        TXO_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bit 0 - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously"]
    #[inline(always)]
    pub fn cmdcmp(&mut self) -> CMDCMP_W {
        CMDCMP_W { w: self }
    }
}
