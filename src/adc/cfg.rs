#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the source and frequency for the ADC clock. All values not enumerated below are undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Off mode. The HFRC or HFRC_DIV2 clock must be selected for the ADC to function. The ADC controller automatically shuts off the clock in it's low power modes.  When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing. value."]
    OFF = 0,
    #[doc = "1: HFRC Core Clock divided by (CORESEL+1) value."]
    HFRC = 1,
    #[doc = "2: HFRC Core Clock / 2 further divided by (CORESEL+1) value."]
    HFRC_DIV2 = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSEL_A::OFF),
            1 => Val(CLKSEL_A::HFRC),
            2 => Val(CLKSEL_A::HFRC_DIV2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLKSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == CLKSEL_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2`"]
    #[inline(always)]
    pub fn is_hfrc_div2(&self) -> bool {
        *self == CLKSEL_A::HFRC_DIV2
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Off mode. The HFRC or HFRC_DIV2 clock must be selected for the ADC to function. The ADC controller automatically shuts off the clock in it's low power modes. When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing. value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLKSEL_A::OFF)
    }
    #[doc = "HFRC Core Clock divided by (CORESEL+1) value."]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRC)
    }
    #[doc = "HFRC Core Clock / 2 further divided by (CORESEL+1) value."]
    #[inline(always)]
    pub fn hfrc_div2(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRC_DIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "This bit selects the ADC trigger polarity for external off chip triggers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOL_A {
    #[doc = "0: Trigger on rising edge. value."]
    RISING_EDGE = 0,
    #[doc = "1: Trigger on falling edge. value."]
    FALLING_EDGE = 1,
}
impl From<TRIGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGPOL`"]
pub type TRIGPOL_R = crate::R<bool, TRIGPOL_A>;
impl TRIGPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGPOL_A {
        match self.bits {
            false => TRIGPOL_A::RISING_EDGE,
            true => TRIGPOL_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGPOL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGPOL_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `TRIGPOL`"]
pub struct TRIGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger on rising edge. value."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TRIGPOL_A::RISING_EDGE)
    }
    #[doc = "Trigger on falling edge. value."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TRIGPOL_A::FALLING_EDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Select the ADC trigger source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    #[doc = "0: Off chip External Trigger0 (ADC_ET0) value."]
    EXT0 = 0,
    #[doc = "1: Off chip External Trigger1 (ADC_ET1) value."]
    EXT1 = 1,
    #[doc = "2: Off chip External Trigger2 (ADC_ET2) value."]
    EXT2 = 2,
    #[doc = "3: Off chip External Trigger3 (ADC_ET3) value."]
    EXT3 = 3,
    #[doc = "4: Voltage Comparator Output value."]
    VCOMP = 4,
    #[doc = "7: Software Trigger value."]
    SWT = 7,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGSEL`"]
pub type TRIGSEL_R = crate::R<u8, TRIGSEL_A>;
impl TRIGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGSEL_A::EXT0),
            1 => Val(TRIGSEL_A::EXT1),
            2 => Val(TRIGSEL_A::EXT2),
            3 => Val(TRIGSEL_A::EXT3),
            4 => Val(TRIGSEL_A::VCOMP),
            7 => Val(TRIGSEL_A::SWT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXT0`"]
    #[inline(always)]
    pub fn is_ext0(&self) -> bool {
        *self == TRIGSEL_A::EXT0
    }
    #[doc = "Checks if the value of the field is `EXT1`"]
    #[inline(always)]
    pub fn is_ext1(&self) -> bool {
        *self == TRIGSEL_A::EXT1
    }
    #[doc = "Checks if the value of the field is `EXT2`"]
    #[inline(always)]
    pub fn is_ext2(&self) -> bool {
        *self == TRIGSEL_A::EXT2
    }
    #[doc = "Checks if the value of the field is `EXT3`"]
    #[inline(always)]
    pub fn is_ext3(&self) -> bool {
        *self == TRIGSEL_A::EXT3
    }
    #[doc = "Checks if the value of the field is `VCOMP`"]
    #[inline(always)]
    pub fn is_vcomp(&self) -> bool {
        *self == TRIGSEL_A::VCOMP
    }
    #[doc = "Checks if the value of the field is `SWT`"]
    #[inline(always)]
    pub fn is_swt(&self) -> bool {
        *self == TRIGSEL_A::SWT
    }
}
#[doc = "Write proxy for field `TRIGSEL`"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Off chip External Trigger0 (ADC_ET0) value."]
    #[inline(always)]
    pub fn ext0(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT0)
    }
    #[doc = "Off chip External Trigger1 (ADC_ET1) value."]
    #[inline(always)]
    pub fn ext1(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT1)
    }
    #[doc = "Off chip External Trigger2 (ADC_ET2) value."]
    #[inline(always)]
    pub fn ext2(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT2)
    }
    #[doc = "Off chip External Trigger3 (ADC_ET3) value."]
    #[inline(always)]
    pub fn ext3(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT3)
    }
    #[doc = "Voltage Comparator Output value."]
    #[inline(always)]
    pub fn vcomp(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VCOMP)
    }
    #[doc = "Software Trigger value."]
    #[inline(always)]
    pub fn swt(self) -> &'a mut W {
        self.variant(TRIGSEL_A::SWT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFIFORDEN_A {
    #[doc = "0: Destructive Reads are prevented.  Reads to the FIFOPR register will not POP an entry off the FIFO. value."]
    DIS = 0,
    #[doc = "1: Reads to the FIFOPR registger will automatically pop an entry off the FIFO. value."]
    EN = 1,
}
impl From<DFIFORDEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFIFORDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFIFORDEN`"]
pub type DFIFORDEN_R = crate::R<bool, DFIFORDEN_A>;
impl DFIFORDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFIFORDEN_A {
        match self.bits {
            false => DFIFORDEN_A::DIS,
            true => DFIFORDEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DFIFORDEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DFIFORDEN_A::EN
    }
}
#[doc = "Write proxy for field `DFIFORDEN`"]
pub struct DFIFORDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFIFORDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFIFORDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Destructive Reads are prevented. Reads to the FIFOPR register will not POP an entry off the FIFO. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DFIFORDEN_A::DIS)
    }
    #[doc = "Reads to the FIFOPR registger will automatically pop an entry off the FIFO. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DFIFORDEN_A::EN)
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
#[doc = "Select the ADC reference voltage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal 2.0V Bandgap Reference Voltage value."]
    INT2P0 = 0,
    #[doc = "1: Internal 1.5V Bandgap Reference Voltage value."]
    INT1P5 = 1,
    #[doc = "2: Off Chip 2.0V Reference value."]
    EXT2P0 = 2,
    #[doc = "3: Off Chip 1.5V Reference value."]
    EXT1P5 = 3,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::INT2P0,
            1 => REFSEL_A::INT1P5,
            2 => REFSEL_A::EXT2P0,
            3 => REFSEL_A::EXT1P5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INT2P0`"]
    #[inline(always)]
    pub fn is_int2p0(&self) -> bool {
        *self == REFSEL_A::INT2P0
    }
    #[doc = "Checks if the value of the field is `INT1P5`"]
    #[inline(always)]
    pub fn is_int1p5(&self) -> bool {
        *self == REFSEL_A::INT1P5
    }
    #[doc = "Checks if the value of the field is `EXT2P0`"]
    #[inline(always)]
    pub fn is_ext2p0(&self) -> bool {
        *self == REFSEL_A::EXT2P0
    }
    #[doc = "Checks if the value of the field is `EXT1P5`"]
    #[inline(always)]
    pub fn is_ext1p5(&self) -> bool {
        *self == REFSEL_A::EXT1P5
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Internal 2.0V Bandgap Reference Voltage value."]
    #[inline(always)]
    pub fn int2p0(self) -> &'a mut W {
        self.variant(REFSEL_A::INT2P0)
    }
    #[doc = "Internal 1.5V Bandgap Reference Voltage value."]
    #[inline(always)]
    pub fn int1p5(self) -> &'a mut W {
        self.variant(REFSEL_A::INT1P5)
    }
    #[doc = "Off Chip 2.0V Reference value."]
    #[inline(always)]
    pub fn ext2p0(self) -> &'a mut W {
        self.variant(REFSEL_A::EXT2P0)
    }
    #[doc = "Off Chip 1.5V Reference value."]
    #[inline(always)]
    pub fn ext1p5(self) -> &'a mut W {
        self.variant(REFSEL_A::EXT1P5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Clock mode register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKMODE_A {
    #[doc = "0: Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the ADC. value."]
    LPCKMODE = 0,
    #[doc = "1: Low Latency Clock Mode.  When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0. value."]
    LLCKMODE = 1,
}
impl From<CKMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKMODE`"]
pub type CKMODE_R = crate::R<bool, CKMODE_A>;
impl CKMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            false => CKMODE_A::LPCKMODE,
            true => CKMODE_A::LLCKMODE,
        }
    }
    #[doc = "Checks if the value of the field is `LPCKMODE`"]
    #[inline(always)]
    pub fn is_lpckmode(&self) -> bool {
        *self == CKMODE_A::LPCKMODE
    }
    #[doc = "Checks if the value of the field is `LLCKMODE`"]
    #[inline(always)]
    pub fn is_llckmode(&self) -> bool {
        *self == CKMODE_A::LLCKMODE
    }
}
#[doc = "Write proxy for field `CKMODE`"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the ADC. value."]
    #[inline(always)]
    pub fn lpckmode(self) -> &'a mut W {
        self.variant(CKMODE_A::LPCKMODE)
    }
    #[doc = "Low Latency Clock Mode. When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0. value."]
    #[inline(always)]
    pub fn llckmode(self) -> &'a mut W {
        self.variant(CKMODE_A::LLCKMODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Select power mode to enter between active scans.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMODE_A {
    #[doc = "0: Low Power Mode 0.  Leaves the ADC fully powered between scans with minimum latency between a trigger event and sample data collection. value."]
    MODE0 = 0,
    #[doc = "1: Low Power Mode 1.  Powers down all circuity and clocks associated with the ADC until the next trigger event.  Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode. value."]
    MODE1 = 1,
}
impl From<LPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPMODE`"]
pub type LPMODE_R = crate::R<bool, LPMODE_A>;
impl LPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMODE_A {
        match self.bits {
            false => LPMODE_A::MODE0,
            true => LPMODE_A::MODE1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == LPMODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == LPMODE_A::MODE1
    }
}
#[doc = "Write proxy for field `LPMODE`"]
pub struct LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low Power Mode 0. Leaves the ADC fully powered between scans with minimum latency between a trigger event and sample data collection. value."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(LPMODE_A::MODE0)
    }
    #[doc = "Low Power Mode 1. Powers down all circuity and clocks associated with the ADC until the next trigger event. Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode. value."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(LPMODE_A::MODE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "This bit enables Repeating Scan Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPTEN_A {
    #[doc = "0: In Single Scan Mode, the ADC will complete a single scan upon each trigger event. value."]
    SINGLE_SCAN = 0,
    #[doc = "1: In Repeating Scan Mode, the ADC will complete it's first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 internal timer until the timer is disabled or the ADC is disabled.  When disabling the ADC (setting ADCEN to '0'), the RPTEN bit should be cleared. value."]
    REPEATING_SCAN = 1,
}
impl From<RPTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RPTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RPTEN`"]
pub type RPTEN_R = crate::R<bool, RPTEN_A>;
impl RPTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPTEN_A {
        match self.bits {
            false => RPTEN_A::SINGLE_SCAN,
            true => RPTEN_A::REPEATING_SCAN,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_SCAN`"]
    #[inline(always)]
    pub fn is_single_scan(&self) -> bool {
        *self == RPTEN_A::SINGLE_SCAN
    }
    #[doc = "Checks if the value of the field is `REPEATING_SCAN`"]
    #[inline(always)]
    pub fn is_repeating_scan(&self) -> bool {
        *self == RPTEN_A::REPEATING_SCAN
    }
}
#[doc = "Write proxy for field `RPTEN`"]
pub struct RPTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RPTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In Single Scan Mode, the ADC will complete a single scan upon each trigger event. value."]
    #[inline(always)]
    pub fn single_scan(self) -> &'a mut W {
        self.variant(RPTEN_A::SINGLE_SCAN)
    }
    #[doc = "In Repeating Scan Mode, the ADC will complete it's first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 internal timer until the timer is disabled or the ADC is disabled. When disabling the ADC (setting ADCEN to '0'), the RPTEN bit should be cleared. value."]
    #[inline(always)]
    pub fn repeating_scan(self) -> &'a mut W {
        self.variant(RPTEN_A::REPEATING_SCAN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEN_A {
    #[doc = "0: Disable the ADC module. value."]
    DIS = 0,
    #[doc = "1: Enable the ADC module. value."]
    EN = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, ADCEN_A>;
impl ADCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::DIS,
            true => ADCEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADCEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADCEN_A::EN
    }
}
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the ADC module. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCEN_A::DIS)
    }
    #[doc = "Enable the ADC module. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCEN_A::EN)
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
    #[doc = "Bits 24:25 - Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 19 - This bit selects the ADC trigger polarity for external off chip triggers."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Select the ADC trigger source."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
    #[inline(always)]
    pub fn dfiforden(&self) -> DFIFORDEN_R {
        DFIFORDEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Select the ADC reference voltage."]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Clock mode register"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select power mode to enter between active scans."]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit enables Repeating Scan Mode."]
    #[inline(always)]
    pub fn rpten(&self) -> RPTEN_R {
        RPTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 19 - This bit selects the ADC trigger polarity for external off chip triggers."]
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W {
        TRIGPOL_W { w: self }
    }
    #[doc = "Bits 16:18 - Select the ADC trigger source."]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bit 12 - Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
    #[inline(always)]
    pub fn dfiforden(&mut self) -> DFIFORDEN_W {
        DFIFORDEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Select the ADC reference voltage."]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 4 - Clock mode register"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    #[doc = "Bit 3 - Select power mode to enter between active scans."]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W {
        LPMODE_W { w: self }
    }
    #[doc = "Bit 2 - This bit enables Repeating Scan Mode."]
    #[inline(always)]
    pub fn rpten(&mut self) -> RPTEN_W {
        RPTEN_W { w: self }
    }
    #[doc = "Bit 0 - This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
}
