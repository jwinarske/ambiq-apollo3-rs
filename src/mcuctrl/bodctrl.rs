#[doc = "Reader of register BODCTRL"]
pub type R = crate::R<u32, super::BODCTRL>;
#[doc = "Writer for register BODCTRL"]
pub type W = crate::W<u32, super::BODCTRL>;
#[doc = "Register BODCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BODCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BODHVREFSEL`"]
pub type BODHVREFSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODHVREFSEL`"]
pub struct BODHVREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODHVREFSEL_W<'a> {
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
#[doc = "Reader of field `BODLVREFSEL`"]
pub type BODLVREFSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODLVREFSEL`"]
pub struct BODLVREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODLVREFSEL_W<'a> {
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
#[doc = "Reader of field `BODFPWD`"]
pub type BODFPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODFPWD`"]
pub struct BODFPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BODFPWD_W<'a> {
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
#[doc = "Reader of field `BODCPWD`"]
pub type BODCPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODCPWD`"]
pub struct BODCPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BODCPWD_W<'a> {
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
#[doc = "Reader of field `BODHPWD`"]
pub type BODHPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODHPWD`"]
pub struct BODHPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BODHPWD_W<'a> {
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
#[doc = "Reader of field `BODLPWD`"]
pub type BODLPWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODLPWD`"]
pub struct BODLPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BODLPWD_W<'a> {
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
    #[doc = "Bit 5 - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodhvrefsel(&self) -> BODHVREFSEL_R {
        BODHVREFSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodlvrefsel(&self) -> BODLVREFSEL_R {
        BODLVREFSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BODF Power Down."]
    #[inline(always)]
    pub fn bodfpwd(&self) -> BODFPWD_R {
        BODFPWD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BODC Power Down."]
    #[inline(always)]
    pub fn bodcpwd(&self) -> BODCPWD_R {
        BODCPWD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - BODH Power Down."]
    #[inline(always)]
    pub fn bodhpwd(&self) -> BODHPWD_R {
        BODHPWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BODL Power Down."]
    #[inline(always)]
    pub fn bodlpwd(&self) -> BODLPWD_R {
        BODLPWD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodhvrefsel(&mut self) -> BODHVREFSEL_W {
        BODHVREFSEL_W { w: self }
    }
    #[doc = "Bit 4 - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodlvrefsel(&mut self) -> BODLVREFSEL_W {
        BODLVREFSEL_W { w: self }
    }
    #[doc = "Bit 3 - BODF Power Down."]
    #[inline(always)]
    pub fn bodfpwd(&mut self) -> BODFPWD_W {
        BODFPWD_W { w: self }
    }
    #[doc = "Bit 2 - BODC Power Down."]
    #[inline(always)]
    pub fn bodcpwd(&mut self) -> BODCPWD_W {
        BODCPWD_W { w: self }
    }
    #[doc = "Bit 1 - BODH Power Down."]
    #[inline(always)]
    pub fn bodhpwd(&mut self) -> BODHPWD_W {
        BODHPWD_W { w: self }
    }
    #[doc = "Bit 0 - BODL Power Down."]
    #[inline(always)]
    pub fn bodlpwd(&mut self) -> BODLPWD_W {
        BODLPWD_W { w: self }
    }
}
