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
#[doc = "Reader of field `CQADDR28`"]
pub type CQADDR28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQADDR28`"]
pub struct CQADDR28_W<'a> {
    w: &'a mut W,
}
impl<'a> CQADDR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
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
        self.w.bits = (self.w.bits & !(0x0003_ffff << 2)) | (((value as u32) & 0x0003_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - Bit 28 of target byte address for source of CQ (read only). Used to denote Flash (0) or SRAM (1) access"]
    #[inline(always)]
    pub fn cqaddr28(&self) -> CQADDR28_R {
        CQADDR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 2:19 - Bits 19:2 of target byte address for source of CQ (read only). The buffer must be aligned on a word boundary"]
    #[inline(always)]
    pub fn cqaddr(&self) -> CQADDR_R {
        CQADDR_R::new(((self.bits >> 2) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 28 - Bit 28 of target byte address for source of CQ (read only). Used to denote Flash (0) or SRAM (1) access"]
    #[inline(always)]
    pub fn cqaddr28(&mut self) -> CQADDR28_W {
        CQADDR28_W { w: self }
    }
    #[doc = "Bits 2:19 - Bits 19:2 of target byte address for source of CQ (read only). The buffer must be aligned on a word boundary"]
    #[inline(always)]
    pub fn cqaddr(&mut self) -> CQADDR_W {
        CQADDR_W { w: self }
    }
}
