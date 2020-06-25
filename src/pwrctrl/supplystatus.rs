#[doc = "Reader of register SUPPLYSTATUS"]
pub type R = crate::R<u32, super::SUPPLYSTATUS>;
#[doc = "Writer for register SUPPLYSTATUS"]
pub type W = crate::W<u32, super::SUPPLYSTATUS>;
#[doc = "Register SUPPLYSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::SUPPLYSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicates whether the BLE (if supported) domain and burst (if supported) domain is supplied from the LDO or the Buck. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate reature is allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEBUCKON_A {
    #[doc = "0: Indicates the the LDO is supplying the BLE/Burst power domain value."]
    LDO = 0,
    #[doc = "1: Indicates the the Buck is supplying the BLE/Burst power domain value."]
    BUCK = 1,
}
impl From<BLEBUCKON_A> for bool {
    #[inline(always)]
    fn from(variant: BLEBUCKON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLEBUCKON`"]
pub type BLEBUCKON_R = crate::R<bool, BLEBUCKON_A>;
impl BLEBUCKON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEBUCKON_A {
        match self.bits {
            false => BLEBUCKON_A::LDO,
            true => BLEBUCKON_A::BUCK,
        }
    }
    #[doc = "Checks if the value of the field is `LDO`"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == BLEBUCKON_A::LDO
    }
    #[doc = "Checks if the value of the field is `BUCK`"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        *self == BLEBUCKON_A::BUCK
    }
}
#[doc = "Write proxy for field `BLEBUCKON`"]
pub struct BLEBUCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEBUCKON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Indicates the the LDO is supplying the BLE/Burst power domain value."]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut W {
        self.variant(BLEBUCKON_A::LDO)
    }
    #[doc = "Indicates the the Buck is supplying the BLE/Burst power domain value."]
    #[inline(always)]
    pub fn buck(self) -> &'a mut W {
        self.variant(BLEBUCKON_A::BUCK)
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
#[doc = "Indicates whether the Core/Mem low-voltage domains are supplied from the LDO or the Buck.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIMOBUCKON_A {
    #[doc = "0: Indicates the the SIMO Buck is OFF. value."]
    OFF = 0,
    #[doc = "1: Indicates the the SIMO Buck is ON. value."]
    ON = 1,
}
impl From<SIMOBUCKON_A> for bool {
    #[inline(always)]
    fn from(variant: SIMOBUCKON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SIMOBUCKON`"]
pub type SIMOBUCKON_R = crate::R<bool, SIMOBUCKON_A>;
impl SIMOBUCKON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIMOBUCKON_A {
        match self.bits {
            false => SIMOBUCKON_A::OFF,
            true => SIMOBUCKON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SIMOBUCKON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == SIMOBUCKON_A::ON
    }
}
#[doc = "Write proxy for field `SIMOBUCKON`"]
pub struct SIMOBUCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIMOBUCKON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Indicates the the SIMO Buck is OFF. value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SIMOBUCKON_A::OFF)
    }
    #[doc = "Indicates the the SIMO Buck is ON. value."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SIMOBUCKON_A::ON)
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
    #[doc = "Bit 1 - Indicates whether the BLE (if supported) domain and burst (if supported) domain is supplied from the LDO or the Buck. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate reature is allowed."]
    #[inline(always)]
    pub fn blebuckon(&self) -> BLEBUCKON_R {
        BLEBUCKON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates whether the Core/Mem low-voltage domains are supplied from the LDO or the Buck."]
    #[inline(always)]
    pub fn simobuckon(&self) -> SIMOBUCKON_R {
        SIMOBUCKON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Indicates whether the BLE (if supported) domain and burst (if supported) domain is supplied from the LDO or the Buck. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate reature is allowed."]
    #[inline(always)]
    pub fn blebuckon(&mut self) -> BLEBUCKON_W {
        BLEBUCKON_W { w: self }
    }
    #[doc = "Bit 0 - Indicates whether the Core/Mem low-voltage domains are supplied from the LDO or the Buck."]
    #[inline(always)]
    pub fn simobuckon(&mut self) -> SIMOBUCKON_W {
        SIMOBUCKON_W { w: self }
    }
}
