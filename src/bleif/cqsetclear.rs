#[doc = "Reader of register CQSETCLEAR"]
pub type R = crate::R<u32, super::CQSETCLEAR>;
#[doc = "Writer for register CQSETCLEAR"]
pub type W = crate::W<u32, super::CQSETCLEAR>;
#[doc = "Register CQSETCLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CQSETCLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQFCLR`"]
pub type CQFCLR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CQFCLR`"]
pub struct CQFCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CQFTGL`"]
pub type CQFTGL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CQFTGL`"]
pub struct CQFTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFTGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CQFSET`"]
pub type CQFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CQFSET`"]
pub struct CQFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfclr(&self) -> CQFCLR_R {
        CQFCLR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqftgl(&self) -> CQFTGL_R {
        CQFTGL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfset(&self) -> CQFSET_R {
        CQFSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfclr(&mut self) -> CQFCLR_W {
        CQFCLR_W { w: self }
    }
    #[doc = "Bits 8:15 - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqftgl(&mut self) -> CQFTGL_W {
        CQFTGL_W { w: self }
    }
    #[doc = "Bits 0:7 - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfset(&mut self) -> CQFSET_W {
        CQFSET_W { w: self }
    }
}
