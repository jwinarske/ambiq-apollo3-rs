#[doc = "Reader of register DEVPWREN"]
pub type R = crate::R<u32, super::DEVPWREN>;
#[doc = "Writer for register DEVPWREN"]
pub type W = crate::W<u32, super::DEVPWREN>;
#[doc = "Register DEVPWREN `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVPWREN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power up BLE controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRBLEL_A {
    #[doc = "1: Power up BLE controller value."]
    EN = 1,
    #[doc = "0: Power down BLE controller value."]
    DIS = 0,
}
impl From<PWRBLEL_A> for bool {
    #[inline(always)]
    fn from(variant: PWRBLEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRBLEL`"]
pub type PWRBLEL_R = crate::R<bool, PWRBLEL_A>;
impl PWRBLEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRBLEL_A {
        match self.bits {
            true => PWRBLEL_A::EN,
            false => PWRBLEL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRBLEL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRBLEL_A::DIS
    }
}
#[doc = "Write proxy for field `PWRBLEL`"]
pub struct PWRBLEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRBLEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRBLEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up BLE controller value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRBLEL_A::EN)
    }
    #[doc = "Power down BLE controller value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRBLEL_A::DIS)
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
#[doc = "Power up PDM block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRPDM_A {
    #[doc = "1: Power up PDM value."]
    EN = 1,
    #[doc = "0: Power down PDM value."]
    DIS = 0,
}
impl From<PWRPDM_A> for bool {
    #[inline(always)]
    fn from(variant: PWRPDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRPDM`"]
pub type PWRPDM_R = crate::R<bool, PWRPDM_A>;
impl PWRPDM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRPDM_A {
        match self.bits {
            true => PWRPDM_A::EN,
            false => PWRPDM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRPDM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRPDM_A::DIS
    }
}
#[doc = "Write proxy for field `PWRPDM`"]
pub struct PWRPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRPDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRPDM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up PDM value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRPDM_A::EN)
    }
    #[doc = "Power down PDM value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRPDM_A::DIS)
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
#[doc = "Power up MSPI Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRMSPI_A {
    #[doc = "1: Power up MSPI value."]
    EN = 1,
    #[doc = "0: Power down MSPI value."]
    DIS = 0,
}
impl From<PWRMSPI_A> for bool {
    #[inline(always)]
    fn from(variant: PWRMSPI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRMSPI`"]
pub type PWRMSPI_R = crate::R<bool, PWRMSPI_A>;
impl PWRMSPI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRMSPI_A {
        match self.bits {
            true => PWRMSPI_A::EN,
            false => PWRMSPI_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRMSPI_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRMSPI_A::DIS
    }
}
#[doc = "Write proxy for field `PWRMSPI`"]
pub struct PWRMSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRMSPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRMSPI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up MSPI value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRMSPI_A::EN)
    }
    #[doc = "Power down MSPI value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRMSPI_A::DIS)
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
#[doc = "Power up SCARD Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSCARD_A {
    #[doc = "1: Power up SCARD value."]
    EN = 1,
    #[doc = "0: Power down SCARD value."]
    DIS = 0,
}
impl From<PWRSCARD_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSCARD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRSCARD`"]
pub type PWRSCARD_R = crate::R<bool, PWRSCARD_A>;
impl PWRSCARD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSCARD_A {
        match self.bits {
            true => PWRSCARD_A::EN,
            false => PWRSCARD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRSCARD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRSCARD_A::DIS
    }
}
#[doc = "Write proxy for field `PWRSCARD`"]
pub struct PWRSCARD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSCARD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSCARD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up SCARD value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRSCARD_A::EN)
    }
    #[doc = "Power down SCARD value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRSCARD_A::DIS)
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
#[doc = "Power up ADC Digital Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRADC_A {
    #[doc = "1: Power up ADC value."]
    EN = 1,
    #[doc = "0: Power Down ADC value."]
    DIS = 0,
}
impl From<PWRADC_A> for bool {
    #[inline(always)]
    fn from(variant: PWRADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRADC`"]
pub type PWRADC_R = crate::R<bool, PWRADC_A>;
impl PWRADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRADC_A {
        match self.bits {
            true => PWRADC_A::EN,
            false => PWRADC_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRADC_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRADC_A::DIS
    }
}
#[doc = "Write proxy for field `PWRADC`"]
pub struct PWRADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up ADC value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRADC_A::EN)
    }
    #[doc = "Power Down ADC value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRADC_A::DIS)
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
#[doc = "Power up UART Controller 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUART1_A {
    #[doc = "1: Power up UART 1 value."]
    EN = 1,
    #[doc = "0: Power down UART 1 value."]
    DIS = 0,
}
impl From<PWRUART1_A> for bool {
    #[inline(always)]
    fn from(variant: PWRUART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRUART1`"]
pub type PWRUART1_R = crate::R<bool, PWRUART1_A>;
impl PWRUART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRUART1_A {
        match self.bits {
            true => PWRUART1_A::EN,
            false => PWRUART1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRUART1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRUART1_A::DIS
    }
}
#[doc = "Write proxy for field `PWRUART1`"]
pub struct PWRUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRUART1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up UART 1 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRUART1_A::EN)
    }
    #[doc = "Power down UART 1 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRUART1_A::DIS)
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
#[doc = "Power up UART Controller 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUART0_A {
    #[doc = "1: Power up UART 0 value."]
    EN = 1,
    #[doc = "0: Power down UART 0 value."]
    DIS = 0,
}
impl From<PWRUART0_A> for bool {
    #[inline(always)]
    fn from(variant: PWRUART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRUART0`"]
pub type PWRUART0_R = crate::R<bool, PWRUART0_A>;
impl PWRUART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRUART0_A {
        match self.bits {
            true => PWRUART0_A::EN,
            false => PWRUART0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRUART0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRUART0_A::DIS
    }
}
#[doc = "Write proxy for field `PWRUART0`"]
pub struct PWRUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRUART0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up UART 0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRUART0_A::EN)
    }
    #[doc = "Power down UART 0 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRUART0_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Power up IO Master 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM5_A {
    #[doc = "1: Power up IO Master 5 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 5 value."]
    DIS = 0,
}
impl From<PWRIOM5_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRIOM5`"]
pub type PWRIOM5_R = crate::R<bool, PWRIOM5_A>;
impl PWRIOM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM5_A {
        match self.bits {
            true => PWRIOM5_A::EN,
            false => PWRIOM5_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM5_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM5_A::DIS
    }
}
#[doc = "Write proxy for field `PWRIOM5`"]
pub struct PWRIOM5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up IO Master 5 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM5_A::EN)
    }
    #[doc = "Power down IO Master 5 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM5_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Power up IO Master 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM4_A {
    #[doc = "1: Power up IO Master 4 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 4 value."]
    DIS = 0,
}
impl From<PWRIOM4_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRIOM4`"]
pub type PWRIOM4_R = crate::R<bool, PWRIOM4_A>;
impl PWRIOM4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM4_A {
        match self.bits {
            true => PWRIOM4_A::EN,
            false => PWRIOM4_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM4_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM4_A::DIS
    }
}
#[doc = "Write proxy for field `PWRIOM4`"]
pub struct PWRIOM4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up IO Master 4 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM4_A::EN)
    }
    #[doc = "Power down IO Master 4 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM4_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Power up IO Master 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM3_A {
    #[doc = "1: Power up IO Master 3 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 3 value."]
    DIS = 0,
}
impl From<PWRIOM3_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRIOM3`"]
pub type PWRIOM3_R = crate::R<bool, PWRIOM3_A>;
impl PWRIOM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM3_A {
        match self.bits {
            true => PWRIOM3_A::EN,
            false => PWRIOM3_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM3_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM3_A::DIS
    }
}
#[doc = "Write proxy for field `PWRIOM3`"]
pub struct PWRIOM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up IO Master 3 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM3_A::EN)
    }
    #[doc = "Power down IO Master 3 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM3_A::DIS)
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
#[doc = "Power up IO Master 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM2_A {
    #[doc = "1: Power up IO Master 2 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 2 value."]
    DIS = 0,
}
impl From<PWRIOM2_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRIOM2`"]
pub type PWRIOM2_R = crate::R<bool, PWRIOM2_A>;
impl PWRIOM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM2_A {
        match self.bits {
            true => PWRIOM2_A::EN,
            false => PWRIOM2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM2_A::DIS
    }
}
#[doc = "Write proxy for field `PWRIOM2`"]
pub struct PWRIOM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up IO Master 2 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM2_A::EN)
    }
    #[doc = "Power down IO Master 2 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM2_A::DIS)
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
#[doc = "Power up IO Master 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM1_A {
    #[doc = "1: Power up IO Master 1 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 1 value."]
    DIS = 0,
}
impl From<PWRIOM1_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRIOM1`"]
pub type PWRIOM1_R = crate::R<bool, PWRIOM1_A>;
impl PWRIOM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM1_A {
        match self.bits {
            true => PWRIOM1_A::EN,
            false => PWRIOM1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM1_A::DIS
    }
}
#[doc = "Write proxy for field `PWRIOM1`"]
pub struct PWRIOM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up IO Master 1 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM1_A::EN)
    }
    #[doc = "Power down IO Master 1 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM1_A::DIS)
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
#[doc = "Power up IO Master 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM0_A {
    #[doc = "1: Power up IO Master 0 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 0 value."]
    DIS = 0,
}
impl From<PWRIOM0_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRIOM0`"]
pub type PWRIOM0_R = crate::R<bool, PWRIOM0_A>;
impl PWRIOM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM0_A {
        match self.bits {
            true => PWRIOM0_A::EN,
            false => PWRIOM0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM0_A::DIS
    }
}
#[doc = "Write proxy for field `PWRIOM0`"]
pub struct PWRIOM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up IO Master 0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM0_A::EN)
    }
    #[doc = "Power down IO Master 0 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM0_A::DIS)
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
#[doc = "Power up IO Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOS_A {
    #[doc = "1: Power up IO slave value."]
    EN = 1,
    #[doc = "0: Power down IO slave value."]
    DIS = 0,
}
impl From<PWRIOS_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRIOS`"]
pub type PWRIOS_R = crate::R<bool, PWRIOS_A>;
impl PWRIOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOS_A {
        match self.bits {
            true => PWRIOS_A::EN,
            false => PWRIOS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRIOS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOS_A::DIS
    }
}
#[doc = "Write proxy for field `PWRIOS`"]
pub struct PWRIOS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up IO slave value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOS_A::EN)
    }
    #[doc = "Power down IO slave value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOS_A::DIS)
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
    #[doc = "Bit 13 - Power up BLE controller"]
    #[inline(always)]
    pub fn pwrblel(&self) -> PWRBLEL_R {
        PWRBLEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Power up PDM block"]
    #[inline(always)]
    pub fn pwrpdm(&self) -> PWRPDM_R {
        PWRPDM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Power up MSPI Controller"]
    #[inline(always)]
    pub fn pwrmspi(&self) -> PWRMSPI_R {
        PWRMSPI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Power up SCARD Controller"]
    #[inline(always)]
    pub fn pwrscard(&self) -> PWRSCARD_R {
        PWRSCARD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Power up ADC Digital Controller"]
    #[inline(always)]
    pub fn pwradc(&self) -> PWRADC_R {
        PWRADC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Power up UART Controller 1"]
    #[inline(always)]
    pub fn pwruart1(&self) -> PWRUART1_R {
        PWRUART1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power up UART Controller 0"]
    #[inline(always)]
    pub fn pwruart0(&self) -> PWRUART0_R {
        PWRUART0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Power up IO Master 5"]
    #[inline(always)]
    pub fn pwriom5(&self) -> PWRIOM5_R {
        PWRIOM5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Power up IO Master 4"]
    #[inline(always)]
    pub fn pwriom4(&self) -> PWRIOM4_R {
        PWRIOM4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Power up IO Master 3"]
    #[inline(always)]
    pub fn pwriom3(&self) -> PWRIOM3_R {
        PWRIOM3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power up IO Master 2"]
    #[inline(always)]
    pub fn pwriom2(&self) -> PWRIOM2_R {
        PWRIOM2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power up IO Master 1"]
    #[inline(always)]
    pub fn pwriom1(&self) -> PWRIOM1_R {
        PWRIOM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power up IO Master 0"]
    #[inline(always)]
    pub fn pwriom0(&self) -> PWRIOM0_R {
        PWRIOM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Power up IO Slave"]
    #[inline(always)]
    pub fn pwrios(&self) -> PWRIOS_R {
        PWRIOS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Power up BLE controller"]
    #[inline(always)]
    pub fn pwrblel(&mut self) -> PWRBLEL_W {
        PWRBLEL_W { w: self }
    }
    #[doc = "Bit 12 - Power up PDM block"]
    #[inline(always)]
    pub fn pwrpdm(&mut self) -> PWRPDM_W {
        PWRPDM_W { w: self }
    }
    #[doc = "Bit 11 - Power up MSPI Controller"]
    #[inline(always)]
    pub fn pwrmspi(&mut self) -> PWRMSPI_W {
        PWRMSPI_W { w: self }
    }
    #[doc = "Bit 10 - Power up SCARD Controller"]
    #[inline(always)]
    pub fn pwrscard(&mut self) -> PWRSCARD_W {
        PWRSCARD_W { w: self }
    }
    #[doc = "Bit 9 - Power up ADC Digital Controller"]
    #[inline(always)]
    pub fn pwradc(&mut self) -> PWRADC_W {
        PWRADC_W { w: self }
    }
    #[doc = "Bit 8 - Power up UART Controller 1"]
    #[inline(always)]
    pub fn pwruart1(&mut self) -> PWRUART1_W {
        PWRUART1_W { w: self }
    }
    #[doc = "Bit 7 - Power up UART Controller 0"]
    #[inline(always)]
    pub fn pwruart0(&mut self) -> PWRUART0_W {
        PWRUART0_W { w: self }
    }
    #[doc = "Bit 6 - Power up IO Master 5"]
    #[inline(always)]
    pub fn pwriom5(&mut self) -> PWRIOM5_W {
        PWRIOM5_W { w: self }
    }
    #[doc = "Bit 5 - Power up IO Master 4"]
    #[inline(always)]
    pub fn pwriom4(&mut self) -> PWRIOM4_W {
        PWRIOM4_W { w: self }
    }
    #[doc = "Bit 4 - Power up IO Master 3"]
    #[inline(always)]
    pub fn pwriom3(&mut self) -> PWRIOM3_W {
        PWRIOM3_W { w: self }
    }
    #[doc = "Bit 3 - Power up IO Master 2"]
    #[inline(always)]
    pub fn pwriom2(&mut self) -> PWRIOM2_W {
        PWRIOM2_W { w: self }
    }
    #[doc = "Bit 2 - Power up IO Master 1"]
    #[inline(always)]
    pub fn pwriom1(&mut self) -> PWRIOM1_W {
        PWRIOM1_W { w: self }
    }
    #[doc = "Bit 1 - Power up IO Master 0"]
    #[inline(always)]
    pub fn pwriom0(&mut self) -> PWRIOM0_W {
        PWRIOM0_W { w: self }
    }
    #[doc = "Bit 0 - Power up IO Slave"]
    #[inline(always)]
    pub fn pwrios(&mut self) -> PWRIOS_W {
        PWRIOS_W { w: self }
    }
}
