#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRCERROR`"]
pub type CRCERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCERROR`"]
pub struct CRCERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERROR_W<'a> {
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
#[doc = "Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNCTION_A {
    #[doc = "0: Perform CRC32 operation value."]
    CRC32 = 0,
}
impl From<FUNCTION_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FUNCTION`"]
pub type FUNCTION_R = crate::R<u8, FUNCTION_A>;
impl FUNCTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FUNCTION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FUNCTION_A::CRC32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == FUNCTION_A::CRC32
    }
}
#[doc = "Write proxy for field `FUNCTION`"]
pub struct FUNCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNCTION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Perform CRC32 operation value."]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut W {
        self.variant(FUNCTION_A::CRC32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bit 31 - CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
    #[inline(always)]
    pub fn crcerror(&self) -> CRCERROR_R {
        CRCERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Function Select"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
    #[inline(always)]
    pub fn crcerror(&mut self) -> CRCERROR_W {
        CRCERROR_W { w: self }
    }
    #[doc = "Bits 4:7 - Function Select"]
    #[inline(always)]
    pub fn function(&mut self) -> FUNCTION_W {
        FUNCTION_W { w: self }
    }
    #[doc = "Bit 0 - Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
