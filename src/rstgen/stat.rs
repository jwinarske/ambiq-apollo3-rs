#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBOOT`"]
pub type SBOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBOOT`"]
pub struct SBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> SBOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `FBOOT`"]
pub type FBOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBOOT`"]
pub struct FBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> FBOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `BOBSTAT`"]
pub type BOBSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOBSTAT`"]
pub struct BOBSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOBSTAT_W<'a> {
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
#[doc = "Reader of field `BOFSTAT`"]
pub type BOFSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOFSTAT`"]
pub struct BOFSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFSTAT_W<'a> {
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
#[doc = "Reader of field `BOCSTAT`"]
pub type BOCSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOCSTAT`"]
pub struct BOCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOCSTAT_W<'a> {
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
#[doc = "Reader of field `BOUSTAT`"]
pub type BOUSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOUSTAT`"]
pub struct BOUSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUSTAT_W<'a> {
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
#[doc = "Reader of field `WDRSTAT`"]
pub type WDRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDRSTAT`"]
pub struct WDRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRSTAT_W<'a> {
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
#[doc = "Reader of field `DBGRSTAT`"]
pub type DBGRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGRSTAT`"]
pub struct DBGRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRSTAT_W<'a> {
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
#[doc = "Reader of field `POIRSTAT`"]
pub type POIRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POIRSTAT`"]
pub struct POIRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> POIRSTAT_W<'a> {
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
#[doc = "Reader of field `SWRSTAT`"]
pub type SWRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRSTAT`"]
pub struct SWRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTAT_W<'a> {
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
#[doc = "Reader of field `BORSTAT`"]
pub type BORSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BORSTAT`"]
pub struct BORSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BORSTAT_W<'a> {
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
#[doc = "Reader of field `PORSTAT`"]
pub type PORSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORSTAT`"]
pub struct PORSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORSTAT_W<'a> {
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
#[doc = "Reader of field `EXRSTAT`"]
pub type EXRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXRSTAT`"]
pub struct EXRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXRSTAT_W<'a> {
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
    #[doc = "Bit 31 - Set when booting securely (SBL)."]
    #[inline(always)]
    pub fn sboot(&self) -> SBOOT_R {
        SBOOT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set if current boot was initiated by soft reset and resulted in Fast Boot (SBL)."]
    #[inline(always)]
    pub fn fboot(&self) -> FBOOT_R {
        FBOOT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 10 - A BLE/Burst Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bobstat(&self) -> BOBSTAT_R {
        BOBSTAT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - A Memory Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bofstat(&self) -> BOFSTAT_R {
        BOFSTAT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - A Core Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bocstat(&self) -> BOCSTAT_R {
        BOCSTAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - An Unregulated Supply Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn boustat(&self) -> BOUSTAT_R {
        BOUSTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset was initiated by a Watchdog Timer Reset (SBL)."]
    #[inline(always)]
    pub fn wdrstat(&self) -> WDRSTAT_R {
        WDRSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset was a initiated by Debugger Reset (SBL)."]
    #[inline(always)]
    pub fn dbgrstat(&self) -> DBGRSTAT_R {
        DBGRSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset was a initiated by Software POI Reset (SBL)."]
    #[inline(always)]
    pub fn poirstat(&self) -> POIRSTAT_R {
        POIRSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset was a initiated by SW POR or AIRCR Reset (SBL)."]
    #[inline(always)]
    pub fn swrstat(&self) -> SWRSTAT_R {
        SWRSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset was initiated by a Brown-Out Reset (SBL)."]
    #[inline(always)]
    pub fn borstat(&self) -> BORSTAT_R {
        BORSTAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset was initiated by a Power-On Reset (SBL)."]
    #[inline(always)]
    pub fn porstat(&self) -> PORSTAT_R {
        PORSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Reset was initiated by an External Reset (SBL)."]
    #[inline(always)]
    pub fn exrstat(&self) -> EXRSTAT_R {
        EXRSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Set when booting securely (SBL)."]
    #[inline(always)]
    pub fn sboot(&mut self) -> SBOOT_W {
        SBOOT_W { w: self }
    }
    #[doc = "Bit 30 - Set if current boot was initiated by soft reset and resulted in Fast Boot (SBL)."]
    #[inline(always)]
    pub fn fboot(&mut self) -> FBOOT_W {
        FBOOT_W { w: self }
    }
    #[doc = "Bit 10 - A BLE/Burst Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bobstat(&mut self) -> BOBSTAT_W {
        BOBSTAT_W { w: self }
    }
    #[doc = "Bit 9 - A Memory Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bofstat(&mut self) -> BOFSTAT_W {
        BOFSTAT_W { w: self }
    }
    #[doc = "Bit 8 - A Core Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bocstat(&mut self) -> BOCSTAT_W {
        BOCSTAT_W { w: self }
    }
    #[doc = "Bit 7 - An Unregulated Supply Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn boustat(&mut self) -> BOUSTAT_W {
        BOUSTAT_W { w: self }
    }
    #[doc = "Bit 6 - Reset was initiated by a Watchdog Timer Reset (SBL)."]
    #[inline(always)]
    pub fn wdrstat(&mut self) -> WDRSTAT_W {
        WDRSTAT_W { w: self }
    }
    #[doc = "Bit 5 - Reset was a initiated by Debugger Reset (SBL)."]
    #[inline(always)]
    pub fn dbgrstat(&mut self) -> DBGRSTAT_W {
        DBGRSTAT_W { w: self }
    }
    #[doc = "Bit 4 - Reset was a initiated by Software POI Reset (SBL)."]
    #[inline(always)]
    pub fn poirstat(&mut self) -> POIRSTAT_W {
        POIRSTAT_W { w: self }
    }
    #[doc = "Bit 3 - Reset was a initiated by SW POR or AIRCR Reset (SBL)."]
    #[inline(always)]
    pub fn swrstat(&mut self) -> SWRSTAT_W {
        SWRSTAT_W { w: self }
    }
    #[doc = "Bit 2 - Reset was initiated by a Brown-Out Reset (SBL)."]
    #[inline(always)]
    pub fn borstat(&mut self) -> BORSTAT_W {
        BORSTAT_W { w: self }
    }
    #[doc = "Bit 1 - Reset was initiated by a Power-On Reset (SBL)."]
    #[inline(always)]
    pub fn porstat(&mut self) -> PORSTAT_W {
        PORSTAT_W { w: self }
    }
    #[doc = "Bit 0 - Reset was initiated by an External Reset (SBL)."]
    #[inline(always)]
    pub fn exrstat(&mut self) -> EXRSTAT_W {
        EXRSTAT_W { w: self }
    }
}
