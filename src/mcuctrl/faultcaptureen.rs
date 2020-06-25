#[doc = "Reader of register FAULTCAPTUREEN"]
pub type R = crate::R<u32, super::FAULTCAPTUREEN>;
#[doc = "Writer for register FAULTCAPTUREEN"]
pub type W = crate::W<u32, super::FAULTCAPTUREEN>;
#[doc = "Register FAULTCAPTUREEN `reset()`'s with value 0"]
impl crate::ResetValue for super::FAULTCAPTUREEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTCAPTUREEN_A {
    #[doc = "0: Disable fault capture. value."]
    DIS = 0,
    #[doc = "1: Enable fault capture. value."]
    EN = 1,
}
impl From<FAULTCAPTUREEN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTCAPTUREEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTCAPTUREEN`"]
pub type FAULTCAPTUREEN_R = crate::R<bool, FAULTCAPTUREEN_A>;
impl FAULTCAPTUREEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTCAPTUREEN_A {
        match self.bits {
            false => FAULTCAPTUREEN_A::DIS,
            true => FAULTCAPTUREEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FAULTCAPTUREEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FAULTCAPTUREEN_A::EN
    }
}
#[doc = "Write proxy for field `FAULTCAPTUREEN`"]
pub struct FAULTCAPTUREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTCAPTUREEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTCAPTUREEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable fault capture. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FAULTCAPTUREEN_A::DIS)
    }
    #[doc = "Enable fault capture. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FAULTCAPTUREEN_A::EN)
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
    #[doc = "Bit 0 - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[inline(always)]
    pub fn faultcaptureen(&self) -> FAULTCAPTUREEN_R {
        FAULTCAPTUREEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[inline(always)]
    pub fn faultcaptureen(&mut self) -> FAULTCAPTUREEN_W {
        FAULTCAPTUREEN_W { w: self }
    }
}
