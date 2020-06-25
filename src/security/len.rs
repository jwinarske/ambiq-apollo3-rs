#[doc = "Reader of register LEN"]
pub type R = crate::R<u32, super::LEN>;
#[doc = "Writer for register LEN"]
pub type W = crate::W<u32, super::LEN>;
#[doc = "Register LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEN`"]
pub type LEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEN`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 2)) | (((value as u32) & 0x0003_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:19 - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 2) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:19 - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
}
