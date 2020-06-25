#[doc = "Reader of register DMASRAMREADPROTECT1"]
pub type R = crate::R<u32, super::DMASRAMREADPROTECT1>;
#[doc = "Writer for register DMASRAMREADPROTECT1"]
pub type W = crate::W<u32, super::DMASRAMREADPROTECT1>;
#[doc = "Register DMASRAMREADPROTECT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMASRAMREADPROTECT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_RPROT1`"]
pub type DMA_RPROT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMA_RPROT1`"]
pub struct DMA_RPROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RPROT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dma_rprot1(&self) -> DMA_RPROT1_R {
        DMA_RPROT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dma_rprot1(&mut self) -> DMA_RPROT1_W {
        DMA_RPROT1_W { w: self }
    }
}
