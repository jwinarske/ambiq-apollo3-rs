#[doc = "Reader of register CQSTAT"]
pub type R = crate::R<u32, super::CQSTAT>;
#[doc = "Writer for register CQSTAT"]
pub type W = crate::W<u32, super::CQSTAT>;
#[doc = "Register CQSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CQSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQERR`"]
pub type CQERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQERR`"]
pub struct CQERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQERR_W<'a> {
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
#[doc = "Reader of field `CQPAUSED`"]
pub type CQPAUSED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQPAUSED`"]
pub struct CQPAUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPAUSED_W<'a> {
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
#[doc = "Reader of field `CQTIP`"]
pub type CQTIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQTIP`"]
pub struct CQTIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CQTIP_W<'a> {
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
    #[doc = "Bit 2 - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
    #[inline(always)]
    pub fn cqerr(&self) -> CQERR_R {
        CQERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command queue operation is currently paused."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CQPAUSED_R {
        CQPAUSED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
    #[inline(always)]
    pub fn cqtip(&self) -> CQTIP_R {
        CQTIP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
    #[inline(always)]
    pub fn cqerr(&mut self) -> CQERR_W {
        CQERR_W { w: self }
    }
    #[doc = "Bit 1 - Command queue operation is currently paused."]
    #[inline(always)]
    pub fn cqpaused(&mut self) -> CQPAUSED_W {
        CQPAUSED_W { w: self }
    }
    #[doc = "Bit 0 - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
    #[inline(always)]
    pub fn cqtip(&mut self) -> CQTIP_W {
        CQTIP_W { w: self }
    }
}
