#[doc = "Reader of register CQENDIDX"]
pub type R = crate::R<u32, super::CQENDIDX>;
#[doc = "Writer for register CQENDIDX"]
pub type W = crate::W<u32, super::CQENDIDX>;
#[doc = "Register CQENDIDX `reset()`'s with value 0"]
impl crate::ResetValue for super::CQENDIDX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQENDIDX`"]
pub type CQENDIDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CQENDIDX`"]
pub struct CQENDIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CQENDIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
    #[inline(always)]
    pub fn cqendidx(&self) -> CQENDIDX_R {
        CQENDIDX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
    #[inline(always)]
    pub fn cqendidx(&mut self) -> CQENDIDX_W {
        CQENDIDX_W { w: self }
    }
}
