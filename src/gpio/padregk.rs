#[doc = "Reader of register PADREGK"]
pub type R = crate::R<u32, super::PADREGK>;
#[doc = "Writer for register PADREGK"]
pub type W = crate::W<u32, super::PADREGK>;
#[doc = "Register PADREGK `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 43 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD43RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD43RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD43RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD43RSEL`"]
pub type PAD43RSEL_R = crate::R<u8, PAD43RSEL_A>;
impl PAD43RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD43RSEL_A {
        match self.bits {
            0 => PAD43RSEL_A::PULL1_5K,
            1 => PAD43RSEL_A::PULL6K,
            2 => PAD43RSEL_A::PULL12K,
            3 => PAD43RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD43RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD43RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD43RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD43RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD43RSEL`"]
pub struct PAD43RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD43RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD43RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD43RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD43RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Pad 43 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD43FNCSEL_A {
    #[doc = "0: Configure as the UART1 RX input signal value."]
    UART1RX = 0,
    #[doc = "1: IOM/MSPI nCE group 43 value."]
    NCE43 = 1,
    #[doc = "2: CTIMER connection 18 value."]
    CT18 = 2,
    #[doc = "3: Configure as GPIO43 value."]
    GPIO43 = 3,
    #[doc = "4: Configure as the IOMSTR3 I2C SDA or SPI WIR3 signal value."]
    M3SDAWIR3 = 4,
    #[doc = "5: Configure as the IOMSTR3 SPI MISO signal value."]
    M3MISO = 5,
}
impl From<PAD43FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD43FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD43FNCSEL`"]
pub type PAD43FNCSEL_R = crate::R<u8, PAD43FNCSEL_A>;
impl PAD43FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD43FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD43FNCSEL_A::UART1RX),
            1 => Val(PAD43FNCSEL_A::NCE43),
            2 => Val(PAD43FNCSEL_A::CT18),
            3 => Val(PAD43FNCSEL_A::GPIO43),
            4 => Val(PAD43FNCSEL_A::M3SDAWIR3),
            5 => Val(PAD43FNCSEL_A::M3MISO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD43FNCSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `NCE43`"]
    #[inline(always)]
    pub fn is_nce43(&self) -> bool {
        *self == PAD43FNCSEL_A::NCE43
    }
    #[doc = "Checks if the value of the field is `CT18`"]
    #[inline(always)]
    pub fn is_ct18(&self) -> bool {
        *self == PAD43FNCSEL_A::CT18
    }
    #[doc = "Checks if the value of the field is `GPIO43`"]
    #[inline(always)]
    pub fn is_gpio43(&self) -> bool {
        *self == PAD43FNCSEL_A::GPIO43
    }
    #[doc = "Checks if the value of the field is `M3SDAWIR3`"]
    #[inline(always)]
    pub fn is_m3sdawir3(&self) -> bool {
        *self == PAD43FNCSEL_A::M3SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M3MISO`"]
    #[inline(always)]
    pub fn is_m3miso(&self) -> bool {
        *self == PAD43FNCSEL_A::M3MISO
    }
}
#[doc = "Write proxy for field `PAD43FNCSEL`"]
pub struct PAD43FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::UART1RX)
    }
    #[doc = "IOM/MSPI nCE group 43 value."]
    #[inline(always)]
    pub fn nce43(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::NCE43)
    }
    #[doc = "CTIMER connection 18 value."]
    #[inline(always)]
    pub fn ct18(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::CT18)
    }
    #[doc = "Configure as GPIO43 value."]
    #[inline(always)]
    pub fn gpio43(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::GPIO43)
    }
    #[doc = "Configure as the IOMSTR3 I2C SDA or SPI WIR3 signal value."]
    #[inline(always)]
    pub fn m3sdawir3(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::M3SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR3 SPI MISO signal value."]
    #[inline(always)]
    pub fn m3miso(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::M3MISO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 43 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD43STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD43STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD43STRNG`"]
pub type PAD43STRNG_R = crate::R<bool, PAD43STRNG_A>;
impl PAD43STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD43STRNG_A {
        match self.bits {
            false => PAD43STRNG_A::LOW,
            true => PAD43STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD43STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD43STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD43STRNG`"]
pub struct PAD43STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD43STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD43STRNG_A::HIGH)
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
#[doc = "Pad 43 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD43INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD43INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD43INPEN`"]
pub type PAD43INPEN_R = crate::R<bool, PAD43INPEN_A>;
impl PAD43INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD43INPEN_A {
        match self.bits {
            false => PAD43INPEN_A::DIS,
            true => PAD43INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD43INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD43INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD43INPEN`"]
pub struct PAD43INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD43INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD43INPEN_A::EN)
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
#[doc = "Pad 43 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD43PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD43PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD43PULL`"]
pub type PAD43PULL_R = crate::R<bool, PAD43PULL_A>;
impl PAD43PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD43PULL_A {
        match self.bits {
            false => PAD43PULL_A::DIS,
            true => PAD43PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD43PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD43PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD43PULL`"]
pub struct PAD43PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD43PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD43PULL_A::EN)
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
#[doc = "Pad 42 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD42RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD42RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD42RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD42RSEL`"]
pub type PAD42RSEL_R = crate::R<u8, PAD42RSEL_A>;
impl PAD42RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD42RSEL_A {
        match self.bits {
            0 => PAD42RSEL_A::PULL1_5K,
            1 => PAD42RSEL_A::PULL6K,
            2 => PAD42RSEL_A::PULL12K,
            3 => PAD42RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD42RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD42RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD42RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD42RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD42RSEL`"]
pub struct PAD42RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD42RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD42RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD42RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD42RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Pad 42 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD42FNCSEL_A {
    #[doc = "0: Configure as the UART1 TX output signal value."]
    UART1TX = 0,
    #[doc = "1: IOM/MSPI nCE group 42 value."]
    NCE42 = 1,
    #[doc = "2: CTIMER connection 16 value."]
    CT16 = 2,
    #[doc = "3: Configure as GPIO42 value."]
    GPIO42 = 3,
    #[doc = "4: Configure as the IOMSTR3 I2C SCL clock I/O signal value."]
    M3SCL = 4,
    #[doc = "5: Configure as the IOMSTR3 SPI SCK output value."]
    M3SCK = 5,
}
impl From<PAD42FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD42FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD42FNCSEL`"]
pub type PAD42FNCSEL_R = crate::R<u8, PAD42FNCSEL_A>;
impl PAD42FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD42FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD42FNCSEL_A::UART1TX),
            1 => Val(PAD42FNCSEL_A::NCE42),
            2 => Val(PAD42FNCSEL_A::CT16),
            3 => Val(PAD42FNCSEL_A::GPIO42),
            4 => Val(PAD42FNCSEL_A::M3SCL),
            5 => Val(PAD42FNCSEL_A::M3SCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD42FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `NCE42`"]
    #[inline(always)]
    pub fn is_nce42(&self) -> bool {
        *self == PAD42FNCSEL_A::NCE42
    }
    #[doc = "Checks if the value of the field is `CT16`"]
    #[inline(always)]
    pub fn is_ct16(&self) -> bool {
        *self == PAD42FNCSEL_A::CT16
    }
    #[doc = "Checks if the value of the field is `GPIO42`"]
    #[inline(always)]
    pub fn is_gpio42(&self) -> bool {
        *self == PAD42FNCSEL_A::GPIO42
    }
    #[doc = "Checks if the value of the field is `M3SCL`"]
    #[inline(always)]
    pub fn is_m3scl(&self) -> bool {
        *self == PAD42FNCSEL_A::M3SCL
    }
    #[doc = "Checks if the value of the field is `M3SCK`"]
    #[inline(always)]
    pub fn is_m3sck(&self) -> bool {
        *self == PAD42FNCSEL_A::M3SCK
    }
}
#[doc = "Write proxy for field `PAD42FNCSEL`"]
pub struct PAD42FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::UART1TX)
    }
    #[doc = "IOM/MSPI nCE group 42 value."]
    #[inline(always)]
    pub fn nce42(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::NCE42)
    }
    #[doc = "CTIMER connection 16 value."]
    #[inline(always)]
    pub fn ct16(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::CT16)
    }
    #[doc = "Configure as GPIO42 value."]
    #[inline(always)]
    pub fn gpio42(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::GPIO42)
    }
    #[doc = "Configure as the IOMSTR3 I2C SCL clock I/O signal value."]
    #[inline(always)]
    pub fn m3scl(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::M3SCL)
    }
    #[doc = "Configure as the IOMSTR3 SPI SCK output value."]
    #[inline(always)]
    pub fn m3sck(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::M3SCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 42 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD42STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD42STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD42STRNG`"]
pub type PAD42STRNG_R = crate::R<bool, PAD42STRNG_A>;
impl PAD42STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD42STRNG_A {
        match self.bits {
            false => PAD42STRNG_A::LOW,
            true => PAD42STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD42STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD42STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD42STRNG`"]
pub struct PAD42STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD42STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD42STRNG_A::HIGH)
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
#[doc = "Pad 42 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD42INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD42INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD42INPEN`"]
pub type PAD42INPEN_R = crate::R<bool, PAD42INPEN_A>;
impl PAD42INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD42INPEN_A {
        match self.bits {
            false => PAD42INPEN_A::DIS,
            true => PAD42INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD42INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD42INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD42INPEN`"]
pub struct PAD42INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD42INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD42INPEN_A::EN)
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
#[doc = "Pad 42 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD42PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD42PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD42PULL`"]
pub type PAD42PULL_R = crate::R<bool, PAD42PULL_A>;
impl PAD42PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD42PULL_A {
        match self.bits {
            false => PAD42PULL_A::DIS,
            true => PAD42PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD42PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD42PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD42PULL`"]
pub struct PAD42PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD42PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD42PULL_A::EN)
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
#[doc = "Pad 41 power switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41PWRDN_A {
    #[doc = "0: Power switch disabled value."]
    DIS = 0,
    #[doc = "1: Power switch enabled (Switch pad to VSS) value."]
    EN = 1,
}
impl From<PAD41PWRDN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41PWRDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD41PWRDN`"]
pub type PAD41PWRDN_R = crate::R<bool, PAD41PWRDN_A>;
impl PAD41PWRDN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD41PWRDN_A {
        match self.bits {
            false => PAD41PWRDN_A::DIS,
            true => PAD41PWRDN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD41PWRDN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD41PWRDN_A::EN
    }
}
#[doc = "Write proxy for field `PAD41PWRDN`"]
pub struct PAD41PWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41PWRDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41PWRDN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power switch disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41PWRDN_A::DIS)
    }
    #[doc = "Power switch enabled (Switch pad to VSS) value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD41PWRDN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Pad 41 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD41FNCSEL_A {
    #[doc = "0: IOM/MSPI nCE group 41 value."]
    NCE41 = 0,
    #[doc = "2: Configure as the serial wire debug SWO signal value."]
    SWO = 2,
    #[doc = "3: Configure as GPIO41 value."]
    GPIO41 = 3,
    #[doc = "4: I2S word clock input value."]
    I2SWCLK = 4,
    #[doc = "5: Configure as the UART1 RTS output signal value."]
    UA1RTS = 5,
    #[doc = "6: Configure as the UART0 TX output signal value."]
    UART0TX = 6,
    #[doc = "7: Configure as the UART0 RTS output signal value."]
    UA0RTS = 7,
}
impl From<PAD41FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD41FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD41FNCSEL`"]
pub type PAD41FNCSEL_R = crate::R<u8, PAD41FNCSEL_A>;
impl PAD41FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD41FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD41FNCSEL_A::NCE41),
            2 => Val(PAD41FNCSEL_A::SWO),
            3 => Val(PAD41FNCSEL_A::GPIO41),
            4 => Val(PAD41FNCSEL_A::I2SWCLK),
            5 => Val(PAD41FNCSEL_A::UA1RTS),
            6 => Val(PAD41FNCSEL_A::UART0TX),
            7 => Val(PAD41FNCSEL_A::UA0RTS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NCE41`"]
    #[inline(always)]
    pub fn is_nce41(&self) -> bool {
        *self == PAD41FNCSEL_A::NCE41
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD41FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `GPIO41`"]
    #[inline(always)]
    pub fn is_gpio41(&self) -> bool {
        *self == PAD41FNCSEL_A::GPIO41
    }
    #[doc = "Checks if the value of the field is `I2SWCLK`"]
    #[inline(always)]
    pub fn is_i2swclk(&self) -> bool {
        *self == PAD41FNCSEL_A::I2SWCLK
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD41FNCSEL_A::UA1RTS
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD41FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD41FNCSEL_A::UA0RTS
    }
}
#[doc = "Write proxy for field `PAD41FNCSEL`"]
pub struct PAD41FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IOM/MSPI nCE group 41 value."]
    #[inline(always)]
    pub fn nce41(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::NCE41)
    }
    #[doc = "Configure as the serial wire debug SWO signal value."]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::SWO)
    }
    #[doc = "Configure as GPIO41 value."]
    #[inline(always)]
    pub fn gpio41(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::GPIO41)
    }
    #[doc = "I2S word clock input value."]
    #[inline(always)]
    pub fn i2swclk(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::I2SWCLK)
    }
    #[doc = "Configure as the UART1 RTS output signal value."]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::UA1RTS)
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART0 RTS output signal value."]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::UA0RTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 41 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD41STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD41STRNG`"]
pub type PAD41STRNG_R = crate::R<bool, PAD41STRNG_A>;
impl PAD41STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD41STRNG_A {
        match self.bits {
            false => PAD41STRNG_A::LOW,
            true => PAD41STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD41STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD41STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD41STRNG`"]
pub struct PAD41STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD41STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD41STRNG_A::HIGH)
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
#[doc = "Pad 41 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD41INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD41INPEN`"]
pub type PAD41INPEN_R = crate::R<bool, PAD41INPEN_A>;
impl PAD41INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD41INPEN_A {
        match self.bits {
            false => PAD41INPEN_A::DIS,
            true => PAD41INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD41INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD41INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD41INPEN`"]
pub struct PAD41INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD41INPEN_A::EN)
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
#[doc = "Pad 41 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD41PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD41PULL`"]
pub type PAD41PULL_R = crate::R<bool, PAD41PULL_A>;
impl PAD41PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD41PULL_A {
        match self.bits {
            false => PAD41PULL_A::DIS,
            true => PAD41PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD41PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD41PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD41PULL`"]
pub struct PAD41PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD41PULL_A::EN)
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
#[doc = "Pad 40 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD40RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD40RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD40RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD40RSEL`"]
pub type PAD40RSEL_R = crate::R<u8, PAD40RSEL_A>;
impl PAD40RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD40RSEL_A {
        match self.bits {
            0 => PAD40RSEL_A::PULL1_5K,
            1 => PAD40RSEL_A::PULL6K,
            2 => PAD40RSEL_A::PULL12K,
            3 => PAD40RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD40RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD40RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD40RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD40RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD40RSEL`"]
pub struct PAD40RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD40RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD40RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD40RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD40RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pad 40 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD40FNCSEL_A {
    #[doc = "0: Configure as the UART0 RX input signal value."]
    UART0RX = 0,
    #[doc = "1: Configure as the UART1 RX input signal value."]
    UART1RX = 1,
    #[doc = "2: Configure as the ADC Trigger 0 signal value."]
    TRIG0 = 2,
    #[doc = "3: Configure as GPIO40 value."]
    GPIO40 = 3,
    #[doc = "4: Configure as the IOMSTR4 I2C SDA or SPI WIR3 signal value."]
    M4SDAWIR3 = 4,
    #[doc = "5: Configure as the IOMSTR4 SPI MISO input signal value."]
    M4MISO = 5,
}
impl From<PAD40FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD40FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD40FNCSEL`"]
pub type PAD40FNCSEL_R = crate::R<u8, PAD40FNCSEL_A>;
impl PAD40FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD40FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD40FNCSEL_A::UART0RX),
            1 => Val(PAD40FNCSEL_A::UART1RX),
            2 => Val(PAD40FNCSEL_A::TRIG0),
            3 => Val(PAD40FNCSEL_A::GPIO40),
            4 => Val(PAD40FNCSEL_A::M4SDAWIR3),
            5 => Val(PAD40FNCSEL_A::M4MISO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD40FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD40FNCSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `TRIG0`"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == PAD40FNCSEL_A::TRIG0
    }
    #[doc = "Checks if the value of the field is `GPIO40`"]
    #[inline(always)]
    pub fn is_gpio40(&self) -> bool {
        *self == PAD40FNCSEL_A::GPIO40
    }
    #[doc = "Checks if the value of the field is `M4SDAWIR3`"]
    #[inline(always)]
    pub fn is_m4sdawir3(&self) -> bool {
        *self == PAD40FNCSEL_A::M4SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M4MISO`"]
    #[inline(always)]
    pub fn is_m4miso(&self) -> bool {
        *self == PAD40FNCSEL_A::M4MISO
    }
}
#[doc = "Write proxy for field `PAD40FNCSEL`"]
pub struct PAD40FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::UART1RX)
    }
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::TRIG0)
    }
    #[doc = "Configure as GPIO40 value."]
    #[inline(always)]
    pub fn gpio40(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::GPIO40)
    }
    #[doc = "Configure as the IOMSTR4 I2C SDA or SPI WIR3 signal value."]
    #[inline(always)]
    pub fn m4sdawir3(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::M4SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR4 SPI MISO input signal value."]
    #[inline(always)]
    pub fn m4miso(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::M4MISO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 40 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD40STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD40STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD40STRNG`"]
pub type PAD40STRNG_R = crate::R<bool, PAD40STRNG_A>;
impl PAD40STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD40STRNG_A {
        match self.bits {
            false => PAD40STRNG_A::LOW,
            true => PAD40STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD40STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD40STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD40STRNG`"]
pub struct PAD40STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD40STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD40STRNG_A::HIGH)
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
#[doc = "Pad 40 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD40INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD40INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD40INPEN`"]
pub type PAD40INPEN_R = crate::R<bool, PAD40INPEN_A>;
impl PAD40INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD40INPEN_A {
        match self.bits {
            false => PAD40INPEN_A::DIS,
            true => PAD40INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD40INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD40INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD40INPEN`"]
pub struct PAD40INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD40INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD40INPEN_A::EN)
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
#[doc = "Pad 40 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD40PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD40PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD40PULL`"]
pub type PAD40PULL_R = crate::R<bool, PAD40PULL_A>;
impl PAD40PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD40PULL_A {
        match self.bits {
            false => PAD40PULL_A::DIS,
            true => PAD40PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD40PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD40PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD40PULL`"]
pub struct PAD40PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD40PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD40PULL_A::EN)
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
    #[doc = "Bits 30:31 - Pad 43 pullup resistor selection."]
    #[inline(always)]
    pub fn pad43rsel(&self) -> PAD43RSEL_R {
        PAD43RSEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:29 - Pad 43 function select"]
    #[inline(always)]
    pub fn pad43fncsel(&self) -> PAD43FNCSEL_R {
        PAD43FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 43 drive strength"]
    #[inline(always)]
    pub fn pad43strng(&self) -> PAD43STRNG_R {
        PAD43STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 43 input enable"]
    #[inline(always)]
    pub fn pad43inpen(&self) -> PAD43INPEN_R {
        PAD43INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 43 pullup enable"]
    #[inline(always)]
    pub fn pad43pull(&self) -> PAD43PULL_R {
        PAD43PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Pad 42 pullup resistor selection."]
    #[inline(always)]
    pub fn pad42rsel(&self) -> PAD42RSEL_R {
        PAD42RSEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 19:21 - Pad 42 function select"]
    #[inline(always)]
    pub fn pad42fncsel(&self) -> PAD42FNCSEL_R {
        PAD42FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 42 drive strength"]
    #[inline(always)]
    pub fn pad42strng(&self) -> PAD42STRNG_R {
        PAD42STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 42 input enable"]
    #[inline(always)]
    pub fn pad42inpen(&self) -> PAD42INPEN_R {
        PAD42INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 42 pullup enable"]
    #[inline(always)]
    pub fn pad42pull(&self) -> PAD42PULL_R {
        PAD42PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pad 41 power switch enable"]
    #[inline(always)]
    pub fn pad41pwrdn(&self) -> PAD41PWRDN_R {
        PAD41PWRDN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 41 function select"]
    #[inline(always)]
    pub fn pad41fncsel(&self) -> PAD41FNCSEL_R {
        PAD41FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 41 drive strength"]
    #[inline(always)]
    pub fn pad41strng(&self) -> PAD41STRNG_R {
        PAD41STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 41 input enable"]
    #[inline(always)]
    pub fn pad41inpen(&self) -> PAD41INPEN_R {
        PAD41INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 41 pullup enable"]
    #[inline(always)]
    pub fn pad41pull(&self) -> PAD41PULL_R {
        PAD41PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pad 40 pullup resistor selection."]
    #[inline(always)]
    pub fn pad40rsel(&self) -> PAD40RSEL_R {
        PAD40RSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Pad 40 function select"]
    #[inline(always)]
    pub fn pad40fncsel(&self) -> PAD40FNCSEL_R {
        PAD40FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 40 drive strength"]
    #[inline(always)]
    pub fn pad40strng(&self) -> PAD40STRNG_R {
        PAD40STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 40 input enable"]
    #[inline(always)]
    pub fn pad40inpen(&self) -> PAD40INPEN_R {
        PAD40INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 40 pullup enable"]
    #[inline(always)]
    pub fn pad40pull(&self) -> PAD40PULL_R {
        PAD40PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - Pad 43 pullup resistor selection."]
    #[inline(always)]
    pub fn pad43rsel(&mut self) -> PAD43RSEL_W {
        PAD43RSEL_W { w: self }
    }
    #[doc = "Bits 27:29 - Pad 43 function select"]
    #[inline(always)]
    pub fn pad43fncsel(&mut self) -> PAD43FNCSEL_W {
        PAD43FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 43 drive strength"]
    #[inline(always)]
    pub fn pad43strng(&mut self) -> PAD43STRNG_W {
        PAD43STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 43 input enable"]
    #[inline(always)]
    pub fn pad43inpen(&mut self) -> PAD43INPEN_W {
        PAD43INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 43 pullup enable"]
    #[inline(always)]
    pub fn pad43pull(&mut self) -> PAD43PULL_W {
        PAD43PULL_W { w: self }
    }
    #[doc = "Bits 22:23 - Pad 42 pullup resistor selection."]
    #[inline(always)]
    pub fn pad42rsel(&mut self) -> PAD42RSEL_W {
        PAD42RSEL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 42 function select"]
    #[inline(always)]
    pub fn pad42fncsel(&mut self) -> PAD42FNCSEL_W {
        PAD42FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 42 drive strength"]
    #[inline(always)]
    pub fn pad42strng(&mut self) -> PAD42STRNG_W {
        PAD42STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 42 input enable"]
    #[inline(always)]
    pub fn pad42inpen(&mut self) -> PAD42INPEN_W {
        PAD42INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 42 pullup enable"]
    #[inline(always)]
    pub fn pad42pull(&mut self) -> PAD42PULL_W {
        PAD42PULL_W { w: self }
    }
    #[doc = "Bit 15 - Pad 41 power switch enable"]
    #[inline(always)]
    pub fn pad41pwrdn(&mut self) -> PAD41PWRDN_W {
        PAD41PWRDN_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 41 function select"]
    #[inline(always)]
    pub fn pad41fncsel(&mut self) -> PAD41FNCSEL_W {
        PAD41FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 41 drive strength"]
    #[inline(always)]
    pub fn pad41strng(&mut self) -> PAD41STRNG_W {
        PAD41STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 41 input enable"]
    #[inline(always)]
    pub fn pad41inpen(&mut self) -> PAD41INPEN_W {
        PAD41INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 41 pullup enable"]
    #[inline(always)]
    pub fn pad41pull(&mut self) -> PAD41PULL_W {
        PAD41PULL_W { w: self }
    }
    #[doc = "Bits 6:7 - Pad 40 pullup resistor selection."]
    #[inline(always)]
    pub fn pad40rsel(&mut self) -> PAD40RSEL_W {
        PAD40RSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 40 function select"]
    #[inline(always)]
    pub fn pad40fncsel(&mut self) -> PAD40FNCSEL_W {
        PAD40FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 40 drive strength"]
    #[inline(always)]
    pub fn pad40strng(&mut self) -> PAD40STRNG_W {
        PAD40STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 40 input enable"]
    #[inline(always)]
    pub fn pad40inpen(&mut self) -> PAD40INPEN_W {
        PAD40INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 40 pullup enable"]
    #[inline(always)]
    pub fn pad40pull(&mut self) -> PAD40PULL_W {
        PAD40PULL_W { w: self }
    }
}
