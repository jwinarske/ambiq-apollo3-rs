#[doc = "Reader of register BOOTLOADER"]
pub type R = crate::R<u32, super::BOOTLOADER>;
#[doc = "Writer for register BOOTLOADER"]
pub type W = crate::W<u32, super::BOOTLOADER>;
#[doc = "Register BOOTLOADER `reset()`'s with value 0x07"]
impl crate::ResetValue for super::BOOTLOADER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Indicates whether the secure boot on warm reset is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SECBOOTONRST_A {
    #[doc = "0: Secure boot disabled value."]
    DISABLED = 0,
    #[doc = "1: Secure boot enabled value."]
    ENABLED = 1,
    #[doc = "2: Error in secure boot configuration value."]
    ERROR = 2,
}
impl From<SECBOOTONRST_A> for u8 {
    #[inline(always)]
    fn from(variant: SECBOOTONRST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SECBOOTONRST`"]
pub type SECBOOTONRST_R = crate::R<u8, SECBOOTONRST_A>;
impl SECBOOTONRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SECBOOTONRST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SECBOOTONRST_A::DISABLED),
            1 => Val(SECBOOTONRST_A::ENABLED),
            2 => Val(SECBOOTONRST_A::ERROR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SECBOOTONRST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SECBOOTONRST_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SECBOOTONRST_A::ERROR
    }
}
#[doc = "Write proxy for field `SECBOOTONRST`"]
pub struct SECBOOTONRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBOOTONRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECBOOTONRST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Secure boot disabled value."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECBOOTONRST_A::DISABLED)
    }
    #[doc = "Secure boot enabled value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECBOOTONRST_A::ENABLED)
    }
    #[doc = "Error in secure boot configuration value."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(SECBOOTONRST_A::ERROR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Indicates whether the secure boot on cold reset is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SECBOOT_A {
    #[doc = "0: Secure boot disabled value."]
    DISABLED = 0,
    #[doc = "1: Secure boot enabled value."]
    ENABLED = 1,
    #[doc = "2: Error in secure boot configuration value."]
    ERROR = 2,
}
impl From<SECBOOT_A> for u8 {
    #[inline(always)]
    fn from(variant: SECBOOT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SECBOOT`"]
pub type SECBOOT_R = crate::R<u8, SECBOOT_A>;
impl SECBOOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SECBOOT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SECBOOT_A::DISABLED),
            1 => Val(SECBOOT_A::ENABLED),
            2 => Val(SECBOOT_A::ERROR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SECBOOT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SECBOOT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SECBOOT_A::ERROR
    }
}
#[doc = "Write proxy for field `SECBOOT`"]
pub struct SECBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBOOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECBOOT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Secure boot disabled value."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECBOOT_A::DISABLED)
    }
    #[doc = "Secure boot enabled value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECBOOT_A::ENABLED)
    }
    #[doc = "Error in secure boot configuration value."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(SECBOOT_A::ERROR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Indicates whether the secure boot feature is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SECBOOTFEATURE_A {
    #[doc = "0: Secure boot disabled value."]
    DISABLED = 0,
    #[doc = "1: Secure boot enabled value."]
    ENABLED = 1,
    #[doc = "2: Error in secure boot configuration value."]
    ERROR = 2,
}
impl From<SECBOOTFEATURE_A> for u8 {
    #[inline(always)]
    fn from(variant: SECBOOTFEATURE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SECBOOTFEATURE`"]
pub type SECBOOTFEATURE_R = crate::R<u8, SECBOOTFEATURE_A>;
impl SECBOOTFEATURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SECBOOTFEATURE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SECBOOTFEATURE_A::DISABLED),
            1 => Val(SECBOOTFEATURE_A::ENABLED),
            2 => Val(SECBOOTFEATURE_A::ERROR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SECBOOTFEATURE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SECBOOTFEATURE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SECBOOTFEATURE_A::ERROR
    }
}
#[doc = "Write proxy for field `SECBOOTFEATURE`"]
pub struct SECBOOTFEATURE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBOOTFEATURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECBOOTFEATURE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Secure boot disabled value."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECBOOTFEATURE_A::DISABLED)
    }
    #[doc = "Secure boot enabled value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECBOOTFEATURE_A::ENABLED)
    }
    #[doc = "Error in secure boot configuration value."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(SECBOOTFEATURE_A::ERROR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Flash protection lock. Always resets to 1, write 1 to clear. Enables writes to flash protection register set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTLOCK_A {
    #[doc = "1: Enable the secure boot lock value."]
    LOCK = 1,
}
impl From<PROTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PROTLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROTLOCK`"]
pub type PROTLOCK_R = crate::R<bool, PROTLOCK_A>;
impl PROTLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PROTLOCK_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PROTLOCK_A::LOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == PROTLOCK_A::LOCK
    }
}
#[doc = "Write proxy for field `PROTLOCK`"]
pub struct PROTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the secure boot lock value."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(PROTLOCK_A::LOCK)
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
#[doc = "Secure boot lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBLOCK_A {
    #[doc = "1: Enable the secure boot lock value."]
    LOCK = 1,
}
impl From<SBLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SBLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBLOCK`"]
pub type SBLOCK_R = crate::R<bool, SBLOCK_A>;
impl SBLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SBLOCK_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SBLOCK_A::LOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == SBLOCK_A::LOCK
    }
}
#[doc = "Write proxy for field `SBLOCK`"]
pub struct SBLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SBLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the secure boot lock value."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(SBLOCK_A::LOCK)
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
#[doc = "Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTLOADERLOW_A {
    #[doc = "1: Bootloader code at 0x00000000. value."]
    ADDR0 = 1,
}
impl From<BOOTLOADERLOW_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTLOADERLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOOTLOADERLOW`"]
pub type BOOTLOADERLOW_R = crate::R<bool, BOOTLOADERLOW_A>;
impl BOOTLOADERLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BOOTLOADERLOW_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BOOTLOADERLOW_A::ADDR0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDR0`"]
    #[inline(always)]
    pub fn is_addr0(&self) -> bool {
        *self == BOOTLOADERLOW_A::ADDR0
    }
}
#[doc = "Write proxy for field `BOOTLOADERLOW`"]
pub struct BOOTLOADERLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTLOADERLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOTLOADERLOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bootloader code at 0x00000000. value."]
    #[inline(always)]
    pub fn addr0(self) -> &'a mut W {
        self.variant(BOOTLOADERLOW_A::ADDR0)
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
    #[doc = "Bits 30:31 - Indicates whether the secure boot on warm reset is enabled"]
    #[inline(always)]
    pub fn secbootonrst(&self) -> SECBOOTONRST_R {
        SECBOOTONRST_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Indicates whether the secure boot on cold reset is enabled"]
    #[inline(always)]
    pub fn secboot(&self) -> SECBOOT_R {
        SECBOOT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Indicates whether the secure boot feature is enabled."]
    #[inline(always)]
    pub fn secbootfeature(&self) -> SECBOOTFEATURE_R {
        SECBOOTFEATURE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Flash protection lock. Always resets to 1, write 1 to clear. Enables writes to flash protection register set."]
    #[inline(always)]
    pub fn protlock(&self) -> PROTLOCK_R {
        PROTLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure boot lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline(always)]
    pub fn sblock(&self) -> SBLOCK_R {
        SBLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
    #[inline(always)]
    pub fn bootloaderlow(&self) -> BOOTLOADERLOW_R {
        BOOTLOADERLOW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - Indicates whether the secure boot on warm reset is enabled"]
    #[inline(always)]
    pub fn secbootonrst(&mut self) -> SECBOOTONRST_W {
        SECBOOTONRST_W { w: self }
    }
    #[doc = "Bits 28:29 - Indicates whether the secure boot on cold reset is enabled"]
    #[inline(always)]
    pub fn secboot(&mut self) -> SECBOOT_W {
        SECBOOT_W { w: self }
    }
    #[doc = "Bits 26:27 - Indicates whether the secure boot feature is enabled."]
    #[inline(always)]
    pub fn secbootfeature(&mut self) -> SECBOOTFEATURE_W {
        SECBOOTFEATURE_W { w: self }
    }
    #[doc = "Bit 2 - Flash protection lock. Always resets to 1, write 1 to clear. Enables writes to flash protection register set."]
    #[inline(always)]
    pub fn protlock(&mut self) -> PROTLOCK_W {
        PROTLOCK_W { w: self }
    }
    #[doc = "Bit 1 - Secure boot lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline(always)]
    pub fn sblock(&mut self) -> SBLOCK_W {
        SBLOCK_W { w: self }
    }
    #[doc = "Bit 0 - Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
    #[inline(always)]
    pub fn bootloaderlow(&mut self) -> BOOTLOADERLOW_W {
        BOOTLOADERLOW_W { w: self }
    }
}
