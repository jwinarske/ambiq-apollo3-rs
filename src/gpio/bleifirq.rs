#[doc = "Reader of register BLEIFIRQ"]
pub type R = crate::R<u32, super::BLEIFIRQ>;
#[doc = "Writer for register BLEIFIRQ"]
pub type W = crate::W<u32, super::BLEIFIRQ>;
#[doc = "Register BLEIFIRQ `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::BLEIFIRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `BLEIFIRQ`"]
pub type BLEIFIRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLEIFIRQ`"]
pub struct BLEIFIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEIFIRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - BLEIF IRQ pad select."]
    #[inline(always)]
    pub fn bleifirq(&self) -> BLEIFIRQ_R {
        BLEIFIRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - BLEIF IRQ pad select."]
    #[inline(always)]
    pub fn bleifirq(&mut self) -> BLEIFIRQ_W {
        BLEIFIRQ_W { w: self }
    }
}
