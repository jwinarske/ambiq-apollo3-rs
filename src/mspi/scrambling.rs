#[doc = "Reader of register SCRAMBLING"]
pub type R = crate::R<u32, super::SCRAMBLING>;
#[doc = "Writer for register SCRAMBLING"]
pub type W = crate::W<u32, super::SCRAMBLING>;
#[doc = "Register SCRAMBLING `reset()`'s with value 0"]
impl crate::ResetValue for super::SCRAMBLING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCRENABLE`"]
pub type SCRENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCRENABLE`"]
pub struct SCRENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SCREND`"]
pub type SCREND_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCREND`"]
pub struct SCREND_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCRSTART`"]
pub type SCRSTART_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCRSTART`"]
pub struct SCRSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRSTART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
    #[inline(always)]
    pub fn screnable(&self) -> SCRENABLE_R {
        SCRENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrend(&self) -> SCREND_R {
        SCREND_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrstart(&self) -> SCRSTART_R {
        SCRSTART_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
    #[inline(always)]
    pub fn screnable(&mut self) -> SCRENABLE_W {
        SCRENABLE_W { w: self }
    }
    #[doc = "Bits 16:25 - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrend(&mut self) -> SCREND_W {
        SCREND_W { w: self }
    }
    #[doc = "Bits 0:9 - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrstart(&mut self) -> SCRSTART_W {
        SCRSTART_W { w: self }
    }
}
