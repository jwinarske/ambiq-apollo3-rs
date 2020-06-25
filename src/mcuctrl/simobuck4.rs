#[doc = "Reader of register SIMOBUCK4"]
pub type R = crate::R<u32, super::SIMOBUCK4>;
#[doc = "Writer for register SIMOBUCK4"]
pub type W = crate::W<u32, super::SIMOBUCK4>;
#[doc = "Register SIMOBUCK4 `reset()`'s with value 0x3c8d_80aa"]
impl crate::ResetValue for super::SIMOBUCK4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3c8d_80aa
    }
}
#[doc = "Reader of field `SIMOBUCKIBIASTRIM`"]
pub type SIMOBUCKIBIASTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKIBIASTRIM`"]
pub struct SIMOBUCKIBIASTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKIBIASTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKUVLOMODE`"]
pub type SIMOBUCKUVLOMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKUVLOMODE`"]
pub struct SIMOBUCKUVLOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKUVLOMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKPRIORITYSEL`"]
pub type SIMOBUCKPRIORITYSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMOBUCKPRIORITYSEL`"]
pub struct SIMOBUCKPRIORITYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKPRIORITYSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKCOMP2TIMEOUTEN`"]
pub type SIMOBUCKCOMP2TIMEOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMOBUCKCOMP2TIMEOUTEN`"]
pub struct SIMOBUCKCOMP2TIMEOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCOMP2TIMEOUTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKCOMP2LPEN`"]
pub type SIMOBUCKCOMP2LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMOBUCKCOMP2LPEN`"]
pub struct SIMOBUCKCOMP2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCOMP2LPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKCLKDIVSEL`"]
pub type SIMOBUCKCLKDIVSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKCLKDIVSEL`"]
pub struct SIMOBUCKCLKDIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCLKDIVSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKEXTCLKSEL`"]
pub type SIMOBUCKEXTCLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMOBUCKEXTCLKSEL`"]
pub struct SIMOBUCKEXTCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKEXTCLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKUVLODRVSTRTRIM`"]
pub type SIMOBUCKUVLODRVSTRTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKUVLODRVSTRTRIM`"]
pub struct SIMOBUCKUVLODRVSTRTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKUVLODRVSTRTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKUVLOCNTRTRIM`"]
pub type SIMOBUCKUVLOCNTRTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKUVLOCNTRTRIM`"]
pub struct SIMOBUCKUVLOCNTRTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKUVLOCNTRTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKZXTRIM`"]
pub type SIMOBUCKZXTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKZXTRIM`"]
pub struct SIMOBUCKZXTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKZXTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKMEMLEAKAGETRIM`"]
pub type SIMOBUCKMEMLEAKAGETRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKMEMLEAKAGETRIM`"]
pub struct SIMOBUCKMEMLEAKAGETRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKMEMLEAKAGETRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKMEMLPDRVSTRTRIM`"]
pub type SIMOBUCKMEMLPDRVSTRTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKMEMLPDRVSTRTRIM`"]
pub struct SIMOBUCKMEMLPDRVSTRTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKMEMLPDRVSTRTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKMEMACTDRVSTRTRIM`"]
pub type SIMOBUCKMEMACTDRVSTRTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKMEMACTDRVSTRTRIM`"]
pub struct SIMOBUCKMEMACTDRVSTRTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKMEMACTDRVSTRTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKMEMLPLOWTONTRIM`"]
pub type SIMOBUCKMEMLPLOWTONTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKMEMLPLOWTONTRIM`"]
pub struct SIMOBUCKMEMLPLOWTONTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKMEMLPLOWTONTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - simobuck_bias_trim"]
    #[inline(always)]
    pub fn simobuckibiastrim(&self) -> SIMOBUCKIBIASTRIM_R {
        SIMOBUCKIBIASTRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27 - simobuck_uvlo_mode"]
    #[inline(always)]
    pub fn simobuckuvlomode(&self) -> SIMOBUCKUVLOMODE_R {
        SIMOBUCKUVLOMODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 25 - simobuck_priority_sel"]
    #[inline(always)]
    pub fn simobuckprioritysel(&self) -> SIMOBUCKPRIORITYSEL_R {
        SIMOBUCKPRIORITYSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - simobuck_comp2_timeout_en"]
    #[inline(always)]
    pub fn simobuckcomp2timeouten(&self) -> SIMOBUCKCOMP2TIMEOUTEN_R {
        SIMOBUCKCOMP2TIMEOUTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - simobuck_comp2_lp_en"]
    #[inline(always)]
    pub fn simobuckcomp2lpen(&self) -> SIMOBUCKCOMP2LPEN_R {
        SIMOBUCKCOMP2LPEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - simobuck_clkdiv_sel"]
    #[inline(always)]
    pub fn simobuckclkdivsel(&self) -> SIMOBUCKCLKDIVSEL_R {
        SIMOBUCKCLKDIVSEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - simobuck_extclk_sel"]
    #[inline(always)]
    pub fn simobuckextclksel(&self) -> SIMOBUCKEXTCLKSEL_R {
        SIMOBUCKEXTCLKSEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - simobuck_uvlo_drvstr_trim"]
    #[inline(always)]
    pub fn simobuckuvlodrvstrtrim(&self) -> SIMOBUCKUVLODRVSTRTRIM_R {
        SIMOBUCKUVLODRVSTRTRIM_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - simobuck_uvlo_cntr_trim"]
    #[inline(always)]
    pub fn simobuckuvlocntrtrim(&self) -> SIMOBUCKUVLOCNTRTRIM_R {
        SIMOBUCKUVLOCNTRTRIM_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 10:13 - simobuck_zx_trim"]
    #[inline(always)]
    pub fn simobuckzxtrim(&self) -> SIMOBUCKZXTRIM_R {
        SIMOBUCKZXTRIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - simobuck_mem_leakage_trim"]
    #[inline(always)]
    pub fn simobuckmemleakagetrim(&self) -> SIMOBUCKMEMLEAKAGETRIM_R {
        SIMOBUCKMEMLEAKAGETRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - simobuck_mem_lp_drvstr_trim"]
    #[inline(always)]
    pub fn simobuckmemlpdrvstrtrim(&self) -> SIMOBUCKMEMLPDRVSTRTRIM_R {
        SIMOBUCKMEMLPDRVSTRTRIM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - simobuck_mem_act_drvstr_trim"]
    #[inline(always)]
    pub fn simobuckmemactdrvstrtrim(&self) -> SIMOBUCKMEMACTDRVSTRTRIM_R {
        SIMOBUCKMEMACTDRVSTRTRIM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - simobuck_mem_lp_low_ton_trim"]
    #[inline(always)]
    pub fn simobuckmemlplowtontrim(&self) -> SIMOBUCKMEMLPLOWTONTRIM_R {
        SIMOBUCKMEMLPLOWTONTRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - simobuck_bias_trim"]
    #[inline(always)]
    pub fn simobuckibiastrim(&mut self) -> SIMOBUCKIBIASTRIM_W {
        SIMOBUCKIBIASTRIM_W { w: self }
    }
    #[doc = "Bits 26:27 - simobuck_uvlo_mode"]
    #[inline(always)]
    pub fn simobuckuvlomode(&mut self) -> SIMOBUCKUVLOMODE_W {
        SIMOBUCKUVLOMODE_W { w: self }
    }
    #[doc = "Bit 25 - simobuck_priority_sel"]
    #[inline(always)]
    pub fn simobuckprioritysel(&mut self) -> SIMOBUCKPRIORITYSEL_W {
        SIMOBUCKPRIORITYSEL_W { w: self }
    }
    #[doc = "Bit 24 - simobuck_comp2_timeout_en"]
    #[inline(always)]
    pub fn simobuckcomp2timeouten(&mut self) -> SIMOBUCKCOMP2TIMEOUTEN_W {
        SIMOBUCKCOMP2TIMEOUTEN_W { w: self }
    }
    #[doc = "Bit 23 - simobuck_comp2_lp_en"]
    #[inline(always)]
    pub fn simobuckcomp2lpen(&mut self) -> SIMOBUCKCOMP2LPEN_W {
        SIMOBUCKCOMP2LPEN_W { w: self }
    }
    #[doc = "Bits 21:22 - simobuck_clkdiv_sel"]
    #[inline(always)]
    pub fn simobuckclkdivsel(&mut self) -> SIMOBUCKCLKDIVSEL_W {
        SIMOBUCKCLKDIVSEL_W { w: self }
    }
    #[doc = "Bit 20 - simobuck_extclk_sel"]
    #[inline(always)]
    pub fn simobuckextclksel(&mut self) -> SIMOBUCKEXTCLKSEL_W {
        SIMOBUCKEXTCLKSEL_W { w: self }
    }
    #[doc = "Bits 17:19 - simobuck_uvlo_drvstr_trim"]
    #[inline(always)]
    pub fn simobuckuvlodrvstrtrim(&mut self) -> SIMOBUCKUVLODRVSTRTRIM_W {
        SIMOBUCKUVLODRVSTRTRIM_W { w: self }
    }
    #[doc = "Bits 14:16 - simobuck_uvlo_cntr_trim"]
    #[inline(always)]
    pub fn simobuckuvlocntrtrim(&mut self) -> SIMOBUCKUVLOCNTRTRIM_W {
        SIMOBUCKUVLOCNTRTRIM_W { w: self }
    }
    #[doc = "Bits 10:13 - simobuck_zx_trim"]
    #[inline(always)]
    pub fn simobuckzxtrim(&mut self) -> SIMOBUCKZXTRIM_W {
        SIMOBUCKZXTRIM_W { w: self }
    }
    #[doc = "Bits 8:9 - simobuck_mem_leakage_trim"]
    #[inline(always)]
    pub fn simobuckmemleakagetrim(&mut self) -> SIMOBUCKMEMLEAKAGETRIM_W {
        SIMOBUCKMEMLEAKAGETRIM_W { w: self }
    }
    #[doc = "Bits 6:7 - simobuck_mem_lp_drvstr_trim"]
    #[inline(always)]
    pub fn simobuckmemlpdrvstrtrim(&mut self) -> SIMOBUCKMEMLPDRVSTRTRIM_W {
        SIMOBUCKMEMLPDRVSTRTRIM_W { w: self }
    }
    #[doc = "Bits 4:5 - simobuck_mem_act_drvstr_trim"]
    #[inline(always)]
    pub fn simobuckmemactdrvstrtrim(&mut self) -> SIMOBUCKMEMACTDRVSTRTRIM_W {
        SIMOBUCKMEMACTDRVSTRTRIM_W { w: self }
    }
    #[doc = "Bits 0:3 - simobuck_mem_lp_low_ton_trim"]
    #[inline(always)]
    pub fn simobuckmemlplowtontrim(&mut self) -> SIMOBUCKMEMLPLOWTONTRIM_W {
        SIMOBUCKMEMLPLOWTONTRIM_W { w: self }
    }
}
