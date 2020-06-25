#[doc = "Reader of register SUBMODCTRL"]
pub type R = crate::R<u32, super::SUBMODCTRL>;
#[doc = "Writer for register SUBMODCTRL"]
pub type W = crate::W<u32, super::SUBMODCTRL>;
#[doc = "Register SUBMODCTRL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::SUBMODCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Submodule 0 module type. This is the I2C Master interface\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD1TYPE_A {
    #[doc = "0: SPI Master submodule value."]
    MSPI = 0,
    #[doc = "1: MI2C submodule value."]
    I2C_MASTER = 1,
    #[doc = "2: SPI Slave submodule value."]
    SSPI = 2,
    #[doc = "3: I2C Slave submodule value."]
    SI2C = 3,
    #[doc = "7: NOT INSTALLED value."]
    NA = 7,
}
impl From<SMOD1TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD1TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMOD1TYPE`"]
pub type SMOD1TYPE_R = crate::R<u8, SMOD1TYPE_A>;
impl SMOD1TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMOD1TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMOD1TYPE_A::MSPI),
            1 => Val(SMOD1TYPE_A::I2C_MASTER),
            2 => Val(SMOD1TYPE_A::SSPI),
            3 => Val(SMOD1TYPE_A::SI2C),
            7 => Val(SMOD1TYPE_A::NA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI`"]
    #[inline(always)]
    pub fn is_mspi(&self) -> bool {
        *self == SMOD1TYPE_A::MSPI
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == SMOD1TYPE_A::I2C_MASTER
    }
    #[doc = "Checks if the value of the field is `SSPI`"]
    #[inline(always)]
    pub fn is_sspi(&self) -> bool {
        *self == SMOD1TYPE_A::SSPI
    }
    #[doc = "Checks if the value of the field is `SI2C`"]
    #[inline(always)]
    pub fn is_si2c(&self) -> bool {
        *self == SMOD1TYPE_A::SI2C
    }
    #[doc = "Checks if the value of the field is `NA`"]
    #[inline(always)]
    pub fn is_na(&self) -> bool {
        *self == SMOD1TYPE_A::NA
    }
}
#[doc = "Write proxy for field `SMOD1TYPE`"]
pub struct SMOD1TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD1TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD1TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI Master submodule value."]
    #[inline(always)]
    pub fn mspi(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::MSPI)
    }
    #[doc = "MI2C submodule value."]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::I2C_MASTER)
    }
    #[doc = "SPI Slave submodule value."]
    #[inline(always)]
    pub fn sspi(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::SSPI)
    }
    #[doc = "I2C Slave submodule value."]
    #[inline(always)]
    pub fn si2c(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::SI2C)
    }
    #[doc = "NOT INSTALLED value."]
    #[inline(always)]
    pub fn na(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::NA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `SMOD1EN`"]
pub type SMOD1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMOD1EN`"]
pub struct SMOD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD1EN_W<'a> {
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
#[doc = "Submodule 0 module type. This is the SPI Master interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD0TYPE_A {
    #[doc = "0: MSPI submodule value."]
    SPI_MASTER = 0,
    #[doc = "1: I2C Master submodule value."]
    I2C_MASTER = 1,
    #[doc = "2: SPI Slave submodule value."]
    SSPI = 2,
    #[doc = "3: I2C Slave submodule value."]
    SI2C = 3,
    #[doc = "7: NOT INSTALLED value."]
    NA = 7,
}
impl From<SMOD0TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD0TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMOD0TYPE`"]
pub type SMOD0TYPE_R = crate::R<u8, SMOD0TYPE_A>;
impl SMOD0TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMOD0TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMOD0TYPE_A::SPI_MASTER),
            1 => Val(SMOD0TYPE_A::I2C_MASTER),
            2 => Val(SMOD0TYPE_A::SSPI),
            3 => Val(SMOD0TYPE_A::SI2C),
            7 => Val(SMOD0TYPE_A::NA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == SMOD0TYPE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == SMOD0TYPE_A::I2C_MASTER
    }
    #[doc = "Checks if the value of the field is `SSPI`"]
    #[inline(always)]
    pub fn is_sspi(&self) -> bool {
        *self == SMOD0TYPE_A::SSPI
    }
    #[doc = "Checks if the value of the field is `SI2C`"]
    #[inline(always)]
    pub fn is_si2c(&self) -> bool {
        *self == SMOD0TYPE_A::SI2C
    }
    #[doc = "Checks if the value of the field is `NA`"]
    #[inline(always)]
    pub fn is_na(&self) -> bool {
        *self == SMOD0TYPE_A::NA
    }
}
#[doc = "Write proxy for field `SMOD0TYPE`"]
pub struct SMOD0TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD0TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD0TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MSPI submodule value."]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::SPI_MASTER)
    }
    #[doc = "I2C Master submodule value."]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::I2C_MASTER)
    }
    #[doc = "SPI Slave submodule value."]
    #[inline(always)]
    pub fn sspi(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::SSPI)
    }
    #[doc = "I2C Slave submodule value."]
    #[inline(always)]
    pub fn si2c(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::SI2C)
    }
    #[doc = "NOT INSTALLED value."]
    #[inline(always)]
    pub fn na(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::NA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `SMOD0EN`"]
pub type SMOD0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMOD0EN`"]
pub struct SMOD0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD0EN_W<'a> {
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
    #[doc = "Bits 5:7 - Submodule 0 module type. This is the I2C Master interface"]
    #[inline(always)]
    pub fn smod1type(&self) -> SMOD1TYPE_R {
        SMOD1TYPE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Submodule 1 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod1en(&self) -> SMOD1EN_R {
        SMOD1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Submodule 0 module type. This is the SPI Master interface."]
    #[inline(always)]
    pub fn smod0type(&self) -> SMOD0TYPE_R {
        SMOD0TYPE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Submodule 0 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod0en(&self) -> SMOD0EN_R {
        SMOD0EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - Submodule 0 module type. This is the I2C Master interface"]
    #[inline(always)]
    pub fn smod1type(&mut self) -> SMOD1TYPE_W {
        SMOD1TYPE_W { w: self }
    }
    #[doc = "Bit 4 - Submodule 1 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod1en(&mut self) -> SMOD1EN_W {
        SMOD1EN_W { w: self }
    }
    #[doc = "Bits 1:3 - Submodule 0 module type. This is the SPI Master interface."]
    #[inline(always)]
    pub fn smod0type(&mut self) -> SMOD0TYPE_W {
        SMOD0TYPE_W { w: self }
    }
    #[doc = "Bit 0 - Submodule 0 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod0en(&mut self) -> SMOD0EN_W {
        SMOD0EN_W { w: self }
    }
}
