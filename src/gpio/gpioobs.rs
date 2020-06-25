#[doc = "Reader of register GPIOOBS"]
pub type R = crate::R<u32, super::GPIOOBS>;
#[doc = "Writer for register GPIOOBS"]
pub type W = crate::W<u32, super::GPIOOBS>;
#[doc = "Register GPIOOBS `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOOBS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OBS_DATA`"]
pub type OBS_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OBS_DATA`"]
pub struct OBS_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> OBS_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only"]
    #[inline(always)]
    pub fn obs_data(&self) -> OBS_DATA_R {
        OBS_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only"]
    #[inline(always)]
    pub fn obs_data(&mut self) -> OBS_DATA_W {
        OBS_DATA_W { w: self }
    }
}
