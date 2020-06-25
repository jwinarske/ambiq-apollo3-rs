#[doc = "Reader of register BLEDBG"]
pub type R = crate::R<u32, super::BLEDBG>;
#[doc = "Writer for register BLEDBG"]
pub type W = crate::W<u32, super::BLEDBG>;
#[doc = "Register BLEDBG `reset()`'s with value 0"]
impl crate::ResetValue for super::BLEDBG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBGDATA`"]
pub type DBGDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DBGDATA`"]
pub struct DBGDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `APBCLKON`"]
pub type APBCLKON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APBCLKON`"]
pub struct APBCLKON_W<'a> {
    w: &'a mut W,
}
impl<'a> APBCLKON_W<'a> {
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
#[doc = "Reader of field `IOCLKON`"]
pub type IOCLKON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOCLKON`"]
pub struct IOCLKON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCLKON_W<'a> {
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
#[doc = "Reader of field `DBGEN`"]
pub type DBGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGEN`"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
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
    #[doc = "Bits 3:31 - Debug data"]
    #[inline(always)]
    pub fn dbgdata(&self) -> DBGDATA_R {
        DBGDATA_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn apbclkon(&self) -> APBCLKON_R {
        APBCLKON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn ioclkon(&self) -> IOCLKON_R {
        IOCLKON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Debug Enable. Setting this bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - Debug data"]
    #[inline(always)]
    pub fn dbgdata(&mut self) -> DBGDATA_W {
        DBGDATA_W { w: self }
    }
    #[doc = "Bit 2 - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn apbclkon(&mut self) -> APBCLKON_W {
        APBCLKON_W { w: self }
    }
    #[doc = "Bit 1 - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn ioclkon(&mut self) -> IOCLKON_W {
        IOCLKON_W { w: self }
    }
    #[doc = "Bit 0 - Debug Enable. Setting this bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
}
