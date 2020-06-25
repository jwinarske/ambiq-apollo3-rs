#[doc = "Reader of register SHADOWVALID"]
pub type R = crate::R<u32, super::SHADOWVALID>;
#[doc = "Writer for register SHADOWVALID"]
pub type W = crate::W<u32, super::SHADOWVALID>;
#[doc = "Register SHADOWVALID `reset()`'s with value 0x07"]
impl crate::ResetValue for super::SHADOWVALID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Indicates whether info0 contains valid data\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INFO0_VALID_A {
    #[doc = "1: Flash info0 (customer) space contains valid data. value."]
    VALID = 1,
}
impl From<INFO0_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: INFO0_VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INFO0_VALID`"]
pub type INFO0_VALID_R = crate::R<bool, INFO0_VALID_A>;
impl INFO0_VALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, INFO0_VALID_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(INFO0_VALID_A::VALID),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == INFO0_VALID_A::VALID
    }
}
#[doc = "Write proxy for field `INFO0_VALID`"]
pub struct INFO0_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> INFO0_VALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INFO0_VALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash info0 (customer) space contains valid data. value."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(INFO0_VALID_A::VALID)
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
#[doc = "Indicates whether the bootloader should sleep or deep sleep if no image loaded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLDSLEEP_A {
    #[doc = "1: Bootloader will go to deep sleep if no flash image loaded value."]
    DEEPSLEEP = 1,
}
impl From<BLDSLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: BLDSLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLDSLEEP`"]
pub type BLDSLEEP_R = crate::R<bool, BLDSLEEP_A>;
impl BLDSLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BLDSLEEP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BLDSLEEP_A::DEEPSLEEP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == BLDSLEEP_A::DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BLDSLEEP`"]
pub struct BLDSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> BLDSLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLDSLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bootloader will go to deep sleep if no flash image loaded value."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(BLDSLEEP_A::DEEPSLEEP)
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
#[doc = "Indicates whether the shadow registers contain valid data from the Flash Information Space.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_A {
    #[doc = "1: Flash information space contains valid data. value."]
    VALID = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, VALID_A>;
impl VALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, VALID_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(VALID_A::VALID),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VALID_A::VALID
    }
}
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash information space contains valid data. value."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VALID_A::VALID)
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
    #[doc = "Bit 2 - Indicates whether info0 contains valid data"]
    #[inline(always)]
    pub fn info0_valid(&self) -> INFO0_VALID_R {
        INFO0_VALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    pub fn bldsleep(&self) -> BLDSLEEP_R {
        BLDSLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Indicates whether info0 contains valid data"]
    #[inline(always)]
    pub fn info0_valid(&mut self) -> INFO0_VALID_W {
        INFO0_VALID_W { w: self }
    }
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    pub fn bldsleep(&mut self) -> BLDSLEEP_W {
        BLDSLEEP_W { w: self }
    }
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
}
