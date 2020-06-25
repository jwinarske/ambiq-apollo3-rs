#[doc = "Reader of register MISCCTRL"]
pub type R = crate::R<u32, super::MISCCTRL>;
#[doc = "Writer for register MISCCTRL"]
pub type W = crate::W<u32, super::MISCCTRL>;
#[doc = "Register MISCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MISCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLE_RESETN`"]
pub type BLE_RESETN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLE_RESETN`"]
pub struct BLE_RESETN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_RESETN_W<'a> {
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
#[doc = "Reader of field `RESERVED_RW_0`"]
pub type RESERVED_RW_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED_RW_0`"]
pub struct RESERVED_RW_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_RW_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - BLE reset signal."]
    #[inline(always)]
    pub fn ble_resetn(&self) -> BLE_RESETN_R {
        BLE_RESETN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Reserved bits, always leave unchanged. The MISCCTRL register must be modified via atomic RMW, leaving this bitfield completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_0(&self) -> RESERVED_RW_0_R {
        RESERVED_RW_0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - BLE reset signal."]
    #[inline(always)]
    pub fn ble_resetn(&mut self) -> BLE_RESETN_W {
        BLE_RESETN_W { w: self }
    }
    #[doc = "Bits 0:4 - Reserved bits, always leave unchanged. The MISCCTRL register must be modified via atomic RMW, leaving this bitfield completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_0(&mut self) -> RESERVED_RW_0_W {
        RESERVED_RW_0_W { w: self }
    }
}
