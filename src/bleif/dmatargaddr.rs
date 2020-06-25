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
#[doc = "Reader of field `TARGADDR28`"]
pub type TARGADDR28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TARGADDR28`"]
pub struct TARGADDR28_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGADDR28_W<'a> {
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
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - Bit 28 of the target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written. Setting to '1' will select the SRAM. Setting to '0' will select the flash"]
    #[inline(always)]
    pub fn targaddr28(&self) -> TARGADDR28_R {
        TARGADDR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 0:19 - Bits \\[19:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    pub fn targaddr(&self) -> TARGADDR_R {
        TARGADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 28 - Bit 28 of the target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written. Setting to '1' will select the SRAM. Setting to '0' will select the flash"]
    #[inline(always)]
    pub fn targaddr28(&mut self) -> TARGADDR28_W {
        TARGADDR28_W { w: self }
    }
    #[doc = "Bits 0:19 - Bits \\[19:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    pub fn targaddr(&mut self) -> TARGADDR_W {
        TARGADDR_W { w: self }
    }
}
