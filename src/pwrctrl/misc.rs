#[doc = "Reader of register MISC"]
pub type R = crate::R<u32, super::MISC>;
#[doc = "Writer for register MISC"]
pub type W = crate::W<u32, super::MISC>;
#[doc = "Register MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORCEBLEBUCKACT`"]
pub type FORCEBLEBUCKACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEBLEBUCKACT`"]
pub struct FORCEBLEBUCKACT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEBLEBUCKACT_W<'a> {
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
#[doc = "Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMVRLPBLE_A {
    #[doc = "1: Mem VR can go to lp mode even when BLE is powered on. value."]
    EN = 1,
    #[doc = "0: Mem VR will stay in active mode when BLE is powered on. value."]
    DIS = 0,
}
impl From<MEMVRLPBLE_A> for bool {
    #[inline(always)]
    fn from(variant: MEMVRLPBLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMVRLPBLE`"]
pub type MEMVRLPBLE_R = crate::R<bool, MEMVRLPBLE_A>;
impl MEMVRLPBLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMVRLPBLE_A {
        match self.bits {
            true => MEMVRLPBLE_A::EN,
            false => MEMVRLPBLE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MEMVRLPBLE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MEMVRLPBLE_A::DIS
    }
}
#[doc = "Write proxy for field `MEMVRLPBLE`"]
pub struct MEMVRLPBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMVRLPBLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMVRLPBLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mem VR can go to lp mode even when BLE is powered on. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMVRLPBLE_A::EN)
    }
    #[doc = "Mem VR will stay in active mode when BLE is powered on. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEMVRLPBLE_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Control Bit to force mem VR to LP or ACT mode in deep sleep when ADC is powered ON. 0x3 results in picking LP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORCEMEMVRADC_A {
    #[doc = "2: In this mode if all the other domains but ADC are powered down, mem VR will stay in ACT mode. value."]
    ACT = 2,
    #[doc = "1: In this mode if all the other domains but ADC are powered down, mem VR will stay in LP mode. value."]
    LP = 1,
    #[doc = "0: In this mode if all the other domains but ADC are powered down, mem VR will duty cycle between active and LP modes depending on ADC sampling. value."]
    DIS = 0,
}
impl From<FORCEMEMVRADC_A> for u8 {
    #[inline(always)]
    fn from(variant: FORCEMEMVRADC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORCEMEMVRADC`"]
pub type FORCEMEMVRADC_R = crate::R<u8, FORCEMEMVRADC_A>;
impl FORCEMEMVRADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FORCEMEMVRADC_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(FORCEMEMVRADC_A::ACT),
            1 => Val(FORCEMEMVRADC_A::LP),
            0 => Val(FORCEMEMVRADC_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACT`"]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == FORCEMEMVRADC_A::ACT
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == FORCEMEMVRADC_A::LP
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FORCEMEMVRADC_A::DIS
    }
}
#[doc = "Write proxy for field `FORCEMEMVRADC`"]
pub struct FORCEMEMVRADC_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEMEMVRADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEMEMVRADC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "In this mode if all the other domains but ADC are powered down, mem VR will stay in ACT mode. value."]
    #[inline(always)]
    pub fn act(self) -> &'a mut W {
        self.variant(FORCEMEMVRADC_A::ACT)
    }
    #[doc = "In this mode if all the other domains but ADC are powered down, mem VR will stay in LP mode. value."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(FORCEMEMVRADC_A::LP)
    }
    #[doc = "In this mode if all the other domains but ADC are powered down, mem VR will duty cycle between active and LP modes depending on ADC sampling. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FORCEMEMVRADC_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `FORCEMEMVRLPTIMERS`"]
