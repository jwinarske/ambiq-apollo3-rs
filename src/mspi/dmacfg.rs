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
#[doc = "Reader of field `DMAPWROFF`"]
pub type DMAPWROFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAPWROFF`"]
pub struct DMAPWROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAPWROFF_W<'a> {
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
#[doc = "Sets the Priority of the DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMAPRI_A {
    #[doc = "0: Low Priority (service as best effort) value."]
    LOW = 0,
    #[doc = "1: High Priority (service immediately) value."]
    HIGH = 1,
    #[doc = "2: Auto Priority (priority raised once TX FIFO empties or RX FIFO fills) value."]
    AUTO = 2,
}
impl From<DMAPRI_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAPRI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMAPRI`"]
pub type DMAPRI_R = crate::R<u8, DMAPRI_A>;
impl DMAPRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMAPRI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMAPRI_A::LOW),
            1 => Val(DMAPRI_A::HIGH),
            2 => Val(DMAPRI_A::AUTO),
            i => Res(i),
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
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == DMAPRI_A::AUTO
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = "Auto Priority (priority raised once TX FIFO empties or RX FIFO fills) value."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(DMAPRI_A::AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
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
#[doc = "DMA Enable. Setting this bit to EN will start the DMA operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMAEN_A {
    #[doc = "0: Disable DMA Function value."]
    DIS = 0,
    #[doc = "3: Enable HW controlled DMA Function to manage DMA to flash devices.  HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register. value."]
    EN = 3,
}
impl From<DMAEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<u8, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMAEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMAEN_A::DIS),
            3 => Val(DMAEN_A::EN),
            i => Res(i),
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable DMA Function value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAEN_A::DIS)
    }
    #[doc = "Enable HW controlled DMA Function to manage DMA to flash devices. HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAEN_A::EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub fn dmapwroff(&self) -> DMAPWROFF_R {
        DMAPWROFF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&self) -> DMAPRI_R {
        DMAPRI_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dmadir(&self) -> DMADIR_R {
        DMADIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - DMA Enable. Setting this bit to EN will start the DMA operation"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 18 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub fn dmapwroff(&mut self) -> DMAPWROFF_W {
        DMAPWROFF_W { w: self }
    }
    #[doc = "Bits 3:4 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&mut self) -> DMAPRI_W {
        DMAPRI_W { w: self }
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dmadir(&mut self) -> DMADIR_W {
        DMADIR_W { w: self }
    }
    #[doc = "Bits 0:1 - DMA Enable. Setting this bit to EN will start the DMA operation"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
