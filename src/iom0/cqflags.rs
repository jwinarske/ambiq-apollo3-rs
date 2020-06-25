#[doc = "Reader of register CQFLAGS"]
pub type R = crate::R<u32, super::CQFLAGS>;
#[doc = "Writer for register CQFLAGS"]
pub type W = crate::W<u32, super::CQFLAGS>;
#[doc = "Register CQFLAGS `reset()`'s with value 0"]
impl crate::ResetValue for super::CQFLAGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQIRQMASK`"]
pub type CQIRQMASK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CQIRQMASK`"]
pub struct CQIRQMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CQIRQMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CQFLAGS`"]
pub type CQFLAGS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CQFLAGS`"]
pub struct CQFLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFLAGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
    #[inline(always)]
    pub fn cqirqmask(&self) -> CQIRQMASK_R {
        CQIRQMASK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    pub fn cqflags(&self) -> CQFLAGS_R {
        CQFLAGS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
    #[inline(always)]
    pub fn cqirqmask(&mut self) -> CQIRQMASK_W {
        CQIRQMASK_W { w: self }
    }
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    pub fn cqflags(&mut self) -> CQFLAGS_W {
        CQFLAGS_W { w: self }
    }
}
