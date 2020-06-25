#[doc = "Reader of register DBGR1"]
pub type R = crate::R<u32, super::DBGR1>;
#[doc = "Writer for register DBGR1"]
pub type W = crate::W<u32, super::DBGR1>;
#[doc = "Register DBGR1 `reset()`'s with value 0x1234_5678"]
impl crate::ResetValue for super::DBGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1234_5678
    }
}
#[doc = "Reader of field `ONETO8`"]
pub type ONETO8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ONETO8`"]
pub struct ONETO8_W<'a> {
    w: &'a mut W,
}
impl<'a> ONETO8_W<'a> {
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
    pub fn oneto8(&self) -> ONETO8_R {
        ONETO8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    pub fn oneto8(&mut self) -> ONETO8_W {
        ONETO8_W { w: self }
    }
}
