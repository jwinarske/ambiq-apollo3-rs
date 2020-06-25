#[doc = "Reader of register LOCKSTAT"]
pub type R = crate::R<u32, super::LOCKSTAT>;
#[doc = "Writer for register LOCKSTAT"]
pub type W = crate::W<u32, super::LOCKSTAT>;
#[doc = "Register LOCKSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCKSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum STATUS_A {
    #[doc = "1: Customer Key is unlocked (access is granted to top half of info0) value."]
    CUSTOMER_KEY = 1,
    #[doc = "0: No resources are unlocked value."]
    NONE = 0,
}
impl From<STATUS_A> for u32 {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<u32, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(STATUS_A::CUSTOMER_KEY),
            0 => Val(STATUS_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CUSTOMER_KEY`"]
    #[inline(always)]
    pub fn is_customer_key(&self) -> bool {
        *self == STATUS_A::CUSTOMER_KEY
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == STATUS_A::NONE
    }
}
#[doc = "Write proxy for field `STATUS`"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Customer Key is unlocked (access is granted to top half of info0) value."]
    #[inline(always)]
    pub fn customer_key(self) -> &'a mut W {
        self.variant(STATUS_A::CUSTOMER_KEY)
    }
    #[doc = "No resources are unlocked value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(STATUS_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
}
