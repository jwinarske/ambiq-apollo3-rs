#[doc = "Reader of register SNVR2"]
pub type R = crate::R<u32, super::SNVR2>;
#[doc = "Writer for register SNVR2"]
pub type W = crate::W<u32, super::SNVR2>;
#[doc = "Register SNVR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SNVR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SNVR2`"]
pub type SNVR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SNVR2`"]
pub struct SNVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SNVR2_W<'a> {
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
    pub fn snvr2(&self) -> SNVR2_R {
        SNVR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr2(&mut self) -> SNVR2_W {
        SNVR2_W { w: self }
    }
}
