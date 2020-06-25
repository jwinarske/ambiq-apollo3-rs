#[doc = "Reader of register DMATRIGSTAT"]
pub type R = crate::R<u32, super::DMATRIGSTAT>;
#[doc = "Writer for register DMATRIGSTAT"]
pub type W = crate::W<u32, super::DMATRIGSTAT>;
#[doc = "Register DMATRIGSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATRIGSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DFULLSTAT`"]
pub type DFULLSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFULLSTAT`"]
pub struct DFULLSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DFULLSTAT_W<'a> {
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
#[doc = "Reader of field `D75STAT`"]
pub type D75STAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D75STAT`"]
pub struct D75STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> D75STAT_W<'a> {
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
    #[doc = "Bit 1 - Triggered DMA from FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfullstat(&self) -> DFULLSTAT_R {
        DFULLSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Triggered DMA from FIFO 75 percent Full"]
    #[inline(always)]
    pub fn d75stat(&self) -> D75STAT_R {
        D75STAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Triggered DMA from FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfullstat(&mut self) -> DFULLSTAT_W {
        DFULLSTAT_W { w: self }
    }
    #[doc = "Bit 0 - Triggered DMA from FIFO 75 percent Full"]
    #[inline(always)]
    pub fn d75stat(&mut self) -> D75STAT_W {
        D75STAT_W { w: self }
    }
}
