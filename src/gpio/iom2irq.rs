#[doc = "Reader of register IOM2IRQ"]
pub type R = crate::R<u32, super::IOM2IRQ>;
#[doc = "Writer for register IOM2IRQ"]
pub type W = crate::W<u32, super::IOM2IRQ>;
#[doc = "Register IOM2IRQ `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::IOM2IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `IOM2IRQ`"]
pub type IOM2IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOM2IRQ`"]
pub struct IOM2IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM2IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR2 IRQ pad select."]
    #[inline(always)]
    pub fn iom2irq(&self) -> IOM2IRQ_R {
        IOM2IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR2 IRQ pad select."]
    #[inline(always)]
    pub fn iom2irq(&mut self) -> IOM2IRQ_W {
        IOM2IRQ_W { w: self }
    }
}
