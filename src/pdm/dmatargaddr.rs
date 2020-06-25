#[doc = "Reader of register DMATARGADDR"]
pub type R = crate::R<u32, super::DMATARGADDR>;
#[doc = "Writer for register DMATARGADDR"]
pub type W = crate::W<u32, super::DMATARGADDR>;
#[doc = "Register DMATARGADDR `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::DMATARGADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Reader of field `UTARGADDR`"]
pub type UTARGADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UTARGADDR`"]
pub struct UTARGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UTARGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `LTARGADDR`"]
pub type LTARGADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LTARGADDR`"]
pub struct LTARGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LTARGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - SRAM Target"]
    #[inline(always)]
    pub fn utargaddr(&self) -> UTARGADDR_R {
        UTARGADDR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:19 - DMA Target Address. This register is not updated with the current address of the DMA, but will remain static with the original address during the DMA transfer."]
    #[inline(always)]
    pub fn ltargaddr(&self) -> LTARGADDR_R {
        LTARGADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 20:31 - SRAM Target"]
    #[inline(always)]
    pub fn utargaddr(&mut self) -> UTARGADDR_W {
        UTARGADDR_W { w: self }
    }
    #[doc = "Bits 0:19 - DMA Target Address. This register is not updated with the current address of the DMA, but will remain static with the original address during the DMA transfer."]
    #[inline(always)]
    pub fn ltargaddr(&mut self) -> LTARGADDR_W {
        LTARGADDR_W { w: self }
    }
}
