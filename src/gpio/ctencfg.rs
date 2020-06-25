#[doc = "Reader of register CTENCFG"]
pub type R = crate::R<u32, super::CTENCFG>;
#[doc = "Writer for register CTENCFG"]
pub type W = crate::W<u32, super::CTENCFG>;
#[doc = "Register CTENCFG `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CTENCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "CT31 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN31_A {
    #[doc = "1: Disable CT31 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT31 for output value."]
    EN = 0,
}
impl From<EN31_A> for bool {
    #[inline(always)]
    fn from(variant: EN31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN31`"]
pub type EN31_R = crate::R<bool, EN31_A>;
impl EN31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN31_A {
        match self.bits {
            true => EN31_A::DIS,
            false => EN31_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN31_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN31_A::EN
    }
}
#[doc = "Write proxy for field `EN31`"]
pub struct EN31_W<'a> {
    w: &'a mut W,
}
impl<'a> EN31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT31 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN31_A::DIS)
    }
    #[doc = "Enable CT31 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN31_A::EN)
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
#[doc = "CT30 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN30_A {
    #[doc = "1: Disable CT30 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT30 for output value."]
    EN = 0,
}
impl From<EN30_A> for bool {
    #[inline(always)]
    fn from(variant: EN30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN30`"]
pub type EN30_R = crate::R<bool, EN30_A>;
impl EN30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN30_A {
        match self.bits {
            true => EN30_A::DIS,
            false => EN30_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN30_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN30_A::EN
    }
}
#[doc = "Write proxy for field `EN30`"]
pub struct EN30_W<'a> {
    w: &'a mut W,
}
impl<'a> EN30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT30 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN30_A::DIS)
    }
    #[doc = "Enable CT30 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN30_A::EN)
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
#[doc = "CT29 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN29_A {
    #[doc = "1: Disable CT29 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT29 for output value."]
    EN = 0,
}
impl From<EN29_A> for bool {
    #[inline(always)]
    fn from(variant: EN29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN29`"]
pub type EN29_R = crate::R<bool, EN29_A>;
impl EN29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN29_A {
        match self.bits {
            true => EN29_A::DIS,
            false => EN29_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN29_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN29_A::EN
    }
}
#[doc = "Write proxy for field `EN29`"]
pub struct EN29_W<'a> {
    w: &'a mut W,
}
impl<'a> EN29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT29 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN29_A::DIS)
    }
    #[doc = "Enable CT29 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN29_A::EN)
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
#[doc = "CT28 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN28_A {
    #[doc = "1: Disable CT28 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT28 for output value."]
    EN = 0,
}
impl From<EN28_A> for bool {
    #[inline(always)]
    fn from(variant: EN28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN28`"]
pub type EN28_R = crate::R<bool, EN28_A>;
impl EN28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN28_A {
        match self.bits {
            true => EN28_A::DIS,
            false => EN28_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN28_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN28_A::EN
    }
}
#[doc = "Write proxy for field `EN28`"]
pub struct EN28_W<'a> {
    w: &'a mut W,
}
impl<'a> EN28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT28 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN28_A::DIS)
    }
    #[doc = "Enable CT28 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN28_A::EN)
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
#[doc = "CT27 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN27_A {
    #[doc = "1: Disable CT27 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT27 for output value."]
    EN = 0,
}
impl From<EN27_A> for bool {
    #[inline(always)]
    fn from(variant: EN27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN27`"]
pub type EN27_R = crate::R<bool, EN27_A>;
impl EN27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN27_A {
        match self.bits {
            true => EN27_A::DIS,
            false => EN27_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN27_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN27_A::EN
    }
}
#[doc = "Write proxy for field `EN27`"]
pub struct EN27_W<'a> {
    w: &'a mut W,
}
impl<'a> EN27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT27 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN27_A::DIS)
    }
    #[doc = "Enable CT27 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN27_A::EN)
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
#[doc = "CT26 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN26_A {
    #[doc = "1: Disable CT26 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT26 for output value."]
    EN = 0,
}
impl From<EN26_A> for bool {
    #[inline(always)]
    fn from(variant: EN26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN26`"]
pub type EN26_R = crate::R<bool, EN26_A>;
impl EN26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN26_A {
        match self.bits {
            true => EN26_A::DIS,
            false => EN26_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN26_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN26_A::EN
    }
}
#[doc = "Write proxy for field `EN26`"]
pub struct EN26_W<'a> {
    w: &'a mut W,
}
impl<'a> EN26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT26 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN26_A::DIS)
    }
    #[doc = "Enable CT26 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN26_A::EN)
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
#[doc = "CT25 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN25_A {
    #[doc = "1: Disable CT25 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT25 for output value."]
    EN = 0,
}
impl From<EN25_A> for bool {
    #[inline(always)]
    fn from(variant: EN25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN25`"]
pub type EN25_R = crate::R<bool, EN25_A>;
impl EN25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN25_A {
        match self.bits {
            true => EN25_A::DIS,
            false => EN25_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN25_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN25_A::EN
    }
}
#[doc = "Write proxy for field `EN25`"]
pub struct EN25_W<'a> {
    w: &'a mut W,
}
impl<'a> EN25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT25 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN25_A::DIS)
    }
    #[doc = "Enable CT25 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN25_A::EN)
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
#[doc = "CT24 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN24_A {
    #[doc = "1: Disable CT24 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT24 for output value."]
    EN = 0,
}
impl From<EN24_A> for bool {
    #[inline(always)]
    fn from(variant: EN24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN24`"]
pub type EN24_R = crate::R<bool, EN24_A>;
impl EN24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN24_A {
        match self.bits {
            true => EN24_A::DIS,
            false => EN24_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN24_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN24_A::EN
    }
}
#[doc = "Write proxy for field `EN24`"]
pub struct EN24_W<'a> {
    w: &'a mut W,
}
impl<'a> EN24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT24 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN24_A::DIS)
    }
    #[doc = "Enable CT24 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN24_A::EN)
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
#[doc = "CT23 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN23_A {
    #[doc = "1: Disable CT23 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT23 for output value."]
    EN = 0,
}
impl From<EN23_A> for bool {
    #[inline(always)]
    fn from(variant: EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN23`"]
pub type EN23_R = crate::R<bool, EN23_A>;
impl EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN23_A {
        match self.bits {
            true => EN23_A::DIS,
            false => EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN23_A::EN
    }
}
#[doc = "Write proxy for field `EN23`"]
pub struct EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT23 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN23_A::DIS)
    }
    #[doc = "Enable CT23 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN23_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "CT22 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN22_A {
    #[doc = "1: Disable CT22 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT22 for output value."]
    EN = 0,
}
impl From<EN22_A> for bool {
    #[inline(always)]
    fn from(variant: EN22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN22`"]
pub type EN22_R = crate::R<bool, EN22_A>;
impl EN22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN22_A {
        match self.bits {
            true => EN22_A::DIS,
            false => EN22_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN22_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN22_A::EN
    }
}
#[doc = "Write proxy for field `EN22`"]
pub struct EN22_W<'a> {
    w: &'a mut W,
}
impl<'a> EN22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT22 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN22_A::DIS)
    }
    #[doc = "Enable CT22 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN22_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "CT21 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN21_A {
    #[doc = "1: Disable CT21 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT21 for output value."]
    EN = 0,
}
impl From<EN21_A> for bool {
    #[inline(always)]
    fn from(variant: EN21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN21`"]
pub type EN21_R = crate::R<bool, EN21_A>;
impl EN21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN21_A {
        match self.bits {
            true => EN21_A::DIS,
            false => EN21_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN21_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN21_A::EN
    }
}
#[doc = "Write proxy for field `EN21`"]
pub struct EN21_W<'a> {
    w: &'a mut W,
}
impl<'a> EN21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT21 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN21_A::DIS)
    }
    #[doc = "Enable CT21 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN21_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "CT20 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN20_A {
    #[doc = "1: Disable CT20 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT20 for output value."]
    EN = 0,
}
impl From<EN20_A> for bool {
    #[inline(always)]
    fn from(variant: EN20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN20`"]
pub type EN20_R = crate::R<bool, EN20_A>;
impl EN20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN20_A {
        match self.bits {
            true => EN20_A::DIS,
            false => EN20_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN20_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN20_A::EN
    }
}
#[doc = "Write proxy for field `EN20`"]
pub struct EN20_W<'a> {
    w: &'a mut W,
}
impl<'a> EN20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT20 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN20_A::DIS)
    }
    #[doc = "Enable CT20 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN20_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "CT19 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN19_A {
    #[doc = "1: Disable CT19 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT19 for output value."]
    EN = 0,
}
impl From<EN19_A> for bool {
    #[inline(always)]
    fn from(variant: EN19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN19`"]
pub type EN19_R = crate::R<bool, EN19_A>;
impl EN19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN19_A {
        match self.bits {
            true => EN19_A::DIS,
            false => EN19_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN19_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN19_A::EN
    }
}
#[doc = "Write proxy for field `EN19`"]
pub struct EN19_W<'a> {
    w: &'a mut W,
}
impl<'a> EN19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT19 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN19_A::DIS)
    }
    #[doc = "Enable CT19 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN19_A::EN)
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
#[doc = "CT18 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN18_A {
    #[doc = "1: Disable CT18 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT18 for output value."]
    EN = 0,
}
impl From<EN18_A> for bool {
    #[inline(always)]
    fn from(variant: EN18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN18`"]
pub type EN18_R = crate::R<bool, EN18_A>;
impl EN18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN18_A {
        match self.bits {
            true => EN18_A::DIS,
            false => EN18_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN18_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN18_A::EN
    }
}
#[doc = "Write proxy for field `EN18`"]
pub struct EN18_W<'a> {
    w: &'a mut W,
}
impl<'a> EN18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT18 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN18_A::DIS)
    }
    #[doc = "Enable CT18 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN18_A::EN)
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
#[doc = "CT17 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN17_A {
    #[doc = "1: Disable CT17 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT17 for output value."]
    EN = 0,
}
impl From<EN17_A> for bool {
    #[inline(always)]
    fn from(variant: EN17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN17`"]
pub type EN17_R = crate::R<bool, EN17_A>;
impl EN17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN17_A {
        match self.bits {
            true => EN17_A::DIS,
            false => EN17_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN17_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN17_A::EN
    }
}
#[doc = "Write proxy for field `EN17`"]
pub struct EN17_W<'a> {
    w: &'a mut W,
}
impl<'a> EN17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT17 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN17_A::DIS)
    }
    #[doc = "Enable CT17 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN17_A::EN)
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
#[doc = "CT16 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN16_A {
    #[doc = "1: Disable CT16 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT16 for output value."]
    EN = 0,
}
impl From<EN16_A> for bool {
    #[inline(always)]
    fn from(variant: EN16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN16`"]
pub type EN16_R = crate::R<bool, EN16_A>;
impl EN16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN16_A {
        match self.bits {
            true => EN16_A::DIS,
            false => EN16_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN16_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN16_A::EN
    }
}
#[doc = "Write proxy for field `EN16`"]
pub struct EN16_W<'a> {
    w: &'a mut W,
}
impl<'a> EN16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT16 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN16_A::DIS)
    }
    #[doc = "Enable CT16 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN16_A::EN)
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
#[doc = "CT15 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN15_A {
    #[doc = "1: Disable CT15 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT15 for output value."]
    EN = 0,
}
impl From<EN15_A> for bool {
    #[inline(always)]
    fn from(variant: EN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN15`"]
pub type EN15_R = crate::R<bool, EN15_A>;
impl EN15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN15_A {
        match self.bits {
            true => EN15_A::DIS,
            false => EN15_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN15_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN15_A::EN
    }
}
#[doc = "Write proxy for field `EN15`"]
pub struct EN15_W<'a> {
    w: &'a mut W,
}
impl<'a> EN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT15 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN15_A::DIS)
    }
    #[doc = "Enable CT15 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN15_A::EN)
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
#[doc = "CT14 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN14_A {
    #[doc = "1: Disable CT14 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT14 for output value."]
    EN = 0,
}
impl From<EN14_A> for bool {
    #[inline(always)]
    fn from(variant: EN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN14`"]
pub type EN14_R = crate::R<bool, EN14_A>;
impl EN14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN14_A {
        match self.bits {
            true => EN14_A::DIS,
            false => EN14_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN14_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN14_A::EN
    }
}
#[doc = "Write proxy for field `EN14`"]
pub struct EN14_W<'a> {
    w: &'a mut W,
}
impl<'a> EN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT14 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN14_A::DIS)
    }
    #[doc = "Enable CT14 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN14_A::EN)
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
#[doc = "CT13 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN13_A {
    #[doc = "1: Disable CT13 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT13 for output value."]
    EN = 0,
}
impl From<EN13_A> for bool {
    #[inline(always)]
    fn from(variant: EN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN13`"]
pub type EN13_R = crate::R<bool, EN13_A>;
impl EN13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN13_A {
        match self.bits {
            true => EN13_A::DIS,
            false => EN13_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN13_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN13_A::EN
    }
}
#[doc = "Write proxy for field `EN13`"]
pub struct EN13_W<'a> {
    w: &'a mut W,
}
impl<'a> EN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT13 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN13_A::DIS)
    }
    #[doc = "Enable CT13 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN13_A::EN)
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
#[doc = "CT12 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN12_A {
    #[doc = "1: Disable CT12 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT12 for output value."]
    EN = 0,
}
impl From<EN12_A> for bool {
    #[inline(always)]
    fn from(variant: EN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN12`"]
pub type EN12_R = crate::R<bool, EN12_A>;
impl EN12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN12_A {
        match self.bits {
            true => EN12_A::DIS,
            false => EN12_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN12_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN12_A::EN
    }
}
#[doc = "Write proxy for field `EN12`"]
pub struct EN12_W<'a> {
    w: &'a mut W,
}
impl<'a> EN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT12 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN12_A::DIS)
    }
    #[doc = "Enable CT12 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN12_A::EN)
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
#[doc = "CT11 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN11_A {
    #[doc = "1: Disable CT11 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT11 for output value."]
    EN = 0,
}
impl From<EN11_A> for bool {
    #[inline(always)]
    fn from(variant: EN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN11`"]
pub type EN11_R = crate::R<bool, EN11_A>;
impl EN11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN11_A {
        match self.bits {
            true => EN11_A::DIS,
            false => EN11_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN11_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN11_A::EN
    }
}
#[doc = "Write proxy for field `EN11`"]
pub struct EN11_W<'a> {
    w: &'a mut W,
}
impl<'a> EN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT11 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN11_A::DIS)
    }
    #[doc = "Enable CT11 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN11_A::EN)
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
#[doc = "CT10 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN10_A {
    #[doc = "1: Disable CT10 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT10 for output value."]
    EN = 0,
}
impl From<EN10_A> for bool {
    #[inline(always)]
    fn from(variant: EN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN10`"]
pub type EN10_R = crate::R<bool, EN10_A>;
impl EN10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN10_A {
        match self.bits {
            true => EN10_A::DIS,
            false => EN10_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN10_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN10_A::EN
    }
}
#[doc = "Write proxy for field `EN10`"]
pub struct EN10_W<'a> {
    w: &'a mut W,
}
impl<'a> EN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT10 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN10_A::DIS)
    }
    #[doc = "Enable CT10 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN10_A::EN)
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
#[doc = "CT9 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN9_A {
    #[doc = "0: Disable CT9 for output value."]
    DIS = 0,
}
impl From<EN9_A> for bool {
    #[inline(always)]
    fn from(variant: EN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN9`"]
pub type EN9_R = crate::R<bool, EN9_A>;
impl EN9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, EN9_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(EN9_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN9_A::DIS
    }
}
#[doc = "Write proxy for field `EN9`"]
pub struct EN9_W<'a> {
    w: &'a mut W,
}
impl<'a> EN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT9 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN9_A::DIS)
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
#[doc = "CT8 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN8_A {
    #[doc = "1: Disable CT8 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT8 for output value."]
    EN = 0,
}
impl From<EN8_A> for bool {
    #[inline(always)]
    fn from(variant: EN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN8`"]
pub type EN8_R = crate::R<bool, EN8_A>;
impl EN8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN8_A {
        match self.bits {
            true => EN8_A::DIS,
            false => EN8_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN8_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN8_A::EN
    }
}
#[doc = "Write proxy for field `EN8`"]
pub struct EN8_W<'a> {
    w: &'a mut W,
}
impl<'a> EN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT8 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN8_A::DIS)
    }
    #[doc = "Enable CT8 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN8_A::EN)
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
#[doc = "CT7 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN7_A {
    #[doc = "1: Disable CT7 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT7 for output value."]
    EN = 0,
}
impl From<EN7_A> for bool {
    #[inline(always)]
    fn from(variant: EN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN7`"]
pub type EN7_R = crate::R<bool, EN7_A>;
impl EN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN7_A {
        match self.bits {
            true => EN7_A::DIS,
            false => EN7_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN7_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN7_A::EN
    }
}
#[doc = "Write proxy for field `EN7`"]
pub struct EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT7 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN7_A::DIS)
    }
    #[doc = "Enable CT7 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN7_A::EN)
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
#[doc = "CT6 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN6_A {
    #[doc = "1: Disable CT6 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT6 for output value."]
    EN = 0,
}
impl From<EN6_A> for bool {
    #[inline(always)]
    fn from(variant: EN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN6`"]
pub type EN6_R = crate::R<bool, EN6_A>;
impl EN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN6_A {
        match self.bits {
            true => EN6_A::DIS,
            false => EN6_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN6_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN6_A::EN
    }
}
#[doc = "Write proxy for field `EN6`"]
pub struct EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT6 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN6_A::DIS)
    }
    #[doc = "Enable CT6 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN6_A::EN)
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
#[doc = "CT5 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN5_A {
    #[doc = "1: Disable CT5 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT5 for output value."]
    EN = 0,
}
impl From<EN5_A> for bool {
    #[inline(always)]
    fn from(variant: EN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN5`"]
pub type EN5_R = crate::R<bool, EN5_A>;
impl EN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN5_A {
        match self.bits {
            true => EN5_A::DIS,
            false => EN5_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN5_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN5_A::EN
    }
}
#[doc = "Write proxy for field `EN5`"]
pub struct EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT5 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN5_A::DIS)
    }
    #[doc = "Enable CT5 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN5_A::EN)
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
#[doc = "CT4 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN4_A {
    #[doc = "1: Disable CT4 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT4 for output value."]
    EN = 0,
}
impl From<EN4_A> for bool {
    #[inline(always)]
    fn from(variant: EN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN4`"]
pub type EN4_R = crate::R<bool, EN4_A>;
impl EN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN4_A {
        match self.bits {
            true => EN4_A::DIS,
            false => EN4_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN4_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN4_A::EN
    }
}
#[doc = "Write proxy for field `EN4`"]
pub struct EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT4 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN4_A::DIS)
    }
    #[doc = "Enable CT4 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN4_A::EN)
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
#[doc = "CT3 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN3_A {
    #[doc = "1: Disable CT3 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT3 for output value."]
    EN = 0,
}
impl From<EN3_A> for bool {
    #[inline(always)]
    fn from(variant: EN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN3`"]
pub type EN3_R = crate::R<bool, EN3_A>;
impl EN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN3_A {
        match self.bits {
            true => EN3_A::DIS,
            false => EN3_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN3_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN3_A::EN
    }
}
#[doc = "Write proxy for field `EN3`"]
pub struct EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT3 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN3_A::DIS)
    }
    #[doc = "Enable CT3 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN3_A::EN)
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
#[doc = "CT2 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN2_A {
    #[doc = "1: Disable CT2 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT2 for output value."]
    EN = 0,
}
impl From<EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN2`"]
pub type EN2_R = crate::R<bool, EN2_A>;
impl EN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN2_A {
        match self.bits {
            true => EN2_A::DIS,
            false => EN2_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN2_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN2_A::EN
    }
}
#[doc = "Write proxy for field `EN2`"]
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT2 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN2_A::DIS)
    }
    #[doc = "Enable CT2 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN2_A::EN)
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
#[doc = "CT1 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1_A {
    #[doc = "1: Disable CT1 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT1 for output value."]
    EN = 0,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN1`"]
pub type EN1_R = crate::R<bool, EN1_A>;
impl EN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            true => EN1_A::DIS,
            false => EN1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN1_A::EN
    }
}
#[doc = "Write proxy for field `EN1`"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT1 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN1_A::DIS)
    }
    #[doc = "Enable CT1 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN1_A::EN)
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
#[doc = "CT0 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN0_A {
    #[doc = "1: Disable CT0 for output value."]
    DIS = 1,
    #[doc = "0: Enable CT0 for output value."]
    EN = 0,
}
impl From<EN0_A> for bool {
    #[inline(always)]
    fn from(variant: EN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN0`"]
