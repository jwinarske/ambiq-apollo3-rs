#[doc = "Reader of register ADCTRIM"]
pub type R = crate::R<u32, super::ADCTRIM>;
#[doc = "Writer for register ADCTRIM"]
pub type W = crate::W<u32, super::ADCTRIM>;
#[doc = "Register ADCTRIM `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::ADCTRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `ADCRFBUFIBTRIM`"]
pub type ADCRFBUFIBTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCRFBUFIBTRIM`"]
pub struct ADCRFBUFIBTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRFBUFIBTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `ADCREFBUFTRIM`"]
pub type ADCREFBUFTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCREFBUFTRIM`"]
pub struct ADCREFBUFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCREFBUFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADCREFKEEPIBTRIM`"]
pub type ADCREFKEEPIBTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCREFKEEPIBTRIM`"]
pub struct ADCREFKEEPIBTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCREFKEEPIBTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:12 - ADC reference buffer input bias trim"]
    #[inline(always)]
    pub fn adcrfbufibtrim(&self) -> ADCRFBUFIBTRIM_R {
        ADCRFBUFIBTRIM_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 6:10 - ADC Reference buffer trim"]
    #[inline(always)]
    pub fn adcrefbuftrim(&self) -> ADCREFBUFTRIM_R {
        ADCREFBUFTRIM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:1 - ADC Reference Ibias trim"]
    #[inline(always)]
    pub fn adcrefkeepibtrim(&self) -> ADCREFKEEPIBTRIM_R {
        ADCREFKEEPIBTRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 11:12 - ADC reference buffer input bias trim"]
    #[inline(always)]
    pub fn adcrfbufibtrim(&mut self) -> ADCRFBUFIBTRIM_W {
        ADCRFBUFIBTRIM_W { w: self }
    }
    #[doc = "Bits 6:10 - ADC Reference buffer trim"]
    #[inline(always)]
    pub fn adcrefbuftrim(&mut self) -> ADCREFBUFTRIM_W {
        ADCREFBUFTRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - ADC Reference Ibias trim"]
    #[inline(always)]
    pub fn adcrefkeepibtrim(&mut self) -> ADCREFKEEPIBTRIM_W {
        ADCREFKEEPIBTRIM_W { w: self }
    }
}
