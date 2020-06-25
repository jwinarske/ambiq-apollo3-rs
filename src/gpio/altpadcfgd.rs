#[doc = "Reader of register ALTPADCFGD"]
pub type R = crate::R<u32, super::ALTPADCFGD>;
#[doc = "Writer for register ALTPADCFGD"]
pub type W = crate::W<u32, super::ALTPADCFGD>;
#[doc = "Register ALTPADCFGD `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 15 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD15_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD15_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD15_SR`"]
pub type PAD15_SR_R = crate::R<bool, PAD15_SR_A>;
impl PAD15_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD15_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD15_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD15_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD15_SR`"]
pub struct PAD15_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD15_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD15_DS1`"]
pub type PAD15_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD15_DS1`"]
pub struct PAD15_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15_DS1_W<'a> {
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
#[doc = "Pad 14 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD14_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD14_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD14_SR`"]
pub type PAD14_SR_R = crate::R<bool, PAD14_SR_A>;
impl PAD14_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD14_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD14_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD14_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD14_SR`"]
pub struct PAD14_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD14_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD14_DS1`"]
pub type PAD14_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD14_DS1`"]
pub struct PAD14_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14_DS1_W<'a> {
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
#[doc = "Pad 13 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD13_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD13_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD13_SR`"]
pub type PAD13_SR_R = crate::R<bool, PAD13_SR_A>;
impl PAD13_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD13_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD13_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD13_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD13_SR`"]
pub struct PAD13_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD13_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD13_DS1`"]
pub type PAD13_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD13_DS1`"]
pub struct PAD13_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13_DS1_W<'a> {
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
#[doc = "Pad 12 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD12_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD12_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD12_SR`"]
pub type PAD12_SR_R = crate::R<bool, PAD12_SR_A>;
impl PAD12_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD12_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD12_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD12_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD12_SR`"]
pub struct PAD12_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD12_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD12_DS1`"]
pub type PAD12_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD12_DS1`"]
pub struct PAD12_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 15 slew rate selection."]
    #[inline(always)]
    pub fn pad15_sr(&self) -> PAD15_SR_R {
        PAD15_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad15_ds1(&self) -> PAD15_DS1_R {
        PAD15_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 14 slew rate selection."]
    #[inline(always)]
    pub fn pad14_sr(&self) -> PAD14_SR_R {
        PAD14_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad14_ds1(&self) -> PAD14_DS1_R {
        PAD14_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 13 slew rate selection."]
    #[inline(always)]
    pub fn pad13_sr(&self) -> PAD13_SR_R {
        PAD13_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad13_ds1(&self) -> PAD13_DS1_R {
        PAD13_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 12 slew rate selection."]
    #[inline(always)]
    pub fn pad12_sr(&self) -> PAD12_SR_R {
        PAD12_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad12_ds1(&self) -> PAD12_DS1_R {
        PAD12_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 15 slew rate selection."]
    #[inline(always)]
    pub fn pad15_sr(&mut self) -> PAD15_SR_W {
        PAD15_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad15_ds1(&mut self) -> PAD15_DS1_W {
        PAD15_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 14 slew rate selection."]
    #[inline(always)]
    pub fn pad14_sr(&mut self) -> PAD14_SR_W {
        PAD14_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad14_ds1(&mut self) -> PAD14_DS1_W {
        PAD14_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 13 slew rate selection."]
    #[inline(always)]
    pub fn pad13_sr(&mut self) -> PAD13_SR_W {
        PAD13_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad13_ds1(&mut self) -> PAD13_DS1_W {
        PAD13_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 12 slew rate selection."]
    #[inline(always)]
    pub fn pad12_sr(&mut self) -> PAD12_SR_W {
        PAD12_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad12_ds1(&mut self) -> PAD12_DS1_W {
        PAD12_DS1_W { w: self }
    }
}
