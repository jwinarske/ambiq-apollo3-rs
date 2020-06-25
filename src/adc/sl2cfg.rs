#[doc = "Reader of register SL2CFG"]
pub type R = crate::R<u32, super::SL2CFG>;
#[doc = "Writer for register SL2CFG"]
pub type W = crate::W<u32, super::SL2CFG>;
#[doc = "Register SL2CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SL2CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADSEL2_A {
    #[doc = "0: Average in 1 measurement in the accumulate divide module for this slot. value."]
    AVG_1_MSRMT = 0,
    #[doc = "1: Average in 2 measurements in the accumulate divide module for this slot. value."]
    AVG_2_MSRMTS = 1,
    #[doc = "2: Average in 4 measurements in the accumulate divide module for this slot. value."]
    AVG_4_MSRMTS = 2,
    #[doc = "3: Average in 8 measurements in the accumulate divide module for this slot. value."]
    AVG_8_MSRMT = 3,
    #[doc = "4: Average in 16 measurements in the accumulate divide module for this slot. value."]
    AVG_16_MSRMTS = 4,
    #[doc = "5: Average in 32 measurements in the accumulate divide module for this slot. value."]
    AVG_32_MSRMTS = 5,
    #[doc = "6: Average in 64 measurements in the accumulate divide module for this slot. value."]
    AVG_64_MSRMTS = 6,
    #[doc = "7: Average in 128 measurements in the accumulate divide module for this slot. value."]
    AVG_128_MSRMTS = 7,
}
impl From<ADSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADSEL2`"]
pub type ADSEL2_R = crate::R<u8, ADSEL2_A>;
impl ADSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSEL2_A {
        match self.bits {
            0 => ADSEL2_A::AVG_1_MSRMT,
            1 => ADSEL2_A::AVG_2_MSRMTS,
            2 => ADSEL2_A::AVG_4_MSRMTS,
            3 => ADSEL2_A::AVG_8_MSRMT,
            4 => ADSEL2_A::AVG_16_MSRMTS,
            5 => ADSEL2_A::AVG_32_MSRMTS,
            6 => ADSEL2_A::AVG_64_MSRMTS,
            7 => ADSEL2_A::AVG_128_MSRMTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVG_1_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == ADSEL2_A::AVG_1_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_2_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_2_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_4_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_4_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_8_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == ADSEL2_A::AVG_8_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_16_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_16_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_32_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_32_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_64_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_64_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_128_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_128_MSRMTS
    }
}
#[doc = "Write proxy for field `ADSEL2`"]
pub struct ADSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSEL2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_1_MSRMT)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_2_MSRMTS)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_4_MSRMTS)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_8_MSRMT)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_16_MSRMTS)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_32_MSRMTS)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_64_MSRMTS)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_128_MSRMTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Set the Precision Mode For Slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRMODE2_A {
    #[doc = "0: 14-bit precision mode value."]
    P14B = 0,
    #[doc = "1: 12-bit precision mode value."]
    P12B = 1,
    #[doc = "2: 10-bit precision mode value."]
    P10B = 2,
    #[doc = "3: 8-bit precision mode value."]
    P8B = 3,
}
impl From<PRMODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMODE2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRMODE2`"]
pub type PRMODE2_R = crate::R<u8, PRMODE2_A>;
impl PRMODE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRMODE2_A {
        match self.bits {
            0 => PRMODE2_A::P14B,
            1 => PRMODE2_A::P12B,
            2 => PRMODE2_A::P10B,
            3 => PRMODE2_A::P8B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P14B`"]
    #[inline(always)]
    pub fn is_p14b(&self) -> bool {
        *self == PRMODE2_A::P14B
    }
    #[doc = "Checks if the value of the field is `P12B`"]
    #[inline(always)]
    pub fn is_p12b(&self) -> bool {
        *self == PRMODE2_A::P12B
    }
    #[doc = "Checks if the value of the field is `P10B`"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == PRMODE2_A::P10B
    }
    #[doc = "Checks if the value of the field is `P8B`"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == PRMODE2_A::P8B
    }
}
#[doc = "Write proxy for field `PRMODE2`"]
pub struct PRMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRMODE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRMODE2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "14-bit precision mode value."]
    #[inline(always)]
    pub fn p14b(self) -> &'a mut W {
        self.variant(PRMODE2_A::P14B)
    }
    #[doc = "12-bit precision mode value."]
    #[inline(always)]
    pub fn p12b(self) -> &'a mut W {
        self.variant(PRMODE2_A::P12B)
    }
    #[doc = "10-bit precision mode value."]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut W {
        self.variant(PRMODE2_A::P10B)
    }
    #[doc = "8-bit precision mode value."]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut W {
        self.variant(PRMODE2_A::P8B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Select one of the 14 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSEL2_A {
    #[doc = "0: single ended external GPIO connection to pad16. value."]
    SE0 = 0,
    #[doc = "1: single ended external GPIO connection to pad29. value."]
    SE1 = 1,
    #[doc = "2: single ended external GPIO connection to pad11. value."]
    SE2 = 2,
    #[doc = "3: single ended external GPIO connection to pad31. value."]
    SE3 = 3,
    #[doc = "4: single ended external GPIO connection to pad32. value."]
    SE4 = 4,
    #[doc = "5: single ended external GPIO connection to pad33. value."]
    SE5 = 5,
    #[doc = "6: single ended external GPIO connection to pad34. value."]
    SE6 = 6,
    #[doc = "7: single ended external GPIO connection to pad35. value."]
    SE7 = 7,
    #[doc = "8: single ended external GPIO connection to pad13. value."]
    SE8 = 8,
    #[doc = "9: single ended external GPIO connection to pad12. value."]
    SE9 = 9,
    #[doc = "10: differential external GPIO connections to pad12(N) and pad13(P). value."]
    DF0 = 10,
    #[doc = "11: differential external GPIO connections to pad15(N) and pad14(P). value."]
    DF1 = 11,
    #[doc = "12: internal temperature sensor. value."]
    TEMP = 12,
    #[doc = "13: internal voltage divide-by-3 connection. value."]
    BATT = 13,
    #[doc = "14: Input VSS value."]
    VSS = 14,
}
impl From<CHSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHSEL2`"]
pub type CHSEL2_R = crate::R<u8, CHSEL2_A>;
impl CHSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHSEL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHSEL2_A::SE0),
            1 => Val(CHSEL2_A::SE1),
            2 => Val(CHSEL2_A::SE2),
            3 => Val(CHSEL2_A::SE3),
            4 => Val(CHSEL2_A::SE4),
            5 => Val(CHSEL2_A::SE5),
            6 => Val(CHSEL2_A::SE6),
            7 => Val(CHSEL2_A::SE7),
            8 => Val(CHSEL2_A::SE8),
            9 => Val(CHSEL2_A::SE9),
            10 => Val(CHSEL2_A::DF0),
            11 => Val(CHSEL2_A::DF1),
            12 => Val(CHSEL2_A::TEMP),
            13 => Val(CHSEL2_A::BATT),
            14 => Val(CHSEL2_A::VSS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == CHSEL2_A::SE0
    }
    #[doc = "Checks if the value of the field is `SE1`"]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == CHSEL2_A::SE1
    }
    #[doc = "Checks if the value of the field is `SE2`"]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == CHSEL2_A::SE2
    }
    #[doc = "Checks if the value of the field is `SE3`"]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == CHSEL2_A::SE3
    }
    #[doc = "Checks if the value of the field is `SE4`"]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == CHSEL2_A::SE4
    }
    #[doc = "Checks if the value of the field is `SE5`"]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == CHSEL2_A::SE5
    }
    #[doc = "Checks if the value of the field is `SE6`"]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == CHSEL2_A::SE6
    }
    #[doc = "Checks if the value of the field is `SE7`"]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == CHSEL2_A::SE7
    }
    #[doc = "Checks if the value of the field is `SE8`"]
    #[inline(always)]
    pub fn is_se8(&self) -> bool {
        *self == CHSEL2_A::SE8
    }
    #[doc = "Checks if the value of the field is `SE9`"]
    #[inline(always)]
    pub fn is_se9(&self) -> bool {
        *self == CHSEL2_A::SE9
    }
    #[doc = "Checks if the value of the field is `DF0`"]
    #[inline(always)]
    pub fn is_df0(&self) -> bool {
        *self == CHSEL2_A::DF0
    }
    #[doc = "Checks if the value of the field is `DF1`"]
    #[inline(always)]
    pub fn is_df1(&self) -> bool {
        *self == CHSEL2_A::DF1
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == CHSEL2_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BATT`"]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == CHSEL2_A::BATT
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == CHSEL2_A::VSS
    }
}
#[doc = "Write proxy for field `CHSEL2`"]
pub struct CHSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "single ended external GPIO connection to pad16. value."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE0)
    }
    #[doc = "single ended external GPIO connection to pad29. value."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE1)
    }
    #[doc = "single ended external GPIO connection to pad11. value."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE2)
    }
    #[doc = "single ended external GPIO connection to pad31. value."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE3)
    }
    #[doc = "single ended external GPIO connection to pad32. value."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE4)
    }
    #[doc = "single ended external GPIO connection to pad33. value."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE5)
    }
    #[doc = "single ended external GPIO connection to pad34. value."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE6)
    }
    #[doc = "single ended external GPIO connection to pad35. value."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE7)
    }
    #[doc = "single ended external GPIO connection to pad13. value."]
    #[inline(always)]
    pub fn se8(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE8)
    }
    #[doc = "single ended external GPIO connection to pad12. value."]
    #[inline(always)]
    pub fn se9(self) -> &'a mut W {
        self.variant(CHSEL2_A::SE9)
    }
    #[doc = "differential external GPIO connections to pad12(N) and pad13(P). value."]
    #[inline(always)]
    pub fn df0(self) -> &'a mut W {
        self.variant(CHSEL2_A::DF0)
    }
    #[doc = "differential external GPIO connections to pad15(N) and pad14(P). value."]
    #[inline(always)]
    pub fn df1(self) -> &'a mut W {
        self.variant(CHSEL2_A::DF1)
    }
    #[doc = "internal temperature sensor. value."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(CHSEL2_A::TEMP)
    }
    #[doc = "internal voltage divide-by-3 connection. value."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut W {
        self.variant(CHSEL2_A::BATT)
    }
    #[doc = "Input VSS value."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(CHSEL2_A::VSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "This bit enables the window compare function for slot 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEN2_A {
    #[doc = "1: Enable the window compare for slot 2. value."]
    WCEN = 1,
}
impl From<WCEN2_A> for bool {
    #[inline(always)]
    fn from(variant: WCEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCEN2`"]
pub type WCEN2_R = crate::R<bool, WCEN2_A>;
impl WCEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WCEN2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WCEN2_A::WCEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCEN`"]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == WCEN2_A::WCEN
    }
}
#[doc = "Write proxy for field `WCEN2`"]
pub struct WCEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> WCEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the window compare for slot 2. value."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut W {
        self.variant(WCEN2_A::WCEN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "This bit enables slot 2 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEN2_A {
    #[doc = "1: Enable slot 2 for ADC conversions. value."]
    SLEN = 1,
}
impl From<SLEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SLEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEN2`"]
pub type SLEN2_R = crate::R<bool, SLEN2_A>;
impl SLEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SLEN2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SLEN2_A::SLEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLEN`"]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == SLEN2_A::SLEN
    }
}
#[doc = "Write proxy for field `SLEN2`"]
pub struct SLEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable slot 2 for ADC conversions. value."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut W {
        self.variant(SLEN2_A::SLEN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel2(&self) -> ADSEL2_R {
        ADSEL2_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline(always)]
    pub fn prmode2(&self) -> PRMODE2_R {
        PRMODE2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 2."]
    #[inline(always)]
    pub fn wcen2(&self) -> WCEN2_R {
        WCEN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit enables slot 2 for ADC conversions."]
    #[inline(always)]
    pub fn slen2(&self) -> SLEN2_R {
        SLEN2_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel2(&mut self) -> ADSEL2_W {
        ADSEL2_W { w: self }
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline(always)]
    pub fn prmode2(&mut self) -> PRMODE2_W {
        PRMODE2_W { w: self }
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W {
        CHSEL2_W { w: self }
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 2."]
    #[inline(always)]
    pub fn wcen2(&mut self) -> WCEN2_W {
        WCEN2_W { w: self }
    }
    #[doc = "Bit 0 - This bit enables slot 2 for ADC conversions."]
    #[inline(always)]
    pub fn slen2(&mut self) -> SLEN2_W {
        SLEN2_W { w: self }
    }
}
