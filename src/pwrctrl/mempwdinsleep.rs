#[doc = "Reader of register MEMPWDINSLEEP"]
pub type R = crate::R<u32, super::MEMPWDINSLEEP>;
#[doc = "Writer for register MEMPWDINSLEEP"]
pub type W = crate::W<u32, super::MEMPWDINSLEEP>;
#[doc = "Register MEMPWDINSLEEP `reset()`'s with value 0x6000"]
impl crate::ResetValue for super::MEMPWDINSLEEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6000
    }
}
#[doc = "power down cache in deep sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEPWDSLP_A {
    #[doc = "1: Power down cache in deep sleep value."]
    EN = 1,
    #[doc = "0: Retain cache in deep sleep value."]
    DIS = 0,
}
impl From<CACHEPWDSLP_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEPWDSLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEPWDSLP`"]
pub type CACHEPWDSLP_R = crate::R<bool, CACHEPWDSLP_A>;
impl CACHEPWDSLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEPWDSLP_A {
        match self.bits {
            true => CACHEPWDSLP_A::EN,
            false => CACHEPWDSLP_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHEPWDSLP_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHEPWDSLP_A::DIS
    }
}
#[doc = "Write proxy for field `CACHEPWDSLP`"]
pub struct CACHEPWDSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEPWDSLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEPWDSLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power down cache in deep sleep value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEPWDSLP_A::EN)
    }
    #[doc = "Retain cache in deep sleep value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEPWDSLP_A::DIS)
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
#[doc = "Powerdown flash1 in deep sleep\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1PWDSLP_A {
    #[doc = "1: Flash1 is powered down during deepsleep value."]
    EN = 1,
    #[doc = "0: Flash1 is kept powered on during deepsleep value."]
    DIS = 0,
}
impl From<FLASH1PWDSLP_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH1PWDSLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH1PWDSLP`"]
pub type FLASH1PWDSLP_R = crate::R<bool, FLASH1PWDSLP_A>;
impl FLASH1PWDSLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH1PWDSLP_A {
        match self.bits {
            true => FLASH1PWDSLP_A::EN,
            false => FLASH1PWDSLP_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLASH1PWDSLP_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLASH1PWDSLP_A::DIS
    }
}
#[doc = "Write proxy for field `FLASH1PWDSLP`"]
pub struct FLASH1PWDSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1PWDSLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH1PWDSLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash1 is powered down during deepsleep value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1PWDSLP_A::EN)
    }
    #[doc = "Flash1 is kept powered on during deepsleep value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1PWDSLP_A::DIS)
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
#[doc = "Powerdown flash0 in deep sleep\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0PWDSLP_A {
    #[doc = "1: Flash0 is powered down during deepsleep value."]
    EN = 1,
    #[doc = "0: Flash0 is kept powered on during deepsleep value."]
    DIS = 0,
}
impl From<FLASH0PWDSLP_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH0PWDSLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH0PWDSLP`"]
pub type FLASH0PWDSLP_R = crate::R<bool, FLASH0PWDSLP_A>;
impl FLASH0PWDSLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH0PWDSLP_A {
        match self.bits {
            true => FLASH0PWDSLP_A::EN,
            false => FLASH0PWDSLP_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLASH0PWDSLP_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLASH0PWDSLP_A::DIS
    }
}
#[doc = "Write proxy for field `FLASH0PWDSLP`"]
pub struct FLASH0PWDSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0PWDSLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH0PWDSLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash0 is powered down during deepsleep value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0PWDSLP_A::EN)
    }
    #[doc = "Flash0 is kept powered on during deepsleep value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0PWDSLP_A::DIS)
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
#[doc = "Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SRAMPWDSLP_A {
    #[doc = "0: All banks retained value."]
    NONE = 0,
    #[doc = "1: SRAM GROUP0 powered down (64KB-96KB) value."]
    GROUP0 = 1,
    #[doc = "2: SRAM GROUP1 powered down (96KB-128KB) value."]
    GROUP1 = 2,
    #[doc = "4: SRAM GROUP2 powered down (128KB-160KB) value."]
    GROUP2 = 4,
    #[doc = "8: SRAM GROUP3 powered down (160KB-192KB) value."]
    GROUP3 = 8,
    #[doc = "16: SRAM GROUP4 powered down (192KB-224KB) value."]
    GROUP4 = 16,
    #[doc = "32: SRAM GROUP5 powered down (224KB-256KB) value."]
    GROUP5 = 32,
    #[doc = "64: SRAM GROUP6 powered down (256KB-288KB) value."]
    GROUP6 = 64,
    #[doc = "128: SRAM GROUP7 powered down (288KB-320KB) value."]
    GROUP7 = 128,
    #[doc = "256: SRAM GROUP8 powered down (320KB-352KB) value."]
    GROUP8 = 256,
    #[doc = "512: SRAM GROUP9 powered down (352KB-384KB) value."]
    GROUP9 = 512,
    #[doc = "3: Powerdown lower 64k SRAM (64KB-128KB) value."]
    SRAM64K = 3,
    #[doc = "15: Powerdown lower 128k SRAM (64KB-192KB) value."]
    SRAM128K = 15,
    #[doc = "1022: All SRAM banks but lower 32k powered down (96KB-384KB). value."]
    ALLBUTLOWER32K = 1022,
    #[doc = "1020: All banks but lower 64k powered down. value."]
    ALLBUTLOWER64K = 1020,
    #[doc = "1008: All banks but lower 128k powered down. value."]
    ALLBUTLOWER128K = 1008,
    #[doc = "1023: All banks powered down. value."]
    ALL = 1023,
}
impl From<SRAMPWDSLP_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAMPWDSLP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMPWDSLP`"]
pub type SRAMPWDSLP_R = crate::R<u16, SRAMPWDSLP_A>;
impl SRAMPWDSLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SRAMPWDSLP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRAMPWDSLP_A::NONE),
            1 => Val(SRAMPWDSLP_A::GROUP0),
            2 => Val(SRAMPWDSLP_A::GROUP1),
            4 => Val(SRAMPWDSLP_A::GROUP2),
            8 => Val(SRAMPWDSLP_A::GROUP3),
            16 => Val(SRAMPWDSLP_A::GROUP4),
            32 => Val(SRAMPWDSLP_A::GROUP5),
            64 => Val(SRAMPWDSLP_A::GROUP6),
            128 => Val(SRAMPWDSLP_A::GROUP7),
            256 => Val(SRAMPWDSLP_A::GROUP8),
            512 => Val(SRAMPWDSLP_A::GROUP9),
            3 => Val(SRAMPWDSLP_A::SRAM64K),
            15 => Val(SRAMPWDSLP_A::SRAM128K),
            1022 => Val(SRAMPWDSLP_A::ALLBUTLOWER32K),
            1020 => Val(SRAMPWDSLP_A::ALLBUTLOWER64K),
            1008 => Val(SRAMPWDSLP_A::ALLBUTLOWER128K),
            1023 => Val(SRAMPWDSLP_A::ALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRAMPWDSLP_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP1
    }
    #[doc = "Checks if the value of the field is `GROUP2`"]
    #[inline(always)]
    pub fn is_group2(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP2
    }
    #[doc = "Checks if the value of the field is `GROUP3`"]
    #[inline(always)]
    pub fn is_group3(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP3
    }
    #[doc = "Checks if the value of the field is `GROUP4`"]
    #[inline(always)]
    pub fn is_group4(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP4
    }
    #[doc = "Checks if the value of the field is `GROUP5`"]
    #[inline(always)]
    pub fn is_group5(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP5
    }
    #[doc = "Checks if the value of the field is `GROUP6`"]
    #[inline(always)]
    pub fn is_group6(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP6
    }
    #[doc = "Checks if the value of the field is `GROUP7`"]
    #[inline(always)]
    pub fn is_group7(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP7
    }
    #[doc = "Checks if the value of the field is `GROUP8`"]
    #[inline(always)]
    pub fn is_group8(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP8
    }
    #[doc = "Checks if the value of the field is `GROUP9`"]
    #[inline(always)]
    pub fn is_group9(&self) -> bool {
        *self == SRAMPWDSLP_A::GROUP9
    }
    #[doc = "Checks if the value of the field is `SRAM64K`"]
    #[inline(always)]
    pub fn is_sram64k(&self) -> bool {
        *self == SRAMPWDSLP_A::SRAM64K
    }
    #[doc = "Checks if the value of the field is `SRAM128K`"]
    #[inline(always)]
    pub fn is_sram128k(&self) -> bool {
        *self == SRAMPWDSLP_A::SRAM128K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER32K`"]
    #[inline(always)]
    pub fn is_allbutlower32k(&self) -> bool {
        *self == SRAMPWDSLP_A::ALLBUTLOWER32K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER64K`"]
    #[inline(always)]
    pub fn is_allbutlower64k(&self) -> bool {
        *self == SRAMPWDSLP_A::ALLBUTLOWER64K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER128K`"]
    #[inline(always)]
    pub fn is_allbutlower128k(&self) -> bool {
        *self == SRAMPWDSLP_A::ALLBUTLOWER128K
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == SRAMPWDSLP_A::ALL
    }
}
#[doc = "Write proxy for field `SRAMPWDSLP`"]
pub struct SRAMPWDSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMPWDSLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMPWDSLP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All banks retained value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::NONE)
    }
    #[doc = "SRAM GROUP0 powered down (64KB-96KB) value."]
    #[inline(always)]
    pub fn group0(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP0)
    }
    #[doc = "SRAM GROUP1 powered down (96KB-128KB) value."]
    #[inline(always)]
    pub fn group1(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP1)
    }
    #[doc = "SRAM GROUP2 powered down (128KB-160KB) value."]
    #[inline(always)]
    pub fn group2(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP2)
    }
    #[doc = "SRAM GROUP3 powered down (160KB-192KB) value."]
    #[inline(always)]
    pub fn group3(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP3)
    }
    #[doc = "SRAM GROUP4 powered down (192KB-224KB) value."]
    #[inline(always)]
    pub fn group4(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP4)
    }
    #[doc = "SRAM GROUP5 powered down (224KB-256KB) value."]
    #[inline(always)]
    pub fn group5(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP5)
    }
    #[doc = "SRAM GROUP6 powered down (256KB-288KB) value."]
    #[inline(always)]
    pub fn group6(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP6)
    }
    #[doc = "SRAM GROUP7 powered down (288KB-320KB) value."]
    #[inline(always)]
    pub fn group7(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP7)
    }
    #[doc = "SRAM GROUP8 powered down (320KB-352KB) value."]
    #[inline(always)]
    pub fn group8(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP8)
    }
    #[doc = "SRAM GROUP9 powered down (352KB-384KB) value."]
    #[inline(always)]
    pub fn group9(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::GROUP9)
    }
    #[doc = "Powerdown lower 64k SRAM (64KB-128KB) value."]
    #[inline(always)]
    pub fn sram64k(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::SRAM64K)
    }
    #[doc = "Powerdown lower 128k SRAM (64KB-192KB) value."]
    #[inline(always)]
    pub fn sram128k(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::SRAM128K)
    }
    #[doc = "All SRAM banks but lower 32k powered down (96KB-384KB). value."]
    #[inline(always)]
    pub fn allbutlower32k(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::ALLBUTLOWER32K)
    }
    #[doc = "All banks but lower 64k powered down. value."]
    #[inline(always)]
    pub fn allbutlower64k(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::ALLBUTLOWER64K)
    }
    #[doc = "All banks but lower 128k powered down. value."]
    #[inline(always)]
    pub fn allbutlower128k(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::ALLBUTLOWER128K)
    }
    #[doc = "All banks powered down. value."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(SRAMPWDSLP_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 3)) | (((value as u32) & 0x03ff) << 3);
        self.w
    }
}
#[doc = "power down DTCM in deep sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTCMPWDSLP_A {
    #[doc = "0: All DTCM retained value."]
    NONE = 0,
    #[doc = "1: Group0_DTCM0 powered down in deep sleep (0KB-8KB) value."]
    GROUP0DTCM0 = 1,
    #[doc = "2: Group0_DTCM1 powered down in deep sleep (8KB-32KB) value."]
    GROUP0DTCM1 = 2,
    #[doc = "3: Both DTCMs in group0 are powered down in deep sleep (0KB-32KB) value."]
    GROUP0 = 3,
    #[doc = "6: Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-64KB) value."]
    ALLBUTGROUP0DTCM0 = 6,
    #[doc = "4: Group1 DTCM powered down in deep sleep (32KB-64KB) value."]
    GROUP1 = 4,
    #[doc = "7: All DTCMs powered down in deep sleep (0KB-64KB) value."]
    ALL = 7,
}
impl From<DTCMPWDSLP_A> for u8 {
    #[inline(always)]
    fn from(variant: DTCMPWDSLP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTCMPWDSLP`"]
pub type DTCMPWDSLP_R = crate::R<u8, DTCMPWDSLP_A>;
impl DTCMPWDSLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTCMPWDSLP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTCMPWDSLP_A::NONE),
            1 => Val(DTCMPWDSLP_A::GROUP0DTCM0),
            2 => Val(DTCMPWDSLP_A::GROUP0DTCM1),
            3 => Val(DTCMPWDSLP_A::GROUP0),
            6 => Val(DTCMPWDSLP_A::ALLBUTGROUP0DTCM0),
            4 => Val(DTCMPWDSLP_A::GROUP1),
            7 => Val(DTCMPWDSLP_A::ALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTCMPWDSLP_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM0`"]
    #[inline(always)]
    pub fn is_group0dtcm0(&self) -> bool {
        *self == DTCMPWDSLP_A::GROUP0DTCM0
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM1`"]
    #[inline(always)]
    pub fn is_group0dtcm1(&self) -> bool {
        *self == DTCMPWDSLP_A::GROUP0DTCM1
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        *self == DTCMPWDSLP_A::GROUP0
    }
    #[doc = "Checks if the value of the field is `ALLBUTGROUP0DTCM0`"]
    #[inline(always)]
    pub fn is_allbutgroup0dtcm0(&self) -> bool {
        *self == DTCMPWDSLP_A::ALLBUTGROUP0DTCM0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == DTCMPWDSLP_A::GROUP1
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == DTCMPWDSLP_A::ALL
    }
}
#[doc = "Write proxy for field `DTCMPWDSLP`"]
pub struct DTCMPWDSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCMPWDSLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCMPWDSLP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All DTCM retained value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTCMPWDSLP_A::NONE)
    }
    #[doc = "Group0_DTCM0 powered down in deep sleep (0KB-8KB) value."]
    #[inline(always)]
    pub fn group0dtcm0(self) -> &'a mut W {
        self.variant(DTCMPWDSLP_A::GROUP0DTCM0)
    }
    #[doc = "Group0_DTCM1 powered down in deep sleep (8KB-32KB) value."]
    #[inline(always)]
    pub fn group0dtcm1(self) -> &'a mut W {
        self.variant(DTCMPWDSLP_A::GROUP0DTCM1)
    }
    #[doc = "Both DTCMs in group0 are powered down in deep sleep (0KB-32KB) value."]
    #[inline(always)]
    pub fn group0(self) -> &'a mut W {
        self.variant(DTCMPWDSLP_A::GROUP0)
    }
    #[doc = "Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-64KB) value."]
    #[inline(always)]
    pub fn allbutgroup0dtcm0(self) -> &'a mut W {
        self.variant(DTCMPWDSLP_A::ALLBUTGROUP0DTCM0)
    }
    #[doc = "Group1 DTCM powered down in deep sleep (32KB-64KB) value."]
    #[inline(always)]
    pub fn group1(self) -> &'a mut W {
        self.variant(DTCMPWDSLP_A::GROUP1)
    }
    #[doc = "All DTCMs powered down in deep sleep (0KB-64KB) value."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(DTCMPWDSLP_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - power down cache in deep sleep"]
    #[inline(always)]
    pub fn cachepwdslp(&self) -> CACHEPWDSLP_R {
        CACHEPWDSLP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Powerdown flash1 in deep sleep"]
    #[inline(always)]
    pub fn flash1pwdslp(&self) -> FLASH1PWDSLP_R {
        FLASH1PWDSLP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Powerdown flash0 in deep sleep"]
    #[inline(always)]
    pub fn flash0pwdslp(&self) -> FLASH0PWDSLP_R {
        FLASH0PWDSLP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 3:12 - Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn srampwdslp(&self) -> SRAMPWDSLP_R {
        SRAMPWDSLP_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:2 - power down DTCM in deep sleep"]
    #[inline(always)]
    pub fn dtcmpwdslp(&self) -> DTCMPWDSLP_R {
        DTCMPWDSLP_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - power down cache in deep sleep"]
    #[inline(always)]
    pub fn cachepwdslp(&mut self) -> CACHEPWDSLP_W {
        CACHEPWDSLP_W { w: self }
    }
    #[doc = "Bit 14 - Powerdown flash1 in deep sleep"]
    #[inline(always)]
    pub fn flash1pwdslp(&mut self) -> FLASH1PWDSLP_W {
        FLASH1PWDSLP_W { w: self }
    }
    #[doc = "Bit 13 - Powerdown flash0 in deep sleep"]
    #[inline(always)]
    pub fn flash0pwdslp(&mut self) -> FLASH0PWDSLP_W {
        FLASH0PWDSLP_W { w: self }
    }
    #[doc = "Bits 3:12 - Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn srampwdslp(&mut self) -> SRAMPWDSLP_W {
        SRAMPWDSLP_W { w: self }
    }
    #[doc = "Bits 0:2 - power down DTCM in deep sleep"]
    #[inline(always)]
    pub fn dtcmpwdslp(&mut self) -> DTCMPWDSLP_W {
        DTCMPWDSLP_W { w: self }
    }
}
