#[doc = "Reader of register ALTPADCFGK"]
pub type R = crate::R<u32, super::ALTPADCFGK>;
#[doc = "Writer for register ALTPADCFGK"]
pub type W = crate::W<u32, super::ALTPADCFGK>;
#[doc = "Register ALTPADCFGK `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 43 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD43_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD43_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD43_SR`"]
pub type PAD43_SR_R = crate::R<bool, PAD43_SR_A>;
impl PAD43_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD43_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD43_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD43_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD43_SR`"]
pub struct PAD43_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD43_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD43_DS1`"]
pub type PAD43_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD43_DS1`"]
pub struct PAD43_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43_DS1_W<'a> {
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
#[doc = "Pad 42 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD42_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD42_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD42_SR`"]
pub type PAD42_SR_R = crate::R<bool, PAD42_SR_A>;
impl PAD42_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD42_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD42_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD42_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD42_SR`"]
pub struct PAD42_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD42_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD42_DS1`"]
pub type PAD42_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD42_DS1`"]
pub struct PAD42_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42_DS1_W<'a> {
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
#[doc = "Pad 41 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD41_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD41_SR`"]
pub type PAD41_SR_R = crate::R<bool, PAD41_SR_A>;
impl PAD41_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD41_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD41_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD41_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD41_SR`"]
pub struct PAD41_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD41_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD41_DS1`"]
pub type PAD41_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD41_DS1`"]
pub struct PAD41_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41_DS1_W<'a> {
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
#[doc = "Pad 40 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD40_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD40_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD40_SR`"]
pub type PAD40_SR_R = crate::R<bool, PAD40_SR_A>;
impl PAD40_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD40_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD40_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD40_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD40_SR`"]
pub struct PAD40_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD40_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD40_DS1`"]
pub type PAD40_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD40_DS1`"]
pub struct PAD40_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 43 slew rate selection."]
    #[inline(always)]
    pub fn pad43_sr(&self) -> PAD43_SR_R {
        PAD43_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad43_ds1(&self) -> PAD43_DS1_R {
        PAD43_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 42 slew rate selection."]
    #[inline(always)]
    pub fn pad42_sr(&self) -> PAD42_SR_R {
        PAD42_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad42_ds1(&self) -> PAD42_DS1_R {
        PAD42_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 41 slew rate selection."]
    #[inline(always)]
    pub fn pad41_sr(&self) -> PAD41_SR_R {
        PAD41_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad41_ds1(&self) -> PAD41_DS1_R {
        PAD41_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 40 slew rate selection."]
    #[inline(always)]
    pub fn pad40_sr(&self) -> PAD40_SR_R {
        PAD40_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad40_ds1(&self) -> PAD40_DS1_R {
        PAD40_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 43 slew rate selection."]
    #[inline(always)]
    pub fn pad43_sr(&mut self) -> PAD43_SR_W {
        PAD43_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad43_ds1(&mut self) -> PAD43_DS1_W {
        PAD43_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 42 slew rate selection."]
    #[inline(always)]
    pub fn pad42_sr(&mut self) -> PAD42_SR_W {
        PAD42_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad42_ds1(&mut self) -> PAD42_DS1_W {
        PAD42_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 41 slew rate selection."]
    #[inline(always)]
    pub fn pad41_sr(&mut self) -> PAD41_SR_W {
        PAD41_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad41_ds1(&mut self) -> PAD41_DS1_W {
        PAD41_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 40 slew rate selection."]
    #[inline(always)]
    pub fn pad40_sr(&mut self) -> PAD40_SR_W {
        PAD40_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad40_ds1(&mut self) -> PAD40_DS1_W {
        PAD40_DS1_W { w: self }
    }
}
