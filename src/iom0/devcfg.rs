#[doc = "Reader of register DEVCFG"]
pub type R = crate::R<u32, super::DEVCFG>;
#[doc = "Writer for register DEVCFG"]
pub type W = crate::W<u32, super::DEVCFG>;
#[doc = "Register DEVCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVADDR`"]
pub type DEVADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEVADDR`"]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
}
