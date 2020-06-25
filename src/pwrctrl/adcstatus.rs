#[doc = "Reader of register ADCSTATUS"]
pub type R = crate::R<u32, super::ADCSTATUS>;
#[doc = "Writer for register ADCSTATUS"]
pub type W = crate::W<u32, super::ADCSTATUS>;
#[doc = "Register ADCSTATUS `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::ADCSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `REFBUFPWD`"]
pub type REFBUFPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFBUFPWD`"]
pub struct REFBUFPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBUFPWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `REFKEEPPWD`"]
pub type REFKEEPPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFKEEPPWD`"]
pub struct REFKEEPPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFKEEPPWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `VBATPWD`"]
pub type VBATPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBATPWD`"]
pub struct VBATPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATPWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `VPTATPWD`"]
pub type VPTATPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VPTATPWD`"]
pub struct VPTATPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> VPTATPWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BGTPWD`"]
pub type BGTPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGTPWD`"]
pub struct BGTPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BGTPWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADCPWD`"]
pub type ADCPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPWD`"]
pub struct ADCPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPWD_W<'a> {
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
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    pub fn refbufpwd(&self) -> REFBUFPWD_R {
        REFBUFPWD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    pub fn refkeeppwd(&self) -> REFKEEPPWD_R {
        REFKEEPPWD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    pub fn vbatpwd(&self) -> VBATPWD_R {
        VBATPWD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    pub fn vptatpwd(&self) -> VPTATPWD_R {
        VPTATPWD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    pub fn bgtpwd(&self) -> BGTPWD_R {
        BGTPWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    pub fn adcpwd(&self) -> ADCPWD_R {
        ADCPWD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    pub fn refbufpwd(&mut self) -> REFBUFPWD_W {
        REFBUFPWD_W { w: self }
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    pub fn refkeeppwd(&mut self) -> REFKEEPPWD_W {
        REFKEEPPWD_W { w: self }
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    pub fn vbatpwd(&mut self) -> VBATPWD_W {
        VBATPWD_W { w: self }
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    pub fn vptatpwd(&mut self) -> VPTATPWD_W {
        VPTATPWD_W { w: self }
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    pub fn bgtpwd(&mut self) -> BGTPWD_W {
        BGTPWD_W { w: self }
    }
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    pub fn adcpwd(&mut self) -> ADCPWD_W {
        ADCPWD_W { w: self }
    }
}
