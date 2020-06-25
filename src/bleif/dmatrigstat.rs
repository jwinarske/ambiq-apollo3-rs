#[doc = "Reader of register DMATRIGSTAT"]
pub type R = crate::R<u32, super::DMATRIGSTAT>;
#[doc = "Writer for register DMATRIGSTAT"]
pub type W = crate::W<u32, super::DMATRIGSTAT>;
#[doc = "Register DMATRIGSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATRIGSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTOTCMP`"]
pub type DTOTCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTOTCMP`"]
pub struct DTOTCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOTCMP_W<'a> {
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
#[doc = "Reader of field `DTHR`"]
pub type DTHR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTHR`"]
pub struct DTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHR_W<'a> {
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
#[doc = "Reader of field `DCMDCMP`"]
pub type DCMDCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMDCMP`"]
pub struct DCMDCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMDCMP_W<'a> {
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
    #[doc = "Bit 2 - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
    #[inline(always)]
    pub fn dtotcmp(&self) -> DTOTCMP_R {
        DTOTCMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dthr(&self) -> DTHR_R {
        DTHR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dcmdcmp(&self) -> DCMDCMP_R {
        DCMDCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
    #[inline(always)]
    pub fn dtotcmp(&mut self) -> DTOTCMP_W {
        DTOTCMP_W { w: self }
    }
    #[doc = "Bit 1 - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dthr(&mut self) -> DTHR_W {
        DTHR_W { w: self }
    }
    #[doc = "Bit 0 - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dcmdcmp(&mut self) -> DCMDCMP_W {
        DCMDCMP_W { w: self }
    }
}
