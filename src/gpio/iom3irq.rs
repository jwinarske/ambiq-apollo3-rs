#[doc = "Reader of register IOM3IRQ"]
pub type R = crate::R<u32, super::IOM3IRQ>;
#[doc = "Writer for register IOM3IRQ"]
pub type W = crate::W<u32, super::IOM3IRQ>;
#[doc = "Register IOM3IRQ `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::IOM3IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `IOM3IRQ`"]
pub type IOM3IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOM3IRQ`"]
pub struct IOM3IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM3IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR3 IRQ pad select."]
    #[inline(always)]
    pub fn iom3irq(&self) -> IOM3IRQ_R {
        IOM3IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR3 IRQ pad select."]
    #[inline(always)]
    pub fn iom3irq(&mut self) -> IOM3IRQ_W {
        IOM3IRQ_W { w: self }
    }
}
