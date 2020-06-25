#[doc = "Reader of register CALRC"]
pub type R = crate::R<u32, super::CALRC>;
#[doc = "Writer for register CALRC"]
pub type W = crate::W<u32, super::CALRC>;
#[doc = "Register CALRC `reset()`'s with value 0"]
impl crate::ResetValue for super::CALRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALRC`"]
pub type CALRC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CALRC`"]
pub struct CALRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - LFRC Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 512 Hz clock derived from the original 1024 version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The range is from -131072 (decimal) to 131071 (decimal). This register is normally used in conjuction with ACALCTR register. The CALRC register will load the ACALCTR register (bits 17:0) if the ACALCTR register is set to measure the LFRC with the XT clock."]
    #[inline(always)]
    pub fn calrc(&self) -> CALRC_R {
        CALRC_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - LFRC Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 512 Hz clock derived from the original 1024 version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The range is from -131072 (decimal) to 131071 (decimal). This register is normally used in conjuction with ACALCTR register. The CALRC register will load the ACALCTR register (bits 17:0) if the ACALCTR register is set to measure the LFRC with the XT clock."]
    #[inline(always)]
    pub fn calrc(&mut self) -> CALRC_W {
        CALRC_W { w: self }
    }
}
