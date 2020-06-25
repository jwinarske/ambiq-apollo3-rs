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
#[doc = "Reader of field `XCMPWR`"]
pub type XCMPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCMPWR`"]
pub struct XCMPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMPWR_W<'a> {
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
#[doc = "Reader of field `XCMPWF`"]
pub type XCMPWF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCMPWF`"]
pub struct XCMPWF_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMPWF_W<'a> {
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
#[doc = "Reader of field `XCMPRR`"]
pub type XCMPRR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCMPRR`"]
pub struct XCMPRR_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMPRR_W<'a> {
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
#[doc = "Reader of field `XCMPRF`"]
pub type XCMPRF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCMPRF`"]
pub struct XCMPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMPRF_W<'a> {
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
#[doc = "Reader of field `IOINTW`"]
pub type IOINTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOINTW`"]
pub struct IOINTW_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINTW_W<'a> {
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
#[doc = "Reader of field `GENAD`"]
pub type GENAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GENAD`"]
pub struct GENAD_W<'a> {
    w: &'a mut W,
}
impl<'a> GENAD_W<'a> {
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
#[doc = "Reader of field `FRDERR`"]
pub type FRDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRDERR`"]
pub struct FRDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FSIZE`"]
pub type FSIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSIZE`"]
pub struct FSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSIZE_W<'a> {
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
    #[doc = "Bit 9 - Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub fn xcmpwr(&self) -> XCMPWR_R {
        XCMPWR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub fn xcmpwf(&self) -> XCMPWF_R {
        XCMPWF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub fn xcmprr(&self) -> XCMPRR_R {
        XCMPRR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub fn xcmprf(&self) -> XCMPRF_R {
        XCMPRF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO Write interrupt."]
    #[inline(always)]
    pub fn iointw(&self) -> IOINTW_R {
        IOINTW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C General Address interrupt."]
    #[inline(always)]
    pub fn genad(&self) -> GENAD_R {
        GENAD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO Read Error interrupt."]
    #[inline(always)]
    pub fn frderr(&self) -> FRDERR_R {
        FRDERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO Underflow interrupt."]
    #[inline(always)]
    pub fn fundfl(&self) -> FUNDFL_R {
        FUNDFL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Overflow interrupt."]
    #[inline(always)]
    pub fn fovfl(&self) -> FOVFL_R {
        FOVFL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FIFO Size interrupt."]
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub fn xcmpwr(&mut self) -> XCMPWR_W {
        XCMPWR_W { w: self }
    }
    #[doc = "Bit 8 - Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub fn xcmpwf(&mut self) -> XCMPWF_W {
        XCMPWF_W { w: self }
    }
    #[doc = "Bit 7 - Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub fn xcmprr(&mut self) -> XCMPRR_W {
        XCMPRR_W { w: self }
    }
    #[doc = "Bit 6 - Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub fn xcmprf(&mut self) -> XCMPRF_W {
        XCMPRF_W { w: self }
    }
    #[doc = "Bit 5 - IO Write interrupt."]
    #[inline(always)]
    pub fn iointw(&mut self) -> IOINTW_W {
        IOINTW_W { w: self }
    }
    #[doc = "Bit 4 - I2C General Address interrupt."]
    #[inline(always)]
    pub fn genad(&mut self) -> GENAD_W {
        GENAD_W { w: self }
    }
    #[doc = "Bit 3 - FIFO Read Error interrupt."]
    #[inline(always)]
    pub fn frderr(&mut self) -> FRDERR_W {
        FRDERR_W { w: self }
    }
    #[doc = "Bit 2 - FIFO Underflow interrupt."]
    #[inline(always)]
    pub fn fundfl(&mut self) -> FUNDFL_W {
        FUNDFL_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Overflow interrupt."]
    #[inline(always)]
    pub fn fovfl(&mut self) -> FOVFL_W {
        FOVFL_W { w: self }
    }
    #[doc = "Bit 0 - FIFO Size interrupt."]
    #[inline(always)]
    pub fn fsize(&mut self) -> FSIZE_W {
        FSIZE_W { w: self }
    }
}
