#[doc = "Reader of register CHIPID0"]
pub type R = crate::R<u32, super::CHIPID0>;
#[doc = "Writer for register CHIPID0"]
pub type W = crate::W<u32, super::CHIPID0>;
#[doc = "Register CHIPID0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHIPID0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Unique chip ID 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CHIPID0_A {
    #[doc = "0: Apollo3 CHIPID0. value."]
    APOLLO3 = 0,
}
impl From<CHIPID0_A> for u32 {
    #[inline(always)]
    fn from(variant: CHIPID0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHIPID0`"]
pub type CHIPID0_R = crate::R<u32, CHIPID0_A>;
impl CHIPID0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CHIPID0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHIPID0_A::APOLLO3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO3`"]
    #[inline(always)]
    pub fn is_apollo3(&self) -> bool {
        *self == CHIPID0_A::APOLLO3
    }
}
#[doc = "Write proxy for field `CHIPID0`"]
pub struct CHIPID0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIPID0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHIPID0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 CHIPID0. value."]
    #[inline(always)]
    pub fn apollo3(self) -> &'a mut W {
        self.variant(CHIPID0_A::APOLLO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique chip ID 0."]
    #[inline(always)]
    pub fn chipid0(&self) -> CHIPID0_R {
        CHIPID0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unique chip ID 0."]
    #[inline(always)]
    pub fn chipid0(&mut self) -> CHIPID0_W {
        CHIPID0_W { w: self }
    }
}
