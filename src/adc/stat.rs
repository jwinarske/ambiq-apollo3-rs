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
#[doc = "Indicates the power-status of the ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDSTAT_A {
    #[doc = "0: Powered on. value."]
    ON = 0,
    #[doc = "1: ADC Low Power Mode 1. value."]
    POWERED_DOWN = 1,
}
impl From<PWDSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: PWDSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWDSTAT`"]
pub type PWDSTAT_R = crate::R<bool, PWDSTAT_A>;
impl PWDSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWDSTAT_A {
        match self.bits {
            false => PWDSTAT_A::ON,
            true => PWDSTAT_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PWDSTAT_A::ON
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PWDSTAT_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `PWDSTAT`"]
pub struct PWDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDSTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered on. value."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PWDSTAT_A::ON)
    }
    #[doc = "ADC Low Power Mode 1. value."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PWDSTAT_A::POWERED_DOWN)
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
    #[doc = "Bit 0 - Indicates the power-status of the ADC."]
    #[inline(always)]
    pub fn pwdstat(&self) -> PWDSTAT_R {
        PWDSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates the power-status of the ADC."]
    #[inline(always)]
    pub fn pwdstat(&mut self) -> PWDSTAT_W {
        PWDSTAT_W { w: self }
    }
}
