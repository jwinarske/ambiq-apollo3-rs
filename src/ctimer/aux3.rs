#[doc = "Reader of register AUX3"]
pub type R = crate::R<u32, super::AUX3>;
#[doc = "Writer for register AUX3"]
pub type W = crate::W<u32, super::AUX3>;
#[doc = "Register AUX3 `reset()`'s with value 0"]
impl crate::ResetValue for super::AUX3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer B3 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRB3EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3EN23`"]
pub type TMRB3EN23_R = crate::R<bool, TMRB3EN23_A>;
impl TMRB3EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3EN23_A {
        match self.bits {
            true => TMRB3EN23_A::DIS,
            false => TMRB3EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB3EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRB3EN23`"]
pub struct TMRB3EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3EN23_A::EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRB3POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3POL23`"]
pub type TMRB3POL23_R = crate::R<bool, TMRB3POL23_A>;
impl TMRB3POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3POL23_A {
        match self.bits {
            false => TMRB3POL23_A::NORM,
            true => TMRB3POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == TMRB3POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRB3POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRB3POL23`"]
pub struct TMRB3POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB3POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB3POL23_A::INV)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Counter/Timer B3 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRB3TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3TINV`"]
pub type TMRB3TINV_R = crate::R<bool, TMRB3TINV_A>;
impl TMRB3TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3TINV_A {
        match self.bits {
            false => TMRB3TINV_A::DIS,
            true => TMRB3TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB3TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRB3TINV`"]
pub struct TMRB3TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3TINV_A::EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Source clock synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRB3NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3NOSYNC`"]
pub type TMRB3NOSYNC_R = crate::R<bool, TMRB3NOSYNC_A>;
impl TMRB3NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3NOSYNC_A {
        match self.bits {
            false => TMRB3NOSYNC_A::DIS,
            true => TMRB3NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB3NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRB3NOSYNC`"]
pub struct TMRB3NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB3NOSYNC_A::NOSYNC)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Counter/Timer B3 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB3TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 1,
    #[doc = "2: Trigger source is CTIMERB2 OUT. value."]
    B2OUT = 2,
    #[doc = "3: Trigger source is CTIMERA2 OUT. value."]
    A2OUT = 3,
    #[doc = "4: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 4,
    #[doc = "5: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 5,
    #[doc = "6: Trigger source is CTIMERA6 OUT. value."]
    A6OUT = 6,
    #[doc = "7: Trigger source is CTIMERB6 OUT. value."]
    B6OUT = 7,
    #[doc = "8: Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2OUT2DUAL = 15,
}
impl From<TMRB3TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB3TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB3TRIG`"]
pub type TMRB3TRIG_R = crate::R<u8, TMRB3TRIG_A>;
impl TMRB3TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3TRIG_A {
        match self.bits {
            0 => TMRB3TRIG_A::DIS,
            1 => TMRB3TRIG_A::A3OUT,
            2 => TMRB3TRIG_A::B2OUT,
            3 => TMRB3TRIG_A::A2OUT,
            4 => TMRB3TRIG_A::A4OUT,
            5 => TMRB3TRIG_A::B4OUT,
            6 => TMRB3TRIG_A::A6OUT,
            7 => TMRB3TRIG_A::B6OUT,
            8 => TMRB3TRIG_A::B5OUT2,
            9 => TMRB3TRIG_A::A5OUT2,
            10 => TMRB3TRIG_A::A1OUT2,
            11 => TMRB3TRIG_A::B1OUT2,
            12 => TMRB3TRIG_A::A6OUT2DUAL,
            13 => TMRB3TRIG_A::A7OUT2DUAL,
            14 => TMRB3TRIG_A::B2OUT2DUAL,
            15 => TMRB3TRIG_A::A2OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB3TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        *self == TMRB3TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        *self == TMRB3TRIG_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        *self == TMRB3TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        *self == TMRB3TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        *self == TMRB3TRIG_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        *self == TMRB3TRIG_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        *self == TMRB3TRIG_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        *self == TMRB3TRIG_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        *self == TMRB3TRIG_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        *self == TMRB3TRIG_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB3TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB3TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B2OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b2out2dual(&self) -> bool {
        *self == TMRB3TRIG_A::B2OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A2OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a2out2dual(&self) -> bool {
        *self == TMRB3TRIG_A::A2OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRB3TRIG`"]
