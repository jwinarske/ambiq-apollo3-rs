#[doc = "Reader of register SNVR3"]
pub type R = crate::R<u32, super::SNVR3>;
#[doc = "Writer for register SNVR3"]
pub type W = crate::W<u32, super::SNVR3>;
#[doc = "Register SNVR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SNVR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SNVR3`"]
pub type SNVR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SNVR3`"]
pub struct SNVR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SNVR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr3(&self) -> SNVR3_R {
        SNVR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr3(&mut self) -> SNVR3_W {
        SNVR3_W { w: self }
    }
}
