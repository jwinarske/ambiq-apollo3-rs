#[doc = "Reader of register INSTR"]
pub type R = crate::R<u32, super::INSTR>;
#[doc = "Writer for register INSTR"]
pub type W = crate::W<u32, super::INSTR>;
#[doc = "Register INSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR`"]
pub type INSTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INSTR`"]
pub struct INSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Optional Instruction field to send (1st byte) - qualified by ISEND/ISIZE"]
    #[inline(always)]
    pub fn instr(&self) -> INSTR_R {
        INSTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Optional Instruction field to send (1st byte) - qualified by ISEND/ISIZE"]
    #[inline(always)]
    pub fn instr(&mut self) -> INSTR_W {
        INSTR_W { w: self }
    }
}
