#[doc = "Reader of register IOM4IRQ"]
pub type R = crate::R<u32, super::IOM4IRQ>;
#[doc = "Writer for register IOM4IRQ"]
pub type W = crate::W<u32, super::IOM4IRQ>;
#[doc = "Register IOM4IRQ `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::IOM4IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `IOM4IRQ`"]
pub type IOM4IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOM4IRQ`"]
pub struct IOM4IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM4IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR4 IRQ pad select."]
    #[inline(always)]
    pub fn iom4irq(&self) -> IOM4IRQ_R {
        IOM4IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR4 IRQ pad select."]
    #[inline(always)]
    pub fn iom4irq(&mut self) -> IOM4IRQ_W {
        IOM4IRQ_W { w: self }
    }
}
