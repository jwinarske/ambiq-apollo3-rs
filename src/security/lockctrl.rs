#[doc = "Reader of register LOCKCTRL"]
pub type R = crate::R<u32, super::LOCKCTRL>;
#[doc = "Writer for register LOCKCTRL"]
pub type W = crate::W<u32, super::LOCKCTRL>;
#[doc = "Register LOCKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LOCK Function Select register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELECT_A {
    #[doc = "1: Unlock Customer Key (access to top half of info0) value."]
    CUSTOMER_KEY = 1,
    #[doc = "0: Lock Control should be set to NONE when not in use. value."]
    NONE = 0,
}
impl From<SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELECT`"]
pub type SELECT_R = crate::R<u8, SELECT_A>;
impl SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SELECT_A::CUSTOMER_KEY),
            0 => Val(SELECT_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CUSTOMER_KEY`"]
    #[inline(always)]
    pub fn is_customer_key(&self) -> bool {
        *self == SELECT_A::CUSTOMER_KEY
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SELECT_A::NONE
    }
}
#[doc = "Write proxy for field `SELECT`"]
pub struct SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unlock Customer Key (access to top half of info0) value."]
    #[inline(always)]
    pub fn customer_key(self) -> &'a mut W {
        self.variant(SELECT_A::CUSTOMER_KEY)
    }
    #[doc = "Lock Control should be set to NONE when not in use. value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SELECT_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LOCK Function Select register."]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LOCK Function Select register."]
    #[inline(always)]
    pub fn select(&mut self) -> SELECT_W {
        SELECT_W { w: self }
    }
}
