#[doc = "Reader of register OTAPOINTER"]
pub type R = crate::R<u32, super::OTAPOINTER>;
#[doc = "Writer for register OTAPOINTER"]
pub type W = crate::W<u32, super::OTAPOINTER>;
#[doc = "Register OTAPOINTER `reset()`'s with value 0"]
impl crate::ResetValue for super::OTAPOINTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OTAPOINTER`"]
pub type OTAPOINTER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OTAPOINTER`"]
pub struct OTAPOINTER_W<'a> {
    w: &'a mut W,
}
impl<'a> OTAPOINTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `OTASBLUPDATE`"]
pub type OTASBLUPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTASBLUPDATE`"]
pub struct OTASBLUPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTASBLUPDATE_W<'a> {
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
#[doc = "Reader of field `OTAVALID`"]
pub type OTAVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTAVALID`"]
pub struct OTAVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OTAVALID_W<'a> {
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
    #[doc = "Bits 2:31 - Flash page pointer with updated OTA image"]
    #[inline(always)]
    pub fn otapointer(&self) -> OTAPOINTER_R {
        OTAPOINTER_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - Indicates that the sbl_init has been updated"]
    #[inline(always)]
    pub fn otasblupdate(&self) -> OTASBLUPDATE_R {
        OTASBLUPDATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates that an OTA update is valid"]
    #[inline(always)]
    pub fn otavalid(&self) -> OTAVALID_R {
        OTAVALID_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - Flash page pointer with updated OTA image"]
    #[inline(always)]
    pub fn otapointer(&mut self) -> OTAPOINTER_W {
        OTAPOINTER_W { w: self }
    }
    #[doc = "Bit 1 - Indicates that the sbl_init has been updated"]
    #[inline(always)]
    pub fn otasblupdate(&mut self) -> OTASBLUPDATE_W {
        OTASBLUPDATE_W { w: self }
    }
    #[doc = "Bit 0 - Indicates that an OTA update is valid"]
    #[inline(always)]
    pub fn otavalid(&mut self) -> OTAVALID_W {
        OTAVALID_W { w: self }
    }
}
