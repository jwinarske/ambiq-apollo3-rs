#[doc = "Reader of register CQCURIDX"]
pub type R = crate::R<u32, super::CQCURIDX>;
#[doc = "Writer for register CQCURIDX"]
pub type W = crate::W<u32, super::CQCURIDX>;
#[doc = "Register CQCURIDX `reset()`'s with value 0"]
impl crate::ResetValue for super::CQCURIDX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQCURIDX`"]
pub type CQCURIDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CQCURIDX`"]
pub struct CQCURIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CQCURIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
    #[inline(always)]
    pub fn cqcuridx(&self) -> CQCURIDX_R {
        CQCURIDX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
    #[inline(always)]
    pub fn cqcuridx(&mut self) -> CQCURIDX_W {
        CQCURIDX_W { w: self }
    }
}
