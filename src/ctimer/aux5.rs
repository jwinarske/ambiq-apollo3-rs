#[doc = "Reader of register AUX5"]
pub type R = crate::R<u32, super::AUX5>;
#[doc = "Writer for register AUX5"]
pub type W = crate::W<u32, super::AUX5>;
#[doc = "Register AUX5 `reset()`'s with value 0"]
impl crate::ResetValue for super::AUX5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer B5 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRB5EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5EN23`"]
pub type TMRB5EN23_R = crate::R<bool, TMRB5EN23_A>;
impl TMRB5EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5EN23_A {
        match self.bits {
            true => TMRB5EN23_A::DIS,
            false => TMRB5EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB5EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRB5EN23`"]
pub struct TMRB5EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5EN23_A::EN)
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
pub enum TMRB5POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRB5POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5POL23`"]
pub type TMRB5POL23_R = crate::R<bool, TMRB5POL23_A>;
impl TMRB5POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5POL23_A {
        match self.bits {
            false => TMRB5POL23_A::NORM,
            true => TMRB5POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == TMRB5POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRB5POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRB5POL23`"]
pub struct TMRB5POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB5POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB5POL23_A::INV)
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
#[doc = "Counter/Timer B5 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRB5TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5TINV`"]
pub type TMRB5TINV_R = crate::R<bool, TMRB5TINV_A>;
impl TMRB5TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5TINV_A {
        match self.bits {
            false => TMRB5TINV_A::DIS,
            true => TMRB5TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB5TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRB5TINV`"]
pub struct TMRB5TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5TINV_A::EN)
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
pub enum TMRB5NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRB5NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5NOSYNC`"]
pub type TMRB5NOSYNC_R = crate::R<bool, TMRB5NOSYNC_A>;
impl TMRB5NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5NOSYNC_A {
        match self.bits {
            false => TMRB5NOSYNC_A::DIS,
            true => TMRB5NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB5NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRB5NOSYNC`"]
pub struct TMRB5NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB5NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer B5 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB5TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA5 OUT. value."]
    A5OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA6 OUT. value."]
    A6OUT = 4,
    #[doc = "5: Trigger source is CTIMERB6 OUT. value."]
    B6OUT = 5,
    #[doc = "6: Trigger source is CTIMERA1 OUT. value."]
    A1OUT = 6,
    #[doc = "7: Trigger source is CTIMERB1 OUT. value."]
    B1OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA0 OUT2. value."]
    A0OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB0 OUT2. value."]
    B0OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL = 15,
}
impl From<TMRB5TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB5TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB5TRIG`"]
pub type TMRB5TRIG_R = crate::R<u8, TMRB5TRIG_A>;
impl TMRB5TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5TRIG_A {
        match self.bits {
            0 => TMRB5TRIG_A::DIS,
            1 => TMRB5TRIG_A::A5OUT,
            2 => TMRB5TRIG_A::B3OUT,
            3 => TMRB5TRIG_A::A3OUT,
            4 => TMRB5TRIG_A::A6OUT,
            5 => TMRB5TRIG_A::B6OUT,
            6 => TMRB5TRIG_A::A1OUT,
            7 => TMRB5TRIG_A::B1OUT,
            8 => TMRB5TRIG_A::B3OUT2,
            9 => TMRB5TRIG_A::A3OUT2,
            10 => TMRB5TRIG_A::A0OUT2,
            11 => TMRB5TRIG_A::B0OUT2,
            12 => TMRB5TRIG_A::A6OUT2DUAL,
            13 => TMRB5TRIG_A::A7OUT2DUAL,
            14 => TMRB5TRIG_A::B4OUT2DUAL,
            15 => TMRB5TRIG_A::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        *self == TMRB5TRIG_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB5TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB5TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        *self == TMRB5TRIG_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        *self == TMRB5TRIG_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == TMRB5TRIG_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        *self == TMRB5TRIG_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB5TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB5TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline(always)]
    pub fn is_a0out2(&self) -> bool {
        *self == TMRB5TRIG_A::A0OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline(always)]
    pub fn is_b0out2(&self) -> bool {
        *self == TMRB5TRIG_A::B0OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB5TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB5TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRB5TRIG_A::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRB5TRIG_A::A4OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRB5TRIG`"]
pub struct TMRB5TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::B6OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    #[inline(always)]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A0OUT2)
    }
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    #[inline(always)]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::B0OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRB5TRIG_A::A4OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `TMRB5LMT`"]
pub type TMRB5LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRB5LMT`"]
pub struct TMRB5LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A5 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRA5EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5EN23`"]
pub type TMRA5EN23_R = crate::R<bool, TMRA5EN23_A>;
impl TMRA5EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5EN23_A {
        match self.bits {
            true => TMRA5EN23_A::DIS,
            false => TMRA5EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA5EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRA5EN23`"]
pub struct TMRA5EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5EN23_A::EN)
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
#[doc = "Counter/Timer A5 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORMAL = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRA5POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5POL23`"]
pub type TMRA5POL23_R = crate::R<bool, TMRA5POL23_A>;
impl TMRA5POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5POL23_A {
        match self.bits {
            false => TMRA5POL23_A::NORMAL,
            true => TMRA5POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRA5POL23_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRA5POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRA5POL23`"]
pub struct TMRA5POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA5POL23_A::NORMAL)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA5POL23_A::INV)
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
#[doc = "Counter/Timer A5 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRA5TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5TINV`"]
pub type TMRA5TINV_R = crate::R<bool, TMRA5TINV_A>;
impl TMRA5TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5TINV_A {
        match self.bits {
            false => TMRA5TINV_A::DIS,
            true => TMRA5TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA5TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRA5TINV`"]
pub struct TMRA5TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5TINV_A::EN)
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
pub enum TMRA5NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRA5NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5NOSYNC`"]
pub type TMRA5NOSYNC_R = crate::R<bool, TMRA5NOSYNC_A>;
impl TMRA5NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5NOSYNC_A {
        match self.bits {
            false => TMRA5NOSYNC_A::DIS,
            true => TMRA5NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA5NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRA5NOSYNC`"]
pub struct TMRA5NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA5NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer A5 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA5TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERB5 OUT. value."]
    B5OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 4,
    #[doc = "5: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 5,
    #[doc = "6: Trigger source is CTIMERA2 OUT. value."]
    A2OUT = 6,
    #[doc = "7: Trigger source is CTIMERB2 OUT. value."]
    B2OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA0 OUT2. value."]
    A0OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB0 OUT2. value."]
    B0OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL = 15,
}
impl From<TMRA5TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA5TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA5TRIG`"]
pub type TMRA5TRIG_R = crate::R<u8, TMRA5TRIG_A>;
impl TMRA5TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5TRIG_A {
        match self.bits {
            0 => TMRA5TRIG_A::DIS,
            1 => TMRA5TRIG_A::B5OUT,
            2 => TMRA5TRIG_A::B3OUT,
            3 => TMRA5TRIG_A::A3OUT,
            4 => TMRA5TRIG_A::A4OUT,
            5 => TMRA5TRIG_A::B4OUT,
            6 => TMRA5TRIG_A::A2OUT,
            7 => TMRA5TRIG_A::B2OUT,
            8 => TMRA5TRIG_A::B3OUT2,
            9 => TMRA5TRIG_A::A3OUT2,
            10 => TMRA5TRIG_A::A0OUT2,
            11 => TMRA5TRIG_A::B0OUT2,
            12 => TMRA5TRIG_A::A6OUT2DUAL,
            13 => TMRA5TRIG_A::A7OUT2DUAL,
            14 => TMRA5TRIG_A::B4OUT2DUAL,
            15 => TMRA5TRIG_A::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        *self == TMRA5TRIG_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA5TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA5TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        *self == TMRA5TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        *self == TMRA5TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        *self == TMRA5TRIG_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        *self == TMRA5TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA5TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA5TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline(always)]
    pub fn is_a0out2(&self) -> bool {
        *self == TMRA5TRIG_A::A0OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline(always)]
    pub fn is_b0out2(&self) -> bool {
        *self == TMRA5TRIG_A::B0OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA5TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA5TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRA5TRIG_A::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRA5TRIG_A::A4OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRA5TRIG`"]
pub struct TMRA5TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::B5OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::A2OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    #[inline(always)]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::A0OUT2)
    }
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    #[inline(always)]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::B0OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRA5TRIG_A::A4OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `TMRA5LMT`"]
pub type TMRA5LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRA5LMT`"]
pub struct TMRA5LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B5 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb5en23(&self) -> TMRB5EN23_R {
        TMRB5EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb5pol23(&self) -> TMRB5POL23_R {
        TMRB5POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B5 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb5tinv(&self) -> TMRB5TINV_R {
        TMRB5TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb5nosync(&self) -> TMRB5NOSYNC_R {
        TMRB5NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B5 Trigger Select."]
    #[inline(always)]
    pub fn tmrb5trig(&self) -> TMRB5TRIG_R {
        TMRB5TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B5 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb5lmt(&self) -> TMRB5LMT_R {
        TMRB5LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A5 Upper compare enable."]
    #[inline(always)]
    pub fn tmra5en23(&self) -> TMRA5EN23_R {
        TMRA5EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A5 Upper output polarity"]
    #[inline(always)]
    pub fn tmra5pol23(&self) -> TMRA5POL23_R {
        TMRA5POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A5 Invert on trigger."]
    #[inline(always)]
    pub fn tmra5tinv(&self) -> TMRA5TINV_R {
        TMRA5TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra5nosync(&self) -> TMRA5NOSYNC_R {
        TMRA5NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A5 Trigger Select."]
    #[inline(always)]
    pub fn tmra5trig(&self) -> TMRA5TRIG_R {
        TMRA5TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A5 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra5lmt(&self) -> TMRA5LMT_R {
        TMRA5LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B5 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb5en23(&mut self) -> TMRB5EN23_W {
        TMRB5EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb5pol23(&mut self) -> TMRB5POL23_W {
        TMRB5POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B5 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb5tinv(&mut self) -> TMRB5TINV_W {
        TMRB5TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb5nosync(&mut self) -> TMRB5NOSYNC_W {
        TMRB5NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B5 Trigger Select."]
    #[inline(always)]
    pub fn tmrb5trig(&mut self) -> TMRB5TRIG_W {
        TMRB5TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B5 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb5lmt(&mut self) -> TMRB5LMT_W {
        TMRB5LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A5 Upper compare enable."]
    #[inline(always)]
    pub fn tmra5en23(&mut self) -> TMRA5EN23_W {
        TMRA5EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A5 Upper output polarity"]
    #[inline(always)]
    pub fn tmra5pol23(&mut self) -> TMRA5POL23_W {
        TMRA5POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A5 Invert on trigger."]
    #[inline(always)]
    pub fn tmra5tinv(&mut self) -> TMRA5TINV_W {
        TMRA5TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra5nosync(&mut self) -> TMRA5NOSYNC_W {
        TMRA5NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A5 Trigger Select."]
    #[inline(always)]
    pub fn tmra5trig(&mut self) -> TMRA5TRIG_W {
        TMRA5TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A5 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra5lmt(&mut self) -> TMRA5LMT_W {
        TMRA5LMT_W { w: self }
    }
}
