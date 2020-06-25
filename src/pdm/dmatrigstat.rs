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
#[doc = "Reader of field `DTHR90STAT`"]
pub type DTHR90STAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTHR90STAT`"]
pub struct DTHR90STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHR90STAT_W<'a> {
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
#[doc = "Reader of field `DTHRSTAT`"]
pub type DTHRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTHRSTAT`"]
pub struct DTHRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHRSTAT_W<'a> {
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
    #[doc = "Bit 1 - Triggered DMA from FIFO reaching 90 percent full"]
    #[inline(always)]
    pub fn dthr90stat(&self) -> DTHR90STAT_R {
        DTHR90STAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Triggered DMA from FIFO reaching threshold"]
    #[inline(always)]
    pub fn dthrstat(&self) -> DTHRSTAT_R {
        DTHRSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Triggered DMA from FIFO reaching 90 percent full"]
    #[inline(always)]
    pub fn dthr90stat(&mut self) -> DTHR90STAT_W {
        DTHR90STAT_W { w: self }
    }
    #[doc = "Bit 0 - Triggered DMA from FIFO reaching threshold"]
    #[inline(always)]
    pub fn dthrstat(&mut self) -> DTHRSTAT_W {
        DTHRSTAT_W { w: self }
    }
}
