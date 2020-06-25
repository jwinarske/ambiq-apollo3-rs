#[doc = "Reader of register APBDMACTRL"]
pub type R = crate::R<u32, super::APBDMACTRL>;
#[doc = "Writer for register APBDMACTRL"]
pub type W = crate::W<u32, super::APBDMACTRL>;
#[doc = "Register APBDMACTRL `reset()`'s with value 0x0203"]
impl crate::ResetValue for super::APBDMACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0203
    }
}
#[doc = "Reader of field `HYSTERESIS`"]
pub type HYSTERESIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HYSTERESIS`"]
pub struct HYSTERESIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTERESIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECODEABORT_A {
    #[doc = "0: Bus operations to powered down peripherals are quietly discarded value."]
    DISABLE = 0,
    #[doc = "1: Bus operations to powered down peripherals result in a bus fault. value."]
    ENABLE = 1,
}
impl From<DECODEABORT_A> for bool {
    #[inline(always)]
    fn from(variant: DECODEABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECODEABORT`"]
pub type DECODEABORT_R = crate::R<bool, DECODEABORT_A>;
impl DECODEABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECODEABORT_A {
        match self.bits {
            false => DECODEABORT_A::DISABLE,
            true => DECODEABORT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DECODEABORT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DECODEABORT_A::ENABLE
    }
}
#[doc = "Write proxy for field `DECODEABORT`"]
pub struct DECODEABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> DECODEABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECODEABORT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus operations to powered down peripherals are quietly discarded value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DECODEABORT_A::DISABLE)
    }
    #[doc = "Bus operations to powered down peripherals result in a bus fault. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DECODEABORT_A::ENABLE)
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
#[doc = "Enable the DMA controller. When disabled, DMA requests will be ignored by the controller\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENABLE_A {
    #[doc = "0: DMA operations disabled value."]
    DISABLE = 0,
    #[doc = "1: DMA operations enabled value."]
    ENABLE = 1,
}
impl From<DMA_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_ENABLE`"]
pub type DMA_ENABLE_R = crate::R<bool, DMA_ENABLE_A>;
impl DMA_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_ENABLE_A {
        match self.bits {
            false => DMA_ENABLE_A::DISABLE,
            true => DMA_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_ENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `DMA_ENABLE`"]
pub struct DMA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA operations disabled value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_ENABLE_A::DISABLE)
    }
    #[doc = "DMA operations enabled value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_ENABLE_A::ENABLE)
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
    #[doc = "Bits 8:15 - This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
    #[inline(always)]
    pub fn hysteresis(&self) -> HYSTERESIS_R {
        HYSTERESIS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 1 - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline(always)]
    pub fn decodeabort(&self) -> DECODEABORT_R {
        DECODEABORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
    #[inline(always)]
    pub fn hysteresis(&mut self) -> HYSTERESIS_W {
        HYSTERESIS_W { w: self }
    }
    #[doc = "Bit 1 - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline(always)]
    pub fn decodeabort(&mut self) -> DECODEABORT_W {
        DECODEABORT_W { w: self }
    }
    #[doc = "Bit 0 - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
    #[inline(always)]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W {
        DMA_ENABLE_W { w: self }
    }
}
