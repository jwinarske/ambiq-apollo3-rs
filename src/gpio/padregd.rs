#[doc = "Reader of register PADREGD"]
pub type R = crate::R<u32, super::PADREGD>;
#[doc = "Writer for register PADREGD"]
pub type W = crate::W<u32, super::PADREGD>;
#[doc = "Register PADREGD `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 15 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD15FNCSEL_A {
    #[doc = "0: Configure as the analog ADC differential pair 1 N input signal value."]
    ADCD1N = 0,
    #[doc = "1: IOM/MSPI nCE group 15 value."]
    NCE15 = 1,
    #[doc = "2: Configure as the UART1 RX signal value."]
    UART1RX = 2,
    #[doc = "3: Configure as GPIO15 value."]
    GPIO15 = 3,
    #[doc = "4: PDM serial data input value."]
    PDMDATA = 4,
    #[doc = "5: Configure as the external XTAL oscillator input value."]
    EXTXT = 5,
    #[doc = "6: Configure as an alternate port for the SWDIO I/O signal value."]
    SWDIO = 6,
    #[doc = "7: Configure as an SWO (Serial Wire Trace output) value."]
    SWO = 7,
}
impl From<PAD15FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD15FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD15FNCSEL`"]
pub type PAD15FNCSEL_R = crate::R<u8, PAD15FNCSEL_A>;
impl PAD15FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD15FNCSEL_A {
        match self.bits {
            0 => PAD15FNCSEL_A::ADCD1N,
            1 => PAD15FNCSEL_A::NCE15,
            2 => PAD15FNCSEL_A::UART1RX,
            3 => PAD15FNCSEL_A::GPIO15,
            4 => PAD15FNCSEL_A::PDMDATA,
            5 => PAD15FNCSEL_A::EXTXT,
            6 => PAD15FNCSEL_A::SWDIO,
            7 => PAD15FNCSEL_A::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCD1N`"]
    #[inline(always)]
    pub fn is_adcd1n(&self) -> bool {
        *self == PAD15FNCSEL_A::ADCD1N
    }
    #[doc = "Checks if the value of the field is `NCE15`"]
    #[inline(always)]
    pub fn is_nce15(&self) -> bool {
        *self == PAD15FNCSEL_A::NCE15
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD15FNCSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `GPIO15`"]
    #[inline(always)]
    pub fn is_gpio15(&self) -> bool {
        *self == PAD15FNCSEL_A::GPIO15
    }
    #[doc = "Checks if the value of the field is `PDMDATA`"]
    #[inline(always)]
    pub fn is_pdmdata(&self) -> bool {
        *self == PAD15FNCSEL_A::PDMDATA
    }
    #[doc = "Checks if the value of the field is `EXTXT`"]
    #[inline(always)]
    pub fn is_extxt(&self) -> bool {
        *self == PAD15FNCSEL_A::EXTXT
    }
    #[doc = "Checks if the value of the field is `SWDIO`"]
    #[inline(always)]
    pub fn is_swdio(&self) -> bool {
        *self == PAD15FNCSEL_A::SWDIO
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD15FNCSEL_A::SWO
    }
}
#[doc = "Write proxy for field `PAD15FNCSEL`"]
pub struct PAD15FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog ADC differential pair 1 N input signal value."]
    #[inline(always)]
    pub fn adcd1n(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::ADCD1N)
    }
    #[doc = "IOM/MSPI nCE group 15 value."]
    #[inline(always)]
    pub fn nce15(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::NCE15)
    }
    #[doc = "Configure as the UART1 RX signal value."]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::UART1RX)
    }
    #[doc = "Configure as GPIO15 value."]
    #[inline(always)]
    pub fn gpio15(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::GPIO15)
    }
    #[doc = "PDM serial data input value."]
    #[inline(always)]
    pub fn pdmdata(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::PDMDATA)
    }
    #[doc = "Configure as the external XTAL oscillator input value."]
    #[inline(always)]
    pub fn extxt(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::EXTXT)
    }
    #[doc = "Configure as an alternate port for the SWDIO I/O signal value."]
    #[inline(always)]
    pub fn swdio(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::SWDIO)
    }
    #[doc = "Configure as an SWO (Serial Wire Trace output) value."]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::SWO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 15 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD15STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD15STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD15STRNG`"]
