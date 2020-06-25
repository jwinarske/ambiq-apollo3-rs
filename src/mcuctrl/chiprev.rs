#[doc = "Reader of register CHIPREV"]
pub type R = crate::R<u32, super::CHIPREV>;
#[doc = "Writer for register CHIPREV"]
pub type W = crate::W<u32, super::CHIPREV>;
#[doc = "Register CHIPREV `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CHIPREV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `SIPART`"]
pub type SIPART_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIPART`"]
pub struct SIPART_W<'a> {
    w: &'a mut W,
}
impl<'a> SIPART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | (((value as u32) & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Major Revision ID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVMAJ_A {
    #[doc = "1: Apollo3 revision A value."]
    A = 1,
}
impl From<REVMAJ_A> for u8 {
    #[inline(always)]
    fn from(variant: REVMAJ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REVMAJ`"]
pub type REVMAJ_R = crate::R<u8, REVMAJ_A>;
impl REVMAJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVMAJ_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(REVMAJ_A::A),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == REVMAJ_A::A
    }
}
#[doc = "Write proxy for field `REVMAJ`"]
pub struct REVMAJ_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMAJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVMAJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 revision A value."]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(REVMAJ_A::A)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Minor Revision ID.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVMIN_A {
    #[doc = "2: Apollo3 minor rev 1. value."]
    REV1 = 2,
    #[doc = "1: Apollo3 minor rev 0.  Minor revision value, succeeding minor revisions will increment from this value. value."]
    REV0 = 1,
}
impl From<REVMIN_A> for u8 {
    #[inline(always)]
    fn from(variant: REVMIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REVMIN`"]
pub type REVMIN_R = crate::R<u8, REVMIN_A>;
impl REVMIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVMIN_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(REVMIN_A::REV1),
            1 => Val(REVMIN_A::REV0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REV1`"]
    #[inline(always)]
    pub fn is_rev1(&self) -> bool {
        *self == REVMIN_A::REV1
    }
    #[doc = "Checks if the value of the field is `REV0`"]
    #[inline(always)]
    pub fn is_rev0(&self) -> bool {
        *self == REVMIN_A::REV0
    }
}
#[doc = "Write proxy for field `REVMIN`"]
pub struct REVMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVMIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 minor rev 1. value."]
    #[inline(always)]
    pub fn rev1(self) -> &'a mut W {
        self.variant(REVMIN_A::REV1)
    }
    #[doc = "Apollo3 minor rev 0. Minor revision value, succeeding minor revisions will increment from this value. value."]
    #[inline(always)]
    pub fn rev0(self) -> &'a mut W {
        self.variant(REVMIN_A::REV0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:19 - Silicon Part ID"]
    #[inline(always)]
    pub fn sipart(&self) -> SIPART_R {
        SIPART_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline(always)]
    pub fn revmaj(&self) -> REVMAJ_R {
        REVMAJ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline(always)]
    pub fn revmin(&self) -> REVMIN_R {
        REVMIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:19 - Silicon Part ID"]
    #[inline(always)]
    pub fn sipart(&mut self) -> SIPART_W {
        SIPART_W { w: self }
    }
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline(always)]
    pub fn revmaj(&mut self) -> REVMAJ_W {
        REVMAJ_W { w: self }
    }
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline(always)]
    pub fn revmin(&mut self) -> REVMIN_W {
        REVMIN_W { w: self }
    }
}
