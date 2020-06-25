#[doc = "Reader of register IOM5IRQ"]
pub type R = crate::R<u32, super::IOM5IRQ>;
#[doc = "Writer for register IOM5IRQ"]
pub type W = crate::W<u32, super::IOM5IRQ>;
#[doc = "Register IOM5IRQ `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::IOM5IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `IOM5IRQ`"]
pub type IOM5IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOM5IRQ`"]
pub struct IOM5IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM5IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR5 IRQ pad select."]
    #[inline(always)]
    pub fn iom5irq(&self) -> IOM5IRQ_R {
        IOM5IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR5 IRQ pad select."]
    #[inline(always)]
    pub fn iom5irq(&mut self) -> IOM5IRQ_W {
        IOM5IRQ_W { w: self }
    }
}
