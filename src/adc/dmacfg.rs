#[doc = "Reader of register DMACFG"]
pub type R = crate::R<u32, super::DMACFG>;
#[doc = "Writer for register DMACFG"]
pub type W = crate::W<u32, super::DMACFG>;
#[doc = "Register DMACFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPWROFF`"]
pub type DPWROFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPWROFF`"]
pub struct DPWROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPWROFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMSK_A {
    #[doc = "0: FIFO Contents are copied directly to memory without modification. value."]
    DIS = 0,
    #[doc = "1: Only the FIFODATA contents are copied to memory on DMA transfers. The SLOTNUM and FIFOCNT contents are cleared to zero. value."]
    EN = 1,
}
impl From<DMAMSK_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAMSK`"]
pub type DMAMSK_R = crate::R<bool, DMAMSK_A>;
impl DMAMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMSK_A {
        match self.bits {
            false => DMAMSK_A::DIS,
            true => DMAMSK_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMAMSK_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMAMSK_A::EN
    }
}
#[doc = "Write proxy for field `DMAMSK`"]
pub struct DMAMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO Contents are copied directly to memory without modification. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAMSK_A::DIS)
    }
    #[doc = "Only the FIFODATA contents are copied to memory on DMA transfers. The SLOTNUM and FIFOCNT contents are cleared to zero. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAMSK_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Halt New ADC conversions until DMA Status DMAERR and DMACPL Cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAHONSTAT_A {
    #[doc = "0: ADC conversions will continue regardless of DMA status register value."]
    DIS = 0,
    #[doc = "1: ADC conversions will not progress if DMAERR or DMACPL bits in DMA status register are set. value."]
    EN = 1,
}
impl From<DMAHONSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAHONSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAHONSTAT`"]
pub type DMAHONSTAT_R = crate::R<bool, DMAHONSTAT_A>;
impl DMAHONSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAHONSTAT_A {
        match self.bits {
            false => DMAHONSTAT_A::DIS,
            true => DMAHONSTAT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMAHONSTAT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMAHONSTAT_A::EN
    }
}
#[doc = "Write proxy for field `DMAHONSTAT`"]
pub struct DMAHONSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAHONSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAHONSTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC conversions will continue regardless of DMA status register value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAHONSTAT_A::DIS)
    }
    #[doc = "ADC conversions will not progress if DMAERR or DMACPL bits in DMA status register are set. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAHONSTAT_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMADYNPRI_A {
    #[doc = "0: Disable dynamic priority (use DMAPRI setting only) value."]
    DIS = 0,
    #[doc = "1: Enable dynamic priority value."]
    EN = 1,
}
impl From<DMADYNPRI_A> for bool {
    #[inline(always)]
    fn from(variant: DMADYNPRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMADYNPRI`"]
