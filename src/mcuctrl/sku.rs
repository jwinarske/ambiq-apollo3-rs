#[doc = "Reader of register SKU"]
pub type R = crate::R<u32, super::SKU>;
#[doc = "Writer for register SKU"]
pub type W = crate::W<u32, super::SKU>;
#[doc = "Register SKU `reset()`'s with value 0"]
impl crate::ResetValue for super::SKU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECBOOT`"]
pub type SECBOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECBOOT`"]
pub struct SECBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBOOT_W<'a> {
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
#[doc = "Reader of field `ALLOWBLE`"]
pub type ALLOWBLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLOWBLE`"]
pub struct ALLOWBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOWBLE_W<'a> {
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
#[doc = "Reader of field `ALLOWBURST`"]
pub type ALLOWBURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLOWBURST`"]
pub struct ALLOWBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOWBURST_W<'a> {
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
    #[doc = "Bit 2 - Secure boot feature allowed"]
    #[inline(always)]
    pub fn secboot(&self) -> SECBOOT_R {
        SECBOOT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allow BLE feature"]
    #[inline(always)]
    pub fn allowble(&self) -> ALLOWBLE_R {
        ALLOWBLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Allow Burst feature"]
    #[inline(always)]
    pub fn allowburst(&self) -> ALLOWBURST_R {
        ALLOWBURST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Secure boot feature allowed"]
    #[inline(always)]
    pub fn secboot(&mut self) -> SECBOOT_W {
        SECBOOT_W { w: self }
    }
    #[doc = "Bit 1 - Allow BLE feature"]
    #[inline(always)]
    pub fn allowble(&mut self) -> ALLOWBLE_W {
        ALLOWBLE_W { w: self }
    }
    #[doc = "Bit 0 - Allow Burst feature"]
    #[inline(always)]
    pub fn allowburst(&mut self) -> ALLOWBURST_W {
        ALLOWBURST_W { w: self }
    }
}
