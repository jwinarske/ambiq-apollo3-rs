#[doc = "Reader of register ICODEFAULTADDR"]
pub type R = crate::R<u32, super::ICODEFAULTADDR>;
#[doc = "Writer for register ICODEFAULTADDR"]
pub type W = crate::W<u32, super::ICODEFAULTADDR>;
#[doc = "Register ICODEFAULTADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICODEFAULTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICODEFAULTADDR`"]
pub type ICODEFAULTADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICODEFAULTADDR`"]
pub struct ICODEFAULTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICODEFAULTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn icodefaultaddr(&self) -> ICODEFAULTADDR_R {
        ICODEFAULTADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn icodefaultaddr(&mut self) -> ICODEFAULTADDR_W {
        ICODEFAULTADDR_W { w: self }
    }
}
