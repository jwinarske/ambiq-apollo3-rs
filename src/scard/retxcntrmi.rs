#[doc = "Reader of register RETXCNTRMI"]
pub type R = crate::R<u32, super::RETXCNTRMI>;
#[doc = "Writer for register RETXCNTRMI"]
pub type W = crate::W<u32, super::RETXCNTRMI>;
#[doc = "Register RETXCNTRMI `reset()`'s with value 0"]
impl crate::ResetValue for super::RETXCNTRMI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RETXCNTRMI`"]
pub type RETXCNTRMI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETXCNTRMI`"]
pub struct RETXCNTRMI_W<'a> {
    w: &'a mut W,
}
impl<'a> RETXCNTRMI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Resent count inquiry register."]
    #[inline(always)]
    pub fn retxcntrmi(&self) -> RETXCNTRMI_R {
        RETXCNTRMI_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resent count inquiry register."]
    #[inline(always)]
    pub fn retxcntrmi(&mut self) -> RETXCNTRMI_W {
        RETXCNTRMI_W { w: self }
    }
}
