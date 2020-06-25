#[doc = "Reader of register STMRCAP"]
pub type R = crate::R<u32, super::STMRCAP>;
#[doc = "Writer for register STMRCAP"]
pub type W = crate::W<u32, super::STMRCAP>;
#[doc = "Register STMRCAP `reset()`'s with value 0x3f3f_3f3f"]
impl crate::ResetValue for super::STMRCAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f3f_3f3f
    }
}
#[doc = "STIMER Capture 3 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL3_A {
    #[doc = "0: Capture on low to high GPIO transition value."]
    CAPLH = 0,
    #[doc = "1: Capture on high to low GPIO transition value."]
    CAPHL = 1,
}
impl From<STPOL3_A> for bool {
    #[inline(always)]
    fn from(variant: STPOL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STPOL3`"]
pub type STPOL3_R = crate::R<bool, STPOL3_A>;
impl STPOL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPOL3_A {
        match self.bits {
            false => STPOL3_A::CAPLH,
            true => STPOL3_A::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        *self == STPOL3_A::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        *self == STPOL3_A::CAPHL
    }
}
#[doc = "Write proxy for field `STPOL3`"]
pub struct STPOL3_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPOL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL3_A::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL3_A::CAPHL)
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
#[doc = "Reader of field `STSEL3`"]
pub type STSEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STSEL3`"]
pub struct STSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "STIMER Capture 2 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL2_A {
    #[doc = "0: Capture on low to high GPIO transition value."]
    CAPLH = 0,
    #[doc = "1: Capture on high to low GPIO transition value."]
    CAPHL = 1,
}
impl From<STPOL2_A> for bool {
    #[inline(always)]
    fn from(variant: STPOL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STPOL2`"]
pub type STPOL2_R = crate::R<bool, STPOL2_A>;
impl STPOL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPOL2_A {
        match self.bits {
            false => STPOL2_A::CAPLH,
            true => STPOL2_A::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        *self == STPOL2_A::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        *self == STPOL2_A::CAPHL
    }
}
#[doc = "Write proxy for field `STPOL2`"]
pub struct STPOL2_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPOL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL2_A::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL2_A::CAPHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `STSEL2`"]
pub type STSEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STSEL2`"]
pub struct STSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "STIMER Capture 1 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL1_A {
    #[doc = "0: Capture on low to high GPIO transition value."]
    CAPLH = 0,
    #[doc = "1: Capture on high to low GPIO transition value."]
    CAPHL = 1,
}
impl From<STPOL1_A> for bool {
    #[inline(always)]
    fn from(variant: STPOL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STPOL1`"]
pub type STPOL1_R = crate::R<bool, STPOL1_A>;
impl STPOL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPOL1_A {
        match self.bits {
            false => STPOL1_A::CAPLH,
            true => STPOL1_A::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        *self == STPOL1_A::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        *self == STPOL1_A::CAPHL
    }
}
#[doc = "Write proxy for field `STPOL1`"]
pub struct STPOL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPOL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL1_A::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL1_A::CAPHL)
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
#[doc = "Reader of field `STSEL1`"]
pub type STSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STSEL1`"]
pub struct STSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "STIMER Capture 0 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL0_A {
    #[doc = "0: Capture on low to high GPIO transition value."]
    CAPLH = 0,
    #[doc = "1: Capture on high to low GPIO transition value."]
    CAPHL = 1,
}
impl From<STPOL0_A> for bool {
    #[inline(always)]
    fn from(variant: STPOL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STPOL0`"]
pub type STPOL0_R = crate::R<bool, STPOL0_A>;
impl STPOL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPOL0_A {
        match self.bits {
            false => STPOL0_A::CAPLH,
            true => STPOL0_A::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        *self == STPOL0_A::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        *self == STPOL0_A::CAPHL
    }
}
#[doc = "Write proxy for field `STPOL0`"]
pub struct STPOL0_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPOL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL0_A::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL0_A::CAPHL)
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
#[doc = "Reader of field `STSEL0`"]
pub type STSEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STSEL0`"]
pub struct STSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - STIMER Capture 3 Polarity."]
    #[inline(always)]
    pub fn stpol3(&self) -> STPOL3_R {
        STPOL3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - STIMER Capture 3 Select."]
    #[inline(always)]
    pub fn stsel3(&self) -> STSEL3_R {
        STSEL3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - STIMER Capture 2 Polarity."]
    #[inline(always)]
    pub fn stpol2(&self) -> STPOL2_R {
        STPOL2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - STIMER Capture 2 Select."]
    #[inline(always)]
    pub fn stsel2(&self) -> STSEL2_R {
        STSEL2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - STIMER Capture 1 Polarity."]
    #[inline(always)]
    pub fn stpol1(&self) -> STPOL1_R {
        STPOL1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - STIMER Capture 1 Select."]
    #[inline(always)]
    pub fn stsel1(&self) -> STSEL1_R {
        STSEL1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 6 - STIMER Capture 0 Polarity."]
    #[inline(always)]
    pub fn stpol0(&self) -> STPOL0_R {
        STPOL0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - STIMER Capture 0 Select."]
    #[inline(always)]
    pub fn stsel0(&self) -> STSEL0_R {
        STSEL0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - STIMER Capture 3 Polarity."]
    #[inline(always)]
    pub fn stpol3(&mut self) -> STPOL3_W {
        STPOL3_W { w: self }
    }
    #[doc = "Bits 24:29 - STIMER Capture 3 Select."]
    #[inline(always)]
    pub fn stsel3(&mut self) -> STSEL3_W {
        STSEL3_W { w: self }
    }
    #[doc = "Bit 22 - STIMER Capture 2 Polarity."]
    #[inline(always)]
    pub fn stpol2(&mut self) -> STPOL2_W {
        STPOL2_W { w: self }
    }
    #[doc = "Bits 16:21 - STIMER Capture 2 Select."]
    #[inline(always)]
    pub fn stsel2(&mut self) -> STSEL2_W {
        STSEL2_W { w: self }
    }
    #[doc = "Bit 14 - STIMER Capture 1 Polarity."]
    #[inline(always)]
    pub fn stpol1(&mut self) -> STPOL1_W {
        STPOL1_W { w: self }
    }
    #[doc = "Bits 8:13 - STIMER Capture 1 Select."]
    #[inline(always)]
    pub fn stsel1(&mut self) -> STSEL1_W {
        STSEL1_W { w: self }
    }
    #[doc = "Bit 6 - STIMER Capture 0 Polarity."]
    #[inline(always)]
    pub fn stpol0(&mut self) -> STPOL0_W {
        STPOL0_W { w: self }
    }
    #[doc = "Bits 0:5 - STIMER Capture 0 Select."]
    #[inline(always)]
    pub fn stsel0(&mut self) -> STSEL0_W {
        STSEL0_W { w: self }
    }
}
