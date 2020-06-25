#[doc = "Reader of register CTRL5"]
pub type R = crate::R<u32, super::CTRL5>;
#[doc = "Writer for register CTRL5"]
pub type W = crate::W<u32, super::CTRL5>;
#[doc = "Register CTRL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer A5/B5 Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK5_A {
    #[doc = "0: Use A5/B5 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A5/B5 timers into a single 32-bit timer. value."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK5_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTLINK5`"]
pub type CTLINK5_R = crate::R<bool, CTLINK5_A>;
impl CTLINK5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK5_A {
        match self.bits {
            false => CTLINK5_A::TWO_16BIT_TIMERS,
            true => CTLINK5_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK5_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK5_A::_32BIT_TIMER
    }
}
#[doc = "Write proxy for field `CTLINK5`"]
pub struct CTLINK5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use A5/B5 timers as two independent 16-bit timers (default). value."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK5_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A5/B5 timers into a single 32-bit timer. value."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK5_A::_32BIT_TIMER)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Counter/Timer B5 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5POL_A {
    #[doc = "0: The polarity of the TMRPINB5 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB5 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRB5POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5POL`"]
pub type TMRB5POL_R = crate::R<bool, TMRB5POL_A>;
impl TMRB5POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5POL_A {
        match self.bits {
            false => TMRB5POL_A::NORMAL,
            true => TMRB5POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRB5POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB5POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRB5POL`"]
pub struct TMRB5POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINB5 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB5POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB5 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB5POL_A::INVERTED)
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
#[doc = "Counter/Timer B5 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5CLR_A {
    #[doc = "0: Allow counter/timer B5 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer B5 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRB5CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5CLR`"]
pub type TMRB5CLR_R = crate::R<bool, TMRB5CLR_A>;
impl TMRB5CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5CLR_A {
        match self.bits {
            false => TMRB5CLR_A::RUN,
            true => TMRB5CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRB5CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRB5CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRB5CLR`"]
pub struct TMRB5CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer B5 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB5CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B5 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB5CLR_A::CLEAR)
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
#[doc = "Counter/Timer B5 Interrupt Enable bit for COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5IE1_A {
    #[doc = "0: Disable counter/timer B5 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B5 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRB5IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5IE1`"]
pub type TMRB5IE1_R = crate::R<bool, TMRB5IE1_A>;
impl TMRB5IE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5IE1_A {
        match self.bits {
            false => TMRB5IE1_A::DIS,
            true => TMRB5IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB5IE1_A::EN
    }
}
#[doc = "Write proxy for field `TMRB5IE1`"]
pub struct TMRB5IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5IE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer B5 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5IE1_A::DIS)
    }
    #[doc = "Enable counter/timer B5 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5IE1_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Counter/Timer B5 Interrupt Enable bit for COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5IE0_A {
    #[doc = "0: Disable counter/timer B5 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B5 to generate an interrupt based on COMPR0 value."]
    EN = 1,
}
impl From<TMRB5IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5IE0`"]
pub type TMRB5IE0_R = crate::R<bool, TMRB5IE0_A>;
impl TMRB5IE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5IE0_A {
        match self.bits {
            false => TMRB5IE0_A::DIS,
            true => TMRB5IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB5IE0_A::EN
    }
}
#[doc = "Write proxy for field `TMRB5IE0`"]
pub struct TMRB5IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5IE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer B5 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5IE0_A::DIS)
    }
    #[doc = "Enable counter/timer B5 to generate an interrupt based on COMPR0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5IE0_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Counter/Timer B5 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB5FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B5, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B5, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B5, assert, count to CMPR1B5, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B5, assert, count to CMPR1B5, deassert, restart. value."]
    PULSE_CONT = 3,
    #[doc = "4: Single pattern. value."]
    SINGLEPATTERN = 4,
    #[doc = "5: Repeated pattern. value."]
    REPEATPATTERN = 5,
    #[doc = "6: Continuous run (aka Free Run).  Count continuously. value."]
    CONTINUOUS = 6,
    #[doc = "7: Alternate PWM value."]
    ALTPWN = 7,
}
impl From<TMRB5FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB5FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB5FN`"]
pub type TMRB5FN_R = crate::R<u8, TMRB5FN_A>;
impl TMRB5FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5FN_A {
        match self.bits {
            0 => TMRB5FN_A::SINGLECOUNT,
            1 => TMRB5FN_A::REPEATEDCOUNT,
            2 => TMRB5FN_A::PULSE_ONCE,
            3 => TMRB5FN_A::PULSE_CONT,
            4 => TMRB5FN_A::SINGLEPATTERN,
            5 => TMRB5FN_A::REPEATPATTERN,
            6 => TMRB5FN_A::CONTINUOUS,
            7 => TMRB5FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB5FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB5FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB5FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB5FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB5FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB5FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB5FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB5FN_A::ALTPWN
    }
}
#[doc = "Write proxy for field `TMRB5FN`"]
pub struct TMRB5FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5FN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B5, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB5FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B5, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB5FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B5, assert, count to CMPR1B5, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB5FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B5, assert, count to CMPR1B5, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB5FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB5FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB5FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB5FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB5FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B5 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB5CLK_A {
    #[doc = "0: Clock source is TMRPINB. value."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC / 4 value."]
    HFRC_DIV4 = 1,
    #[doc = "2: Clock source is HFRC / 16 value."]
    HFRC_DIV16 = 2,
    #[doc = "3: Clock source is HFRC / 256 value."]
    HFRC_DIV256 = 3,
    #[doc = "4: Clock source is HFRC / 1024 value."]
    HFRC_DIV1024 = 4,
    #[doc = "5: Clock source is HFRC / 4096 value."]
    HFRC_DIV4K = 5,
    #[doc = "6: Clock source is the XT (uncalibrated). value."]
    XT = 6,
    #[doc = "7: Clock source is XT / 2 value."]
    XT_DIV2 = 7,
    #[doc = "8: Clock source is XT / 16 value."]
    XT_DIV16 = 8,
    #[doc = "9: Clock source is XT / 128 value."]
    XT_DIV128 = 9,
    #[doc = "10: Clock source is LFRC / 2 value."]
    LFRC_DIV2 = 10,
    #[doc = "11: Clock source is LFRC / 32 value."]
    LFRC_DIV32 = 11,
    #[doc = "12: Clock source is LFRC / 1024 value."]
    LFRC_DIV1K = 12,
    #[doc = "13: Clock source is LFRC value."]
    LFRC = 13,
    #[doc = "14: Clock source is 100 Hz from the current RTC oscillator. value."]
    RTC_100HZ = 14,
    #[doc = "15: Clock source is HCLK. value."]
    HCLK = 15,
    #[doc = "16: Clock source is XT / 4 value."]
    XT_DIV4 = 16,
    #[doc = "17: Clock source is XT / 8 value."]
    XT_DIV8 = 17,
    #[doc = "18: Clock source is XT / 32 value."]
    XT_DIV32 = 18,
    #[doc = "20: Clock source is CTIMERA5 OUT. value."]
    CTMRA5 = 20,
    #[doc = "21: Clock source is CTIMERA0 OUT. value."]
    CTMRA0 = 21,
    #[doc = "22: Clock source is CTIMERB0 OUT. value."]
    CTMRB0 = 22,
    #[doc = "23: Clock source is CTIMERA6 OUT. value."]
    CTMRA6 = 23,
    #[doc = "24: Clock source is CTIMERB6 OUT. value."]
    CTMRB6 = 24,
    #[doc = "25: Clock source is CTIMERB1 OUT. value."]
    CTMRB1 = 25,
    #[doc = "26: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 26,
    #[doc = "27: Clock source is CTIMERB3 OUT. value."]
    CTMRB3 = 27,
    #[doc = "28: Clock source is CTIMERB4 OUT. value."]
    CTMRB4 = 28,
    #[doc = "29: Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE = 29,
    #[doc = "30: Clock source is Memory buck converter TON pulses. value."]
    BUCKB = 30,
    #[doc = "31: Clock source is CPU buck converter TON pulses. value."]
    BUCKA = 31,
}
impl From<TMRB5CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB5CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB5CLK`"]
pub type TMRB5CLK_R = crate::R<u8, TMRB5CLK_A>;
impl TMRB5CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRB5CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRB5CLK_A::TMRPIN),
            1 => Val(TMRB5CLK_A::HFRC_DIV4),
            2 => Val(TMRB5CLK_A::HFRC_DIV16),
            3 => Val(TMRB5CLK_A::HFRC_DIV256),
            4 => Val(TMRB5CLK_A::HFRC_DIV1024),
            5 => Val(TMRB5CLK_A::HFRC_DIV4K),
            6 => Val(TMRB5CLK_A::XT),
            7 => Val(TMRB5CLK_A::XT_DIV2),
            8 => Val(TMRB5CLK_A::XT_DIV16),
            9 => Val(TMRB5CLK_A::XT_DIV128),
            10 => Val(TMRB5CLK_A::LFRC_DIV2),
            11 => Val(TMRB5CLK_A::LFRC_DIV32),
            12 => Val(TMRB5CLK_A::LFRC_DIV1K),
            13 => Val(TMRB5CLK_A::LFRC),
            14 => Val(TMRB5CLK_A::RTC_100HZ),
            15 => Val(TMRB5CLK_A::HCLK),
            16 => Val(TMRB5CLK_A::XT_DIV4),
            17 => Val(TMRB5CLK_A::XT_DIV8),
            18 => Val(TMRB5CLK_A::XT_DIV32),
            20 => Val(TMRB5CLK_A::CTMRA5),
            21 => Val(TMRB5CLK_A::CTMRA0),
            22 => Val(TMRB5CLK_A::CTMRB0),
            23 => Val(TMRB5CLK_A::CTMRA6),
            24 => Val(TMRB5CLK_A::CTMRB6),
            25 => Val(TMRB5CLK_A::CTMRB1),
            26 => Val(TMRB5CLK_A::CTMRB2),
            27 => Val(TMRB5CLK_A::CTMRB3),
            28 => Val(TMRB5CLK_A::CTMRB4),
            29 => Val(TMRB5CLK_A::BUCKBLE),
            30 => Val(TMRB5CLK_A::BUCKB),
            31 => Val(TMRB5CLK_A::BUCKA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB5CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB5CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB5CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB5CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB5CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB5CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRB5CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB5CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB5CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB5CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB5CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB5CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB5CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB5CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB5CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMRB5CLK_A::HCLK
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB5CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB5CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB5CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA5`"]
    #[inline(always)]
    pub fn is_ctmra5(&self) -> bool {
        *self == TMRB5CLK_A::CTMRA5
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline(always)]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRB5CLK_A::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB5CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRA6`"]
    #[inline(always)]
    pub fn is_ctmra6(&self) -> bool {
        *self == TMRB5CLK_A::CTMRA6
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRB5CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB5CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB5CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB5CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB5CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB5CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB5CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB5CLK_A::BUCKA
    }
}
#[doc = "Write proxy for field `TMRB5CLK`"]
pub struct TMRB5CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK. value."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::HCLK)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    #[inline(always)]
    pub fn ctmra5(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRA5)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline(always)]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    #[inline(always)]
    pub fn ctmra6(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRA6)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRB6)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::CTMRB4)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB5CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B5 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5EN_A {
    #[doc = "0: Counter/Timer B5 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer B5 Enable. value."]
    EN = 1,
}
impl From<TMRB5EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB5EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB5EN`"]
pub type TMRB5EN_R = crate::R<bool, TMRB5EN_A>;
impl TMRB5EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB5EN_A {
        match self.bits {
            false => TMRB5EN_A::DIS,
            true => TMRB5EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB5EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRB5EN`"]
pub struct TMRB5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB5EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer B5 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5EN_A::DIS)
    }
    #[doc = "Counter/Timer B5 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A5 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5POL_A {
    #[doc = "0: The polarity of the TMRPINA5 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA5 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRA5POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5POL`"]
pub type TMRA5POL_R = crate::R<bool, TMRA5POL_A>;
impl TMRA5POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5POL_A {
        match self.bits {
            false => TMRA5POL_A::NORMAL,
            true => TMRA5POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRA5POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA5POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRA5POL`"]
pub struct TMRA5POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINA5 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA5POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA5 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA5POL_A::INVERTED)
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
#[doc = "Counter/Timer A5 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5CLR_A {
    #[doc = "0: Allow counter/timer A5 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer A5 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRA5CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5CLR`"]
pub type TMRA5CLR_R = crate::R<bool, TMRA5CLR_A>;
impl TMRA5CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5CLR_A {
        match self.bits {
            false => TMRA5CLR_A::RUN,
            true => TMRA5CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRA5CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRA5CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRA5CLR`"]
pub struct TMRA5CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer A5 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA5CLR_A::RUN)
    }
    #[doc = "Holds counter/timer A5 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA5CLR_A::CLEAR)
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
#[doc = "Counter/Timer A5 Interrupt Enable bit based on COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5IE1_A {
    #[doc = "0: Disable counter/timer A5 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A5 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRA5IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5IE1`"]
pub type TMRA5IE1_R = crate::R<bool, TMRA5IE1_A>;
impl TMRA5IE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5IE1_A {
        match self.bits {
            false => TMRA5IE1_A::DIS,
            true => TMRA5IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA5IE1_A::EN
    }
}
#[doc = "Write proxy for field `TMRA5IE1`"]
pub struct TMRA5IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5IE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer A5 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5IE1_A::DIS)
    }
    #[doc = "Enable counter/timer A5 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5IE1_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Counter/Timer A5 Interrupt Enable bit based on COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5IE0_A {
    #[doc = "0: Disable counter/timer A5 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A5 to generate an interrupt based on COMPR0. value."]
    EN = 1,
}
impl From<TMRA5IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5IE0`"]
pub type TMRA5IE0_R = crate::R<bool, TMRA5IE0_A>;
impl TMRA5IE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5IE0_A {
        match self.bits {
            false => TMRA5IE0_A::DIS,
            true => TMRA5IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA5IE0_A::EN
    }
}
#[doc = "Write proxy for field `TMRA5IE0`"]
pub struct TMRA5IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5IE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer A5 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5IE0_A::DIS)
    }
    #[doc = "Enable counter/timer A5 to generate an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5IE0_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Counter/Timer A5 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA5FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A5, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A5, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A5, assert, count to CMPR1A5, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A5, assert, count to CMPR1A5, deassert, restart. value."]
    PULSE_CONT = 3,
    #[doc = "4: Single pattern. value."]
    SINGLEPATTERN = 4,
    #[doc = "5: Repeated pattern. value."]
    REPEATPATTERN = 5,
    #[doc = "6: Continuous run (aka Free Run).  Count continuously. value."]
    CONTINUOUS = 6,
    #[doc = "7: Alternate PWM value."]
    ALTPWN = 7,
}
impl From<TMRA5FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA5FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA5FN`"]
pub type TMRA5FN_R = crate::R<u8, TMRA5FN_A>;
impl TMRA5FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5FN_A {
        match self.bits {
            0 => TMRA5FN_A::SINGLECOUNT,
            1 => TMRA5FN_A::REPEATEDCOUNT,
            2 => TMRA5FN_A::PULSE_ONCE,
            3 => TMRA5FN_A::PULSE_CONT,
            4 => TMRA5FN_A::SINGLEPATTERN,
            5 => TMRA5FN_A::REPEATPATTERN,
            6 => TMRA5FN_A::CONTINUOUS,
            7 => TMRA5FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA5FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA5FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA5FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA5FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA5FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA5FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA5FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA5FN_A::ALTPWN
    }
}
#[doc = "Write proxy for field `TMRA5FN`"]
pub struct TMRA5FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5FN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A5, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA5FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A5, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA5FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A5, assert, count to CMPR1A5, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA5FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A5, assert, count to CMPR1A5, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA5FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA5FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA5FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA5FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA5FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A5 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA5CLK_A {
    #[doc = "0: Clock source is TMRPINA. value."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC / 4 value."]
    HFRC_DIV4 = 1,
    #[doc = "2: Clock source is HFRC / 16 value."]
    HFRC_DIV16 = 2,
    #[doc = "3: Clock source is HFRC / 256 value."]
    HFRC_DIV256 = 3,
    #[doc = "4: Clock source is HFRC / 1024 value."]
    HFRC_DIV1024 = 4,
    #[doc = "5: Clock source is HFRC / 4096 value."]
    HFRC_DIV4K = 5,
    #[doc = "6: Clock source is the XT (uncalibrated). value."]
    XT = 6,
    #[doc = "7: Clock source is XT / 2 value."]
    XT_DIV2 = 7,
    #[doc = "8: Clock source is XT / 16 value."]
    XT_DIV16 = 8,
    #[doc = "9: Clock source is XT / 128 value."]
    XT_DIV128 = 9,
    #[doc = "10: Clock source is LFRC / 2 value."]
    LFRC_DIV2 = 10,
    #[doc = "11: Clock source is LFRC / 32 value."]
    LFRC_DIV32 = 11,
    #[doc = "12: Clock source is LFRC / 1024 value."]
    LFRC_DIV1K = 12,
    #[doc = "13: Clock source is LFRC value."]
    LFRC = 13,
    #[doc = "14: Clock source is 100 Hz from the current RTC oscillator. value."]
    RTC_100HZ = 14,
    #[doc = "15: Clock source is HCLK. value."]
    HCLK = 15,
    #[doc = "16: Clock source is XT / 4 value."]
    XT_DIV4 = 16,
    #[doc = "17: Clock source is XT / 8 value."]
    XT_DIV8 = 17,
    #[doc = "18: Clock source is XT / 32 value."]
    XT_DIV32 = 18,
    #[doc = "20: Clock source is CTIMERB5 OUT. value."]
    CTMRB5 = 20,
    #[doc = "21: Clock source is CTIMERA0 OUT. value."]
    CTMRA0 = 21,
    #[doc = "22: Clock source is CTIMERB0 OUT. value."]
    CTMRB0 = 22,
    #[doc = "23: Clock source is CTIMERA6 OUT. value."]
    CTMRA6 = 23,
    #[doc = "24: Clock source is CTIMERB6 OUT. value."]
    CTMRB6 = 24,
    #[doc = "25: Clock source is CTIMERB1 OUT. value."]
    CTMRB1 = 25,
    #[doc = "26: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 26,
    #[doc = "27: Clock source is CTIMERB3 OUT. value."]
    CTMRB3 = 27,
    #[doc = "28: Clock source is CTIMERB4 OUT. value."]
    CTMRB4 = 28,
    #[doc = "29: Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE = 29,
    #[doc = "30: Clock source is Memory buck converter TON pulses. value."]
    BUCKB = 30,
    #[doc = "31: Clock source is CPU buck converter TON pulses. value."]
    BUCKA = 31,
}
impl From<TMRA5CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA5CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA5CLK`"]
pub type TMRA5CLK_R = crate::R<u8, TMRA5CLK_A>;
impl TMRA5CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRA5CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRA5CLK_A::TMRPIN),
            1 => Val(TMRA5CLK_A::HFRC_DIV4),
            2 => Val(TMRA5CLK_A::HFRC_DIV16),
            3 => Val(TMRA5CLK_A::HFRC_DIV256),
            4 => Val(TMRA5CLK_A::HFRC_DIV1024),
            5 => Val(TMRA5CLK_A::HFRC_DIV4K),
            6 => Val(TMRA5CLK_A::XT),
            7 => Val(TMRA5CLK_A::XT_DIV2),
            8 => Val(TMRA5CLK_A::XT_DIV16),
            9 => Val(TMRA5CLK_A::XT_DIV128),
            10 => Val(TMRA5CLK_A::LFRC_DIV2),
            11 => Val(TMRA5CLK_A::LFRC_DIV32),
            12 => Val(TMRA5CLK_A::LFRC_DIV1K),
            13 => Val(TMRA5CLK_A::LFRC),
            14 => Val(TMRA5CLK_A::RTC_100HZ),
            15 => Val(TMRA5CLK_A::HCLK),
            16 => Val(TMRA5CLK_A::XT_DIV4),
            17 => Val(TMRA5CLK_A::XT_DIV8),
            18 => Val(TMRA5CLK_A::XT_DIV32),
            20 => Val(TMRA5CLK_A::CTMRB5),
            21 => Val(TMRA5CLK_A::CTMRA0),
            22 => Val(TMRA5CLK_A::CTMRB0),
            23 => Val(TMRA5CLK_A::CTMRA6),
            24 => Val(TMRA5CLK_A::CTMRB6),
            25 => Val(TMRA5CLK_A::CTMRB1),
            26 => Val(TMRA5CLK_A::CTMRB2),
            27 => Val(TMRA5CLK_A::CTMRB3),
            28 => Val(TMRA5CLK_A::CTMRB4),
            29 => Val(TMRA5CLK_A::BUCKBLE),
            30 => Val(TMRA5CLK_A::BUCKB),
            31 => Val(TMRA5CLK_A::BUCKA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA5CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA5CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA5CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA5CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA5CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA5CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRA5CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA5CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA5CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA5CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA5CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA5CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA5CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA5CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA5CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMRA5CLK_A::HCLK
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA5CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA5CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA5CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA5CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline(always)]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRA5CLK_A::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA5CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRA6`"]
    #[inline(always)]
    pub fn is_ctmra6(&self) -> bool {
        *self == TMRA5CLK_A::CTMRA6
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRA5CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA5CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA5CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA5CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA5CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA5CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA5CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA5CLK_A::BUCKA
    }
}
#[doc = "Write proxy for field `TMRA5CLK`"]
pub struct TMRA5CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK. value."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::HCLK)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRB5)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline(always)]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    #[inline(always)]
    pub fn ctmra6(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRA6)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRB6)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::CTMRB4)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA5CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A5 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5EN_A {
    #[doc = "0: Counter/Timer A5 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer A5 Enable. value."]
    EN = 1,
}
impl From<TMRA5EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA5EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA5EN`"]
pub type TMRA5EN_R = crate::R<bool, TMRA5EN_A>;
impl TMRA5EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA5EN_A {
        match self.bits {
            false => TMRA5EN_A::DIS,
            true => TMRA5EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA5EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRA5EN`"]
pub struct TMRA5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA5EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer A5 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5EN_A::DIS)
    }
    #[doc = "Counter/Timer A5 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A5/B5 Link bit."]
    #[inline(always)]
    pub fn ctlink5(&self) -> CTLINK5_R {
        CTLINK5_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B5 output polarity."]
    #[inline(always)]
    pub fn tmrb5pol(&self) -> TMRB5POL_R {
        TMRB5POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B5 Clear bit."]
    #[inline(always)]
    pub fn tmrb5clr(&self) -> TMRB5CLR_R {
        TMRB5CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B5 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb5ie1(&self) -> TMRB5IE1_R {
        TMRB5IE1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B5 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb5ie0(&self) -> TMRB5IE0_R {
        TMRB5IE0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B5 Function Select."]
    #[inline(always)]
    pub fn tmrb5fn(&self) -> TMRB5FN_R {
        TMRB5FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B5 Clock Select."]
    #[inline(always)]
    pub fn tmrb5clk(&self) -> TMRB5CLK_R {
        TMRB5CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B5 Enable bit."]
    #[inline(always)]
    pub fn tmrb5en(&self) -> TMRB5EN_R {
        TMRB5EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A5 output polarity."]
    #[inline(always)]
    pub fn tmra5pol(&self) -> TMRA5POL_R {
        TMRA5POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A5 Clear bit."]
    #[inline(always)]
    pub fn tmra5clr(&self) -> TMRA5CLR_R {
        TMRA5CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A5 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra5ie1(&self) -> TMRA5IE1_R {
        TMRA5IE1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A5 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra5ie0(&self) -> TMRA5IE0_R {
        TMRA5IE0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A5 Function Select."]
    #[inline(always)]
    pub fn tmra5fn(&self) -> TMRA5FN_R {
        TMRA5FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A5 Clock Select."]
    #[inline(always)]
    pub fn tmra5clk(&self) -> TMRA5CLK_R {
        TMRA5CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A5 Enable bit."]
    #[inline(always)]
    pub fn tmra5en(&self) -> TMRA5EN_R {
        TMRA5EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A5/B5 Link bit."]
    #[inline(always)]
    pub fn ctlink5(&mut self) -> CTLINK5_W {
        CTLINK5_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B5 output polarity."]
    #[inline(always)]
    pub fn tmrb5pol(&mut self) -> TMRB5POL_W {
        TMRB5POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B5 Clear bit."]
    #[inline(always)]
    pub fn tmrb5clr(&mut self) -> TMRB5CLR_W {
        TMRB5CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B5 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb5ie1(&mut self) -> TMRB5IE1_W {
        TMRB5IE1_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B5 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb5ie0(&mut self) -> TMRB5IE0_W {
        TMRB5IE0_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B5 Function Select."]
    #[inline(always)]
    pub fn tmrb5fn(&mut self) -> TMRB5FN_W {
        TMRB5FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B5 Clock Select."]
    #[inline(always)]
    pub fn tmrb5clk(&mut self) -> TMRB5CLK_W {
        TMRB5CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B5 Enable bit."]
    #[inline(always)]
    pub fn tmrb5en(&mut self) -> TMRB5EN_W {
        TMRB5EN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A5 output polarity."]
    #[inline(always)]
    pub fn tmra5pol(&mut self) -> TMRA5POL_W {
        TMRA5POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A5 Clear bit."]
    #[inline(always)]
    pub fn tmra5clr(&mut self) -> TMRA5CLR_W {
        TMRA5CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A5 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra5ie1(&mut self) -> TMRA5IE1_W {
        TMRA5IE1_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A5 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra5ie0(&mut self) -> TMRA5IE0_W {
        TMRA5IE0_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A5 Function Select."]
    #[inline(always)]
    pub fn tmra5fn(&mut self) -> TMRA5FN_W {
        TMRA5FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A5 Clock Select."]
    #[inline(always)]
    pub fn tmra5clk(&mut self) -> TMRA5CLK_W {
        TMRA5CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A5 Enable bit."]
    #[inline(always)]
    pub fn tmra5en(&mut self) -> TMRA5EN_W {
        TMRA5EN_W { w: self }
    }
}
