#[doc = "Reader of register SUPPLYSRC"]
pub type R = crate::R<u32, super::SUPPLYSRC>;
#[doc = "Writer for register SUPPLYSRC"]
pub type W = crate::W<u32, super::SUPPLYSRC>;
#[doc = "Register SUPPLYSRC `reset()`'s with value 0"]
impl crate::ResetValue for super::SUPPLYSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEBUCKEN_A {
    #[doc = "1: Enable the BLE Buck. value."]
    EN = 1,
    #[doc = "0: Disable the BLE Buck. value."]
    DIS = 0,
}
impl From<BLEBUCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: BLEBUCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLEBUCKEN`"]
pub type BLEBUCKEN_R = crate::R<bool, BLEBUCKEN_A>;
impl BLEBUCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEBUCKEN_A {
        match self.bits {
            true => BLEBUCKEN_A::EN,
            false => BLEBUCKEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BLEBUCKEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BLEBUCKEN_A::DIS
    }
}
#[doc = "Write proxy for field `BLEBUCKEN`"]
pub struct BLEBUCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEBUCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the BLE Buck. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BLEBUCKEN_A::EN)
    }
    #[doc = "Disable the BLE Buck. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLEBUCKEN_A::DIS)
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
    #[doc = "Bit 0 - Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
    #[inline(always)]
    pub fn blebucken(&self) -> BLEBUCKEN_R {
        BLEBUCKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
    #[inline(always)]
    pub fn blebucken(&mut self) -> BLEBUCKEN_W {
        BLEBUCKEN_W { w: self }
    }
}