pub struct TMRB3TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A2OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B6OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b2out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B2OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a2out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A2OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `TMRB3LMT`"]
pub type TMRB3LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRB3LMT`"]
pub struct TMRB3LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A3 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRA3EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3EN23`"]
pub type TMRA3EN23_R = crate::R<bool, TMRA3EN23_A>;
impl TMRA3EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3EN23_A {
        match self.bits {
            true => TMRA3EN23_A::DIS,
            false => TMRA3EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA3EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRA3EN23`"]
pub struct TMRA3EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3EN23_A::EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Counter/Timer A3 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRA3POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3POL23`"]
pub type TMRA3POL23_R = crate::R<bool, TMRA3POL23_A>;
impl TMRA3POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3POL23_A {
        match self.bits {
            false => TMRA3POL23_A::NORM,
            true => TMRA3POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == TMRA3POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRA3POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRA3POL23`"]
pub struct TMRA3POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA3POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA3POL23_A::INV)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Counter/Timer A3 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRA3TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3TINV`"]
pub type TMRA3TINV_R = crate::R<bool, TMRA3TINV_A>;
impl TMRA3TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3TINV_A {
        match self.bits {
            false => TMRA3TINV_A::DIS,
            true => TMRA3TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA3TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRA3TINV`"]
pub struct TMRA3TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3TINV_A::EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Source clock synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRA3NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3NOSYNC`"]
pub type TMRA3NOSYNC_R = crate::R<bool, TMRA3NOSYNC_A>;
impl TMRA3NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3NOSYNC_A {
        match self.bits {
            false => TMRA3NOSYNC_A::DIS,
            true => TMRA3NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA3NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRA3NOSYNC`"]
pub struct TMRA3NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA3NOSYNC_A::NOSYNC)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Counter/Timer A3 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA3TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 1,
    #[doc = "2: Trigger source is CTIMERB2 OUT. value."]
    B2OUT = 2,
    #[doc = "3: Trigger source is CTIMERA2 OUT. value."]
    A2OUT = 3,
    #[doc = "4: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 4,
    #[doc = "5: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 5,
    #[doc = "6: Trigger source is CTIMERA7 OUT. value."]
    A7OUT = 6,
    #[doc = "7: Trigger source is CTIMERB7 OUT. value."]
    B7OUT = 7,
    #[doc = "8: Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2OUT2DUAL = 15,
}
impl From<TMRA3TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA3TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA3TRIG`"]
pub type TMRA3TRIG_R = crate::R<u8, TMRA3TRIG_A>;
impl TMRA3TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3TRIG_A {
        match self.bits {
            0 => TMRA3TRIG_A::DIS,
            1 => TMRA3TRIG_A::B3OUT,
            2 => TMRA3TRIG_A::B2OUT,
            3 => TMRA3TRIG_A::A2OUT,
            4 => TMRA3TRIG_A::A4OUT,
            5 => TMRA3TRIG_A::B4OUT,
            6 => TMRA3TRIG_A::A7OUT,
            7 => TMRA3TRIG_A::B7OUT,
            8 => TMRA3TRIG_A::B5OUT2,
            9 => TMRA3TRIG_A::A5OUT2,
            10 => TMRA3TRIG_A::A1OUT2,
            11 => TMRA3TRIG_A::B1OUT2,
            12 => TMRA3TRIG_A::A6OUT2DUAL,
            13 => TMRA3TRIG_A::A7OUT2DUAL,
            14 => TMRA3TRIG_A::B2OUT2DUAL,
            15 => TMRA3TRIG_A::A2OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA3TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        *self == TMRA3TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        *self == TMRA3TRIG_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        *self == TMRA3TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        *self == TMRA3TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        *self == TMRA3TRIG_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline(always)]
    pub fn is_b7out(&self) -> bool {
        *self == TMRA3TRIG_A::B7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        *self == TMRA3TRIG_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        *self == TMRA3TRIG_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        *self == TMRA3TRIG_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        *self == TMRA3TRIG_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA3TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA3TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B2OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b2out2dual(&self) -> bool {
        *self == TMRA3TRIG_A::B2OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A2OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a2out2dual(&self) -> bool {
        *self == TMRA3TRIG_A::A2OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRA3TRIG`"]
pub struct TMRA3TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A2OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A7OUT)
    }
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    #[inline(always)]
    pub fn b7out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B7OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b2out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B2OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a2out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A2OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `TMRA3LMT`"]
pub type TMRA3LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRA3LMT`"]
pub struct TMRA3LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B3 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb3en23(&self) -> TMRB3EN23_R {
        TMRB3EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb3pol23(&self) -> TMRB3POL23_R {
        TMRB3POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B3 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb3tinv(&self) -> TMRB3TINV_R {
        TMRB3TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb3nosync(&self) -> TMRB3NOSYNC_R {
        TMRB3NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B3 Trigger Select."]
    #[inline(always)]
    pub fn tmrb3trig(&self) -> TMRB3TRIG_R {
        TMRB3TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B3 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb3lmt(&self) -> TMRB3LMT_R {
        TMRB3LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A3 Upper compare enable."]
    #[inline(always)]
    pub fn tmra3en23(&self) -> TMRA3EN23_R {
        TMRA3EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A3 Upper output polarity"]
    #[inline(always)]
    pub fn tmra3pol23(&self) -> TMRA3POL23_R {
        TMRA3POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A3 Invert on trigger."]
    #[inline(always)]
    pub fn tmra3tinv(&self) -> TMRA3TINV_R {
        TMRA3TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra3nosync(&self) -> TMRA3NOSYNC_R {
        TMRA3NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A3 Trigger Select."]
    #[inline(always)]
    pub fn tmra3trig(&self) -> TMRA3TRIG_R {
        TMRA3TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A3 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra3lmt(&self) -> TMRA3LMT_R {
        TMRA3LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B3 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb3en23(&mut self) -> TMRB3EN23_W {
        TMRB3EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb3pol23(&mut self) -> TMRB3POL23_W {
        TMRB3POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B3 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb3tinv(&mut self) -> TMRB3TINV_W {
        TMRB3TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb3nosync(&mut self) -> TMRB3NOSYNC_W {
        TMRB3NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B3 Trigger Select."]
    #[inline(always)]
    pub fn tmrb3trig(&mut self) -> TMRB3TRIG_W {
        TMRB3TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B3 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb3lmt(&mut self) -> TMRB3LMT_W {
        TMRB3LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A3 Upper compare enable."]
    #[inline(always)]
    pub fn tmra3en23(&mut self) -> TMRA3EN23_W {
        TMRA3EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A3 Upper output polarity"]
    #[inline(always)]
    pub fn tmra3pol23(&mut self) -> TMRA3POL23_W {
        TMRA3POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A3 Invert on trigger."]
    #[inline(always)]
    pub fn tmra3tinv(&mut self) -> TMRA3TINV_W {
        TMRA3TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra3nosync(&mut self) -> TMRA3NOSYNC_W {
        TMRA3NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A3 Trigger Select."]
    #[inline(always)]
    pub fn tmra3trig(&mut self) -> TMRA3TRIG_W {
        TMRA3TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A3 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra3lmt(&mut self) -> TMRA3LMT_W {
        TMRA3LMT_W { w: self }
    }
}
