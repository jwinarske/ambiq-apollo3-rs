#[doc = "Reader of register VENDORID"]
pub type R = crate::R<u32, super::VENDORID>;
#[doc = "Writer for register VENDORID"]
pub type W = crate::W<u32, super::VENDORID>;
#[doc = "Register VENDORID `reset()`'s with value 0"]
impl crate::ResetValue for super::VENDORID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Unique Vendor ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum VENDORID_A {
    #[doc = "1095582289: Ambiq Vendor ID value."]
    AMBIQ = 1095582289,
}
impl From<VENDORID_A> for u32 {
    #[inline(always)]
    fn from(variant: VENDORID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VENDORID`"]
pub type VENDORID_R = crate::R<u32, VENDORID_A>;
impl VENDORID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, VENDORID_A> {
        use crate::Variant::*;
        match self.bits {
            1095582289 => Val(VENDORID_A::AMBIQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AMBIQ`"]
    #[inline(always)]
    pub fn is_ambiq(&self) -> bool {
        *self == VENDORID_A::AMBIQ
    }
}
#[doc = "Write proxy for field `VENDORID`"]
pub struct VENDORID_W<'a> {
    w: &'a mut W,
}
impl<'a> VENDORID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VENDORID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Ambiq Vendor ID value."]
    #[inline(always)]
    pub fn ambiq(self) -> &'a mut W {
        self.variant(VENDORID_A::AMBIQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique Vendor ID"]
    #[inline(always)]
    pub fn vendorid(&self) -> VENDORID_R {
        VENDORID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unique Vendor ID"]
    #[inline(always)]
    pub fn vendorid(&mut self) -> VENDORID_W {
        VENDORID_W { w: self }
    }
}
