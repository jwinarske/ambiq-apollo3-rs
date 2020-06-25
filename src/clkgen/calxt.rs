#[doc = "Reader of register CALXT"]
pub type R = crate::R<u32, super::CALXT>;
#[doc = "Writer for register CALXT"]
pub type W = crate::W<u32, super::CALXT>;
#[doc = "Register CALXT `reset()`'s with value 0"]
impl crate::ResetValue for super::CALXT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALXT`"]
pub type CALXT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CALXT`"]
pub struct CALXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - XT Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 16KHz clock derived from the original 32KHz version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The maximum value that is effective is from -1024 to 1023."]
    #[inline(always)]
    pub fn calxt(&self) -> CALXT_R {
        CALXT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - XT Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 16KHz clock derived from the original 32KHz version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The maximum value that is effective is from -1024 to 1023."]
    #[inline(always)]
    pub fn calxt(&mut self) -> CALXT_W {
        CALXT_W { w: self }
    }
}
