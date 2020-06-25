#[doc = "Reader of register AUX2"]
pub type R = crate::R<u32, super::AUX2>;
#[doc = "Writer for register AUX2"]
pub type W = crate::W<u32, super::AUX2>;
#[doc = "Register AUX2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AUX2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer B2 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRB2EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB2EN23`"]
pub type TMRB2EN23_R = crate::R<bool, TMRB2EN23_A>;
impl TMRB2EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2EN23_A {
        match self.bits {
            true => TMRB2EN23_A::DIS,
            false => TMRB2EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB2EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRB2EN23`"]
pub struct TMRB2EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2EN23_A::EN)
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
pub enum TMRB2POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRB2POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB2POL23`"]
pub type TMRB2POL23_R = crate::R<bool, TMRB2POL23_A>;
impl TMRB2POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2POL23_A {
        match self.bits {
            false => TMRB2POL23_A::NORM,
            true => TMRB2POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == TMRB2POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRB2POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRB2POL23`"]
pub struct TMRB2POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB2POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB2POL23_A::INV)
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
#[doc = "Counter/Timer B2 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRB2TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB2TINV`"]
pub type TMRB2TINV_R = crate::R<bool, TMRB2TINV_A>;
impl TMRB2TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2TINV_A {
        match self.bits {
            false => TMRB2TINV_A::DIS,
            true => TMRB2TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB2TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRB2TINV`"]
pub struct TMRB2TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2TINV_A::EN)
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
pub enum TMRB2NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRB2NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB2NOSYNC`"]
pub type TMRB2NOSYNC_R = crate::R<bool, TMRB2NOSYNC_A>;
impl TMRB2NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2NOSYNC_A {
        match self.bits {
            false => TMRB2NOSYNC_A::DIS,
            true => TMRB2NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB2NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRB2NOSYNC`"]
pub struct TMRB2NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB2NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer B2 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB2TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA2 OUT. value."]
    A2OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA1 OUT. value."]
    A1OUT = 4,
    #[doc = "5: Trigger source is CTIMERB1 OUT. value."]
    B1OUT = 5,
    #[doc = "6: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 6,
    #[doc = "7: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL = 15,
}
impl From<TMRB2TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB2TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB2TRIG`"]
pub type TMRB2TRIG_R = crate::R<u8, TMRB2TRIG_A>;
impl TMRB2TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2TRIG_A {
        match self.bits {
            0 => TMRB2TRIG_A::DIS,
            1 => TMRB2TRIG_A::A2OUT,
            2 => TMRB2TRIG_A::B3OUT,
            3 => TMRB2TRIG_A::A3OUT,
            4 => TMRB2TRIG_A::A1OUT,
            5 => TMRB2TRIG_A::B1OUT,
            6 => TMRB2TRIG_A::A4OUT,
            7 => TMRB2TRIG_A::B4OUT,
            8 => TMRB2TRIG_A::B3OUT2,
            9 => TMRB2TRIG_A::A3OUT2,
            10 => TMRB2TRIG_A::A5OUT2,
            11 => TMRB2TRIG_A::B5OUT2,
            12 => TMRB2TRIG_A::A6OUT2DUAL,
            13 => TMRB2TRIG_A::A7OUT2DUAL,
            14 => TMRB2TRIG_A::B4OUT2DUAL,
            15 => TMRB2TRIG_A::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        *self == TMRB2TRIG_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB2TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB2TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == TMRB2TRIG_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        *self == TMRB2TRIG_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        *self == TMRB2TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        *self == TMRB2TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB2TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB2TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        *self == TMRB2TRIG_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        *self == TMRB2TRIG_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB2TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB2TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRB2TRIG_A::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRB2TRIG_A::A4OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRB2TRIG`"]
pub struct TMRB2TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::B1OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRB2TRIG_A::A4OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `TMRB2LMT`"]
pub type TMRB2LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRB2LMT`"]
pub struct TMRB2LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A2 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRA2EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA2EN23`"]
pub type TMRA2EN23_R = crate::R<bool, TMRA2EN23_A>;
impl TMRA2EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2EN23_A {
        match self.bits {
            true => TMRA2EN23_A::DIS,
            false => TMRA2EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA2EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRA2EN23`"]
pub struct TMRA2EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2EN23_A::EN)
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
#[doc = "Counter/Timer A2 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRA2POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA2POL23`"]
pub type TMRA2POL23_R = crate::R<bool, TMRA2POL23_A>;
impl TMRA2POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2POL23_A {
        match self.bits {
            false => TMRA2POL23_A::NORM,
            true => TMRA2POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == TMRA2POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRA2POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRA2POL23`"]
pub struct TMRA2POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA2POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA2POL23_A::INV)
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
#[doc = "Counter/Timer A2 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRA2TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA2TINV`"]
pub type TMRA2TINV_R = crate::R<bool, TMRA2TINV_A>;
impl TMRA2TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2TINV_A {
        match self.bits {
            false => TMRA2TINV_A::DIS,
            true => TMRA2TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA2TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRA2TINV`"]
pub struct TMRA2TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2TINV_A::EN)
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
pub enum TMRA2NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRA2NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA2NOSYNC`"]
pub type TMRA2NOSYNC_R = crate::R<bool, TMRA2NOSYNC_A>;
impl TMRA2NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2NOSYNC_A {
        match self.bits {
            false => TMRA2NOSYNC_A::DIS,
            true => TMRA2NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA2NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRA2NOSYNC`"]
pub struct TMRA2NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA2NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer A2 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA2TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERB2 OUT. value."]
    B2OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA0 OUT. value."]
    A0OUT = 4,
    #[doc = "5: Trigger source is CTIMERB0 OUT. value."]
    B0OUT = 5,
    #[doc = "6: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 6,
    #[doc = "7: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL = 15,
}
impl From<TMRA2TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA2TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA2TRIG`"]
pub type TMRA2TRIG_R = crate::R<u8, TMRA2TRIG_A>;
impl TMRA2TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2TRIG_A {
        match self.bits {
            0 => TMRA2TRIG_A::DIS,
            1 => TMRA2TRIG_A::B2OUT,
            2 => TMRA2TRIG_A::B3OUT,
            3 => TMRA2TRIG_A::A3OUT,
            4 => TMRA2TRIG_A::A0OUT,
            5 => TMRA2TRIG_A::B0OUT,
            6 => TMRA2TRIG_A::A4OUT,
            7 => TMRA2TRIG_A::B4OUT,
            8 => TMRA2TRIG_A::B3OUT2,
            9 => TMRA2TRIG_A::A3OUT2,
            10 => TMRA2TRIG_A::A5OUT2,
            11 => TMRA2TRIG_A::B5OUT2,
            12 => TMRA2TRIG_A::A6OUT2DUAL,
            13 => TMRA2TRIG_A::A7OUT2DUAL,
            14 => TMRA2TRIG_A::B4OUT2DUAL,
            15 => TMRA2TRIG_A::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        *self == TMRA2TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA2TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA2TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline(always)]
    pub fn is_a0out(&self) -> bool {
        *self == TMRA2TRIG_A::A0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline(always)]
    pub fn is_b0out(&self) -> bool {
        *self == TMRA2TRIG_A::B0OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        *self == TMRA2TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        *self == TMRA2TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA2TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA2TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        *self == TMRA2TRIG_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        *self == TMRA2TRIG_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA2TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA2TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRA2TRIG_A::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRA2TRIG_A::A4OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRA2TRIG`"]
pub struct TMRA2TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    #[inline(always)]
    pub fn a0out(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::A0OUT)
    }
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn b0out(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::B0OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRA2TRIG_A::A4OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `TMRA2LMT`"]
pub type TMRA2LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRA2LMT`"]
pub struct TMRA2LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B2 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb2en23(&self) -> TMRB2EN23_R {
        TMRB2EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb2pol23(&self) -> TMRB2POL23_R {
        TMRB2POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B2 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb2tinv(&self) -> TMRB2TINV_R {
        TMRB2TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb2nosync(&self) -> TMRB2NOSYNC_R {
        TMRB2NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B2 Trigger Select."]
    #[inline(always)]
    pub fn tmrb2trig(&self) -> TMRB2TRIG_R {
        TMRB2TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B2 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb2lmt(&self) -> TMRB2LMT_R {
        TMRB2LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A2 Upper compare enable."]
    #[inline(always)]
    pub fn tmra2en23(&self) -> TMRA2EN23_R {
        TMRA2EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A2 Upper output polarity"]
    #[inline(always)]
    pub fn tmra2pol23(&self) -> TMRA2POL23_R {
        TMRA2POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A2 Invert on trigger."]
    #[inline(always)]
    pub fn tmra2tinv(&self) -> TMRA2TINV_R {
        TMRA2TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra2nosync(&self) -> TMRA2NOSYNC_R {
        TMRA2NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A2 Trigger Select."]
    #[inline(always)]
    pub fn tmra2trig(&self) -> TMRA2TRIG_R {
        TMRA2TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A2 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra2lmt(&self) -> TMRA2LMT_R {
        TMRA2LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B2 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb2en23(&mut self) -> TMRB2EN23_W {
        TMRB2EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb2pol23(&mut self) -> TMRB2POL23_W {
        TMRB2POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B2 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb2tinv(&mut self) -> TMRB2TINV_W {
        TMRB2TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb2nosync(&mut self) -> TMRB2NOSYNC_W {
        TMRB2NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B2 Trigger Select."]
    #[inline(always)]
    pub fn tmrb2trig(&mut self) -> TMRB2TRIG_W {
        TMRB2TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B2 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb2lmt(&mut self) -> TMRB2LMT_W {
        TMRB2LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A2 Upper compare enable."]
    #[inline(always)]
    pub fn tmra2en23(&mut self) -> TMRA2EN23_W {
        TMRA2EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A2 Upper output polarity"]
    #[inline(always)]
    pub fn tmra2pol23(&mut self) -> TMRA2POL23_W {
        TMRA2POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A2 Invert on trigger."]
    #[inline(always)]
    pub fn tmra2tinv(&mut self) -> TMRA2TINV_W {
        TMRA2TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra2nosync(&mut self) -> TMRA2NOSYNC_W {
        TMRA2NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A2 Trigger Select."]
    #[inline(always)]
    pub fn tmra2trig(&mut self) -> TMRA2TRIG_W {
        TMRA2TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A2 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra2lmt(&mut self) -> TMRA2LMT_W {
        TMRA2LMT_W { w: self }
    }
}
