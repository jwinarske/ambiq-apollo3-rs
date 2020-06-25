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
#[doc = "Reader of field `CQFTOGGLE`"]
pub type CQFTOGGLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CQFTOGGLE`"]
pub struct CQFTOGGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFTOGGLE_W<'a> {
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
    #[doc = "Bits 16:23 - Clear CQFlag status bits."]
    #[inline(always)]
    pub fn cqfclr(&self) -> CQFCLR_R {
        CQFCLR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Toggle CQFlag status bits"]
    #[inline(always)]
    pub fn cqftoggle(&self) -> CQFTOGGLE_R {
        CQFTOGGLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Set CQFlag status bits. Set has priority over clear if both are high."]
    #[inline(always)]
    pub fn cqfset(&self) -> CQFSET_R {
        CQFSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Clear CQFlag status bits."]
    #[inline(always)]
    pub fn cqfclr(&mut self) -> CQFCLR_W {
        CQFCLR_W { w: self }
    }
    #[doc = "Bits 8:15 - Toggle CQFlag status bits"]
    #[inline(always)]
    pub fn cqftoggle(&mut self) -> CQFTOGGLE_W {
        CQFTOGGLE_W { w: self }
    }
    #[doc = "Bits 0:7 - Set CQFlag status bits. Set has priority over clear if both are high."]
    #[inline(always)]
    pub fn cqfset(&mut self) -> CQFSET_W {
        CQFSET_W { w: self }
    }
}
