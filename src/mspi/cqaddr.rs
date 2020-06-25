#[doc = "Reader of register CQADDR"]
pub type R = crate::R<u32, super::CQADDR>;
#[doc = "Writer for register CQADDR"]
pub type W = crate::W<u32, super::CQADDR>;
#[doc = "Register CQADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CQADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQADDR`"]
pub type CQADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CQADDR`"]
pub struct CQADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Address of command queue buffer in SRAM or flash. The buffer address must be aligned to a word boundary."]
    #[inline(always)]
    pub fn cqaddr(&self) -> CQADDR_R {
        CQADDR_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - Address of command queue buffer in SRAM or flash. The buffer address must be aligned to a word boundary."]
    #[inline(always)]
    pub fn cqaddr(&mut self) -> CQADDR_W {
        CQADDR_W { w: self }
    }
}
