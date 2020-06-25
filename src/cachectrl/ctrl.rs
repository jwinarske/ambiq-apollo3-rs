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
#[doc = "Reader of field `FLASH1_SLM_ENABLE`"]
pub type FLASH1_SLM_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH1_SLM_ENABLE`"]
pub struct FLASH1_SLM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_ENABLE_W<'a> {
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
#[doc = "Reader of field `FLASH1_SLM_DISABLE`"]
pub type FLASH1_SLM_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH1_SLM_DISABLE`"]
pub struct FLASH1_SLM_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_DISABLE_W<'a> {
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
#[doc = "Reader of field `FLASH1_SLM_STATUS`"]
pub type FLASH1_SLM_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH1_SLM_STATUS`"]
pub struct FLASH1_SLM_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_STATUS_W<'a> {
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
#[doc = "Reader of field `FLASH0_SLM_ENABLE`"]
pub type FLASH0_SLM_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH0_SLM_ENABLE`"]
pub struct FLASH0_SLM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_ENABLE_W<'a> {
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
#[doc = "Reader of field `FLASH0_SLM_DISABLE`"]
pub type FLASH0_SLM_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH0_SLM_DISABLE`"]
pub struct FLASH0_SLM_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_DISABLE_W<'a> {
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
#[doc = "Reader of field `FLASH0_SLM_STATUS`"]
pub type FLASH0_SLM_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH0_SLM_STATUS`"]
pub struct FLASH0_SLM_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_STATUS_W<'a> {
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
#[doc = "Reader of field `CACHE_READY`"]
pub type CACHE_READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_READY`"]
pub struct CACHE_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_READY_W<'a> {
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
#[doc = "Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_STAT_A {
    #[doc = "1: Clear Cache Stats value."]
    CLEAR = 1,
}
impl From<RESET_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESET_STAT`"]
pub type RESET_STAT_R = crate::R<bool, RESET_STAT_A>;
impl RESET_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RESET_STAT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RESET_STAT_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RESET_STAT_A::CLEAR
    }
}
#[doc = "Write proxy for field `RESET_STAT`"]
pub struct RESET_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_STAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_STAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear Cache Stats value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RESET_STAT_A::CLEAR)
    }
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
#[doc = "Reader of field `INVALIDATE`"]
pub type INVALIDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVALIDATE`"]
pub struct INVALIDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVALIDATE_W<'a> {
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
    #[doc = "Bit 10 - Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub fn flash1_slm_enable(&self) -> FLASH1_SLM_ENABLE_R {
        FLASH1_SLM_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub fn flash1_slm_disable(&self) -> FLASH1_SLM_DISABLE_R {
        FLASH1_SLM_DISABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
    #[inline(always)]
    pub fn flash1_slm_status(&self) -> FLASH1_SLM_STATUS_R {
        FLASH1_SLM_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub fn flash0_slm_enable(&self) -> FLASH0_SLM_ENABLE_R {
        FLASH0_SLM_ENABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub fn flash0_slm_disable(&self) -> FLASH0_SLM_DISABLE_R {
        FLASH0_SLM_DISABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
    #[inline(always)]
    pub fn flash0_slm_status(&self) -> FLASH0_SLM_STATUS_R {
        FLASH0_SLM_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline(always)]
    pub fn cache_ready(&self) -> CACHE_READY_R {
        CACHE_READY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline(always)]
    pub fn reset_stat(&self) -> RESET_STAT_R {
        RESET_STAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline(always)]
    pub fn invalidate(&self) -> INVALIDATE_R {
        INVALIDATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub fn flash1_slm_enable(&mut self) -> FLASH1_SLM_ENABLE_W {
        FLASH1_SLM_ENABLE_W { w: self }
    }
    #[doc = "Bit 9 - Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub fn flash1_slm_disable(&mut self) -> FLASH1_SLM_DISABLE_W {
        FLASH1_SLM_DISABLE_W { w: self }
    }
    #[doc = "Bit 8 - Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
    #[inline(always)]
    pub fn flash1_slm_status(&mut self) -> FLASH1_SLM_STATUS_W {
        FLASH1_SLM_STATUS_W { w: self }
    }
    #[doc = "Bit 6 - Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub fn flash0_slm_enable(&mut self) -> FLASH0_SLM_ENABLE_W {
        FLASH0_SLM_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub fn flash0_slm_disable(&mut self) -> FLASH0_SLM_DISABLE_W {
        FLASH0_SLM_DISABLE_W { w: self }
    }
    #[doc = "Bit 4 - Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
    #[inline(always)]
    pub fn flash0_slm_status(&mut self) -> FLASH0_SLM_STATUS_W {
        FLASH0_SLM_STATUS_W { w: self }
    }
    #[doc = "Bit 2 - Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline(always)]
    pub fn cache_ready(&mut self) -> CACHE_READY_W {
        CACHE_READY_W { w: self }
    }
    #[doc = "Bit 1 - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline(always)]
    pub fn reset_stat(&mut self) -> RESET_STAT_W {
        RESET_STAT_W { w: self }
    }
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline(always)]
    pub fn invalidate(&mut self) -> INVALIDATE_W {
        INVALIDATE_W { w: self }
    }
}
