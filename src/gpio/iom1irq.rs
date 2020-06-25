#[doc = "Reader of register IOM1IRQ"]
pub type R = crate::R<u32, super::IOM1IRQ>;
#[doc = "Writer for register IOM1IRQ"]
pub type W = crate::W<u32, super::IOM1IRQ>;
#[doc = "Register IOM1IRQ `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::IOM1IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `IOM1IRQ`"]
pub type IOM1IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOM1IRQ`"]
pub struct IOM1IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM1IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR1 IRQ pad select."]
    #[inline(always)]
    pub fn iom1irq(&self) -> IOM1IRQ_R {
        IOM1IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR1 IRQ pad select."]
    #[inline(always)]
    pub fn iom1irq(&mut self) -> IOM1IRQ_W {
        IOM1IRQ_W { w: self }
    }
}