pub type FORCEMEMVRLPTIMERS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEMEMVRLPTIMERS`"]
pub struct FORCEMEMVRLPTIMERS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEMEMVRLPTIMERS_W<'a> {
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
#[doc = "Reader of field `FORCECOREVRLPTIMERS`"]
pub type FORCECOREVRLPTIMERS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCECOREVRLPTIMERS`"]
pub struct FORCECOREVRLPTIMERS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCECOREVRLPTIMERS_W<'a> {
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
#[doc = "Reader of field `FORCECOREVRLPPDM`"]
pub type FORCECOREVRLPPDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCECOREVRLPPDM`"]
pub struct FORCECOREVRLPPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCECOREVRLPPDM_W<'a> {
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
#[doc = "Enables and Selects the SIMO Buck as the supply for the low-voltage power domain. It takes the initial value from the bit set in Customer INFO space.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIMOBUCKEN_A {
    #[doc = "1: Enable the SIMO Buck value."]
    EN = 1,
    #[doc = "0: Disable the SIMO Buck value."]
    DIS = 0,
}
impl From<SIMOBUCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIMOBUCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SIMOBUCKEN`"]
pub type SIMOBUCKEN_R = crate::R<bool, SIMOBUCKEN_A>;
impl SIMOBUCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIMOBUCKEN_A {
        match self.bits {
            true => SIMOBUCKEN_A::EN,
            false => SIMOBUCKEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SIMOBUCKEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SIMOBUCKEN_A::DIS
    }
}
#[doc = "Write proxy for field `SIMOBUCKEN`"]
pub struct SIMOBUCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIMOBUCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the SIMO Buck value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SIMOBUCKEN_A::EN)
    }
    #[doc = "Disable the SIMO Buck value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SIMOBUCKEN_A::DIS)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Control Bit to enable BLE Buck to be in active state when BLE Buck is enabled. Default behavior is to be in active only when Burst or BLEH power on are requested."]
    #[inline(always)]
    pub fn forceblebuckact(&self) -> FORCEBLEBUCKACT_R {
        FORCEBLEBUCKACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline(always)]
    pub fn memvrlpble(&self) -> MEMVRLPBLE_R {
        MEMVRLPBLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Control Bit to force mem VR to LP or ACT mode in deep sleep when ADC is powered ON. 0x3 results in picking LP mode."]
    #[inline(always)]
    pub fn forcememvradc(&self) -> FORCEMEMVRADC_R {
        FORCEMEMVRADC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub fn forcememvrlptimers(&self) -> FORCEMEMVRLPTIMERS_R {
        FORCEMEMVRLPTIMERS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control Bit to force Core VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub fn forcecorevrlptimers(&self) -> FORCECOREVRLPTIMERS_R {
        FORCECOREVRLPTIMERS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control bit to enable the core VR to go into LP mode with HCPA/B/C/MSPI are powered off but PDM is powered on"]
    #[inline(always)]
    pub fn forcecorevrlppdm(&self) -> FORCECOREVRLPPDM_R {
        FORCECOREVRLPPDM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables and Selects the SIMO Buck as the supply for the low-voltage power domain. It takes the initial value from the bit set in Customer INFO space."]
    #[inline(always)]
    pub fn simobucken(&self) -> SIMOBUCKEN_R {
        SIMOBUCKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Control Bit to enable BLE Buck to be in active state when BLE Buck is enabled. Default behavior is to be in active only when Burst or BLEH power on are requested."]
    #[inline(always)]
    pub fn forceblebuckact(&mut self) -> FORCEBLEBUCKACT_W {
        FORCEBLEBUCKACT_W { w: self }
    }
    #[doc = "Bit 6 - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline(always)]
    pub fn memvrlpble(&mut self) -> MEMVRLPBLE_W {
        MEMVRLPBLE_W { w: self }
    }
    #[doc = "Bits 4:5 - Control Bit to force mem VR to LP or ACT mode in deep sleep when ADC is powered ON. 0x3 results in picking LP mode."]
    #[inline(always)]
    pub fn forcememvradc(&mut self) -> FORCEMEMVRADC_W {
        FORCEMEMVRADC_W { w: self }
    }
    #[doc = "Bit 3 - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub fn forcememvrlptimers(&mut self) -> FORCEMEMVRLPTIMERS_W {
        FORCEMEMVRLPTIMERS_W { w: self }
    }
    #[doc = "Bit 2 - Control Bit to force Core VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub fn forcecorevrlptimers(&mut self) -> FORCECOREVRLPTIMERS_W {
        FORCECOREVRLPTIMERS_W { w: self }
    }
    #[doc = "Bit 1 - Control bit to enable the core VR to go into LP mode with HCPA/B/C/MSPI are powered off but PDM is powered on"]
    #[inline(always)]
    pub fn forcecorevrlppdm(&mut self) -> FORCECOREVRLPPDM_W {
        FORCECOREVRLPPDM_W { w: self }
    }
    #[doc = "Bit 0 - Enables and Selects the SIMO Buck as the supply for the low-voltage power domain. It takes the initial value from the bit set in Customer INFO space."]
    #[inline(always)]
    pub fn simobucken(&mut self) -> SIMOBUCKEN_W {
        SIMOBUCKEN_W { w: self }
    }
}
