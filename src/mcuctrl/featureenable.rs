#[doc = "Reader of register FEATUREENABLE"]
pub type R = crate::R<u32, super::FEATUREENABLE>;
#[doc = "Writer for register FEATUREENABLE"]
pub type W = crate::W<u32, super::FEATUREENABLE>;
#[doc = "Register FEATUREENABLE `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FEATUREENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Availability of Burst functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTAVAIL_A {
    #[doc = "1: Burst functionality available value."]
    AVAIL = 1,
    #[doc = "0: Burst functionality not available value."]
    NOTAVAIL = 0,
}
impl From<BURSTAVAIL_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTAVAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BURSTAVAIL`"]
pub type BURSTAVAIL_R = crate::R<bool, BURSTAVAIL_A>;
impl BURSTAVAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTAVAIL_A {
        match self.bits {
            true => BURSTAVAIL_A::AVAIL,
            false => BURSTAVAIL_A::NOTAVAIL,
        }
    }
    #[doc = "Checks if the value of the field is `AVAIL`"]
    #[inline(always)]
    pub fn is_avail(&self) -> bool {
        *self == BURSTAVAIL_A::AVAIL
    }
    #[doc = "Checks if the value of the field is `NOTAVAIL`"]
    #[inline(always)]
    pub fn is_notavail(&self) -> bool {
        *self == BURSTAVAIL_A::NOTAVAIL
    }
}
#[doc = "Write proxy for field `BURSTAVAIL`"]
pub struct BURSTAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTAVAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTAVAIL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Burst functionality available value."]
    #[inline(always)]
    pub fn avail(self) -> &'a mut W {
        self.variant(BURSTAVAIL_A::AVAIL)
    }
    #[doc = "Burst functionality not available value."]
    #[inline(always)]
    pub fn notavail(self) -> &'a mut W {
        self.variant(BURSTAVAIL_A::NOTAVAIL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Controls the Burst functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTREQ_A {
    #[doc = "1: Enable the Burst functionality value."]
    EN = 1,
    #[doc = "0: Disable the Burst functionality value."]
    DIS = 0,
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
            true => BURSTREQ_A::EN,
            false => BURSTREQ_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BURSTREQ_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BURSTREQ_A::DIS
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
    #[doc = "Enable the Burst functionality value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTREQ_A::EN)
    }
    #[doc = "Disable the Burst functionality value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTREQ_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "AVAILABILITY of the BLE functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEAVAIL_A {
    #[doc = "1: BLE functionality available value."]
    AVAIL = 1,
    #[doc = "0: BLE functionality not available value."]
    NOTAVAIL = 0,
}
impl From<BLEAVAIL_A> for bool {
    #[inline(always)]
    fn from(variant: BLEAVAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLEAVAIL`"]
pub type BLEAVAIL_R = crate::R<bool, BLEAVAIL_A>;
impl BLEAVAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEAVAIL_A {
        match self.bits {
            true => BLEAVAIL_A::AVAIL,
            false => BLEAVAIL_A::NOTAVAIL,
        }
    }
    #[doc = "Checks if the value of the field is `AVAIL`"]
    #[inline(always)]
    pub fn is_avail(&self) -> bool {
        *self == BLEAVAIL_A::AVAIL
    }
    #[doc = "Checks if the value of the field is `NOTAVAIL`"]
    #[inline(always)]
    pub fn is_notavail(&self) -> bool {
        *self == BLEAVAIL_A::NOTAVAIL
    }
}
#[doc = "Write proxy for field `BLEAVAIL`"]
pub struct BLEAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEAVAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEAVAIL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BLE functionality available value."]
    #[inline(always)]
    pub fn avail(self) -> &'a mut W {
        self.variant(BLEAVAIL_A::AVAIL)
    }
    #[doc = "BLE functionality not available value."]
    #[inline(always)]
    pub fn notavail(self) -> &'a mut W {
        self.variant(BLEAVAIL_A::NOTAVAIL)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BLEACK`"]
pub type BLEACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEACK`"]
pub struct BLEACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEACK_W<'a> {
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
#[doc = "Controls the BLE functionality\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEREQ_A {
    #[doc = "1: Enable the BLE functionality value."]
    EN = 1,
    #[doc = "0: Disable the BLE functionality value."]
    DIS = 0,
}
impl From<BLEREQ_A> for bool {
    #[inline(always)]
    fn from(variant: BLEREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLEREQ`"]
pub type BLEREQ_R = crate::R<bool, BLEREQ_A>;
impl BLEREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEREQ_A {
        match self.bits {
            true => BLEREQ_A::EN,
            false => BLEREQ_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BLEREQ_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BLEREQ_A::DIS
    }
}
#[doc = "Write proxy for field `BLEREQ`"]
pub struct BLEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the BLE functionality value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BLEREQ_A::EN)
    }
    #[doc = "Disable the BLE functionality value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLEREQ_A::DIS)
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
    #[doc = "Bit 6 - Availability of Burst functionality"]
    #[inline(always)]
    pub fn burstavail(&self) -> BURSTAVAIL_R {
        BURSTAVAIL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACK for BURSTREQ"]
    #[inline(always)]
    pub fn burstack(&self) -> BURSTACK_R {
        BURSTACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls the Burst functionality"]
    #[inline(always)]
    pub fn burstreq(&self) -> BURSTREQ_R {
        BURSTREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AVAILABILITY of the BLE functionality"]
    #[inline(always)]
    pub fn bleavail(&self) -> BLEAVAIL_R {
        BLEAVAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACK for BLEREQ"]
    #[inline(always)]
    pub fn bleack(&self) -> BLEACK_R {
        BLEACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Controls the BLE functionality"]
    #[inline(always)]
    pub fn blereq(&self) -> BLEREQ_R {
        BLEREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Availability of Burst functionality"]
    #[inline(always)]
    pub fn burstavail(&mut self) -> BURSTAVAIL_W {
        BURSTAVAIL_W { w: self }
    }
    #[doc = "Bit 5 - ACK for BURSTREQ"]
    #[inline(always)]
    pub fn burstack(&mut self) -> BURSTACK_W {
        BURSTACK_W { w: self }
    }
    #[doc = "Bit 4 - Controls the Burst functionality"]
    #[inline(always)]
    pub fn burstreq(&mut self) -> BURSTREQ_W {
        BURSTREQ_W { w: self }
    }
    #[doc = "Bit 2 - AVAILABILITY of the BLE functionality"]
    #[inline(always)]
    pub fn bleavail(&mut self) -> BLEAVAIL_W {
        BLEAVAIL_W { w: self }
    }
    #[doc = "Bit 1 - ACK for BLEREQ"]
    #[inline(always)]
    pub fn bleack(&mut self) -> BLEACK_W {
        BLEACK_W { w: self }
    }
    #[doc = "Bit 0 - Controls the BLE functionality"]
    #[inline(always)]
    pub fn blereq(&mut self) -> BLEREQ_W {
        BLEREQ_W { w: self }
    }
}
