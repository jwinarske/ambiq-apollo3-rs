#[doc = "Reader of register DEBUG"]
pub type R = crate::R<u32, super::DEBUG>;
#[doc = "Writer for register DEBUG"]
pub type W = crate::W<u32, super::DEBUG>;
#[doc = "Register DEBUG `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEBUGEN_A {
    #[doc = "0: Debug Disabled value."]
    OFF = 0,
    #[doc = "1: Debug Arb values value."]
    ARB = 1,
}
impl From<DEBUGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBUGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEBUGEN`"]
pub type DEBUGEN_R = crate::R<u8, DEBUGEN_A>;
impl DEBUGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEBUGEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEBUGEN_A::OFF),
            1 => Val(DEBUGEN_A::ARB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DEBUGEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ARB`"]
    #[inline(always)]
    pub fn is_arb(&self) -> bool {
        *self == DEBUGEN_A::ARB
    }
}
#[doc = "Write proxy for field `DEBUGEN`"]
pub struct DEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Debug Disabled value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DEBUGEN_A::OFF)
    }
    #[doc = "Debug Arb values value."]
    #[inline(always)]
    pub fn arb(self) -> &'a mut W {
        self.variant(DEBUGEN_A::ARB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Debug Enable"]
    #[inline(always)]
    pub fn debugen(&self) -> DEBUGEN_R {
        DEBUGEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Debug Enable"]
    #[inline(always)]
    pub fn debugen(&mut self) -> DEBUGEN_W {
        DEBUGEN_W { w: self }
    }
}
