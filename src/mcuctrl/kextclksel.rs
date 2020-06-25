#[doc = "Reader of register KEXTCLKSEL"]
pub type R = crate::R<u32, super::KEXTCLKSEL>;
#[doc = "Writer for register KEXTCLKSEL"]
pub type W = crate::W<u32, super::KEXTCLKSEL>;
#[doc = "Register KEXTCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::KEXTCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KEXTCLKSEL_A {
    #[doc = "83: Key value."]
    KEY = 83,
}
impl From<KEXTCLKSEL_A> for u32 {
    #[inline(always)]
    fn from(variant: KEXTCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KEXTCLKSEL`"]
pub type KEXTCLKSEL_R = crate::R<u32, KEXTCLKSEL_A>;
impl KEXTCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, KEXTCLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            83 => Val(KEXTCLKSEL_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == KEXTCLKSEL_A::KEY
    }
}
#[doc = "Write proxy for field `KEXTCLKSEL`"]
pub struct KEXTCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> KEXTCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEXTCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key value."]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(KEXTCLKSEL_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn kextclksel(&self) -> KEXTCLKSEL_R {
        KEXTCLKSEL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn kextclksel(&mut self) -> KEXTCLKSEL_W {
        KEXTCLKSEL_W { w: self }
    }
}
