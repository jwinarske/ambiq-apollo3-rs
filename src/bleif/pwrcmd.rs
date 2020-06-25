#[doc = "Reader of register PWRCMD"]
pub type R = crate::R<u32, super::PWRCMD>;
#[doc = "Writer for register PWRCMD"]
pub type W = crate::W<u32, super::PWRCMD>;
#[doc = "Register PWRCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESTART`"]
pub type RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESTART`"]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
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
#[doc = "Reader of field `WAKEREQ`"]
pub type WAKEREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEREQ`"]
pub struct WAKEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEREQ_W<'a> {
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
    #[doc = "Bit 1 - Restart the BLE Core after going into the shutdown state. Only valid when in the shutdown state."]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wake request from the MCU. When asserted (1), the BLE Interface logic will assert the wakeup request signal to the BLE Core. Only recognized when in the sleep state"]
    #[inline(always)]
    pub fn wakereq(&self) -> WAKEREQ_R {
        WAKEREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Restart the BLE Core after going into the shutdown state. Only valid when in the shutdown state."]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bit 0 - Wake request from the MCU. When asserted (1), the BLE Interface logic will assert the wakeup request signal to the BLE Core. Only recognized when in the sleep state"]
    #[inline(always)]
    pub fn wakereq(&mut self) -> WAKEREQ_W {
        WAKEREQ_W { w: self }
    }
}
