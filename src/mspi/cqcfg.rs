#[doc = "Reader of register CQCFG"]
pub type R = crate::R<u32, super::CQCFG>;
#[doc = "Writer for register CQCFG"]
pub type W = crate::W<u32, super::CQCFG>;
#[doc = "Register CQCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CQCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQAUTOCLEARMASK`"]
pub type CQAUTOCLEARMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQAUTOCLEARMASK`"]
pub struct CQAUTOCLEARMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CQAUTOCLEARMASK_W<'a> {
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
#[doc = "Reader of field `CQPWROFF`"]
pub type CQPWROFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQPWROFF`"]
pub struct CQPWROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPWROFF_W<'a> {
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
#[doc = "Sets the Priority of the command queue dma request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQPRI_A {
    #[doc = "0: Low Priority (service as best effort) value."]
    LOW = 0,
    #[doc = "1: High Priority (service immediately) value."]
    HIGH = 1,
}
impl From<CQPRI_A> for bool {
    #[inline(always)]
    fn from(variant: CQPRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CQPRI`"]
pub type CQPRI_R = crate::R<bool, CQPRI_A>;
impl CQPRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CQPRI_A {
        match self.bits {
            false => CQPRI_A::LOW,
            true => CQPRI_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CQPRI_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CQPRI_A::HIGH
    }
}
#[doc = "Write proxy for field `CQPRI`"]
pub struct CQPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQPRI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low Priority (service as best effort) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CQPRI_A::LOW)
    }
    #[doc = "High Priority (service immediately) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CQPRI_A::HIGH)
    }
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
#[doc = "Command queue enable. When set, will enable the processing of the command queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQEN_A {
    #[doc = "0: Disable CQ Function value."]
    DIS = 0,
    #[doc = "1: Enable CQ Function value."]
    EN = 1,
}
impl From<CQEN_A> for bool {
    #[inline(always)]
    fn from(variant: CQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CQEN`"]
pub type CQEN_R = crate::R<bool, CQEN_A>;
impl CQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CQEN_A {
        match self.bits {
            false => CQEN_A::DIS,
            true => CQEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CQEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CQEN_A::EN
    }
}
#[doc = "Write proxy for field `CQEN`"]
pub struct CQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CQ Function value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CQEN_A::DIS)
    }
    #[doc = "Enable CQ Function value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CQEN_A::EN)
    }
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
    #[doc = "Bit 3 - Eanble clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
    #[inline(always)]
    pub fn cqautoclearmask(&self) -> CQAUTOCLEARMASK_R {
        CQAUTOCLEARMASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub fn cqpwroff(&self) -> CQPWROFF_R {
        CQPWROFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue dma request"]
    #[inline(always)]
    pub fn cqpri(&self) -> CQPRI_R {
        CQPRI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue"]
    #[inline(always)]
    pub fn cqen(&self) -> CQEN_R {
        CQEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Eanble clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
    #[inline(always)]
    pub fn cqautoclearmask(&mut self) -> CQAUTOCLEARMASK_W {
        CQAUTOCLEARMASK_W { w: self }
    }
    #[doc = "Bit 2 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub fn cqpwroff(&mut self) -> CQPWROFF_W {
        CQPWROFF_W { w: self }
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue dma request"]
    #[inline(always)]
    pub fn cqpri(&mut self) -> CQPRI_W {
        CQPRI_W { w: self }
    }
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue"]
    #[inline(always)]
    pub fn cqen(&mut self) -> CQEN_W {
        CQEN_W { w: self }
    }
}
