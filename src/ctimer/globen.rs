#[doc = "Reader of register GLOBEN"]
pub type R = crate::R<u32, super::GLOBEN>;
#[doc = "Writer for register GLOBEN"]
pub type W = crate::W<u32, super::GLOBEN>;
#[doc = "Register GLOBEN `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::GLOBEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Alternate enable for B7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB7_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENB7_A> for bool {
    #[inline(always)]
    fn from(variant: ENB7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENB7`"]
pub type ENB7_R = crate::R<bool, ENB7_A>;
impl ENB7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB7_A {
        match self.bits {
            true => ENB7_A::LCO,
            false => ENB7_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENB7_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENB7_A::DIS
    }
}
#[doc = "Write proxy for field `ENB7`"]
pub struct ENB7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB7_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB7_A::DIS)
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
#[doc = "Alternate enable for A7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA7_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENA7_A> for bool {
    #[inline(always)]
    fn from(variant: ENA7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA7`"]
pub type ENA7_R = crate::R<bool, ENA7_A>;
impl ENA7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA7_A {
        match self.bits {
            true => ENA7_A::LCO,
            false => ENA7_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENA7_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENA7_A::DIS
    }
}
#[doc = "Write proxy for field `ENA7`"]
pub struct ENA7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA7_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA7_A::DIS)
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
#[doc = "Alternate enable for B6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB6_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENB6_A> for bool {
    #[inline(always)]
    fn from(variant: ENB6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENB6`"]
pub type ENB6_R = crate::R<bool, ENB6_A>;
impl ENB6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB6_A {
        match self.bits {
            true => ENB6_A::LCO,
            false => ENB6_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENB6_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENB6_A::DIS
    }
}
#[doc = "Write proxy for field `ENB6`"]
pub struct ENB6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB6_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB6_A::DIS)
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
#[doc = "Alternate enable for A6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA6_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENA6_A> for bool {
    #[inline(always)]
    fn from(variant: ENA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA6`"]
pub type ENA6_R = crate::R<bool, ENA6_A>;
impl ENA6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA6_A {
        match self.bits {
            true => ENA6_A::LCO,
            false => ENA6_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENA6_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENA6_A::DIS
    }
}
#[doc = "Write proxy for field `ENA6`"]
pub struct ENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA6_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA6_A::DIS)
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
#[doc = "Alternate enable for B5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB5_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENB5_A> for bool {
    #[inline(always)]
    fn from(variant: ENB5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENB5`"]
pub type ENB5_R = crate::R<bool, ENB5_A>;
impl ENB5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB5_A {
        match self.bits {
            true => ENB5_A::LCO,
            false => ENB5_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENB5_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENB5_A::DIS
    }
}
#[doc = "Write proxy for field `ENB5`"]
pub struct ENB5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB5_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB5_A::DIS)
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
#[doc = "Alternate enable for A5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA5_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENA5_A> for bool {
    #[inline(always)]
    fn from(variant: ENA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA5`"]
pub type ENA5_R = crate::R<bool, ENA5_A>;
impl ENA5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA5_A {
        match self.bits {
            true => ENA5_A::LCO,
            false => ENA5_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENA5_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENA5_A::DIS
    }
}
#[doc = "Write proxy for field `ENA5`"]
pub struct ENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA5_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA5_A::DIS)
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
#[doc = "Alternate enable for B4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB4_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENB4_A> for bool {
    #[inline(always)]
    fn from(variant: ENB4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENB4`"]
pub type ENB4_R = crate::R<bool, ENB4_A>;
impl ENB4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB4_A {
        match self.bits {
            true => ENB4_A::LCO,
            false => ENB4_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENB4_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENB4_A::DIS
    }
}
#[doc = "Write proxy for field `ENB4`"]
pub struct ENB4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB4_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB4_A::DIS)
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
#[doc = "Alternate enable for A4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA4_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENA4_A> for bool {
    #[inline(always)]
    fn from(variant: ENA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA4`"]
pub type ENA4_R = crate::R<bool, ENA4_A>;
impl ENA4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA4_A {
        match self.bits {
            true => ENA4_A::LCO,
            false => ENA4_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENA4_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENA4_A::DIS
    }
}
#[doc = "Write proxy for field `ENA4`"]
pub struct ENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA4_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA4_A::DIS)
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
#[doc = "Alternate enable for B3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB3_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENB3_A> for bool {
    #[inline(always)]
    fn from(variant: ENB3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENB3`"]
pub type ENB3_R = crate::R<bool, ENB3_A>;
impl ENB3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB3_A {
        match self.bits {
            true => ENB3_A::LCO,
            false => ENB3_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENB3_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENB3_A::DIS
    }
}
#[doc = "Write proxy for field `ENB3`"]
pub struct ENB3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB3_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB3_A::DIS)
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
#[doc = "Alternate enable for A3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA3_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENA3_A> for bool {
    #[inline(always)]
    fn from(variant: ENA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA3`"]
pub type ENA3_R = crate::R<bool, ENA3_A>;
impl ENA3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA3_A {
        match self.bits {
            true => ENA3_A::LCO,
            false => ENA3_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENA3_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENA3_A::DIS
    }
}
#[doc = "Write proxy for field `ENA3`"]
pub struct ENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA3_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA3_A::DIS)
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
#[doc = "Alternate enable for B2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB2_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENB2_A> for bool {
    #[inline(always)]
    fn from(variant: ENB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENB2`"]
pub type ENB2_R = crate::R<bool, ENB2_A>;
impl ENB2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB2_A {
        match self.bits {
            true => ENB2_A::LCO,
            false => ENB2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENB2_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENB2_A::DIS
    }
}
#[doc = "Write proxy for field `ENB2`"]
pub struct ENB2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB2_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB2_A::DIS)
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
#[doc = "Alternate enable for A2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA2_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENA2_A> for bool {
    #[inline(always)]
    fn from(variant: ENA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA2`"]
pub type ENA2_R = crate::R<bool, ENA2_A>;
impl ENA2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA2_A {
        match self.bits {
            true => ENA2_A::LCO,
            false => ENA2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENA2_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENA2_A::DIS
    }
}
#[doc = "Write proxy for field `ENA2`"]
pub struct ENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA2_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA2_A::DIS)
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
#[doc = "Alternate enable for B1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB1_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENB1_A> for bool {
    #[inline(always)]
    fn from(variant: ENB1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENB1`"]
pub type ENB1_R = crate::R<bool, ENB1_A>;
impl ENB1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB1_A {
        match self.bits {
            true => ENB1_A::LCO,
            false => ENB1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENB1_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENB1_A::DIS
    }
}
#[doc = "Write proxy for field `ENB1`"]
pub struct ENB1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB1_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB1_A::DIS)
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
#[doc = "Alternate enable for A1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA1_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENA1_A> for bool {
    #[inline(always)]
    fn from(variant: ENA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA1`"]
pub type ENA1_R = crate::R<bool, ENA1_A>;
impl ENA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA1_A {
        match self.bits {
            true => ENA1_A::LCO,
            false => ENA1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENA1_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENA1_A::DIS
    }
}
#[doc = "Write proxy for field `ENA1`"]
pub struct ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA1_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA1_A::DIS)
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
#[doc = "Alternate enable for B0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB0_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENB0_A> for bool {
    #[inline(always)]
    fn from(variant: ENB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENB0`"]
pub type ENB0_R = crate::R<bool, ENB0_A>;
impl ENB0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB0_A {
        match self.bits {
            true => ENB0_A::LCO,
            false => ENB0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENB0_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENB0_A::DIS
    }
}
#[doc = "Write proxy for field `ENB0`"]
pub struct ENB0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB0_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB0_A::DIS)
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
#[doc = "Alternate enable for A0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA0_A {
    #[doc = "1: Use local enable. value."]
    LCO = 1,
    #[doc = "0: Disable CTIMER. value."]
    DIS = 0,
}
impl From<ENA0_A> for bool {
    #[inline(always)]
    fn from(variant: ENA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA0`"]
pub type ENA0_R = crate::R<bool, ENA0_A>;
impl ENA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA0_A {
        match self.bits {
            true => ENA0_A::LCO,
            false => ENA0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline(always)]
    pub fn is_lco(&self) -> bool {
        *self == ENA0_A::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENA0_A::DIS
    }
}
#[doc = "Write proxy for field `ENA0`"]
pub struct ENA0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline(always)]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA0_A::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA0_A::DIS)
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
    #[doc = "Bit 15 - Alternate enable for B7."]
    #[inline(always)]
    pub fn enb7(&self) -> ENB7_R {
        ENB7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Alternate enable for A7"]
    #[inline(always)]
    pub fn ena7(&self) -> ENA7_R {
        ENA7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Alternate enable for B6"]
    #[inline(always)]
    pub fn enb6(&self) -> ENB6_R {
        ENB6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Alternate enable for A6"]
    #[inline(always)]
    pub fn ena6(&self) -> ENA6_R {
        ENA6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Alternate enable for B5"]
    #[inline(always)]
    pub fn enb5(&self) -> ENB5_R {
        ENB5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Alternate enable for A5"]
    #[inline(always)]
    pub fn ena5(&self) -> ENA5_R {
        ENA5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alternate enable for B4"]
    #[inline(always)]
    pub fn enb4(&self) -> ENB4_R {
        ENB4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alternate enable for A4"]
    #[inline(always)]
    pub fn ena4(&self) -> ENA4_R {
        ENA4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Alternate enable for B3."]
    #[inline(always)]
    pub fn enb3(&self) -> ENB3_R {
        ENB3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Alternate enable for A3"]
    #[inline(always)]
    pub fn ena3(&self) -> ENA3_R {
        ENA3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Alternate enable for B2"]
    #[inline(always)]
    pub fn enb2(&self) -> ENB2_R {
        ENB2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Alternate enable for A2"]
    #[inline(always)]
    pub fn ena2(&self) -> ENA2_R {
        ENA2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Alternate enable for B1"]
    #[inline(always)]
    pub fn enb1(&self) -> ENB1_R {
        ENB1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alternate enable for A1"]
    #[inline(always)]
    pub fn ena1(&self) -> ENA1_R {
        ENA1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alternate enable for B0"]
    #[inline(always)]
    pub fn enb0(&self) -> ENB0_R {
        ENB0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Alternate enable for A0"]
    #[inline(always)]
    pub fn ena0(&self) -> ENA0_R {
        ENA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Alternate enable for B7."]
    #[inline(always)]
    pub fn enb7(&mut self) -> ENB7_W {
        ENB7_W { w: self }
    }
    #[doc = "Bit 14 - Alternate enable for A7"]
    #[inline(always)]
    pub fn ena7(&mut self) -> ENA7_W {
        ENA7_W { w: self }
    }
    #[doc = "Bit 13 - Alternate enable for B6"]
    #[inline(always)]
    pub fn enb6(&mut self) -> ENB6_W {
        ENB6_W { w: self }
    }
    #[doc = "Bit 12 - Alternate enable for A6"]
    #[inline(always)]
    pub fn ena6(&mut self) -> ENA6_W {
        ENA6_W { w: self }
    }
    #[doc = "Bit 11 - Alternate enable for B5"]
    #[inline(always)]
    pub fn enb5(&mut self) -> ENB5_W {
        ENB5_W { w: self }
    }
    #[doc = "Bit 10 - Alternate enable for A5"]
    #[inline(always)]
    pub fn ena5(&mut self) -> ENA5_W {
        ENA5_W { w: self }
    }
    #[doc = "Bit 9 - Alternate enable for B4"]
    #[inline(always)]
    pub fn enb4(&mut self) -> ENB4_W {
        ENB4_W { w: self }
    }
    #[doc = "Bit 8 - Alternate enable for A4"]
    #[inline(always)]
    pub fn ena4(&mut self) -> ENA4_W {
        ENA4_W { w: self }
    }
    #[doc = "Bit 7 - Alternate enable for B3."]
    #[inline(always)]
    pub fn enb3(&mut self) -> ENB3_W {
        ENB3_W { w: self }
    }
    #[doc = "Bit 6 - Alternate enable for A3"]
    #[inline(always)]
    pub fn ena3(&mut self) -> ENA3_W {
        ENA3_W { w: self }
    }
    #[doc = "Bit 5 - Alternate enable for B2"]
    #[inline(always)]
    pub fn enb2(&mut self) -> ENB2_W {
        ENB2_W { w: self }
    }
    #[doc = "Bit 4 - Alternate enable for A2"]
    #[inline(always)]
    pub fn ena2(&mut self) -> ENA2_W {
        ENA2_W { w: self }
    }
    #[doc = "Bit 3 - Alternate enable for B1"]
    #[inline(always)]
    pub fn enb1(&mut self) -> ENB1_W {
        ENB1_W { w: self }
    }
    #[doc = "Bit 2 - Alternate enable for A1"]
    #[inline(always)]
    pub fn ena1(&mut self) -> ENA1_W {
        ENA1_W { w: self }
    }
    #[doc = "Bit 1 - Alternate enable for B0"]
    #[inline(always)]
    pub fn enb0(&mut self) -> ENB0_W {
        ENB0_W { w: self }
    }
    #[doc = "Bit 0 - Alternate enable for A0"]
    #[inline(always)]
    pub fn ena0(&mut self) -> ENA0_W {
        ENA0_W { w: self }
    }
}
