#[doc = "Reader of register DEBUGGER"]
pub type R = crate::R<u32, super::DEBUGGER>;
#[doc = "Writer for register DEBUGGER"]
pub type W = crate::W<u32, super::DEBUGGER>;
#[doc = "Register DEBUGGER `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUGGER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCKOUT`"]
pub type LOCKOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKOUT`"]
pub struct LOCKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKOUT_W<'a> {
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
    #[doc = "Bit 0 - Lockout of debugger (SWD)."]
    #[inline(always)]
    pub fn lockout(&self) -> LOCKOUT_R {
        LOCKOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lockout of debugger (SWD)."]
    #[inline(always)]
    pub fn lockout(&mut self) -> LOCKOUT_W {
        LOCKOUT_W { w: self }
    }
}
