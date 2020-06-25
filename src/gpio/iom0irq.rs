#[doc = "Reader of register IOM0IRQ"]
pub type R = crate::R<u32, super::IOM0IRQ>;
#[doc = "Writer for register IOM0IRQ"]
pub type W = crate::W<u32, super::IOM0IRQ>;
#[doc = "Register IOM0IRQ `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::IOM0IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `IOM0IRQ`"]
pub type IOM0IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOM0IRQ`"]
pub struct IOM0IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM0IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR0 IRQ pad select."]
    #[inline(always)]
    pub fn iom0irq(&self) -> IOM0IRQ_R {
        IOM0IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR0 IRQ pad select."]
    #[inline(always)]
    pub fn iom0irq(&mut self) -> IOM0IRQ_W {
        IOM0IRQ_W { w: self }
    }
}
