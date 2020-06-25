#[doc = "Reader of register XTALGENCTRL"]
pub type R = crate::R<u32, super::XTALGENCTRL>;
#[doc = "Writer for register XTALGENCTRL"]
pub type W = crate::W<u32, super::XTALGENCTRL>;
#[doc = "Register XTALGENCTRL `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::XTALGENCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `XTALKSBIASTRIM`"]
pub type XTALKSBIASTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTALKSBIASTRIM`"]
pub struct XTALKSBIASTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALKSBIASTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `XTALBIASTRIM`"]
pub type XTALBIASTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTALBIASTRIM`"]
pub struct XTALBIASTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALBIASTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Auto-calibration delay control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACWARMUP_A {
    #[doc = "0: Warmup period of 1-2 seconds value."]
    SEC1 = 0,
    #[doc = "1: Warmup period of 2-4 seconds value."]
    SEC2 = 1,
    #[doc = "2: Warmup period of 4-8 seconds value."]
    SEC4 = 2,
    #[doc = "3: Warmup period of 8-16 seconds value."]
    SEC8 = 3,
}
impl From<ACWARMUP_A> for u8 {
    #[inline(always)]
    fn from(variant: ACWARMUP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACWARMUP`"]
pub type ACWARMUP_R = crate::R<u8, ACWARMUP_A>;
impl ACWARMUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACWARMUP_A {
        match self.bits {
            0 => ACWARMUP_A::SEC1,
            1 => ACWARMUP_A::SEC2,
            2 => ACWARMUP_A::SEC4,
            3 => ACWARMUP_A::SEC8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEC1`"]
    #[inline(always)]
    pub fn is_sec1(&self) -> bool {
        *self == ACWARMUP_A::SEC1
    }
    #[doc = "Checks if the value of the field is `SEC2`"]
    #[inline(always)]
    pub fn is_sec2(&self) -> bool {
        *self == ACWARMUP_A::SEC2
    }
    #[doc = "Checks if the value of the field is `SEC4`"]
    #[inline(always)]
    pub fn is_sec4(&self) -> bool {
        *self == ACWARMUP_A::SEC4
    }
    #[doc = "Checks if the value of the field is `SEC8`"]
    #[inline(always)]
    pub fn is_sec8(&self) -> bool {
        *self == ACWARMUP_A::SEC8
    }
}
#[doc = "Write proxy for field `ACWARMUP`"]
pub struct ACWARMUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACWARMUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACWARMUP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Warmup period of 1-2 seconds value."]
    #[inline(always)]
    pub fn sec1(self) -> &'a mut W {
        self.variant(ACWARMUP_A::SEC1)
    }
    #[doc = "Warmup period of 2-4 seconds value."]
    #[inline(always)]
    pub fn sec2(self) -> &'a mut W {
        self.variant(ACWARMUP_A::SEC2)
    }
    #[doc = "Warmup period of 4-8 seconds value."]
    #[inline(always)]
    pub fn sec4(self) -> &'a mut W {
        self.variant(ACWARMUP_A::SEC4)
    }
    #[doc = "Warmup period of 8-16 seconds value."]
    #[inline(always)]
    pub fn sec8(self) -> &'a mut W {
        self.variant(ACWARMUP_A::SEC8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline(always)]
    pub fn xtalksbiastrim(&self) -> XTALKSBIASTRIM_R {
        XTALKSBIASTRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 2:7 - XTAL BIAS trim"]
    #[inline(always)]
    pub fn xtalbiastrim(&self) -> XTALBIASTRIM_R {
        XTALBIASTRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline(always)]
    pub fn acwarmup(&self) -> ACWARMUP_R {
        ACWARMUP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline(always)]
    pub fn xtalksbiastrim(&mut self) -> XTALKSBIASTRIM_W {
        XTALKSBIASTRIM_W { w: self }
    }
    #[doc = "Bits 2:7 - XTAL BIAS trim"]
    #[inline(always)]
    pub fn xtalbiastrim(&mut self) -> XTALBIASTRIM_W {
        XTALBIASTRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline(always)]
    pub fn acwarmup(&mut self) -> ACWARMUP_W {
        ACWARMUP_W { w: self }
    }
}
