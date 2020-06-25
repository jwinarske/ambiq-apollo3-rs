#[doc = "Reader of register MEMPWREVENTEN"]
pub type R = crate::R<u32, super::MEMPWREVENTEN>;
#[doc = "Writer for register MEMPWREVENTEN"]
pub type W = crate::W<u32, super::MEMPWREVENTEN>;
#[doc = "Register MEMPWREVENTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMPWREVENTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Control CACHEB2 power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB2EN_A {
    #[doc = "1: Enable CACHE BANK 2 status event value."]
    EN = 1,
    #[doc = "0: Disable CACHE BANK 2 status event value."]
    DIS = 0,
}
impl From<CACHEB2EN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEB2EN`"]
pub type CACHEB2EN_R = crate::R<bool, CACHEB2EN_A>;
impl CACHEB2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB2EN_A {
        match self.bits {
            true => CACHEB2EN_A::EN,
            false => CACHEB2EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHEB2EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB2EN_A::DIS
    }
}
#[doc = "Write proxy for field `CACHEB2EN`"]
pub struct CACHEB2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable CACHE BANK 2 status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB2EN_A::EN)
    }
    #[doc = "Disable CACHE BANK 2 status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB2EN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Control CACHE BANK 0 power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB0EN_A {
    #[doc = "1: Enable CACHE BANK 0 status event value."]
    EN = 1,
    #[doc = "0: Disable CACHE BANK 0 status event value."]
    DIS = 0,
}
impl From<CACHEB0EN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEB0EN`"]
pub type CACHEB0EN_R = crate::R<bool, CACHEB0EN_A>;
impl CACHEB0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB0EN_A {
        match self.bits {
            true => CACHEB0EN_A::EN,
            false => CACHEB0EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHEB0EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB0EN_A::DIS
    }
}
#[doc = "Write proxy for field `CACHEB0EN`"]
pub struct CACHEB0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable CACHE BANK 0 status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB0EN_A::EN)
    }
    #[doc = "Disable CACHE BANK 0 status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB0EN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Control Flash power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1EN_A {
    #[doc = "1: Enable FLASH status event value."]
    EN = 1,
    #[doc = "0: Disables FLASH status event value."]
    DIS = 0,
}
impl From<FLASH1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH1EN`"]
pub type FLASH1EN_R = crate::R<bool, FLASH1EN_A>;
impl FLASH1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH1EN_A {
        match self.bits {
            true => FLASH1EN_A::EN,
            false => FLASH1EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLASH1EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLASH1EN_A::DIS
    }
}
#[doc = "Write proxy for field `FLASH1EN`"]
pub struct FLASH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable FLASH status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1EN_A::EN)
    }
    #[doc = "Disables FLASH status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1EN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Control Flash power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0EN_A {
    #[doc = "1: Enable FLASH status event value."]
    EN = 1,
    #[doc = "0: Disables FLASH status event value."]
    DIS = 0,
}
impl From<FLASH0EN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH0EN`"]
pub type FLASH0EN_R = crate::R<bool, FLASH0EN_A>;
impl FLASH0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH0EN_A {
        match self.bits {
            true => FLASH0EN_A::EN,
            false => FLASH0EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLASH0EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLASH0EN_A::DIS
    }
}
#[doc = "Write proxy for field `FLASH0EN`"]
pub struct FLASH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable FLASH status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0EN_A::EN)
    }
    #[doc = "Disables FLASH status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0EN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Control SRAM power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SRAMEN_A {
    #[doc = "0: Disable SRAM power-on status event value."]
    NONE = 0,
    #[doc = "1: Enable SRAM group0 (0KB-32KB) power on status event value."]
    GROUP0EN = 1,
    #[doc = "2: Enable SRAM group1 (32KB-64KB) power on status event value."]
    GROUP1EN = 2,
    #[doc = "4: Enable SRAM group2 (64KB-96KB) power on status event value."]
    GROUP2EN = 4,
    #[doc = "8: Enable SRAM group3 (96KB-128KB) power on status event value."]
    GROUP3EN = 8,
    #[doc = "16: Enable SRAM group4 (128KB-160KB) power on status event value."]
    GROUP4EN = 16,
    #[doc = "32: Enable SRAM group5 (160KB-192KB) power on status event value."]
    GROUP5EN = 32,
    #[doc = "64: Enable SRAM group6 (192KB-224KB) power on status event value."]
    GROUP6EN = 64,
    #[doc = "128: Enable SRAM group7 (224KB-256KB) power on status event value."]
    GROUP7EN = 128,
    #[doc = "256: Enable SRAM group8 (256KB-288KB) power on status event value."]
    GROUP8EN = 256,
    #[doc = "512: Enable SRAM group9 (288KB-320KB) power on status event value."]
    GROUP9EN = 512,
}
impl From<SRAMEN_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAMEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMEN`"]
pub type SRAMEN_R = crate::R<u16, SRAMEN_A>;
impl SRAMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SRAMEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRAMEN_A::NONE),
            1 => Val(SRAMEN_A::GROUP0EN),
            2 => Val(SRAMEN_A::GROUP1EN),
            4 => Val(SRAMEN_A::GROUP2EN),
            8 => Val(SRAMEN_A::GROUP3EN),
            16 => Val(SRAMEN_A::GROUP4EN),
            32 => Val(SRAMEN_A::GROUP5EN),
            64 => Val(SRAMEN_A::GROUP6EN),
            128 => Val(SRAMEN_A::GROUP7EN),
            256 => Val(SRAMEN_A::GROUP8EN),
            512 => Val(SRAMEN_A::GROUP9EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRAMEN_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0EN`"]
    #[inline(always)]
    pub fn is_group0en(&self) -> bool {
        *self == SRAMEN_A::GROUP0EN
    }
    #[doc = "Checks if the value of the field is `GROUP1EN`"]
    #[inline(always)]
    pub fn is_group1en(&self) -> bool {
        *self == SRAMEN_A::GROUP1EN
    }
    #[doc = "Checks if the value of the field is `GROUP2EN`"]
    #[inline(always)]
    pub fn is_group2en(&self) -> bool {
        *self == SRAMEN_A::GROUP2EN
    }
    #[doc = "Checks if the value of the field is `GROUP3EN`"]
    #[inline(always)]
    pub fn is_group3en(&self) -> bool {
        *self == SRAMEN_A::GROUP3EN
    }
    #[doc = "Checks if the value of the field is `GROUP4EN`"]
    #[inline(always)]
    pub fn is_group4en(&self) -> bool {
        *self == SRAMEN_A::GROUP4EN
    }
    #[doc = "Checks if the value of the field is `GROUP5EN`"]
    #[inline(always)]
    pub fn is_group5en(&self) -> bool {
        *self == SRAMEN_A::GROUP5EN
    }
    #[doc = "Checks if the value of the field is `GROUP6EN`"]
    #[inline(always)]
    pub fn is_group6en(&self) -> bool {
        *self == SRAMEN_A::GROUP6EN
    }
    #[doc = "Checks if the value of the field is `GROUP7EN`"]
    #[inline(always)]
    pub fn is_group7en(&self) -> bool {
        *self == SRAMEN_A::GROUP7EN
    }
    #[doc = "Checks if the value of the field is `GROUP8EN`"]
    #[inline(always)]
    pub fn is_group8en(&self) -> bool {
        *self == SRAMEN_A::GROUP8EN
    }
    #[doc = "Checks if the value of the field is `GROUP9EN`"]
    #[inline(always)]
    pub fn is_group9en(&self) -> bool {
        *self == SRAMEN_A::GROUP9EN
    }
}
#[doc = "Write proxy for field `SRAMEN`"]
pub struct SRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable SRAM power-on status event value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAMEN_A::NONE)
    }
    #[doc = "Enable SRAM group0 (0KB-32KB) power on status event value."]
    #[inline(always)]
    pub fn group0en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP0EN)
    }
    #[doc = "Enable SRAM group1 (32KB-64KB) power on status event value."]
    #[inline(always)]
    pub fn group1en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP1EN)
    }
    #[doc = "Enable SRAM group2 (64KB-96KB) power on status event value."]
    #[inline(always)]
    pub fn group2en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP2EN)
    }
    #[doc = "Enable SRAM group3 (96KB-128KB) power on status event value."]
    #[inline(always)]
    pub fn group3en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP3EN)
    }
    #[doc = "Enable SRAM group4 (128KB-160KB) power on status event value."]
    #[inline(always)]
    pub fn group4en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP4EN)
    }
    #[doc = "Enable SRAM group5 (160KB-192KB) power on status event value."]
    #[inline(always)]
    pub fn group5en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP5EN)
    }
    #[doc = "Enable SRAM group6 (192KB-224KB) power on status event value."]
    #[inline(always)]
    pub fn group6en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP6EN)
    }
    #[doc = "Enable SRAM group7 (224KB-256KB) power on status event value."]
    #[inline(always)]
    pub fn group7en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP7EN)
    }
    #[doc = "Enable SRAM group8 (256KB-288KB) power on status event value."]
    #[inline(always)]
    pub fn group8en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP8EN)
    }
    #[doc = "Enable SRAM group9 (288KB-320KB) power on status event value."]
    #[inline(always)]
    pub fn group9en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP9EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 3)) | (((value as u32) & 0x03ff) << 3);
        self.w
    }
}
#[doc = "Enable DTCM power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTCMEN_A {
    #[doc = "0: Do not enable DTCM power-on status event value."]
    NONE = 0,
    #[doc = "1: Enable GROUP0_DTCM0 power on status event value."]
    GROUP0DTCM0EN = 1,
    #[doc = "2: Enable GROUP0_DTCM1 power on status event value."]
    GROUP0DTCM1EN = 2,
    #[doc = "3: Enable DTCMs in group0 power on status event value."]
    GROUP0EN = 3,
    #[doc = "4: Enable DTCMs in group1 power on status event value."]
    GROUP1EN = 4,
    #[doc = "7: Enable all DTCM power on status event value."]
    ALL = 7,
}
impl From<DTCMEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DTCMEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTCMEN`"]
pub type DTCMEN_R = crate::R<u8, DTCMEN_A>;
impl DTCMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTCMEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTCMEN_A::NONE),
            1 => Val(DTCMEN_A::GROUP0DTCM0EN),
            2 => Val(DTCMEN_A::GROUP0DTCM1EN),
            3 => Val(DTCMEN_A::GROUP0EN),
            4 => Val(DTCMEN_A::GROUP1EN),
            7 => Val(DTCMEN_A::ALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTCMEN_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM0EN`"]
    #[inline(always)]
    pub fn is_group0dtcm0en(&self) -> bool {
        *self == DTCMEN_A::GROUP0DTCM0EN
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM1EN`"]
    #[inline(always)]
    pub fn is_group0dtcm1en(&self) -> bool {
        *self == DTCMEN_A::GROUP0DTCM1EN
    }
    #[doc = "Checks if the value of the field is `GROUP0EN`"]
    #[inline(always)]
    pub fn is_group0en(&self) -> bool {
        *self == DTCMEN_A::GROUP0EN
    }
    #[doc = "Checks if the value of the field is `GROUP1EN`"]
    #[inline(always)]
    pub fn is_group1en(&self) -> bool {
        *self == DTCMEN_A::GROUP1EN
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == DTCMEN_A::ALL
    }
}
#[doc = "Write proxy for field `DTCMEN`"]
pub struct DTCMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCMEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not enable DTCM power-on status event value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTCMEN_A::NONE)
    }
    #[doc = "Enable GROUP0_DTCM0 power on status event value."]
    #[inline(always)]
    pub fn group0dtcm0en(self) -> &'a mut W {
        self.variant(DTCMEN_A::GROUP0DTCM0EN)
    }
    #[doc = "Enable GROUP0_DTCM1 power on status event value."]
    #[inline(always)]
    pub fn group0dtcm1en(self) -> &'a mut W {
        self.variant(DTCMEN_A::GROUP0DTCM1EN)
    }
    #[doc = "Enable DTCMs in group0 power on status event value."]
    #[inline(always)]
    pub fn group0en(self) -> &'a mut W {
        self.variant(DTCMEN_A::GROUP0EN)
    }
    #[doc = "Enable DTCMs in group1 power on status event value."]
    #[inline(always)]
    pub fn group1en(self) -> &'a mut W {
        self.variant(DTCMEN_A::GROUP1EN)
    }
    #[doc = "Enable all DTCM power on status event value."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(DTCMEN_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Control CACHEB2 power-on status event"]
    #[inline(always)]
    pub fn cacheb2en(&self) -> CACHEB2EN_R {
        CACHEB2EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Control CACHE BANK 0 power-on status event"]
    #[inline(always)]
    pub fn cacheb0en(&self) -> CACHEB0EN_R {
        CACHEB0EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Control Flash power-on status event"]
    #[inline(always)]
    pub fn flash1en(&self) -> FLASH1EN_R {
        FLASH1EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Control Flash power-on status event"]
    #[inline(always)]
    pub fn flash0en(&self) -> FLASH0EN_R {
        FLASH0EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 3:12 - Control SRAM power-on status event"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:2 - Enable DTCM power-on status event"]
    #[inline(always)]
    pub fn dtcmen(&self) -> DTCMEN_R {
        DTCMEN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Control CACHEB2 power-on status event"]
    #[inline(always)]
    pub fn cacheb2en(&mut self) -> CACHEB2EN_W {
        CACHEB2EN_W { w: self }
    }
    #[doc = "Bit 30 - Control CACHE BANK 0 power-on status event"]
    #[inline(always)]
    pub fn cacheb0en(&mut self) -> CACHEB0EN_W {
        CACHEB0EN_W { w: self }
    }
    #[doc = "Bit 14 - Control Flash power-on status event"]
    #[inline(always)]
    pub fn flash1en(&mut self) -> FLASH1EN_W {
        FLASH1EN_W { w: self }
    }
    #[doc = "Bit 13 - Control Flash power-on status event"]
    #[inline(always)]
    pub fn flash0en(&mut self) -> FLASH0EN_W {
        FLASH0EN_W { w: self }
    }
    #[doc = "Bits 3:12 - Control SRAM power-on status event"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W { w: self }
    }
    #[doc = "Bits 0:2 - Enable DTCM power-on status event"]
    #[inline(always)]
    pub fn dtcmen(&mut self) -> DTCMEN_W {
        DTCMEN_W { w: self }
    }
}
