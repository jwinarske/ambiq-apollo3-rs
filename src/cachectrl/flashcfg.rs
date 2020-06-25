#[doc = "Reader of register FLASHCFG"]
pub type R = crate::R<u32, super::FLASHCFG>;
#[doc = "Writer for register FLASHCFG"]
pub type W = crate::W<u32, super::FLASHCFG>;
#[doc = "Register FLASHCFG `reset()`'s with value 0x0873"]
impl crate::ResetValue for super::FLASHCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0873
    }
}
#[doc = "Controls flash low power modes (control of LPM pin).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMMODE_A {
    #[doc = "0: High power mode (LPM not used). value."]
    NEVER = 0,
    #[doc = "1: Fast Standby mode.  LPM deasserted for read operations, but asserted while flash IDLE. value."]
    STANDBY = 1,
    #[doc = "2: Low Power mode.  LPM always asserted for reads.  LPM_RD_WAIT must be programmed to accomodate longer read access times. value."]
    ALWAYS = 2,
}
impl From<LPMMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPMMODE`"]
pub type LPMMODE_R = crate::R<u8, LPMMODE_A>;
impl LPMMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPMMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPMMODE_A::NEVER),
            1 => Val(LPMMODE_A::STANDBY),
            2 => Val(LPMMODE_A::ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == LPMMODE_A::NEVER
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMMODE_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == LPMMODE_A::ALWAYS
    }
}
#[doc = "Write proxy for field `LPMMODE`"]
pub struct LPMMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "High power mode (LPM not used). value."]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(LPMMODE_A::NEVER)
    }
    #[doc = "Fast Standby mode. LPM deasserted for read operations, but asserted while flash IDLE. value."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMMODE_A::STANDBY)
    }
    #[doc = "Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accomodate longer read access times. value."]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(LPMMODE_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `LPM_RD_WAIT`"]
pub type LPM_RD_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_RD_WAIT`"]
pub struct LPM_RD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_RD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEDELAY`"]
pub type SEDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEDELAY`"]
pub struct SEDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SEDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RD_WAIT`"]
pub type RD_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_WAIT`"]
pub struct RD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13 - Controls flash low power modes (control of LPM pin)."]
    #[inline(always)]
    pub fn lpmmode(&self) -> LPMMODE_R {
        LPMMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpm_rd_wait(&self) -> LPM_RD_WAIT_R {
        LPM_RD_WAIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Sets SE delay (flash address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay(&self) -> SEDELAY_R {
        SEDELAY_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rd_wait(&self) -> RD_WAIT_R {
        RD_WAIT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Controls flash low power modes (control of LPM pin)."]
    #[inline(always)]
    pub fn lpmmode(&mut self) -> LPMMODE_W {
        LPMMODE_W { w: self }
    }
    #[doc = "Bits 8:11 - Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpm_rd_wait(&mut self) -> LPM_RD_WAIT_W {
        LPM_RD_WAIT_W { w: self }
    }
    #[doc = "Bits 4:6 - Sets SE delay (flash address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay(&mut self) -> SEDELAY_W {
        SEDELAY_W { w: self }
    }
    #[doc = "Bits 0:3 - Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rd_wait(&mut self) -> RD_WAIT_W {
        RD_WAIT_W { w: self }
    }
}
