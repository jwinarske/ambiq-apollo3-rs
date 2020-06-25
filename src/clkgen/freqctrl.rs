#[doc = "Reader of register FREQCTRL"]
pub type R = crate::R<u32, super::FREQCTRL>;
#[doc = "Writer for register FREQCTRL"]
pub type W = crate::W<u32, super::FREQCTRL>;
#[doc = "Register FREQCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FREQCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BURSTSTATUS`"]
pub type BURSTSTATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURSTSTATUS`"]
pub struct BURSTSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTSTATUS_W<'a> {
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
#[doc = "Reader of field `BURSTACK`"]
pub type BURSTACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURSTACK`"]
pub struct BURSTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTACK_W<'a> {
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
#[doc = "Frequency Burst Enable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTREQ_A {
    #[doc = "0: Frequency for ARM core stays at 48MHz value."]
    DIS = 0,
    #[doc = "1: Frequency for ARM core is increased to 96MHz value."]
    EN = 1,
}
impl From<BURSTREQ_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BURSTREQ`"]
pub type BURSTREQ_R = crate::R<bool, BURSTREQ_A>;
impl BURSTREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTREQ_A {
        match self.bits {
            false => BURSTREQ_A::DIS,
            true => BURSTREQ_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BURSTREQ_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BURSTREQ_A::EN
    }
}
#[doc = "Write proxy for field `BURSTREQ`"]
pub struct BURSTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frequency for ARM core stays at 48MHz value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTREQ_A::DIS)
    }
    #[doc = "Frequency for ARM core is increased to 96MHz value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTREQ_A::EN)
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
    #[doc = "Bit 2 - This represents frequency burst status."]
    #[inline(always)]
    pub fn burststatus(&self) -> BURSTSTATUS_R {
        BURSTSTATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
    #[inline(always)]
    pub fn burstack(&self) -> BURSTACK_R {
        BURSTACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Frequency Burst Enable Request"]
    #[inline(always)]
    pub fn burstreq(&self) -> BURSTREQ_R {
        BURSTREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This represents frequency burst status."]
    #[inline(always)]
    pub fn burststatus(&mut self) -> BURSTSTATUS_W {
        BURSTSTATUS_W { w: self }
    }
    #[doc = "Bit 1 - Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
    #[inline(always)]
    pub fn burstack(&mut self) -> BURSTACK_W {
        BURSTACK_W { w: self }
    }
    #[doc = "Bit 0 - Frequency Burst Enable Request"]
    #[inline(always)]
    pub fn burstreq(&mut self) -> BURSTREQ_W {
        BURSTREQ_W { w: self }
    }
}
