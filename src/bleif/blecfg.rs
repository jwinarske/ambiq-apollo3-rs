#[doc = "Reader of register BLECFG"]
pub type R = crate::R<u32, super::BLECFG>;
#[doc = "Writer for register BLECFG"]
pub type W = crate::W<u32, super::BLECFG>;
#[doc = "Register BLECFG `reset()`'s with value 0"]
impl crate::ResetValue for super::BLECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configuration of BLEH isolation controls for SPI related signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPIISOCTL_A {
    #[doc = "3: SPI signals from BLE Core to/from MCU Core are isolated. value."]
    ON = 3,
    #[doc = "2: SPI signals from BLE Core to/from MCU Core are not isolated. value."]
    OFF = 2,
    #[doc = "0: SPI signals from BLE Core to/from MCU Core are automatically isolated by the logic value."]
    AUTO = 0,
}
impl From<SPIISOCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPIISOCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPIISOCTL`"]
pub type SPIISOCTL_R = crate::R<u8, SPIISOCTL_A>;
impl SPIISOCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPIISOCTL_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(SPIISOCTL_A::ON),
            2 => Val(SPIISOCTL_A::OFF),
            0 => Val(SPIISOCTL_A::AUTO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == SPIISOCTL_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SPIISOCTL_A::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == SPIISOCTL_A::AUTO
    }
}
#[doc = "Write proxy for field `SPIISOCTL`"]
pub struct SPIISOCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIISOCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIISOCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI signals from BLE Core to/from MCU Core are isolated. value."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SPIISOCTL_A::ON)
    }
    #[doc = "SPI signals from BLE Core to/from MCU Core are not isolated. value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SPIISOCTL_A::OFF)
    }
    #[doc = "SPI signals from BLE Core to/from MCU Core are automatically isolated by the logic value."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(SPIISOCTL_A::AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Configuration of BLEH isolation control for power related signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRISOCTL_A {
    #[doc = "3: BLEH power signal isolation to on (isolated). value."]
    ON = 3,
    #[doc = "2: BLEH power signal isolation to off (not isolated). value."]
    OFF = 2,
    #[doc = "0: BLEH Power signal isolation is controlled automatically through the interface logic value."]
    AUTO = 0,
}
impl From<PWRISOCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRISOCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWRISOCTL`"]
pub type PWRISOCTL_R = crate::R<u8, PWRISOCTL_A>;
impl PWRISOCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWRISOCTL_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(PWRISOCTL_A::ON),
            2 => Val(PWRISOCTL_A::OFF),
            0 => Val(PWRISOCTL_A::AUTO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PWRISOCTL_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWRISOCTL_A::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == PWRISOCTL_A::AUTO
    }
}
#[doc = "Write proxy for field `PWRISOCTL`"]
pub struct PWRISOCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRISOCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRISOCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BLEH power signal isolation to on (isolated). value."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PWRISOCTL_A::ON)
    }
    #[doc = "BLEH power signal isolation to off (not isolated). value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWRISOCTL_A::OFF)
    }
    #[doc = "BLEH Power signal isolation is controlled automatically through the interface logic value."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(PWRISOCTL_A::AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `STAYASLEEP`"]
