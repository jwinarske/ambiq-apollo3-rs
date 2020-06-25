#[doc = "Reader of register ALTPADCFGH"]
pub type R = crate::R<u32, super::ALTPADCFGH>;
#[doc = "Writer for register ALTPADCFGH"]
pub type W = crate::W<u32, super::ALTPADCFGH>;
#[doc = "Register ALTPADCFGH `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 31 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD31_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD31_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD31_SR`"]
pub type PAD31_SR_R = crate::R<bool, PAD31_SR_A>;
impl PAD31_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD31_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD31_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD31_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD31_SR`"]
pub struct PAD31_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD31_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD31_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD31_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD31_DS1`"]
pub type PAD31_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD31_DS1`"]
pub struct PAD31_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD31_DS1_W<'a> {
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
#[doc = "Pad 30 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD30_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD30_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD30_SR`"]
pub type PAD30_SR_R = crate::R<bool, PAD30_SR_A>;
impl PAD30_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD30_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD30_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD30_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD30_SR`"]
pub struct PAD30_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD30_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD30_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD30_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD30_DS1`"]
pub type PAD30_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD30_DS1`"]
pub struct PAD30_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD30_DS1_W<'a> {
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
#[doc = "Pad 29 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD29_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD29_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD29_SR`"]
pub type PAD29_SR_R = crate::R<bool, PAD29_SR_A>;
impl PAD29_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD29_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD29_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD29_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD29_SR`"]
pub struct PAD29_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD29_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD29_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD29_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD29_DS1`"]
pub type PAD29_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD29_DS1`"]
pub struct PAD29_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD29_DS1_W<'a> {
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
#[doc = "Pad 28 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD28_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD28_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD28_SR`"]
pub type PAD28_SR_R = crate::R<bool, PAD28_SR_A>;
impl PAD28_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD28_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD28_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD28_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD28_SR`"]
pub struct PAD28_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD28_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD28_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD28_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD28_DS1`"]
pub type PAD28_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD28_DS1`"]
pub struct PAD28_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD28_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 31 slew rate selection."]
    #[inline(always)]
    pub fn pad31_sr(&self) -> PAD31_SR_R {
        PAD31_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad31_ds1(&self) -> PAD31_DS1_R {
        PAD31_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 30 slew rate selection."]
    #[inline(always)]
    pub fn pad30_sr(&self) -> PAD30_SR_R {
        PAD30_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad30_ds1(&self) -> PAD30_DS1_R {
        PAD30_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 29 slew rate selection."]
    #[inline(always)]
    pub fn pad29_sr(&self) -> PAD29_SR_R {
        PAD29_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad29_ds1(&self) -> PAD29_DS1_R {
        PAD29_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 28 slew rate selection."]
    #[inline(always)]
    pub fn pad28_sr(&self) -> PAD28_SR_R {
        PAD28_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad28_ds1(&self) -> PAD28_DS1_R {
        PAD28_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 31 slew rate selection."]
    #[inline(always)]
    pub fn pad31_sr(&mut self) -> PAD31_SR_W {
        PAD31_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad31_ds1(&mut self) -> PAD31_DS1_W {
        PAD31_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 30 slew rate selection."]
    #[inline(always)]
    pub fn pad30_sr(&mut self) -> PAD30_SR_W {
        PAD30_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad30_ds1(&mut self) -> PAD30_DS1_W {
        PAD30_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 29 slew rate selection."]
    #[inline(always)]
    pub fn pad29_sr(&mut self) -> PAD29_SR_W {
        PAD29_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad29_ds1(&mut self) -> PAD29_DS1_W {
        PAD29_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 28 slew rate selection."]
    #[inline(always)]
    pub fn pad28_sr(&mut self) -> PAD28_SR_W {
        PAD28_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad28_ds1(&mut self) -> PAD28_DS1_W {
        PAD28_DS1_W { w: self }
    }
}
