#[doc = "Reader of register DBGR2"]
pub type R = crate::R<u32, super::DBGR2>;
#[doc = "Writer for register DBGR2"]
pub type W = crate::W<u32, super::DBGR2>;
#[doc = "Register DBGR2 `reset()`'s with value 0xc001_c0de"]
impl crate::ResetValue for super::DBGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc001_c0de
    }
}
#[doc = "Reader of field `COOLCODE`"]
pub type COOLCODE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COOLCODE`"]
pub struct COOLCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COOLCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    pub fn coolcode(&self) -> COOLCODE_R {
        COOLCODE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    pub fn coolcode(&mut self) -> COOLCODE_W {
        COOLCODE_W { w: self }
    }
}
