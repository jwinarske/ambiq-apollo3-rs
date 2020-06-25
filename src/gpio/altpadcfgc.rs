#[doc = "Reader of register ALTPADCFGC"]
pub type R = crate::R<u32, super::ALTPADCFGC>;
#[doc = "Writer for register ALTPADCFGC"]
pub type W = crate::W<u32, super::ALTPADCFGC>;
#[doc = "Register ALTPADCFGC `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 11 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD11_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD11_SR`"]
pub type PAD11_SR_R = crate::R<bool, PAD11_SR_A>;
impl PAD11_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD11_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD11_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD11_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD11_SR`"]
pub struct PAD11_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD11_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD11_DS1`"]
pub type PAD11_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD11_DS1`"]
pub struct PAD11_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11_DS1_W<'a> {
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
#[doc = "Pad 10 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD10_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD10_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD10_SR`"]
pub type PAD10_SR_R = crate::R<bool, PAD10_SR_A>;
impl PAD10_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD10_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD10_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD10_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD10_SR`"]
pub struct PAD10_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD10_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD10_DS1`"]
pub type PAD10_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD10_DS1`"]
pub struct PAD10_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10_DS1_W<'a> {
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
#[doc = "Pad 9 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD9_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD9_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD9_SR`"]
pub type PAD9_SR_R = crate::R<bool, PAD9_SR_A>;
impl PAD9_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD9_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD9_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD9_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD9_SR`"]
pub struct PAD9_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD9_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD9_DS1`"]
pub type PAD9_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD9_DS1`"]
pub struct PAD9_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9_DS1_W<'a> {
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
#[doc = "Pad 8 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD8_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD8_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD8_SR`"]
pub type PAD8_SR_R = crate::R<bool, PAD8_SR_A>;
impl PAD8_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD8_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD8_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD8_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD8_SR`"]
pub struct PAD8_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD8_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD8_DS1`"]
pub type PAD8_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD8_DS1`"]
pub struct PAD8_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 11 slew rate selection."]
    #[inline(always)]
    pub fn pad11_sr(&self) -> PAD11_SR_R {
        PAD11_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad11_ds1(&self) -> PAD11_DS1_R {
        PAD11_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 10 slew rate selection."]
    #[inline(always)]
    pub fn pad10_sr(&self) -> PAD10_SR_R {
        PAD10_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad10_ds1(&self) -> PAD10_DS1_R {
        PAD10_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 9 slew rate selection."]
    #[inline(always)]
    pub fn pad9_sr(&self) -> PAD9_SR_R {
        PAD9_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad9_ds1(&self) -> PAD9_DS1_R {
        PAD9_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 8 slew rate selection."]
    #[inline(always)]
    pub fn pad8_sr(&self) -> PAD8_SR_R {
        PAD8_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad8_ds1(&self) -> PAD8_DS1_R {
        PAD8_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 11 slew rate selection."]
    #[inline(always)]
    pub fn pad11_sr(&mut self) -> PAD11_SR_W {
        PAD11_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad11_ds1(&mut self) -> PAD11_DS1_W {
        PAD11_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 10 slew rate selection."]
    #[inline(always)]
    pub fn pad10_sr(&mut self) -> PAD10_SR_W {
        PAD10_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad10_ds1(&mut self) -> PAD10_DS1_W {
        PAD10_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 9 slew rate selection."]
    #[inline(always)]
    pub fn pad9_sr(&mut self) -> PAD9_SR_W {
        PAD9_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad9_ds1(&mut self) -> PAD9_DS1_W {
        PAD9_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 8 slew rate selection."]
    #[inline(always)]
    pub fn pad8_sr(&mut self) -> PAD8_SR_W {
        PAD8_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad8_ds1(&mut self) -> PAD8_DS1_W {
        PAD8_DS1_W { w: self }
    }
}
