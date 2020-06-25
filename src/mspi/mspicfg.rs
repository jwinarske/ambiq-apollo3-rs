#[doc = "Reader of register MSPICFG"]
pub type R = crate::R<u32, super::MSPICFG>;
#[doc = "Writer for register MSPICFG"]
pub type W = crate::W<u32, super::MSPICFG>;
#[doc = "Register MSPICFG `reset()`'s with value 0xc000_0200"]
impl crate::ResetValue for super::MSPICFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0200
    }
}
#[doc = "Reader of field `PRSTN`"]
pub type PRSTN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSTN`"]
pub struct PRSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSTN_W<'a> {
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
#[doc = "Reader of field `IPRSTN`"]
pub type IPRSTN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPRSTN`"]
pub struct IPRSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPRSTN_W<'a> {
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
#[doc = "Reader of field `FIFORESET`"]
pub type FIFORESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFORESET`"]
pub struct FIFORESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFORESET_W<'a> {
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
#[doc = "Clock Divider. Allows dividing 48 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 48 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "2: 24 MHz MSPI clock value."]
    CLK24 = 2,
    #[doc = "4: 12 MHz MSPI clock value."]
    CLK12 = 4,
    #[doc = "8: 6 MHz MSPI clock value."]
    CLK6 = 8,
    #[doc = "16: 3 MHz MSPI clock value."]
    CLK3 = 16,
    #[doc = "32: 1.5 MHz MSPI clock value."]
    CLK1_5 = 32,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, CLKDIV_A>;
impl CLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKDIV_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(CLKDIV_A::CLK24),
            4 => Val(CLKDIV_A::CLK12),
            8 => Val(CLKDIV_A::CLK6),
            16 => Val(CLKDIV_A::CLK3),
            32 => Val(CLKDIV_A::CLK1_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK24`"]
    #[inline(always)]
    pub fn is_clk24(&self) -> bool {
        *self == CLKDIV_A::CLK24
    }
    #[doc = "Checks if the value of the field is `CLK12`"]
    #[inline(always)]
    pub fn is_clk12(&self) -> bool {
        *self == CLKDIV_A::CLK12
    }
    #[doc = "Checks if the value of the field is `CLK6`"]
    #[inline(always)]
    pub fn is_clk6(&self) -> bool {
        *self == CLKDIV_A::CLK6
    }
    #[doc = "Checks if the value of the field is `CLK3`"]
    #[inline(always)]
    pub fn is_clk3(&self) -> bool {
        *self == CLKDIV_A::CLK3
    }
    #[doc = "Checks if the value of the field is `CLK1_5`"]
    #[inline(always)]
    pub fn is_clk1_5(&self) -> bool {
        *self == CLKDIV_A::CLK1_5
    }
}
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "24 MHz MSPI clock value."]
    #[inline(always)]
    pub fn clk24(self) -> &'a mut W {
        self.variant(CLKDIV_A::CLK24)
    }
    #[doc = "12 MHz MSPI clock value."]
    #[inline(always)]
    pub fn clk12(self) -> &'a mut W {
        self.variant(CLKDIV_A::CLK12)
    }
    #[doc = "6 MHz MSPI clock value."]
    #[inline(always)]
    pub fn clk6(self) -> &'a mut W {
        self.variant(CLKDIV_A::CLK6)
    }
    #[doc = "3 MHz MSPI clock value."]
    #[inline(always)]
    pub fn clk3(self) -> &'a mut W {
        self.variant(CLKDIV_A::CLK3)
    }
    #[doc = "1.5 MHz MSPI clock value."]
    #[inline(always)]
    pub fn clk1_5(self) -> &'a mut W {
        self.variant(CLKDIV_A::CLK1_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Selects which IOM is selected for CQ handshake status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IOMSEL_A {
    #[doc = "0: ERROR: desc VALUE MISSING value."]
    IOM0 = 0,
    #[doc = "1: ERROR: desc VALUE MISSING value."]
    IOM1 = 1,
    #[doc = "2: ERROR: desc VALUE MISSING value."]
    IOM2 = 2,
    #[doc = "3: ERROR: desc VALUE MISSING value."]
    IOM3 = 3,
    #[doc = "4: ERROR: desc VALUE MISSING value."]
    IOM4 = 4,
    #[doc = "5: ERROR: desc VALUE MISSING value."]
    IOM5 = 5,
    #[doc = "7: No IOM selected.  Signals always zero. value."]
    DISABLED = 7,
}
impl From<IOMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IOMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IOMSEL`"]
pub type IOMSEL_R = crate::R<u8, IOMSEL_A>;
impl IOMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IOMSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IOMSEL_A::IOM0),
            1 => Val(IOMSEL_A::IOM1),
            2 => Val(IOMSEL_A::IOM2),
            3 => Val(IOMSEL_A::IOM3),
            4 => Val(IOMSEL_A::IOM4),
            5 => Val(IOMSEL_A::IOM5),
            7 => Val(IOMSEL_A::DISABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IOM0`"]
    #[inline(always)]
    pub fn is_iom0(&self) -> bool {
        *self == IOMSEL_A::IOM0
    }
    #[doc = "Checks if the value of the field is `IOM1`"]
    #[inline(always)]
    pub fn is_iom1(&self) -> bool {
        *self == IOMSEL_A::IOM1
    }
    #[doc = "Checks if the value of the field is `IOM2`"]
    #[inline(always)]
    pub fn is_iom2(&self) -> bool {
        *self == IOMSEL_A::IOM2
    }
    #[doc = "Checks if the value of the field is `IOM3`"]
    #[inline(always)]
    pub fn is_iom3(&self) -> bool {
        *self == IOMSEL_A::IOM3
    }
    #[doc = "Checks if the value of the field is `IOM4`"]
    #[inline(always)]
    pub fn is_iom4(&self) -> bool {
        *self == IOMSEL_A::IOM4
    }
    #[doc = "Checks if the value of the field is `IOM5`"]
    #[inline(always)]
    pub fn is_iom5(&self) -> bool {
        *self == IOMSEL_A::IOM5
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOMSEL_A::DISABLED
    }
}
#[doc = "Write proxy for field `IOMSEL`"]
pub struct IOMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline(always)]
    pub fn iom0(self) -> &'a mut W {
        self.variant(IOMSEL_A::IOM0)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline(always)]
    pub fn iom1(self) -> &'a mut W {
        self.variant(IOMSEL_A::IOM1)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline(always)]
    pub fn iom2(self) -> &'a mut W {
        self.variant(IOMSEL_A::IOM2)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline(always)]
    pub fn iom3(self) -> &'a mut W {
        self.variant(IOMSEL_A::IOM3)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline(always)]
    pub fn iom4(self) -> &'a mut W {
        self.variant(IOMSEL_A::IOM4)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline(always)]
    pub fn iom5(self) -> &'a mut W {
        self.variant(IOMSEL_A::IOM5)
    }
    #[doc = "No IOM selected. Signals always zero. value."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOMSEL_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXNEG_A {
    #[doc = "0: TX launched from posedge internal clock value."]
    NORMAL = 0,
    #[doc = "1: TX data launched from negedge of internal clock value."]
    NEGEDGE = 1,
}
impl From<TXNEG_A> for bool {
    #[inline(always)]
    fn from(variant: TXNEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXNEG`"]
pub type TXNEG_R = crate::R<bool, TXNEG_A>;
impl TXNEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXNEG_A {
        match self.bits {
            false => TXNEG_A::NORMAL,
            true => TXNEG_A::NEGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TXNEG_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == TXNEG_A::NEGEDGE
    }
}
#[doc = "Write proxy for field `TXNEG`"]
pub struct TXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXNEG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TX launched from posedge internal clock value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TXNEG_A::NORMAL)
    }
    #[doc = "TX data launched from negedge of internal clock value."]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(TXNEG_A::NEGEDGE)
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
#[doc = "Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEG_A {
    #[doc = "0: RX data sampled on posedge of internal clock value."]
    NORMAL = 0,
    #[doc = "1: RX data sampled on negedge of internal clock value."]
    NEGEDGE = 1,
}
impl From<RXNEG_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXNEG`"]
pub type RXNEG_R = crate::R<bool, RXNEG_A>;
impl RXNEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNEG_A {
        match self.bits {
            false => RXNEG_A::NORMAL,
            true => RXNEG_A::NEGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RXNEG_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == RXNEG_A::NEGEDGE
    }
}
#[doc = "Write proxy for field `RXNEG`"]
pub struct RXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXNEG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RX data sampled on posedge of internal clock value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RXNEG_A::NORMAL)
    }
    #[doc = "RX data sampled on negedge of internal clock value."]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(RXNEG_A::NEGEDGE)
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
#[doc = "Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCAP_A {
    #[doc = "0: RX Capture phase aligns with CPHA setting value."]
    NORMAL = 0,
    #[doc = "1: RX Capture phase is delayed from CPHA setting by one clock edge value."]
    DELAY = 1,
}
impl From<RXCAP_A> for bool {
    #[inline(always)]
    fn from(variant: RXCAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXCAP`"]
pub type RXCAP_R = crate::R<bool, RXCAP_A>;
impl RXCAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCAP_A {
        match self.bits {
            false => RXCAP_A::NORMAL,
            true => RXCAP_A::DELAY,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RXCAP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `DELAY`"]
    #[inline(always)]
    pub fn is_delay(&self) -> bool {
        *self == RXCAP_A::DELAY
    }
}
#[doc = "Write proxy for field `RXCAP`"]
pub struct RXCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RX Capture phase aligns with CPHA setting value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RXCAP_A::NORMAL)
    }
    #[doc = "RX Capture phase is delayed from CPHA setting by one clock edge value."]
    #[inline(always)]
    pub fn delay(self) -> &'a mut W {
        self.variant(RXCAP_A::DELAY)
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
#[doc = "Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APBCLK_A {
    #[doc = "0: Disable continuous clock. value."]
    DIS = 0,
    #[doc = "1: Enable continuous clock. value."]
    EN = 1,
}
impl From<APBCLK_A> for bool {
    #[inline(always)]
    fn from(variant: APBCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `APBCLK`"]
pub type APBCLK_R = crate::R<bool, APBCLK_A>;
impl APBCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APBCLK_A {
        match self.bits {
            false => APBCLK_A::DIS,
            true => APBCLK_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == APBCLK_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == APBCLK_A::EN
    }
}
#[doc = "Write proxy for field `APBCLK`"]
pub struct APBCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> APBCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APBCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable continuous clock. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(APBCLK_A::DIS)
    }
    #[doc = "Enable continuous clock. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(APBCLK_A::EN)
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
    #[doc = "Bit 31 - Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
    #[inline(always)]
    pub fn prstn(&self) -> PRSTN_R {
        PRSTN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
    #[inline(always)]
    pub fn iprstn(&self) -> IPRSTN_R {
        IPRSTN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
    #[inline(always)]
    pub fn fiforeset(&self) -> FIFORESET_R {
        FIFORESET_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Clock Divider. Allows dividing 48 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 48 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 4:6 - Selects which IOM is selected for CQ handshake status."]
    #[inline(always)]
    pub fn iomsel(&self) -> IOMSEL_R {
        IOMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
    #[inline(always)]
    pub fn txneg(&self) -> TXNEG_R {
        TXNEG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
    #[inline(always)]
    pub fn rxneg(&self) -> RXNEG_R {
        RXNEG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
    #[inline(always)]
    pub fn rxcap(&self) -> RXCAP_R {
        RXCAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
    #[inline(always)]
    pub fn apbclk(&self) -> APBCLK_R {
        APBCLK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
    #[inline(always)]
    pub fn prstn(&mut self) -> PRSTN_W {
        PRSTN_W { w: self }
    }
    #[doc = "Bit 30 - IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
    #[inline(always)]
    pub fn iprstn(&mut self) -> IPRSTN_W {
        IPRSTN_W { w: self }
    }
    #[doc = "Bit 29 - Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
    #[inline(always)]
    pub fn fiforeset(&mut self) -> FIFORESET_W {
        FIFORESET_W { w: self }
    }
    #[doc = "Bits 8:13 - Clock Divider. Allows dividing 48 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 48 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bits 4:6 - Selects which IOM is selected for CQ handshake status."]
    #[inline(always)]
    pub fn iomsel(&mut self) -> IOMSEL_W {
        IOMSEL_W { w: self }
    }
    #[doc = "Bit 3 - Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
    #[inline(always)]
    pub fn txneg(&mut self) -> TXNEG_W {
        TXNEG_W { w: self }
    }
    #[doc = "Bit 2 - Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
    #[inline(always)]
    pub fn rxneg(&mut self) -> RXNEG_W {
        RXNEG_W { w: self }
    }
    #[doc = "Bit 1 - Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
    #[inline(always)]
    pub fn rxcap(&mut self) -> RXCAP_W {
        RXCAP_W { w: self }
    }
    #[doc = "Bit 0 - Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
    #[inline(always)]
    pub fn apbclk(&mut self) -> APBCLK_W {
        APBCLK_W { w: self }
    }
}