pub type EN0_R = crate::R<bool, EN0_A>;
impl EN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN0_A {
        match self.bits {
            true => EN0_A::DIS,
            false => EN0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN0_A::EN
    }
}
#[doc = "Write proxy for field `EN0`"]
pub struct EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CT0 for output value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN0_A::DIS)
    }
    #[doc = "Enable CT0 for output value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN0_A::EN)
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
    #[doc = "Bit 31 - CT31 Enable"]
    #[inline(always)]
    pub fn en31(&self) -> EN31_R {
        EN31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - CT30 Enable"]
    #[inline(always)]
    pub fn en30(&self) -> EN30_R {
        EN30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - CT29 Enable"]
    #[inline(always)]
    pub fn en29(&self) -> EN29_R {
        EN29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CT28 Enable"]
    #[inline(always)]
    pub fn en28(&self) -> EN28_R {
        EN28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CT27 Enable"]
    #[inline(always)]
    pub fn en27(&self) -> EN27_R {
        EN27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CT26 Enable"]
    #[inline(always)]
    pub fn en26(&self) -> EN26_R {
        EN26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CT25 Enable"]
    #[inline(always)]
    pub fn en25(&self) -> EN25_R {
        EN25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CT24 Enable"]
    #[inline(always)]
    pub fn en24(&self) -> EN24_R {
        EN24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CT23 Enable"]
    #[inline(always)]
    pub fn en23(&self) -> EN23_R {
        EN23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CT22 Enable"]
    #[inline(always)]
    pub fn en22(&self) -> EN22_R {
        EN22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CT21 Enable"]
    #[inline(always)]
    pub fn en21(&self) -> EN21_R {
        EN21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CT20 Enable"]
    #[inline(always)]
    pub fn en20(&self) -> EN20_R {
        EN20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CT19 Enable"]
    #[inline(always)]
    pub fn en19(&self) -> EN19_R {
        EN19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CT18 Enable"]
    #[inline(always)]
    pub fn en18(&self) -> EN18_R {
        EN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CT17 Enable"]
    #[inline(always)]
    pub fn en17(&self) -> EN17_R {
        EN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CT16 Enable"]
    #[inline(always)]
    pub fn en16(&self) -> EN16_R {
        EN16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CT15 Enable"]
    #[inline(always)]
    pub fn en15(&self) -> EN15_R {
        EN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CT14 Enable"]
    #[inline(always)]
    pub fn en14(&self) -> EN14_R {
        EN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CT13 Enable"]
    #[inline(always)]
    pub fn en13(&self) -> EN13_R {
        EN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CT12 Enable"]
    #[inline(always)]
    pub fn en12(&self) -> EN12_R {
        EN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CT11 Enable"]
    #[inline(always)]
    pub fn en11(&self) -> EN11_R {
        EN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CT10 Enable"]
    #[inline(always)]
    pub fn en10(&self) -> EN10_R {
        EN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CT9 Enable"]
    #[inline(always)]
    pub fn en9(&self) -> EN9_R {
        EN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CT8 Enable"]
    #[inline(always)]
    pub fn en8(&self) -> EN8_R {
        EN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CT7 Enable"]
    #[inline(always)]
    pub fn en7(&self) -> EN7_R {
        EN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CT6 Enable"]
    #[inline(always)]
    pub fn en6(&self) -> EN6_R {
        EN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CT5 Enable"]
    #[inline(always)]
    pub fn en5(&self) -> EN5_R {
        EN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CT4 Enable"]
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CT3 Enable"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CT2 Enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CT1 Enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CT0 Enable"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - CT31 Enable"]
    #[inline(always)]
    pub fn en31(&mut self) -> EN31_W {
        EN31_W { w: self }
    }
    #[doc = "Bit 30 - CT30 Enable"]
    #[inline(always)]
    pub fn en30(&mut self) -> EN30_W {
        EN30_W { w: self }
    }
    #[doc = "Bit 29 - CT29 Enable"]
    #[inline(always)]
    pub fn en29(&mut self) -> EN29_W {
        EN29_W { w: self }
    }
    #[doc = "Bit 28 - CT28 Enable"]
    #[inline(always)]
    pub fn en28(&mut self) -> EN28_W {
        EN28_W { w: self }
    }
    #[doc = "Bit 27 - CT27 Enable"]
    #[inline(always)]
    pub fn en27(&mut self) -> EN27_W {
        EN27_W { w: self }
    }
    #[doc = "Bit 26 - CT26 Enable"]
    #[inline(always)]
    pub fn en26(&mut self) -> EN26_W {
        EN26_W { w: self }
    }
    #[doc = "Bit 25 - CT25 Enable"]
    #[inline(always)]
    pub fn en25(&mut self) -> EN25_W {
        EN25_W { w: self }
    }
    #[doc = "Bit 24 - CT24 Enable"]
    #[inline(always)]
    pub fn en24(&mut self) -> EN24_W {
        EN24_W { w: self }
    }
    #[doc = "Bit 23 - CT23 Enable"]
    #[inline(always)]
    pub fn en23(&mut self) -> EN23_W {
        EN23_W { w: self }
    }
    #[doc = "Bit 22 - CT22 Enable"]
    #[inline(always)]
    pub fn en22(&mut self) -> EN22_W {
        EN22_W { w: self }
    }
    #[doc = "Bit 21 - CT21 Enable"]
    #[inline(always)]
    pub fn en21(&mut self) -> EN21_W {
        EN21_W { w: self }
    }
    #[doc = "Bit 20 - CT20 Enable"]
    #[inline(always)]
    pub fn en20(&mut self) -> EN20_W {
        EN20_W { w: self }
    }
    #[doc = "Bit 19 - CT19 Enable"]
    #[inline(always)]
    pub fn en19(&mut self) -> EN19_W {
        EN19_W { w: self }
    }
    #[doc = "Bit 18 - CT18 Enable"]
    #[inline(always)]
    pub fn en18(&mut self) -> EN18_W {
        EN18_W { w: self }
    }
    #[doc = "Bit 17 - CT17 Enable"]
    #[inline(always)]
    pub fn en17(&mut self) -> EN17_W {
        EN17_W { w: self }
    }
    #[doc = "Bit 16 - CT16 Enable"]
    #[inline(always)]
    pub fn en16(&mut self) -> EN16_W {
        EN16_W { w: self }
    }
    #[doc = "Bit 15 - CT15 Enable"]
    #[inline(always)]
    pub fn en15(&mut self) -> EN15_W {
        EN15_W { w: self }
    }
    #[doc = "Bit 14 - CT14 Enable"]
    #[inline(always)]
    pub fn en14(&mut self) -> EN14_W {
        EN14_W { w: self }
    }
    #[doc = "Bit 13 - CT13 Enable"]
    #[inline(always)]
    pub fn en13(&mut self) -> EN13_W {
        EN13_W { w: self }
    }
    #[doc = "Bit 12 - CT12 Enable"]
    #[inline(always)]
    pub fn en12(&mut self) -> EN12_W {
        EN12_W { w: self }
    }
    #[doc = "Bit 11 - CT11 Enable"]
    #[inline(always)]
    pub fn en11(&mut self) -> EN11_W {
        EN11_W { w: self }
    }
    #[doc = "Bit 10 - CT10 Enable"]
    #[inline(always)]
    pub fn en10(&mut self) -> EN10_W {
        EN10_W { w: self }
    }
    #[doc = "Bit 9 - CT9 Enable"]
    #[inline(always)]
    pub fn en9(&mut self) -> EN9_W {
        EN9_W { w: self }
    }
    #[doc = "Bit 8 - CT8 Enable"]
    #[inline(always)]
    pub fn en8(&mut self) -> EN8_W {
        EN8_W { w: self }
    }
    #[doc = "Bit 7 - CT7 Enable"]
    #[inline(always)]
    pub fn en7(&mut self) -> EN7_W {
        EN7_W { w: self }
    }
    #[doc = "Bit 6 - CT6 Enable"]
    #[inline(always)]
    pub fn en6(&mut self) -> EN6_W {
        EN6_W { w: self }
    }
    #[doc = "Bit 5 - CT5 Enable"]
    #[inline(always)]
    pub fn en5(&mut self) -> EN5_W {
        EN5_W { w: self }
    }
    #[doc = "Bit 4 - CT4 Enable"]
    #[inline(always)]
    pub fn en4(&mut self) -> EN4_W {
        EN4_W { w: self }
    }
    #[doc = "Bit 3 - CT3 Enable"]
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W {
        EN3_W { w: self }
    }
    #[doc = "Bit 2 - CT2 Enable"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    #[doc = "Bit 1 - CT1 Enable"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Bit 0 - CT0 Enable"]
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W {
        EN0_W { w: self }
    }
}
