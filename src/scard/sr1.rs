#[doc = "Reader of register SR1"]
pub type R = crate::R<u32, super::SR1>;
#[doc = "Writer for register SR1"]
pub type W = crate::W<u32, super::SR1>;
#[doc = "Register SR1 `reset()`'s with value 0x08"]
impl crate::ResetValue for super::SR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLE`"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
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
#[doc = "Reader of field `SYNCEND`"]
pub type SYNCEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCEND`"]
pub struct SYNCEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEND_W<'a> {
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
#[doc = "Reader of field `PRL`"]
pub type PRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRL`"]
pub struct PRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRL_W<'a> {
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
#[doc = "Reader of field `ECNTOVER`"]
pub type ECNTOVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECNTOVER`"]
pub struct ECNTOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> ECNTOVER_W<'a> {
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
    #[doc = "Bit 3 - ISO7816 idle."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write complete synchronization."]
    #[inline(always)]
    pub fn syncend(&self) -> SYNCEND_R {
        SYNCEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Card insert/remove."]
    #[inline(always)]
    pub fn prl(&self) -> PRL_R {
        PRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ETU counter overflow."]
    #[inline(always)]
    pub fn ecntover(&self) -> ECNTOVER_R {
        ECNTOVER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ISO7816 idle."]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 2 - Write complete synchronization."]
    #[inline(always)]
    pub fn syncend(&mut self) -> SYNCEND_W {
        SYNCEND_W { w: self }
    }
    #[doc = "Bit 1 - Card insert/remove."]
    #[inline(always)]
    pub fn prl(&mut self) -> PRL_W {
        PRL_W { w: self }
    }
    #[doc = "Bit 0 - ETU counter overflow."]
    #[inline(always)]
    pub fn ecntover(&mut self) -> ECNTOVER_W {
        ECNTOVER_W { w: self }
    }
}
