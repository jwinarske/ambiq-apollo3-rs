#[doc = "Reader of register ALTPADCFGE"]
pub type R = crate::R<u32, super::ALTPADCFGE>;
#[doc = "Writer for register ALTPADCFGE"]
pub type W = crate::W<u32, super::ALTPADCFGE>;
#[doc = "Register ALTPADCFGE `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 19 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD19_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD19_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD19_SR`"]
pub type PAD19_SR_R = crate::R<bool, PAD19_SR_A>;
impl PAD19_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD19_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD19_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD19_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD19_SR`"]
pub struct PAD19_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD19_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD19_DS1`"]
pub type PAD19_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD19_DS1`"]
pub struct PAD19_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19_DS1_W<'a> {
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
#[doc = "Pad 18 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD18_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD18_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD18_SR`"]
pub type PAD18_SR_R = crate::R<bool, PAD18_SR_A>;
impl PAD18_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD18_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD18_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD18_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD18_SR`"]
pub struct PAD18_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD18_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD18_DS1`"]
pub type PAD18_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD18_DS1`"]
pub struct PAD18_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18_DS1_W<'a> {
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
#[doc = "Pad 17 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD17_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD17_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD17_SR`"]
pub type PAD17_SR_R = crate::R<bool, PAD17_SR_A>;
impl PAD17_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD17_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD17_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD17_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD17_SR`"]
pub struct PAD17_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD17_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD17_DS1`"]
pub type PAD17_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD17_DS1`"]
pub struct PAD17_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17_DS1_W<'a> {
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
#[doc = "Pad 16 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD16_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD16_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD16_SR`"]
pub type PAD16_SR_R = crate::R<bool, PAD16_SR_A>;
impl PAD16_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD16_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD16_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD16_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD16_SR`"]
pub struct PAD16_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD16_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD16_DS1`"]
pub type PAD16_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD16_DS1`"]
pub struct PAD16_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 19 slew rate selection."]
    #[inline(always)]
    pub fn pad19_sr(&self) -> PAD19_SR_R {
        PAD19_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad19_ds1(&self) -> PAD19_DS1_R {
        PAD19_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 18 slew rate selection."]
    #[inline(always)]
    pub fn pad18_sr(&self) -> PAD18_SR_R {
        PAD18_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad18_ds1(&self) -> PAD18_DS1_R {
        PAD18_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 17 slew rate selection."]
    #[inline(always)]
    pub fn pad17_sr(&self) -> PAD17_SR_R {
        PAD17_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad17_ds1(&self) -> PAD17_DS1_R {
        PAD17_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 16 slew rate selection."]
    #[inline(always)]
    pub fn pad16_sr(&self) -> PAD16_SR_R {
        PAD16_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad16_ds1(&self) -> PAD16_DS1_R {
        PAD16_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 19 slew rate selection."]
    #[inline(always)]
    pub fn pad19_sr(&mut self) -> PAD19_SR_W {
        PAD19_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad19_ds1(&mut self) -> PAD19_DS1_W {
        PAD19_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 18 slew rate selection."]
    #[inline(always)]
    pub fn pad18_sr(&mut self) -> PAD18_SR_W {
        PAD18_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad18_ds1(&mut self) -> PAD18_DS1_W {
        PAD18_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 17 slew rate selection."]
    #[inline(always)]
    pub fn pad17_sr(&mut self) -> PAD17_SR_W {
        PAD17_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad17_ds1(&mut self) -> PAD17_DS1_W {
        PAD17_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 16 slew rate selection."]
    #[inline(always)]
    pub fn pad16_sr(&mut self) -> PAD16_SR_W {
        PAD16_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad16_ds1(&mut self) -> PAD16_DS1_W {
        PAD16_DS1_W { w: self }
    }
}
