#[doc = "Reader of register WULIM"]
pub type R = crate::R<u32, super::WULIM>;
#[doc = "Writer for register WULIM"]
pub type W = crate::W<u32, super::WULIM>;
#[doc = "Register WULIM `reset()`'s with value 0"]
impl crate::ResetValue for super::WULIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ULIM`"]
pub type ULIM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ULIM`"]
pub struct ULIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ULIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Sets the upper limit for the window comparator."]
    #[inline(always)]
    pub fn ulim(&self) -> ULIM_R {
        ULIM_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Sets the upper limit for the window comparator."]
    #[inline(always)]
    pub fn ulim(&mut self) -> ULIM_W {
        ULIM_W { w: self }
    }
}
