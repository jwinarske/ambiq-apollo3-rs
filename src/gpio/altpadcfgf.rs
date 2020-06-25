#[doc = "Reader of register ALTPADCFGF"]
pub type R = crate::R<u32, super::ALTPADCFGF>;
#[doc = "Writer for register ALTPADCFGF"]
pub type W = crate::W<u32, super::ALTPADCFGF>;
#[doc = "Register ALTPADCFGF `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 23 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD23_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD23_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD23_SR`"]
pub type PAD23_SR_R = crate::R<bool, PAD23_SR_A>;
impl PAD23_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD23_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD23_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD23_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD23_SR`"]
pub struct PAD23_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD23_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD23_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD23_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD23_DS1`"]
pub type PAD23_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD23_DS1`"]
pub struct PAD23_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD23_DS1_W<'a> {
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
#[doc = "Pad 22 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD22_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD22_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD22_SR`"]
pub type PAD22_SR_R = crate::R<bool, PAD22_SR_A>;
impl PAD22_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD22_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD22_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD22_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD22_SR`"]
pub struct PAD22_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD22_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD22_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD22_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD22_DS1`"]
pub type PAD22_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD22_DS1`"]
pub struct PAD22_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD22_DS1_W<'a> {
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
#[doc = "Pad 21 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD21_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD21_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD21_SR`"]
pub type PAD21_SR_R = crate::R<bool, PAD21_SR_A>;
impl PAD21_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD21_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD21_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD21_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD21_SR`"]
pub struct PAD21_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD21_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD21_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD21_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD21_DS1`"]
pub type PAD21_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD21_DS1`"]
pub struct PAD21_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD21_DS1_W<'a> {
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
#[doc = "Pad 20 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD20_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD20_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD20_SR`"]
pub type PAD20_SR_R = crate::R<bool, PAD20_SR_A>;
impl PAD20_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD20_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD20_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD20_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD20_SR`"]
pub struct PAD20_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD20_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD20_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD20_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD20_DS1`"]
pub type PAD20_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD20_DS1`"]
pub struct PAD20_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD20_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 23 slew rate selection."]
    #[inline(always)]
    pub fn pad23_sr(&self) -> PAD23_SR_R {
        PAD23_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad23_ds1(&self) -> PAD23_DS1_R {
        PAD23_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 22 slew rate selection."]
    #[inline(always)]
    pub fn pad22_sr(&self) -> PAD22_SR_R {
        PAD22_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad22_ds1(&self) -> PAD22_DS1_R {
        PAD22_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 21 slew rate selection."]
    #[inline(always)]
    pub fn pad21_sr(&self) -> PAD21_SR_R {
        PAD21_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad21_ds1(&self) -> PAD21_DS1_R {
        PAD21_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 20 slew rate selection."]
    #[inline(always)]
    pub fn pad20_sr(&self) -> PAD20_SR_R {
        PAD20_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad20_ds1(&self) -> PAD20_DS1_R {
        PAD20_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 23 slew rate selection."]
    #[inline(always)]
    pub fn pad23_sr(&mut self) -> PAD23_SR_W {
        PAD23_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad23_ds1(&mut self) -> PAD23_DS1_W {
        PAD23_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 22 slew rate selection."]
    #[inline(always)]
    pub fn pad22_sr(&mut self) -> PAD22_SR_W {
        PAD22_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad22_ds1(&mut self) -> PAD22_DS1_W {
        PAD22_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 21 slew rate selection."]
    #[inline(always)]
    pub fn pad21_sr(&mut self) -> PAD21_SR_W {
        PAD21_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad21_ds1(&mut self) -> PAD21_DS1_W {
        PAD21_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 20 slew rate selection."]
    #[inline(always)]
    pub fn pad20_sr(&mut self) -> PAD20_SR_W {
        PAD20_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad20_ds1(&mut self) -> PAD20_DS1_W {
        PAD20_DS1_W { w: self }
    }
}