pub type DMADYNPRI_R = crate::R<bool, DMADYNPRI_A>;
impl DMADYNPRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADYNPRI_A {
        match self.bits {
            false => DMADYNPRI_A::DIS,
            true => DMADYNPRI_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMADYNPRI_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMADYNPRI_A::EN
    }
}
#[doc = "Write proxy for field `DMADYNPRI`"]
pub struct DMADYNPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADYNPRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADYNPRI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable dynamic priority (use DMAPRI setting only) value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMADYNPRI_A::DIS)
    }
    #[doc = "Enable dynamic priority value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMADYNPRI_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Sets the Priority of the DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAPRI_A {
    #[doc = "0: Low Priority (service as best effort) value."]
    LOW = 0,
    #[doc = "1: High Priority (service immediately) value."]
    HIGH = 1,
}
impl From<DMAPRI_A> for bool {
    #[inline(always)]
    fn from(variant: DMAPRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAPRI`"]
pub type DMAPRI_R = crate::R<bool, DMAPRI_A>;
impl DMAPRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAPRI_A {
        match self.bits {
            false => DMAPRI_A::LOW,
            true => DMAPRI_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DMAPRI_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DMAPRI_A::HIGH
    }
}
#[doc = "Write proxy for field `DMAPRI`"]
pub struct DMAPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAPRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAPRI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low Priority (service as best effort) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DMAPRI_A::LOW)
    }
    #[doc = "High Priority (service immediately) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DMAPRI_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMADIR_A {
    #[doc = "0: Peripheral to Memory (SRAM) transaction value."]
    P2M = 0,
    #[doc = "1: Memory to Peripheral transaction value."]
    M2P = 1,
}
impl From<DMADIR_A> for bool {
    #[inline(always)]
    fn from(variant: DMADIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMADIR`"]
pub type DMADIR_R = crate::R<bool, DMADIR_A>;
impl DMADIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADIR_A {
        match self.bits {
            false => DMADIR_A::P2M,
            true => DMADIR_A::M2P,
        }
    }
    #[doc = "Checks if the value of the field is `P2M`"]
    #[inline(always)]
    pub fn is_p2m(&self) -> bool {
        *self == DMADIR_A::P2M
    }
    #[doc = "Checks if the value of the field is `M2P`"]
    #[inline(always)]
    pub fn is_m2p(&self) -> bool {
        *self == DMADIR_A::M2P
    }
}
#[doc = "Write proxy for field `DMADIR`"]
pub struct DMADIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral to Memory (SRAM) transaction value."]
    #[inline(always)]
    pub fn p2m(self) -> &'a mut W {
        self.variant(DMADIR_A::P2M)
    }
    #[doc = "Memory to Peripheral transaction value."]
    #[inline(always)]
    pub fn m2p(self) -> &'a mut W {
        self.variant(DMADIR_A::M2P)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Disable DMA Function value."]
    DIS = 0,
    #[doc = "1: Enable DMA Function value."]
    EN = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DIS,
            true => DMAEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMAEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMAEN_A::EN
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable DMA Function value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAEN_A::DIS)
    }
    #[doc = "Enable DMA Function value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAEN_A::EN)
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
    #[doc = "Bit 18 - Power Off the ADC System upon DMACPL."]
    #[inline(always)]
    pub fn dpwroff(&self) -> DPWROFF_R {
        DPWROFF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory"]
    #[inline(always)]
    pub fn dmamsk(&self) -> DMAMSK_R {
        DMAMSK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Halt New ADC conversions until DMA Status DMAERR and DMACPL Cleared."]
    #[inline(always)]
    pub fn dmahonstat(&self) -> DMAHONSTAT_R {
        DMAHONSTAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
    #[inline(always)]
    pub fn dmadynpri(&self) -> DMADYNPRI_R {
        DMADYNPRI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&self) -> DMAPRI_R {
        DMAPRI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dmadir(&self) -> DMADIR_R {
        DMADIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Power Off the ADC System upon DMACPL."]
    #[inline(always)]
    pub fn dpwroff(&mut self) -> DPWROFF_W {
        DPWROFF_W { w: self }
    }
    #[doc = "Bit 17 - Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory"]
    #[inline(always)]
    pub fn dmamsk(&mut self) -> DMAMSK_W {
        DMAMSK_W { w: self }
    }
    #[doc = "Bit 16 - Halt New ADC conversions until DMA Status DMAERR and DMACPL Cleared."]
    #[inline(always)]
    pub fn dmahonstat(&mut self) -> DMAHONSTAT_W {
        DMAHONSTAT_W { w: self }
    }
    #[doc = "Bit 9 - Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
    #[inline(always)]
    pub fn dmadynpri(&mut self) -> DMADYNPRI_W {
        DMADYNPRI_W { w: self }
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&mut self) -> DMAPRI_W {
        DMAPRI_W { w: self }
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dmadir(&mut self) -> DMADIR_W {
        DMADIR_W { w: self }
    }
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
