#[doc = "Reader of register FIFOCTRL"]
pub type R = crate::R<u32, super::FIFOCTRL>;
#[doc = "Writer for register FIFOCTRL"]
pub type W = crate::W<u32, super::FIFOCTRL>;
#[doc = "Register FIFOCTRL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FIFOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `FIFORSTN`"]
pub type FIFORSTN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFORSTN`"]
pub struct FIFORSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFORSTN_W<'a> {
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
#[doc = "Reader of field `POPWR`"]
pub type POPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POPWR`"]
pub struct POPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> POPWR_W<'a> {
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
    #[doc = "Bit 1 - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
    #[inline(always)]
    pub fn fiforstn(&self) -> FIFORSTN_R {
        FIFORSTN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
    #[inline(always)]
    pub fn popwr(&self) -> POPWR_R {
        POPWR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
    #[inline(always)]
    pub fn fiforstn(&mut self) -> FIFORSTN_W {
        FIFORSTN_W { w: self }
    }
    #[doc = "Bit 0 - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
    #[inline(always)]
    pub fn popwr(&mut self) -> POPWR_W {
        POPWR_W { w: self }
    }
}
