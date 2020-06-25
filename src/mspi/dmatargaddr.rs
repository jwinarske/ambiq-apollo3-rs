#[doc = "Reader of register DMATARGADDR"]
pub type R = crate::R<u32, super::DMATARGADDR>;
#[doc = "Writer for register DMATARGADDR"]
pub type W = crate::W<u32, super::DMATARGADDR>;
#[doc = "Register DMATARGADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATARGADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TARGADDR`"]
pub type TARGADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TARGADDR`"]
pub struct TARGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    pub fn targaddr(&self) -> TARGADDR_R {
        TARGADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    pub fn targaddr(&mut self) -> TARGADDR_W {
        TARGADDR_W { w: self }
    }
}