pub type STAYASLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STAYASLEEP`"]
pub struct STAYASLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> STAYASLEEP_W<'a> {
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
#[doc = "Reader of field `FRCCLK`"]
pub type FRCCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRCCLK`"]
pub struct FRCCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCCLK_W<'a> {
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
#[doc = "Reader of field `MCUFRCSLP`"]
pub type MCUFRCSLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCUFRCSLP`"]
pub struct MCUFRCSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUFRCSLP_W<'a> {
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
#[doc = "Reader of field `WT4ACTOFF`"]
pub type WT4ACTOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WT4ACTOFF`"]
pub struct WT4ACTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> WT4ACTOFF_W<'a> {
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
#[doc = "BLEH power on request override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEHREQCTL_A {
    #[doc = "3: BLEH Power-on reg signal is set to on (1). value."]
    ON = 3,
    #[doc = "2: BLEH Power-on signal is set to off (0). value."]
    OFF = 2,
    #[doc = "0: BLEH Power-on signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO = 0,
}
impl From<BLEHREQCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEHREQCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEHREQCTL`"]
pub type BLEHREQCTL_R = crate::R<u8, BLEHREQCTL_A>;
impl BLEHREQCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEHREQCTL_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(BLEHREQCTL_A::ON),
            2 => Val(BLEHREQCTL_A::OFF),
            0 => Val(BLEHREQCTL_A::AUTO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == BLEHREQCTL_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BLEHREQCTL_A::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == BLEHREQCTL_A::AUTO
    }
}
#[doc = "Write proxy for field `BLEHREQCTL`"]
pub struct BLEHREQCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEHREQCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEHREQCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BLEH Power-on reg signal is set to on (1). value."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(BLEHREQCTL_A::ON)
    }
    #[doc = "BLEH Power-on signal is set to off (0). value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(BLEHREQCTL_A::OFF)
    }
    #[doc = "BLEH Power-on signal is controlled by the PWRSM logic and automatically controlled value."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(BLEHREQCTL_A::AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "DCDCFLG signal override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCDCFLGCTL_A {
    #[doc = "3: DCDC Flag signal is set to on (1). value."]
    ON = 3,
    #[doc = "2: DCDC Flag signal is set to off (0). value."]
    OFF = 2,
    #[doc = "0: DCDC Flag signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO = 0,
}
impl From<DCDCFLGCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCDCFLGCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCDCFLGCTL`"]
pub type DCDCFLGCTL_R = crate::R<u8, DCDCFLGCTL_A>;
impl DCDCFLGCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCDCFLGCTL_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(DCDCFLGCTL_A::ON),
            2 => Val(DCDCFLGCTL_A::OFF),
            0 => Val(DCDCFLGCTL_A::AUTO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == DCDCFLGCTL_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DCDCFLGCTL_A::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == DCDCFLGCTL_A::AUTO
    }
}
#[doc = "Write proxy for field `DCDCFLGCTL`"]
pub struct DCDCFLGCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCFLGCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDCFLGCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DCDC Flag signal is set to on (1). value."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(DCDCFLGCTL_A::ON)
    }
    #[doc = "DCDC Flag signal is set to off (0). value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DCDCFLGCTL_A::OFF)
    }
    #[doc = "DCDC Flag signal is controlled by the PWRSM logic and automatically controlled value."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(DCDCFLGCTL_A::AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "WAKE signal override. Controls the source of the WAKE signal to the BLE Core.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAKEUPCTL_A {
    #[doc = "3: Wake signal is set to on (1). value."]
    ON = 3,
    #[doc = "2: Wake signal is set to off (0). value."]
    OFF = 2,
    #[doc = "0: Wake signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO = 0,
}
impl From<WAKEUPCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: WAKEUPCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAKEUPCTL`"]
pub type WAKEUPCTL_R = crate::R<u8, WAKEUPCTL_A>;
impl WAKEUPCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAKEUPCTL_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(WAKEUPCTL_A::ON),
            2 => Val(WAKEUPCTL_A::OFF),
            0 => Val(WAKEUPCTL_A::AUTO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == WAKEUPCTL_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == WAKEUPCTL_A::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == WAKEUPCTL_A::AUTO
    }
}
#[doc = "Write proxy for field `WAKEUPCTL`"]
pub struct WAKEUPCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUPCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Wake signal is set to on (1). value."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(WAKEUPCTL_A::ON)
    }
    #[doc = "Wake signal is set to off (0). value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(WAKEUPCTL_A::OFF)
    }
    #[doc = "Wake signal is controlled by the PWRSM logic and automatically controlled value."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(WAKEUPCTL_A::AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reset line to the BLE Core. This will reset the BLE core when asserted ('0') and must be written to '1' prior to performing any BTLE related operations to the core.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLERSTN_A {
    #[doc = "1: The reset signal is active (0) value."]
    ACTIVE = 1,
    #[doc = "0: The reset signal is inactive (1) value."]
    INACTIVE = 0,
}
impl From<BLERSTN_A> for bool {
    #[inline(always)]
    fn from(variant: BLERSTN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLERSTN`"]
