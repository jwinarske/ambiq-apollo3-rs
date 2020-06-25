#[doc = "Reader of register BLEBUCK2"]
pub type R = crate::R<u32, super::BLEBUCK2>;
#[doc = "Writer for register BLEBUCK2"]
pub type W = crate::W<u32, super::BLEBUCK2>;
#[doc = "Register BLEBUCK2 `reset()`'s with value 0x4e"]
impl crate::ResetValue for super::BLEBUCK2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4e
    }
}
#[doc = "Reader of field `BLEBUCKTOND2ATRIM`"]
pub type BLEBUCKTOND2ATRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLEBUCKTOND2ATRIM`"]
pub struct BLEBUCKTOND2ATRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKTOND2ATRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `BLEBUCKTONHITRIM`"]
pub type BLEBUCKTONHITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLEBUCKTONHITRIM`"]
pub struct BLEBUCKTONHITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKTONHITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `BLEBUCKTONLOWTRIM`"]
pub type BLEBUCKTONLOWTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLEBUCKTONLOWTRIM`"]
pub struct BLEBUCKTONLOWTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKTONLOWTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:17 - blebuck_ton_trim"]
    #[inline(always)]
    pub fn blebucktond2atrim(&self) -> BLEBUCKTOND2ATRIM_R {
        BLEBUCKTOND2ATRIM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - blebuck_ton_hi_trim"]
    #[inline(always)]
    pub fn blebucktonhitrim(&self) -> BLEBUCKTONHITRIM_R {
        BLEBUCKTONHITRIM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - blebuck_ton_low_trim"]
    #[inline(always)]
    pub fn blebucktonlowtrim(&self) -> BLEBUCKTONLOWTRIM_R {
        BLEBUCKTONLOWTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:17 - blebuck_ton_trim"]
    #[inline(always)]
    pub fn blebucktond2atrim(&mut self) -> BLEBUCKTOND2ATRIM_W {
        BLEBUCKTOND2ATRIM_W { w: self }
    }
    #[doc = "Bits 6:11 - blebuck_ton_hi_trim"]
    #[inline(always)]
    pub fn blebucktonhitrim(&mut self) -> BLEBUCKTONHITRIM_W {
        BLEBUCKTONHITRIM_W { w: self }
    }
    #[doc = "Bits 0:5 - blebuck_ton_low_trim"]
    #[inline(always)]
    pub fn blebucktonlowtrim(&mut self) -> BLEBUCKTONLOWTRIM_W {
        BLEBUCKTONLOWTRIM_W { w: self }
    }
}
