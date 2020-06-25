#[doc = "Reader of register ALTPADCFGB"]
pub type R = crate::R<u32, super::ALTPADCFGB>;
#[doc = "Writer for register ALTPADCFGB"]
pub type W = crate::W<u32, super::ALTPADCFGB>;
#[doc = "Register ALTPADCFGB `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 7 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD7_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD7_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD7_SR`"]
pub type PAD7_SR_R = crate::R<bool, PAD7_SR_A>;
impl PAD7_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD7_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD7_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD7_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD7_SR`"]
pub struct PAD7_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD7_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD7_DS1`"]
pub type PAD7_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD7_DS1`"]
pub struct PAD7_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7_DS1_W<'a> {
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
#[doc = "Pad 6 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD6_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD6_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD6_SR`"]
pub type PAD6_SR_R = crate::R<bool, PAD6_SR_A>;
impl PAD6_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD6_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD6_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD6_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD6_SR`"]
pub struct PAD6_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD6_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD6_DS1`"]
pub type PAD6_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD6_DS1`"]
pub struct PAD6_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6_DS1_W<'a> {
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
#[doc = "Pad 5 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD5_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD5_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD5_SR`"]
pub type PAD5_SR_R = crate::R<bool, PAD5_SR_A>;
impl PAD5_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD5_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD5_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD5_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD5_SR`"]
pub struct PAD5_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD5_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD5_DS1`"]
pub type PAD5_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD5_DS1`"]
pub struct PAD5_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5_DS1_W<'a> {
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
#[doc = "Pad 4 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD4_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD4_SR`"]
pub type PAD4_SR_R = crate::R<bool, PAD4_SR_A>;
impl PAD4_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD4_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD4_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD4_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD4_SR`"]
pub struct PAD4_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD4_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD4_DS1`"]
pub type PAD4_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD4_DS1`"]
pub struct PAD4_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 7 slew rate selection."]
    #[inline(always)]
    pub fn pad7_sr(&self) -> PAD7_SR_R {
        PAD7_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad7_ds1(&self) -> PAD7_DS1_R {
        PAD7_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 6 slew rate selection."]
    #[inline(always)]
    pub fn pad6_sr(&self) -> PAD6_SR_R {
        PAD6_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad6_ds1(&self) -> PAD6_DS1_R {
        PAD6_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 5 slew rate selection."]
    #[inline(always)]
    pub fn pad5_sr(&self) -> PAD5_SR_R {
        PAD5_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad5_ds1(&self) -> PAD5_DS1_R {
        PAD5_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 4 slew rate selection."]
    #[inline(always)]
    pub fn pad4_sr(&self) -> PAD4_SR_R {
        PAD4_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad4_ds1(&self) -> PAD4_DS1_R {
        PAD4_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 7 slew rate selection."]
    #[inline(always)]
    pub fn pad7_sr(&mut self) -> PAD7_SR_W {
        PAD7_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad7_ds1(&mut self) -> PAD7_DS1_W {
        PAD7_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 6 slew rate selection."]
    #[inline(always)]
    pub fn pad6_sr(&mut self) -> PAD6_SR_W {
        PAD6_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad6_ds1(&mut self) -> PAD6_DS1_W {
        PAD6_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 5 slew rate selection."]
    #[inline(always)]
    pub fn pad5_sr(&mut self) -> PAD5_SR_W {
        PAD5_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad5_ds1(&mut self) -> PAD5_DS1_W {
        PAD5_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 4 slew rate selection."]
    #[inline(always)]
    pub fn pad4_sr(&mut self) -> PAD4_SR_W {
        PAD4_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad4_ds1(&mut self) -> PAD4_DS1_W {
        PAD4_DS1_W { w: self }
    }
}