pub type BLERSTN_R = crate::R<bool, BLERSTN_A>;
impl BLERSTN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLERSTN_A {
        match self.bits {
            true => BLERSTN_A::ACTIVE,
            false => BLERSTN_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BLERSTN_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BLERSTN_A::INACTIVE
    }
}
#[doc = "Write proxy for field `BLERSTN`"]
pub struct BLERSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLERSTN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLERSTN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The reset signal is active (0) value."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(BLERSTN_A::ACTIVE)
    }
    #[doc = "The reset signal is inactive (1) value."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(BLERSTN_A::INACTIVE)
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
#[doc = "Enable the power state machine for automatic sequencing and control of power states of the BLE Core module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSMEN_A {
    #[doc = "1: Internal power state machine is enabled and will sequence the BLEH power domain as indicated in the design document. Overrides for the power signals are not enabled. value."]
    ON = 1,
    #[doc = "0: Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    OFF = 0,
}
impl From<PWRSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRSMEN`"]
pub type PWRSMEN_R = crate::R<bool, PWRSMEN_A>;
impl PWRSMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSMEN_A {
        match self.bits {
            true => PWRSMEN_A::ON,
            false => PWRSMEN_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PWRSMEN_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWRSMEN_A::OFF
    }
}
#[doc = "Write proxy for field `PWRSMEN`"]
pub struct PWRSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal power state machine is enabled and will sequence the BLEH power domain as indicated in the design document. Overrides for the power signals are not enabled. value."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PWRSMEN_A::ON)
    }
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWRSMEN_A::OFF)
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
    #[doc = "Bits 14:15 - Configuration of BLEH isolation controls for SPI related signals."]
    #[inline(always)]
    pub fn spiisoctl(&self) -> SPIISOCTL_R {
        SPIISOCTL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Configuration of BLEH isolation control for power related signals."]
    #[inline(always)]
    pub fn pwrisoctl(&self) -> PWRISOCTL_R {
        PWRISOCTL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Set to prevent the BLE power control module from waking up the BLE Core after going into power down. To be used for graceful shutdown, set by software prior to powering off and will allow assertion of reset from sleep state."]
    #[inline(always)]
    pub fn stayasleep(&self) -> STAYASLEEP_R {
        STAYASLEEP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force the clock in the BLEIF to be always running"]
    #[inline(always)]
    pub fn frcclk(&self) -> FRCCLK_R {
        FRCCLK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force power state machine to go to the sleep state. Intended for debug only. Has no effect on the actual BLE Core state, only the state of the BLEIF interface state machine."]
    #[inline(always)]
    pub fn mcufrcslp(&self) -> MCUFRCSLP_R {
        MCUFRCSLP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Debug control of BLEIF power state machine. Allows transition into the active state in the BLEIF state without waiting for dcdc req from BLE Core."]
    #[inline(always)]
    pub fn wt4actoff(&self) -> WT4ACTOFF_R {
        WT4ACTOFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - BLEH power on request override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline(always)]
    pub fn blehreqctl(&self) -> BLEHREQCTL_R {
        BLEHREQCTL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - DCDCFLG signal override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline(always)]
    pub fn dcdcflgctl(&self) -> DCDCFLGCTL_R {
        DCDCFLGCTL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - WAKE signal override. Controls the source of the WAKE signal to the BLE Core."]
    #[inline(always)]
    pub fn wakeupctl(&self) -> WAKEUPCTL_R {
        WAKEUPCTL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Reset line to the BLE Core. This will reset the BLE core when asserted ('0') and must be written to '1' prior to performing any BTLE related operations to the core."]
    #[inline(always)]
    pub fn blerstn(&self) -> BLERSTN_R {
        BLERSTN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable the power state machine for automatic sequencing and control of power states of the BLE Core module."]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 14:15 - Configuration of BLEH isolation controls for SPI related signals."]
    #[inline(always)]
    pub fn spiisoctl(&mut self) -> SPIISOCTL_W {
        SPIISOCTL_W { w: self }
    }
    #[doc = "Bits 12:13 - Configuration of BLEH isolation control for power related signals."]
    #[inline(always)]
    pub fn pwrisoctl(&mut self) -> PWRISOCTL_W {
        PWRISOCTL_W { w: self }
    }
    #[doc = "Bit 11 - Set to prevent the BLE power control module from waking up the BLE Core after going into power down. To be used for graceful shutdown, set by software prior to powering off and will allow assertion of reset from sleep state."]
    #[inline(always)]
    pub fn stayasleep(&mut self) -> STAYASLEEP_W {
        STAYASLEEP_W { w: self }
    }
    #[doc = "Bit 10 - Force the clock in the BLEIF to be always running"]
    #[inline(always)]
    pub fn frcclk(&mut self) -> FRCCLK_W {
        FRCCLK_W { w: self }
    }
    #[doc = "Bit 9 - Force power state machine to go to the sleep state. Intended for debug only. Has no effect on the actual BLE Core state, only the state of the BLEIF interface state machine."]
    #[inline(always)]
    pub fn mcufrcslp(&mut self) -> MCUFRCSLP_W {
        MCUFRCSLP_W { w: self }
    }
    #[doc = "Bit 8 - Debug control of BLEIF power state machine. Allows transition into the active state in the BLEIF state without waiting for dcdc req from BLE Core."]
    #[inline(always)]
    pub fn wt4actoff(&mut self) -> WT4ACTOFF_W {
        WT4ACTOFF_W { w: self }
    }
    #[doc = "Bits 6:7 - BLEH power on request override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline(always)]
    pub fn blehreqctl(&mut self) -> BLEHREQCTL_W {
        BLEHREQCTL_W { w: self }
    }
    #[doc = "Bits 4:5 - DCDCFLG signal override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline(always)]
    pub fn dcdcflgctl(&mut self) -> DCDCFLGCTL_W {
        DCDCFLGCTL_W { w: self }
    }
    #[doc = "Bits 2:3 - WAKE signal override. Controls the source of the WAKE signal to the BLE Core."]
    #[inline(always)]
    pub fn wakeupctl(&mut self) -> WAKEUPCTL_W {
        WAKEUPCTL_W { w: self }
    }
    #[doc = "Bit 1 - Reset line to the BLE Core. This will reset the BLE core when asserted ('0') and must be written to '1' prior to performing any BTLE related operations to the core."]
    #[inline(always)]
    pub fn blerstn(&mut self) -> BLERSTN_W {
        BLERSTN_W { w: self }
    }
    #[doc = "Bit 0 - Enable the power state machine for automatic sequencing and control of power states of the BLE Core module."]
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W {
        PWRSMEN_W { w: self }
    }
}