pub type PAD15STRNG_R = crate::R<bool, PAD15STRNG_A>;
impl PAD15STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD15STRNG_A {
        match self.bits {
            false => PAD15STRNG_A::LOW,
            true => PAD15STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD15STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD15STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD15STRNG`"]
pub struct PAD15STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD15STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD15STRNG_A::HIGH)
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
#[doc = "Pad 15 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD15INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD15INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD15INPEN`"]
pub type PAD15INPEN_R = crate::R<bool, PAD15INPEN_A>;
impl PAD15INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD15INPEN_A {
        match self.bits {
            false => PAD15INPEN_A::DIS,
            true => PAD15INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD15INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD15INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD15INPEN`"]
pub struct PAD15INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD15INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD15INPEN_A::EN)
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
#[doc = "Pad 15 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD15PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD15PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD15PULL`"]
pub type PAD15PULL_R = crate::R<bool, PAD15PULL_A>;
impl PAD15PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD15PULL_A {
        match self.bits {
            false => PAD15PULL_A::DIS,
            true => PAD15PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD15PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD15PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD15PULL`"]
pub struct PAD15PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD15PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD15PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Pad 14 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD14FNCSEL_A {
    #[doc = "0: Configure as the analog ADC differential pair 1 P input signal value."]
    ADCD1P = 0,
    #[doc = "1: IOM/MSPI nCE group 14 value."]
    NCE14 = 1,
    #[doc = "2: Configure as the UART1 TX output signal value."]
    UART1TX = 2,
    #[doc = "3: Configure as GPIO14 value."]
    GPIO14 = 3,
    #[doc = "4: PDM serial clock output value."]
    PDMCLK = 4,
    #[doc = "5: Configure as the External HFRC oscillator input select value."]
    EXTHFS = 5,
    #[doc = "6: Configure as the alternate input for the SWDCK input signal value."]
    SWDCK = 6,
    #[doc = "7: Configure as the 32kHz crystal output signal value."]
    _32KHZXT = 7,
}
impl From<PAD14FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD14FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD14FNCSEL`"]
pub type PAD14FNCSEL_R = crate::R<u8, PAD14FNCSEL_A>;
impl PAD14FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD14FNCSEL_A {
        match self.bits {
            0 => PAD14FNCSEL_A::ADCD1P,
            1 => PAD14FNCSEL_A::NCE14,
            2 => PAD14FNCSEL_A::UART1TX,
            3 => PAD14FNCSEL_A::GPIO14,
            4 => PAD14FNCSEL_A::PDMCLK,
            5 => PAD14FNCSEL_A::EXTHFS,
            6 => PAD14FNCSEL_A::SWDCK,
            7 => PAD14FNCSEL_A::_32KHZXT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCD1P`"]
    #[inline(always)]
    pub fn is_adcd1p(&self) -> bool {
        *self == PAD14FNCSEL_A::ADCD1P
    }
    #[doc = "Checks if the value of the field is `NCE14`"]
    #[inline(always)]
    pub fn is_nce14(&self) -> bool {
        *self == PAD14FNCSEL_A::NCE14
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD14FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `GPIO14`"]
    #[inline(always)]
    pub fn is_gpio14(&self) -> bool {
        *self == PAD14FNCSEL_A::GPIO14
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline(always)]
    pub fn is_pdmclk(&self) -> bool {
        *self == PAD14FNCSEL_A::PDMCLK
    }
    #[doc = "Checks if the value of the field is `EXTHFS`"]
    #[inline(always)]
    pub fn is_exthfs(&self) -> bool {
        *self == PAD14FNCSEL_A::EXTHFS
    }
    #[doc = "Checks if the value of the field is `SWDCK`"]
    #[inline(always)]
    pub fn is_swdck(&self) -> bool {
        *self == PAD14FNCSEL_A::SWDCK
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline(always)]
    pub fn is_32k_hz_xt(&self) -> bool {
        *self == PAD14FNCSEL_A::_32KHZXT
    }
}
#[doc = "Write proxy for field `PAD14FNCSEL`"]
pub struct PAD14FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog ADC differential pair 1 P input signal value."]
    #[inline(always)]
    pub fn adcd1p(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::ADCD1P)
    }
    #[doc = "IOM/MSPI nCE group 14 value."]
    #[inline(always)]
    pub fn nce14(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::NCE14)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as GPIO14 value."]
    #[inline(always)]
    pub fn gpio14(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::GPIO14)
    }
    #[doc = "PDM serial clock output value."]
    #[inline(always)]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::PDMCLK)
    }
    #[doc = "Configure as the External HFRC oscillator input select value."]
    #[inline(always)]
    pub fn exthfs(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::EXTHFS)
    }
    #[doc = "Configure as the alternate input for the SWDCK input signal value."]
    #[inline(always)]
    pub fn swdck(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::SWDCK)
    }
    #[doc = "Configure as the 32kHz crystal output signal value."]
    #[inline(always)]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::_32KHZXT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 14 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD14STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD14STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD14STRNG`"]
pub type PAD14STRNG_R = crate::R<bool, PAD14STRNG_A>;
impl PAD14STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD14STRNG_A {
        match self.bits {
            false => PAD14STRNG_A::LOW,
            true => PAD14STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD14STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD14STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD14STRNG`"]
pub struct PAD14STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD14STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD14STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Pad 14 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD14INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD14INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD14INPEN`"]
pub type PAD14INPEN_R = crate::R<bool, PAD14INPEN_A>;
impl PAD14INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD14INPEN_A {
        match self.bits {
            false => PAD14INPEN_A::DIS,
            true => PAD14INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD14INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD14INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD14INPEN`"]
pub struct PAD14INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD14INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD14INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Pad 14 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD14PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD14PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD14PULL`"]
pub type PAD14PULL_R = crate::R<bool, PAD14PULL_A>;
impl PAD14PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD14PULL_A {
        match self.bits {
            false => PAD14PULL_A::DIS,
            true => PAD14PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD14PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD14PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD14PULL`"]
pub struct PAD14PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD14PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD14PULL_A::EN)
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
#[doc = "Pad 13 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD13FNCSEL_A {
    #[doc = "0: Configure as the ADC Differential pair 0 P, or Single Ended input 8 analog input signal. Determination of the D0P vs SE8 usage is done when the particular channel is selected within the ADC module value."]
    ADCD0PSE8 = 0,
    #[doc = "1: IOM/MSPI nCE group 13 value."]
    NCE13 = 1,
    #[doc = "2: CTIMER connection 2 value."]
    CT2 = 2,
    #[doc = "3: Configure as GPIO13 value."]
    GPIO13 = 3,
    #[doc = "4: I2C interface bit clock value."]
    I2SBCLK = 4,
    #[doc = "5: Configure as the external HFRC oscillator input value."]
    EXTHFB = 5,
    #[doc = "6: Configure as the UART0 RTS signal output value."]
    UA0RTS = 6,
    #[doc = "7: Configure as the UART1 RX input signal value."]
    UART1RX = 7,
}
impl From<PAD13FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD13FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD13FNCSEL`"]
pub type PAD13FNCSEL_R = crate::R<u8, PAD13FNCSEL_A>;
impl PAD13FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD13FNCSEL_A {
        match self.bits {
            0 => PAD13FNCSEL_A::ADCD0PSE8,
            1 => PAD13FNCSEL_A::NCE13,
            2 => PAD13FNCSEL_A::CT2,
            3 => PAD13FNCSEL_A::GPIO13,
            4 => PAD13FNCSEL_A::I2SBCLK,
            5 => PAD13FNCSEL_A::EXTHFB,
            6 => PAD13FNCSEL_A::UA0RTS,
            7 => PAD13FNCSEL_A::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCD0PSE8`"]
    #[inline(always)]
    pub fn is_adcd0pse8(&self) -> bool {
        *self == PAD13FNCSEL_A::ADCD0PSE8
    }
    #[doc = "Checks if the value of the field is `NCE13`"]
    #[inline(always)]
    pub fn is_nce13(&self) -> bool {
        *self == PAD13FNCSEL_A::NCE13
    }
    #[doc = "Checks if the value of the field is `CT2`"]
    #[inline(always)]
    pub fn is_ct2(&self) -> bool {
        *self == PAD13FNCSEL_A::CT2
    }
    #[doc = "Checks if the value of the field is `GPIO13`"]
    #[inline(always)]
    pub fn is_gpio13(&self) -> bool {
        *self == PAD13FNCSEL_A::GPIO13
    }
    #[doc = "Checks if the value of the field is `I2SBCLK`"]
    #[inline(always)]
    pub fn is_i2sbclk(&self) -> bool {
        *self == PAD13FNCSEL_A::I2SBCLK
    }
    #[doc = "Checks if the value of the field is `EXTHFB`"]
    #[inline(always)]
    pub fn is_exthfb(&self) -> bool {
        *self == PAD13FNCSEL_A::EXTHFB
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD13FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD13FNCSEL_A::UART1RX
    }
}
#[doc = "Write proxy for field `PAD13FNCSEL`"]
pub struct PAD13FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Differential pair 0 P, or Single Ended input 8 analog input signal. Determination of the D0P vs SE8 usage is done when the particular channel is selected within the ADC module value."]
    #[inline(always)]
    pub fn adcd0pse8(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::ADCD0PSE8)
    }
    #[doc = "IOM/MSPI nCE group 13 value."]
    #[inline(always)]
    pub fn nce13(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::NCE13)
    }
    #[doc = "CTIMER connection 2 value."]
    #[inline(always)]
    pub fn ct2(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::CT2)
    }
    #[doc = "Configure as GPIO13 value."]
    #[inline(always)]
    pub fn gpio13(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::GPIO13)
    }
    #[doc = "I2C interface bit clock value."]
    #[inline(always)]
    pub fn i2sbclk(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::I2SBCLK)
    }
    #[doc = "Configure as the external HFRC oscillator input value."]
    #[inline(always)]
    pub fn exthfb(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::EXTHFB)
    }
    #[doc = "Configure as the UART0 RTS signal output value."]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 13 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD13STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD13STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD13STRNG`"]
pub type PAD13STRNG_R = crate::R<bool, PAD13STRNG_A>;
impl PAD13STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD13STRNG_A {
        match self.bits {
            false => PAD13STRNG_A::LOW,
            true => PAD13STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD13STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD13STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD13STRNG`"]
pub struct PAD13STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD13STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD13STRNG_A::HIGH)
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
#[doc = "Pad 13 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD13INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD13INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD13INPEN`"]
pub type PAD13INPEN_R = crate::R<bool, PAD13INPEN_A>;
impl PAD13INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD13INPEN_A {
        match self.bits {
            false => PAD13INPEN_A::DIS,
            true => PAD13INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD13INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD13INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD13INPEN`"]
pub struct PAD13INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD13INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD13INPEN_A::EN)
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
#[doc = "Pad 13 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD13PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD13PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD13PULL`"]
pub type PAD13PULL_R = crate::R<bool, PAD13PULL_A>;
impl PAD13PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD13PULL_A {
        match self.bits {
            false => PAD13PULL_A::DIS,
            true => PAD13PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD13PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD13PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD13PULL`"]
pub struct PAD13PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD13PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD13PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Pad 12 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD12FNCSEL_A {
    #[doc = "0: Configure as the ADC Differential pair 0 N, or Single Ended input 9 analog input signal. Determination of the D0N vs SE9 usage is done when the particular channel is selected within the ADC module value."]
    ADCD0NSE9 = 0,
    #[doc = "1: IOM/MSPI nCE group 12 value."]
    NCE12 = 1,
    #[doc = "2: CTIMER connection 0 value."]
    CT0 = 2,
    #[doc = "3: Configure as GPIO12 value."]
    GPIO12 = 3,
    #[doc = "4: Configure as the IOSLAVE SPI nCE signal value."]
    SLNCE = 4,
    #[doc = "5: PDM serial clock output value."]
    PDMCLK = 5,
    #[doc = "6: Configure as the UART0 CTS input signal value."]
    UA0CTS = 6,
    #[doc = "7: Configure as the UART1 TX output signal value."]
    UART1TX = 7,
}
impl From<PAD12FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD12FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD12FNCSEL`"]
pub type PAD12FNCSEL_R = crate::R<u8, PAD12FNCSEL_A>;
impl PAD12FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD12FNCSEL_A {
        match self.bits {
            0 => PAD12FNCSEL_A::ADCD0NSE9,
            1 => PAD12FNCSEL_A::NCE12,
            2 => PAD12FNCSEL_A::CT0,
            3 => PAD12FNCSEL_A::GPIO12,
            4 => PAD12FNCSEL_A::SLNCE,
            5 => PAD12FNCSEL_A::PDMCLK,
            6 => PAD12FNCSEL_A::UA0CTS,
            7 => PAD12FNCSEL_A::UART1TX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCD0NSE9`"]
    #[inline(always)]
    pub fn is_adcd0nse9(&self) -> bool {
        *self == PAD12FNCSEL_A::ADCD0NSE9
    }
    #[doc = "Checks if the value of the field is `NCE12`"]
    #[inline(always)]
    pub fn is_nce12(&self) -> bool {
        *self == PAD12FNCSEL_A::NCE12
    }
    #[doc = "Checks if the value of the field is `CT0`"]
    #[inline(always)]
    pub fn is_ct0(&self) -> bool {
        *self == PAD12FNCSEL_A::CT0
    }
    #[doc = "Checks if the value of the field is `GPIO12`"]
    #[inline(always)]
    pub fn is_gpio12(&self) -> bool {
        *self == PAD12FNCSEL_A::GPIO12
    }
    #[doc = "Checks if the value of the field is `SLNCE`"]
    #[inline(always)]
    pub fn is_sln_ce(&self) -> bool {
        *self == PAD12FNCSEL_A::SLNCE
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline(always)]
    pub fn is_pdmclk(&self) -> bool {
        *self == PAD12FNCSEL_A::PDMCLK
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD12FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD12FNCSEL_A::UART1TX
    }
}
#[doc = "Write proxy for field `PAD12FNCSEL`"]
pub struct PAD12FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Differential pair 0 N, or Single Ended input 9 analog input signal. Determination of the D0N vs SE9 usage is done when the particular channel is selected within the ADC module value."]
    #[inline(always)]
    pub fn adcd0nse9(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::ADCD0NSE9)
    }
    #[doc = "IOM/MSPI nCE group 12 value."]
    #[inline(always)]
    pub fn nce12(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::NCE12)
    }
    #[doc = "CTIMER connection 0 value."]
    #[inline(always)]
    pub fn ct0(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::CT0)
    }
    #[doc = "Configure as GPIO12 value."]
    #[inline(always)]
    pub fn gpio12(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::GPIO12)
    }
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    #[inline(always)]
    pub fn sln_ce(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::SLNCE)
    }
    #[doc = "PDM serial clock output value."]
    #[inline(always)]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::PDMCLK)
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::UA0CTS)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::UART1TX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 12 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD12STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD12STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD12STRNG`"]
pub type PAD12STRNG_R = crate::R<bool, PAD12STRNG_A>;
impl PAD12STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD12STRNG_A {
        match self.bits {
            false => PAD12STRNG_A::LOW,
            true => PAD12STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD12STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD12STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD12STRNG`"]
pub struct PAD12STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD12STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD12STRNG_A::HIGH)
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
#[doc = "Pad 12 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD12INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD12INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD12INPEN`"]
pub type PAD12INPEN_R = crate::R<bool, PAD12INPEN_A>;
impl PAD12INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD12INPEN_A {
        match self.bits {
            false => PAD12INPEN_A::DIS,
            true => PAD12INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD12INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD12INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD12INPEN`"]
pub struct PAD12INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD12INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD12INPEN_A::EN)
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
#[doc = "Pad 12 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD12PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD12PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD12PULL`"]
pub type PAD12PULL_R = crate::R<bool, PAD12PULL_A>;
impl PAD12PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD12PULL_A {
        match self.bits {
            false => PAD12PULL_A::DIS,
            true => PAD12PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD12PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD12PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD12PULL`"]
pub struct PAD12PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD12PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD12PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 15 function select"]
    #[inline(always)]
    pub fn pad15fncsel(&self) -> PAD15FNCSEL_R {
        PAD15FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 15 drive strength"]
    #[inline(always)]
    pub fn pad15strng(&self) -> PAD15STRNG_R {
        PAD15STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 15 input enable"]
    #[inline(always)]
    pub fn pad15inpen(&self) -> PAD15INPEN_R {
        PAD15INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 15 pullup enable"]
    #[inline(always)]
    pub fn pad15pull(&self) -> PAD15PULL_R {
        PAD15PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 14 function select"]
    #[inline(always)]
    pub fn pad14fncsel(&self) -> PAD14FNCSEL_R {
        PAD14FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 14 drive strength"]
    #[inline(always)]
    pub fn pad14strng(&self) -> PAD14STRNG_R {
        PAD14STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 14 input enable"]
    #[inline(always)]
    pub fn pad14inpen(&self) -> PAD14INPEN_R {
        PAD14INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 14 pullup enable"]
    #[inline(always)]
    pub fn pad14pull(&self) -> PAD14PULL_R {
        PAD14PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 13 function select"]
    #[inline(always)]
    pub fn pad13fncsel(&self) -> PAD13FNCSEL_R {
        PAD13FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 13 drive strength"]
    #[inline(always)]
    pub fn pad13strng(&self) -> PAD13STRNG_R {
        PAD13STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 13 input enable"]
    #[inline(always)]
    pub fn pad13inpen(&self) -> PAD13INPEN_R {
        PAD13INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 13 pullup enable"]
    #[inline(always)]
    pub fn pad13pull(&self) -> PAD13PULL_R {
        PAD13PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 12 function select"]
    #[inline(always)]
    pub fn pad12fncsel(&self) -> PAD12FNCSEL_R {
        PAD12FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 12 drive strength"]
    #[inline(always)]
    pub fn pad12strng(&self) -> PAD12STRNG_R {
        PAD12STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 12 input enable"]
    #[inline(always)]
    pub fn pad12inpen(&self) -> PAD12INPEN_R {
        PAD12INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 12 pullup enable"]
    #[inline(always)]
    pub fn pad12pull(&self) -> PAD12PULL_R {
        PAD12PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 15 function select"]
    #[inline(always)]
    pub fn pad15fncsel(&mut self) -> PAD15FNCSEL_W {
        PAD15FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 15 drive strength"]
    #[inline(always)]
    pub fn pad15strng(&mut self) -> PAD15STRNG_W {
        PAD15STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 15 input enable"]
    #[inline(always)]
    pub fn pad15inpen(&mut self) -> PAD15INPEN_W {
        PAD15INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 15 pullup enable"]
    #[inline(always)]
    pub fn pad15pull(&mut self) -> PAD15PULL_W {
        PAD15PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 14 function select"]
    #[inline(always)]
    pub fn pad14fncsel(&mut self) -> PAD14FNCSEL_W {
        PAD14FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 14 drive strength"]
    #[inline(always)]
    pub fn pad14strng(&mut self) -> PAD14STRNG_W {
        PAD14STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 14 input enable"]
    #[inline(always)]
    pub fn pad14inpen(&mut self) -> PAD14INPEN_W {
        PAD14INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 14 pullup enable"]
    #[inline(always)]
    pub fn pad14pull(&mut self) -> PAD14PULL_W {
        PAD14PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 13 function select"]
    #[inline(always)]
    pub fn pad13fncsel(&mut self) -> PAD13FNCSEL_W {
        PAD13FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 13 drive strength"]
    #[inline(always)]
    pub fn pad13strng(&mut self) -> PAD13STRNG_W {
        PAD13STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 13 input enable"]
    #[inline(always)]
    pub fn pad13inpen(&mut self) -> PAD13INPEN_W {
        PAD13INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 13 pullup enable"]
    #[inline(always)]
    pub fn pad13pull(&mut self) -> PAD13PULL_W {
        PAD13PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 12 function select"]
    #[inline(always)]
    pub fn pad12fncsel(&mut self) -> PAD12FNCSEL_W {
        PAD12FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 12 drive strength"]
    #[inline(always)]
    pub fn pad12strng(&mut self) -> PAD12STRNG_W {
        PAD12STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 12 input enable"]
    #[inline(always)]
    pub fn pad12inpen(&mut self) -> PAD12INPEN_W {
        PAD12INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 12 pullup enable"]
    #[inline(always)]
    pub fn pad12pull(&mut self) -> PAD12PULL_W {
        PAD12PULL_W { w: self }
    }
}
