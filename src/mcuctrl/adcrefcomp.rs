#[doc = "Reader of register ADCREFCOMP"]
pub type R = crate::R<u32, super::ADCREFCOMP>;
#[doc = "Writer for register ADCREFCOMP"]
pub type W = crate::W<u32, super::ADCREFCOMP>;
#[doc = "Register ADCREFCOMP `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCREFCOMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCRFCMPEN`"]
pub type ADCRFCMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCRFCMPEN`"]
pub struct ADCRFCMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRFCMPEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADCREFKEEPTRIM`"]
pub type ADCREFKEEPTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCREFKEEPTRIM`"]
pub struct ADCREFKEEPTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCREFKEEPTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADC_REFCOMP_OUT`"]
pub type ADC_REFCOMP_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_REFCOMP_OUT`"]
pub struct ADC_REFCOMP_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REFCOMP_OUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - ADC Reference comparator power down"]
    #[inline(always)]
    pub fn adcrfcmpen(&self) -> ADCRFCMPEN_R {
        ADCRFCMPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - ADC Reference Keeper Trim"]
    #[inline(always)]
    pub fn adcrefkeeptrim(&self) -> ADCREFKEEPTRIM_R {
        ADCREFKEEPTRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Output of the ADC reference comparator"]
    #[inline(always)]
    pub fn adc_refcomp_out(&self) -> ADC_REFCOMP_OUT_R {
        ADC_REFCOMP_OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ADC Reference comparator power down"]
    #[inline(always)]
    pub fn adcrfcmpen(&mut self) -> ADCRFCMPEN_W {
        ADCRFCMPEN_W { w: self }
    }
    #[doc = "Bits 8:12 - ADC Reference Keeper Trim"]
    #[inline(always)]
    pub fn adcrefkeeptrim(&mut self) -> ADCREFKEEPTRIM_W {
        ADCREFKEEPTRIM_W { w: self }
    }
    #[doc = "Bit 0 - Output of the ADC reference comparator"]
    #[inline(always)]
    pub fn adc_refcomp_out(&mut self) -> ADC_REFCOMP_OUT_W {
        ADC_REFCOMP_OUT_W { w: self }
    }
}
