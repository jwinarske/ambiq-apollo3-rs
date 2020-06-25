#[doc = "Reader of register ADCPWRDLY"]
pub type R = crate::R<u32, super::ADCPWRDLY>;
#[doc = "Writer for register ADCPWRDLY"]
pub type W = crate::W<u32, super::ADCPWRDLY>;
#[doc = "Register ADCPWRDLY `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCPWRDLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCPWR1`"]
pub type ADCPWR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCPWR1`"]
pub struct ADCPWR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPWR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADCPWR0`"]
pub type ADCPWR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCPWR0`"]
pub struct ADCPWR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPWR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn adcpwr1(&self) -> ADCPWR1_R {
        ADCPWR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn adcpwr0(&self) -> ADCPWR0_R {
        ADCPWR0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn adcpwr1(&mut self) -> ADCPWR1_W {
        ADCPWR1_W { w: self }
    }
    #[doc = "Bits 0:7 - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn adcpwr0(&mut self) -> ADCPWR0_W {
        ADCPWR0_W { w: self }
    }
}
