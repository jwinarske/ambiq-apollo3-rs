#[doc = "Reader of register SL0CFG"]
pub type R = crate::R<u32, super::SL0CFG>;
#[doc = "Writer for register SL0CFG"]
pub type W = crate::W<u32, super::SL0CFG>;
#[doc = "Register SL0CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SL0CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADSEL0_A {
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
impl From<ADSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADSEL0`"]
pub type ADSEL0_R = crate::R<u8, ADSEL0_A>;
impl ADSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSEL0_A {
        match self.bits {
            0 => ADSEL0_A::AVG_1_MSRMT,
            1 => ADSEL0_A::AVG_2_MSRMTS,
            2 => ADSEL0_A::AVG_4_MSRMTS,
            3 => ADSEL0_A::AVG_8_MSRMT,
            4 => ADSEL0_A::AVG_16_MSRMTS,
            5 => ADSEL0_A::AVG_32_MSRMTS,
            6 => ADSEL0_A::AVG_64_MSRMTS,
            7 => ADSEL0_A::AVG_128_MSRMTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVG_1_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == ADSEL0_A::AVG_1_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_2_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == ADSEL0_A::AVG_2_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_4_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == ADSEL0_A::AVG_4_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_8_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == ADSEL0_A::AVG_8_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_16_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == ADSEL0_A::AVG_16_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_32_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == ADSEL0_A::AVG_32_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_64_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == ADSEL0_A::AVG_64_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_128_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == ADSEL0_A::AVG_128_MSRMTS
    }
}
#[doc = "Write proxy for field `ADSEL0`"]
pub struct ADSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSEL0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut W {
        self.variant(ADSEL0_A::AVG_1_MSRMT)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut W {
        self.variant(ADSEL0_A::AVG_2_MSRMTS)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut W {
        self.variant(ADSEL0_A::AVG_4_MSRMTS)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut W {
        self.variant(ADSEL0_A::AVG_8_MSRMT)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut W {
        self.variant(ADSEL0_A::AVG_16_MSRMTS)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut W {
        self.variant(ADSEL0_A::AVG_32_MSRMTS)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut W {
        self.variant(ADSEL0_A::AVG_64_MSRMTS)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut W {
        self.variant(ADSEL0_A::AVG_128_MSRMTS)
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
pub enum PRMODE0_A {
    #[doc = "0: 14-bit precision mode value."]
    P14B = 0,
    #[doc = "1: 12-bit precision mode value."]
    P12B = 1,
    #[doc = "2: 10-bit precision mode value."]
    P10B = 2,
    #[doc = "3: 8-bit precision mode value."]
    P8B = 3,
}
impl From<PRMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRMODE0`"]
pub type PRMODE0_R = crate::R<u8, PRMODE0_A>;
impl PRMODE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRMODE0_A {
        match self.bits {
            0 => PRMODE0_A::P14B,
            1 => PRMODE0_A::P12B,
            2 => PRMODE0_A::P10B,
            3 => PRMODE0_A::P8B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P14B`"]
    #[inline(always)]
    pub fn is_p14b(&self) -> bool {
        *self == PRMODE0_A::P14B
    }
    #[doc = "Checks if the value of the field is `P12B`"]
    #[inline(always)]
    pub fn is_p12b(&self) -> bool {
        *self == PRMODE0_A::P12B
    }
    #[doc = "Checks if the value of the field is `P10B`"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == PRMODE0_A::P10B
    }
    #[doc = "Checks if the value of the field is `P8B`"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == PRMODE0_A::P8B
    }
}
#[doc = "Write proxy for field `PRMODE0`"]
pub struct PRMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRMODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRMODE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "14-bit precision mode value."]
    #[inline(always)]
    pub fn p14b(self) -> &'a mut W {
        self.variant(PRMODE0_A::P14B)
    }
    #[doc = "12-bit precision mode value."]
    #[inline(always)]
    pub fn p12b(self) -> &'a mut W {
        self.variant(PRMODE0_A::P12B)
    }
    #[doc = "10-bit precision mode value."]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut W {
        self.variant(PRMODE0_A::P10B)
    }
    #[doc = "8-bit precision mode value."]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut W {
        self.variant(PRMODE0_A::P8B)
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
pub enum CHSEL0_A {
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
impl From<CHSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHSEL0`"]
pub type CHSEL0_R = crate::R<u8, CHSEL0_A>;
impl CHSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHSEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHSEL0_A::SE0),
            1 => Val(CHSEL0_A::SE1),
            2 => Val(CHSEL0_A::SE2),
            3 => Val(CHSEL0_A::SE3),
            4 => Val(CHSEL0_A::SE4),
            5 => Val(CHSEL0_A::SE5),
            6 => Val(CHSEL0_A::SE6),
            7 => Val(CHSEL0_A::SE7),
            8 => Val(CHSEL0_A::SE8),
            9 => Val(CHSEL0_A::SE9),
            10 => Val(CHSEL0_A::DF0),
            11 => Val(CHSEL0_A::DF1),
            12 => Val(CHSEL0_A::TEMP),
            13 => Val(CHSEL0_A::BATT),
            14 => Val(CHSEL0_A::VSS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == CHSEL0_A::SE0
    }
    #[doc = "Checks if the value of the field is `SE1`"]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == CHSEL0_A::SE1
    }
    #[doc = "Checks if the value of the field is `SE2`"]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == CHSEL0_A::SE2
    }
    #[doc = "Checks if the value of the field is `SE3`"]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == CHSEL0_A::SE3
    }
    #[doc = "Checks if the value of the field is `SE4`"]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == CHSEL0_A::SE4
    }
    #[doc = "Checks if the value of the field is `SE5`"]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == CHSEL0_A::SE5
    }
    #[doc = "Checks if the value of the field is `SE6`"]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == CHSEL0_A::SE6
    }
    #[doc = "Checks if the value of the field is `SE7`"]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == CHSEL0_A::SE7
    }
    #[doc = "Checks if the value of the field is `SE8`"]
    #[inline(always)]
    pub fn is_se8(&self) -> bool {
        *self == CHSEL0_A::SE8
    }
    #[doc = "Checks if the value of the field is `SE9`"]
    #[inline(always)]
    pub fn is_se9(&self) -> bool {
        *self == CHSEL0_A::SE9
    }
    #[doc = "Checks if the value of the field is `DF0`"]
    #[inline(always)]
    pub fn is_df0(&self) -> bool {
        *self == CHSEL0_A::DF0
    }
    #[doc = "Checks if the value of the field is `DF1`"]
    #[inline(always)]
    pub fn is_df1(&self) -> bool {
        *self == CHSEL0_A::DF1
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == CHSEL0_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BATT`"]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == CHSEL0_A::BATT
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == CHSEL0_A::VSS
    }
}
#[doc = "Write proxy for field `CHSEL0`"]
pub struct CHSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "single ended external GPIO connection to pad16. value."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE0)
    }
    #[doc = "single ended external GPIO connection to pad29. value."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE1)
    }
    #[doc = "single ended external GPIO connection to pad11. value."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE2)
    }
    #[doc = "single ended external GPIO connection to pad31. value."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE3)
    }
    #[doc = "single ended external GPIO connection to pad32. value."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE4)
    }
    #[doc = "single ended external GPIO connection to pad33. value."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE5)
    }
    #[doc = "single ended external GPIO connection to pad34. value."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE6)
    }
    #[doc = "single ended external GPIO connection to pad35. value."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE7)
    }
    #[doc = "single ended external GPIO connection to pad13. value."]
    #[inline(always)]
    pub fn se8(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE8)
    }
    #[doc = "single ended external GPIO connection to pad12. value."]
    #[inline(always)]
    pub fn se9(self) -> &'a mut W {
        self.variant(CHSEL0_A::SE9)
    }
    #[doc = "differential external GPIO connections to pad12(N) and pad13(P). value."]
    #[inline(always)]
    pub fn df0(self) -> &'a mut W {
        self.variant(CHSEL0_A::DF0)
    }
    #[doc = "differential external GPIO connections to pad15(N) and pad14(P). value."]
    #[inline(always)]
    pub fn df1(self) -> &'a mut W {
        self.variant(CHSEL0_A::DF1)
    }
    #[doc = "internal temperature sensor. value."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(CHSEL0_A::TEMP)
    }
    #[doc = "internal voltage divide-by-3 connection. value."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut W {
        self.variant(CHSEL0_A::BATT)
    }
    #[doc = "Input VSS value."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(CHSEL0_A::VSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "This bit enables the window compare function for slot 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEN0_A {
    #[doc = "1: Enable the window compare for slot 0. value."]
    WCEN = 1,
}
impl From<WCEN0_A> for bool {
    #[inline(always)]
    fn from(variant: WCEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCEN0`"]
pub type WCEN0_R = crate::R<bool, WCEN0_A>;
impl WCEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WCEN0_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WCEN0_A::WCEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCEN`"]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == WCEN0_A::WCEN
    }
}
#[doc = "Write proxy for field `WCEN0`"]
pub struct WCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> WCEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the window compare for slot 0. value."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut W {
        self.variant(WCEN0_A::WCEN)
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
#[doc = "This bit enables slot 0 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEN0_A {
    #[doc = "1: Enable slot 0 for ADC conversions. value."]
    SLEN = 1,
}
impl From<SLEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SLEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEN0`"]
pub type SLEN0_R = crate::R<bool, SLEN0_A>;
impl SLEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SLEN0_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SLEN0_A::SLEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLEN`"]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == SLEN0_A::SLEN
    }
}
#[doc = "Write proxy for field `SLEN0`"]
pub struct SLEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable slot 0 for ADC conversions. value."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut W {
        self.variant(SLEN0_A::SLEN)
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
    pub fn adsel0(&self) -> ADSEL0_R {
        ADSEL0_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline(always)]
    pub fn prmode0(&self) -> PRMODE0_R {
        PRMODE0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 0."]
    #[inline(always)]
    pub fn wcen0(&self) -> WCEN0_R {
        WCEN0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit enables slot 0 for ADC conversions."]
    #[inline(always)]
    pub fn slen0(&self) -> SLEN0_R {
        SLEN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel0(&mut self) -> ADSEL0_W {
        ADSEL0_W { w: self }
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline(always)]
    pub fn prmode0(&mut self) -> PRMODE0_W {
        PRMODE0_W { w: self }
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL0_W {
        CHSEL0_W { w: self }
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 0."]
    #[inline(always)]
    pub fn wcen0(&mut self) -> WCEN0_W {
        WCEN0_W { w: self }
    }
    #[doc = "Bit 0 - This bit enables slot 0 for ADC conversions."]
    #[inline(always)]
    pub fn slen0(&mut self) -> SLEN0_W {
        SLEN0_W { w: self }
    }
}
