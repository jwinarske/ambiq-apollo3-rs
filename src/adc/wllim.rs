#[doc = "Reader of register WLLIM"]
pub type R = crate::R<u32, super::WLLIM>;
#[doc = "Writer for register WLLIM"]
pub type W = crate::W<u32, super::WLLIM>;
#[doc = "Register WLLIM `reset()`'s with value 0"]
impl crate::ResetValue for super::WLLIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LLIM`"]
pub type LLIM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LLIM`"]
pub struct LLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Sets the lower limit for the window comparator."]
    #[inline(always)]
    pub fn llim(&self) -> LLIM_R {
        LLIM_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Sets the lower limit for the window comparator."]
    #[inline(always)]
    pub fn llim(&mut self) -> LLIM_W {
        LLIM_W { w: self }
    }
}
