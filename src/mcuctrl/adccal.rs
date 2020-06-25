#[doc = "Reader of register ADCCAL"]
pub type R = crate::R<u32, super::ADCCAL>;
#[doc = "Writer for register ADCCAL"]
pub type W = crate::W<u32, super::ADCCAL>;
#[doc = "Register ADCCAL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::ADCCAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Status for ADC Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCCALIBRATED_A {
    #[doc = "0: ADC is not calibrated value."]
    FALSE = 0,
    #[doc = "1: ADC is calibrated value."]
    TRUE = 1,
}
impl From<ADCCALIBRATED_A> for bool {
    #[inline(always)]
    fn from(variant: ADCCALIBRATED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCCALIBRATED`"]
pub type ADCCALIBRATED_R = crate::R<bool, ADCCALIBRATED_A>;
impl ADCCALIBRATED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCCALIBRATED_A {
        match self.bits {
            false => ADCCALIBRATED_A::FALSE,
            true => ADCCALIBRATED_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false_(&self) -> bool {
        *self == ADCCALIBRATED_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true_(&self) -> bool {
        *self == ADCCALIBRATED_A::TRUE
    }
}
#[doc = "Write proxy for field `ADCCALIBRATED`"]
pub struct ADCCALIBRATED_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCALIBRATED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCCALIBRATED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC is not calibrated value."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(ADCCALIBRATED_A::FALSE)
    }
    #[doc = "ADC is calibrated value."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(ADCCALIBRATED_A::TRUE)
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
#[doc = "Run ADC Calibration on initial power up sequence\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALONPWRUP_A {
    #[doc = "0: Disable automatic calibration on initial power up value."]
    DIS = 0,
    #[doc = "1: Enable automatic calibration on initial power up value."]
    EN = 1,
}
impl From<CALONPWRUP_A> for bool {
    #[inline(always)]
    fn from(variant: CALONPWRUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALONPWRUP`"]
pub type CALONPWRUP_R = crate::R<bool, CALONPWRUP_A>;
impl CALONPWRUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALONPWRUP_A {
        match self.bits {
            false => CALONPWRUP_A::DIS,
            true => CALONPWRUP_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CALONPWRUP_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CALONPWRUP_A::EN
    }
}
#[doc = "Write proxy for field `CALONPWRUP`"]
pub struct CALONPWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALONPWRUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALONPWRUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable automatic calibration on initial power up value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CALONPWRUP_A::DIS)
    }
    #[doc = "Enable automatic calibration on initial power up value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CALONPWRUP_A::EN)
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
    #[doc = "Bit 1 - Status for ADC Calibration"]
    #[inline(always)]
    pub fn adccalibrated(&self) -> ADCCALIBRATED_R {
        ADCCALIBRATED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Run ADC Calibration on initial power up sequence"]
    #[inline(always)]
    pub fn calonpwrup(&self) -> CALONPWRUP_R {
        CALONPWRUP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Status for ADC Calibration"]
    #[inline(always)]
    pub fn adccalibrated(&mut self) -> ADCCALIBRATED_W {
        ADCCALIBRATED_W { w: self }
    }
    #[doc = "Bit 0 - Run ADC Calibration on initial power up sequence"]
    #[inline(always)]
    pub fn calonpwrup(&mut self) -> CALONPWRUP_W {
        CALONPWRUP_W { w: self }
    }
}
