#[doc = "Reader of register VOICESTAT"]
pub type R = crate::R<u32, super::VOICESTAT>;
#[doc = "Writer for register VOICESTAT"]
pub type W = crate::W<u32, super::VOICESTAT>;
#[doc = "Register VOICESTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::VOICESTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOCNT`"]
pub type FIFOCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOCNT`"]
pub struct FIFOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    pub fn fifocnt(&self) -> FIFOCNT_R {
        FIFOCNT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    pub fn fifocnt(&mut self) -> FIFOCNT_W {
        FIFOCNT_W { w: self }
    }
}
