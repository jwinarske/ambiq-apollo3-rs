#[doc = "Reader of register ADCBATTLOAD"]
pub type R = crate::R<u32, super::ADCBATTLOAD>;
#[doc = "Writer for register ADCBATTLOAD"]
pub type W = crate::W<u32, super::ADCBATTLOAD>;
#[doc = "Register ADCBATTLOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCBATTLOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable the ADC battery load resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATTLOAD_A {
    #[doc = "0: Battery load is disconnected value."]
    DIS = 0,
    #[doc = "1: Battery load is enabled value."]
    EN = 1,
}
impl From<BATTLOAD_A> for bool {
    #[inline(always)]
    fn from(variant: BATTLOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BATTLOAD`"]
pub type BATTLOAD_R = crate::R<bool, BATTLOAD_A>;
impl BATTLOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATTLOAD_A {
        match self.bits {
            false => BATTLOAD_A::DIS,
            true => BATTLOAD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BATTLOAD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BATTLOAD_A::EN
    }
}
#[doc = "Write proxy for field `BATTLOAD`"]
pub struct BATTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BATTLOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BATTLOAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Battery load is disconnected value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BATTLOAD_A::DIS)
    }
    #[doc = "Battery load is enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BATTLOAD_A::EN)
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
    #[doc = "Bit 0 - Enable the ADC battery load resistor"]
    #[inline(always)]
    pub fn battload(&self) -> BATTLOAD_R {
        BATTLOAD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the ADC battery load resistor"]
    #[inline(always)]
    pub fn battload(&mut self) -> BATTLOAD_W {
        BATTLOAD_W { w: self }
    }
}
