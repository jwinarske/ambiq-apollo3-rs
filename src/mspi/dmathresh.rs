#[doc = "Reader of register DMATHRESH"]
pub type R = crate::R<u32, super::DMATHRESH>;
#[doc = "Writer for register DMATHRESH"]
pub type W = crate::W<u32, super::DMATHRESH>;
#[doc = "Register DMATHRESH `reset()`'s with value 0x08"]
impl crate::ResetValue for super::DMATHRESH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `DMATHRESH`"]
pub type DMATHRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMATHRESH`"]
pub struct DMATHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmathresh(&self) -> DMATHRESH_R {
        DMATHRESH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmathresh(&mut self) -> DMATHRESH_W {
        DMATHRESH_W { w: self }
    }
}
