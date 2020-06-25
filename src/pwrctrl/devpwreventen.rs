#[doc = "Reader of register DEVPWREVENTEN"]
pub type R = crate::R<u32, super::DEVPWREVENTEN>;
#[doc = "Writer for register DEVPWREVENTEN"]
pub type W = crate::W<u32, super::DEVPWREVENTEN>;
#[doc = "Register DEVPWREVENTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVPWREVENTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Control BURST status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTEVEN_A {
    #[doc = "1: Enable BURST status event value."]
    EN = 1,
    #[doc = "0: Disable BURST status event value."]
    DIS = 0,
}
impl From<BURSTEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BURSTEVEN`"]
pub type BURSTEVEN_R = crate::R<bool, BURSTEVEN_A>;
impl BURSTEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTEVEN_A {
        match self.bits {
            true => BURSTEVEN_A::EN,
            false => BURSTEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BURSTEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BURSTEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `BURSTEVEN`"]
pub struct BURSTEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable BURST status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTEVEN_A::EN)
    }
    #[doc = "Disable BURST status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTEVEN_A::DIS)
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
#[doc = "Control BURSTFEATURE status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTFEATUREEVEN_A {
    #[doc = "1: Enable BURSTFEATURE status event value."]
    EN = 1,
    #[doc = "0: Disable BURSTFEATURE status event value."]
    DIS = 0,
}
impl From<BURSTFEATUREEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTFEATUREEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BURSTFEATUREEVEN`"]
pub type BURSTFEATUREEVEN_R = crate::R<bool, BURSTFEATUREEVEN_A>;
impl BURSTFEATUREEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTFEATUREEVEN_A {
        match self.bits {
            true => BURSTFEATUREEVEN_A::EN,
            false => BURSTFEATUREEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BURSTFEATUREEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BURSTFEATUREEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `BURSTFEATUREEVEN`"]
pub struct BURSTFEATUREEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTFEATUREEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTFEATUREEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable BURSTFEATURE status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTFEATUREEVEN_A::EN)
    }
    #[doc = "Disable BURSTFEATURE status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTFEATUREEVEN_A::DIS)
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
#[doc = "Control BLEFEATURE status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEFEATUREEVEN_A {
    #[doc = "1: Enable BLEFEATURE status event value."]
    EN = 1,
    #[doc = "0: Disable BLEFEATURE status event value."]
    DIS = 0,
}
impl From<BLEFEATUREEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: BLEFEATUREEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLEFEATUREEVEN`"]
pub type BLEFEATUREEVEN_R = crate::R<bool, BLEFEATUREEVEN_A>;
impl BLEFEATUREEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEFEATUREEVEN_A {
        match self.bits {
            true => BLEFEATUREEVEN_A::EN,
            false => BLEFEATUREEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BLEFEATUREEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BLEFEATUREEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `BLEFEATUREEVEN`"]
pub struct BLEFEATUREEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEFEATUREEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEFEATUREEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable BLEFEATURE status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BLEFEATUREEVEN_A::EN)
    }
    #[doc = "Disable BLEFEATURE status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLEFEATUREEVEN_A::DIS)
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
#[doc = "Control BLE power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLELEVEN_A {
    #[doc = "1: Enable BLE power-on status event value."]
    EN = 1,
    #[doc = "0: Disable BLE power-on status event value."]
    DIS = 0,
}
impl From<BLELEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: BLELEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLELEVEN`"]
pub type BLELEVEN_R = crate::R<bool, BLELEVEN_A>;
impl BLELEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLELEVEN_A {
        match self.bits {
            true => BLELEVEN_A::EN,
            false => BLELEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BLELEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BLELEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `BLELEVEN`"]
pub struct BLELEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLELEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLELEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable BLE power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BLELEVEN_A::EN)
    }
    #[doc = "Disable BLE power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLELEVEN_A::DIS)
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
#[doc = "Control PDM power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMEVEN_A {
    #[doc = "1: Enable PDM power-on status event value."]
    EN = 1,
    #[doc = "0: Disable PDM power-on status event value."]
    DIS = 0,
}
impl From<PDMEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDMEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDMEVEN`"]
pub type PDMEVEN_R = crate::R<bool, PDMEVEN_A>;
impl PDMEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMEVEN_A {
        match self.bits {
            true => PDMEVEN_A::EN,
            false => PDMEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PDMEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PDMEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `PDMEVEN`"]
pub struct PDMEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable PDM power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PDMEVEN_A::EN)
    }
    #[doc = "Disable PDM power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PDMEVEN_A::DIS)
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
#[doc = "Control MSPI power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSPIEVEN_A {
    #[doc = "1: Enable MSPI power-on status event value."]
    EN = 1,
    #[doc = "0: Disable MSPI power-on status event value."]
    DIS = 0,
}
impl From<MSPIEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSPIEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSPIEVEN`"]
pub type MSPIEVEN_R = crate::R<bool, MSPIEVEN_A>;
impl MSPIEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSPIEVEN_A {
        match self.bits {
            true => MSPIEVEN_A::EN,
            false => MSPIEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MSPIEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MSPIEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `MSPIEVEN`"]
pub struct MSPIEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSPIEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSPIEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable MSPI power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MSPIEVEN_A::EN)
    }
    #[doc = "Disable MSPI power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MSPIEVEN_A::DIS)
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
#[doc = "Control ADC power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEVEN_A {
    #[doc = "1: Enable ADC power-on status event value."]
    EN = 1,
    #[doc = "0: Disable ADC power-on status event value."]
    DIS = 0,
}
impl From<ADCEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCEVEN`"]
pub type ADCEVEN_R = crate::R<bool, ADCEVEN_A>;
impl ADCEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEVEN_A {
        match self.bits {
            true => ADCEVEN_A::EN,
            false => ADCEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADCEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADCEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `ADCEVEN`"]
pub struct ADCEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable ADC power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCEVEN_A::EN)
    }
    #[doc = "Disable ADC power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCEVEN_A::DIS)
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
#[doc = "Control HCPC power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCPCEVEN_A {
    #[doc = "1: Enable HCPC power-on status event value."]
    EN = 1,
    #[doc = "0: Disable HCPC power-on status event value."]
    DIS = 0,
}
impl From<HCPCEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: HCPCEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HCPCEVEN`"]
pub type HCPCEVEN_R = crate::R<bool, HCPCEVEN_A>;
impl HCPCEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCPCEVEN_A {
        match self.bits {
            true => HCPCEVEN_A::EN,
            false => HCPCEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HCPCEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HCPCEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `HCPCEVEN`"]
pub struct HCPCEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPCEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCPCEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable HCPC power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HCPCEVEN_A::EN)
    }
    #[doc = "Disable HCPC power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HCPCEVEN_A::DIS)
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
#[doc = "Control HCPB power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCPBEVEN_A {
    #[doc = "1: Enable HCPB power-on status event value."]
    EN = 1,
    #[doc = "0: Disable HCPB power-on status event value."]
    DIS = 0,
}
impl From<HCPBEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: HCPBEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HCPBEVEN`"]
pub type HCPBEVEN_R = crate::R<bool, HCPBEVEN_A>;
impl HCPBEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCPBEVEN_A {
        match self.bits {
            true => HCPBEVEN_A::EN,
            false => HCPBEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HCPBEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HCPBEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `HCPBEVEN`"]
pub struct HCPBEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPBEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCPBEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable HCPB power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HCPBEVEN_A::EN)
    }
    #[doc = "Disable HCPB power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HCPBEVEN_A::DIS)
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
#[doc = "Control HCPA power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCPAEVEN_A {
    #[doc = "1: Enable HCPA power-on status event value."]
    EN = 1,
    #[doc = "0: Disable HCPA power-on status event value."]
    DIS = 0,
}
impl From<HCPAEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: HCPAEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HCPAEVEN`"]
pub type HCPAEVEN_R = crate::R<bool, HCPAEVEN_A>;
impl HCPAEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCPAEVEN_A {
        match self.bits {
            true => HCPAEVEN_A::EN,
            false => HCPAEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HCPAEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HCPAEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `HCPAEVEN`"]
pub struct HCPAEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPAEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCPAEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable HCPA power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HCPAEVEN_A::EN)
    }
    #[doc = "Disable HCPA power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HCPAEVEN_A::DIS)
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
#[doc = "Control MCUH power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCUHEVEN_A {
    #[doc = "1: Enable MCHU power-on status event value."]
    EN = 1,
    #[doc = "0: Disable MCUH power-on status event value."]
    DIS = 0,
}
impl From<MCUHEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCUHEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCUHEVEN`"]
pub type MCUHEVEN_R = crate::R<bool, MCUHEVEN_A>;
impl MCUHEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCUHEVEN_A {
        match self.bits {
            true => MCUHEVEN_A::EN,
            false => MCUHEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MCUHEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MCUHEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `MCUHEVEN`"]
pub struct MCUHEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUHEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCUHEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable MCHU power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MCUHEVEN_A::EN)
    }
    #[doc = "Disable MCUH power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MCUHEVEN_A::DIS)
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
#[doc = "Control MCUL power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCULEVEN_A {
    #[doc = "1: Enable MCUL power-on status event value."]
    EN = 1,
    #[doc = "0: Disable MCUL power-on status event value."]
    DIS = 0,
}
impl From<MCULEVEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCULEVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCULEVEN`"]
pub type MCULEVEN_R = crate::R<bool, MCULEVEN_A>;
impl MCULEVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCULEVEN_A {
        match self.bits {
            true => MCULEVEN_A::EN,
            false => MCULEVEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MCULEVEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MCULEVEN_A::DIS
    }
}
#[doc = "Write proxy for field `MCULEVEN`"]
pub struct MCULEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCULEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCULEVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable MCUL power-on status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MCULEVEN_A::EN)
    }
    #[doc = "Disable MCUL power-on status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MCULEVEN_A::DIS)
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
    #[doc = "Bit 31 - Control BURST status event"]
    #[inline(always)]
    pub fn bursteven(&self) -> BURSTEVEN_R {
        BURSTEVEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Control BURSTFEATURE status event"]
    #[inline(always)]
    pub fn burstfeatureeven(&self) -> BURSTFEATUREEVEN_R {
        BURSTFEATUREEVEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Control BLEFEATURE status event"]
    #[inline(always)]
    pub fn blefeatureeven(&self) -> BLEFEATUREEVEN_R {
        BLEFEATUREEVEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Control BLE power-on status event"]
    #[inline(always)]
    pub fn bleleven(&self) -> BLELEVEN_R {
        BLELEVEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Control PDM power-on status event"]
    #[inline(always)]
    pub fn pdmeven(&self) -> PDMEVEN_R {
        PDMEVEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Control MSPI power-on status event"]
    #[inline(always)]
    pub fn mspieven(&self) -> MSPIEVEN_R {
        MSPIEVEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Control ADC power-on status event"]
    #[inline(always)]
    pub fn adceven(&self) -> ADCEVEN_R {
        ADCEVEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control HCPC power-on status event"]
    #[inline(always)]
    pub fn hcpceven(&self) -> HCPCEVEN_R {
        HCPCEVEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control HCPB power-on status event"]
    #[inline(always)]
    pub fn hcpbeven(&self) -> HCPBEVEN_R {
        HCPBEVEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control HCPA power-on status event"]
    #[inline(always)]
    pub fn hcpaeven(&self) -> HCPAEVEN_R {
        HCPAEVEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control MCUH power-on status event"]
    #[inline(always)]
    pub fn mcuheven(&self) -> MCUHEVEN_R {
        MCUHEVEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Control MCUL power-on status event"]
    #[inline(always)]
    pub fn mculeven(&self) -> MCULEVEN_R {
        MCULEVEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Control BURST status event"]
    #[inline(always)]
    pub fn bursteven(&mut self) -> BURSTEVEN_W {
        BURSTEVEN_W { w: self }
    }
    #[doc = "Bit 30 - Control BURSTFEATURE status event"]
    #[inline(always)]
    pub fn burstfeatureeven(&mut self) -> BURSTFEATUREEVEN_W {
        BURSTFEATUREEVEN_W { w: self }
    }
    #[doc = "Bit 29 - Control BLEFEATURE status event"]
    #[inline(always)]
    pub fn blefeatureeven(&mut self) -> BLEFEATUREEVEN_W {
        BLEFEATUREEVEN_W { w: self }
    }
    #[doc = "Bit 8 - Control BLE power-on status event"]
    #[inline(always)]
    pub fn bleleven(&mut self) -> BLELEVEN_W {
        BLELEVEN_W { w: self }
    }
    #[doc = "Bit 7 - Control PDM power-on status event"]
    #[inline(always)]
    pub fn pdmeven(&mut self) -> PDMEVEN_W {
        PDMEVEN_W { w: self }
    }
    #[doc = "Bit 6 - Control MSPI power-on status event"]
    #[inline(always)]
    pub fn mspieven(&mut self) -> MSPIEVEN_W {
        MSPIEVEN_W { w: self }
    }
    #[doc = "Bit 5 - Control ADC power-on status event"]
    #[inline(always)]
    pub fn adceven(&mut self) -> ADCEVEN_W {
        ADCEVEN_W { w: self }
    }
    #[doc = "Bit 4 - Control HCPC power-on status event"]
    #[inline(always)]
    pub fn hcpceven(&mut self) -> HCPCEVEN_W {
        HCPCEVEN_W { w: self }
    }
    #[doc = "Bit 3 - Control HCPB power-on status event"]
    #[inline(always)]
    pub fn hcpbeven(&mut self) -> HCPBEVEN_W {
        HCPBEVEN_W { w: self }
    }
    #[doc = "Bit 2 - Control HCPA power-on status event"]
    #[inline(always)]
    pub fn hcpaeven(&mut self) -> HCPAEVEN_W {
        HCPAEVEN_W { w: self }
    }
    #[doc = "Bit 1 - Control MCUH power-on status event"]
    #[inline(always)]
    pub fn mcuheven(&mut self) -> MCUHEVEN_W {
        MCUHEVEN_W { w: self }
    }
    #[doc = "Bit 0 - Control MCUL power-on status event"]
    #[inline(always)]
    pub fn mculeven(&mut self) -> MCULEVEN_W {
        MCULEVEN_W { w: self }
    }
}
