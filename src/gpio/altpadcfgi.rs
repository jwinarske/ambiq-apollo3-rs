#[doc = "Reader of register ALTPADCFGI"]
pub type R = crate::R<u32, super::ALTPADCFGI>;
#[doc = "Writer for register ALTPADCFGI"]
pub type W = crate::W<u32, super::ALTPADCFGI>;
#[doc = "Register ALTPADCFGI `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 35 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD35_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD35_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD35_SR`"]
pub type PAD35_SR_R = crate::R<bool, PAD35_SR_A>;
impl PAD35_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD35_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD35_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD35_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD35_SR`"]
pub struct PAD35_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD35_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD35_DS1`"]
pub type PAD35_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD35_DS1`"]
pub struct PAD35_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35_DS1_W<'a> {
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
#[doc = "Pad 34 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD34_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD34_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD34_SR`"]
pub type PAD34_SR_R = crate::R<bool, PAD34_SR_A>;
impl PAD34_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD34_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD34_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD34_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD34_SR`"]
pub struct PAD34_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD34_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD34_DS1`"]
pub type PAD34_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD34_DS1`"]
pub struct PAD34_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34_DS1_W<'a> {
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
#[doc = "Pad 33 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD33_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD33_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD33_SR`"]
pub type PAD33_SR_R = crate::R<bool, PAD33_SR_A>;
impl PAD33_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD33_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD33_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD33_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD33_SR`"]
pub struct PAD33_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD33_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD33_DS1`"]
pub type PAD33_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD33_DS1`"]
pub struct PAD33_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33_DS1_W<'a> {
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
#[doc = "Pad 32 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD32_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD32_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD32_SR`"]
pub type PAD32_SR_R = crate::R<bool, PAD32_SR_A>;
impl PAD32_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD32_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD32_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD32_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD32_SR`"]
pub struct PAD32_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD32_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD32_DS1`"]
pub type PAD32_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD32_DS1`"]
pub struct PAD32_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 35 slew rate selection."]
    #[inline(always)]
    pub fn pad35_sr(&self) -> PAD35_SR_R {
        PAD35_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad35_ds1(&self) -> PAD35_DS1_R {
        PAD35_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 34 slew rate selection."]
    #[inline(always)]
    pub fn pad34_sr(&self) -> PAD34_SR_R {
        PAD34_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad34_ds1(&self) -> PAD34_DS1_R {
        PAD34_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 33 slew rate selection."]
    #[inline(always)]
    pub fn pad33_sr(&self) -> PAD33_SR_R {
        PAD33_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad33_ds1(&self) -> PAD33_DS1_R {
        PAD33_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 32 slew rate selection."]
    #[inline(always)]
    pub fn pad32_sr(&self) -> PAD32_SR_R {
        PAD32_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad32_ds1(&self) -> PAD32_DS1_R {
        PAD32_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 35 slew rate selection."]
    #[inline(always)]
    pub fn pad35_sr(&mut self) -> PAD35_SR_W {
        PAD35_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad35_ds1(&mut self) -> PAD35_DS1_W {
        PAD35_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 34 slew rate selection."]
    #[inline(always)]
    pub fn pad34_sr(&mut self) -> PAD34_SR_W {
        PAD34_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad34_ds1(&mut self) -> PAD34_DS1_W {
        PAD34_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 33 slew rate selection."]
    #[inline(always)]
    pub fn pad33_sr(&mut self) -> PAD33_SR_W {
        PAD33_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad33_ds1(&mut self) -> PAD33_DS1_W {
        PAD33_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 32 slew rate selection."]
    #[inline(always)]
    pub fn pad32_sr(&mut self) -> PAD32_SR_W {
        PAD32_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad32_ds1(&mut self) -> PAD32_DS1_W {
        PAD32_DS1_W { w: self }
    }
}
