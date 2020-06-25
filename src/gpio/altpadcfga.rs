#[doc = "Reader of register ALTPADCFGA"]
pub type R = crate::R<u32, super::ALTPADCFGA>;
#[doc = "Writer for register ALTPADCFGA"]
pub type W = crate::W<u32, super::ALTPADCFGA>;
#[doc = "Register ALTPADCFGA `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 3 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD3_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD3_SR`"]
pub type PAD3_SR_R = crate::R<bool, PAD3_SR_A>;
impl PAD3_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD3_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD3_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD3_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD3_SR`"]
pub struct PAD3_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD3_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD3_DS1`"]
pub type PAD3_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD3_DS1`"]
pub struct PAD3_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3_DS1_W<'a> {
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
#[doc = "Pad 2 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD2_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD2_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD2_SR`"]
pub type PAD2_SR_R = crate::R<bool, PAD2_SR_A>;
impl PAD2_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD2_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD2_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD2_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD2_SR`"]
pub struct PAD2_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD2_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD2_DS1`"]
pub type PAD2_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD2_DS1`"]
pub struct PAD2_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2_DS1_W<'a> {
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
#[doc = "Pad 1 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD1_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD1_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD1_SR`"]
pub type PAD1_SR_R = crate::R<bool, PAD1_SR_A>;
impl PAD1_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD1_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD1_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD1_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD1_SR`"]
pub struct PAD1_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD1_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD1_DS1`"]
pub type PAD1_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD1_DS1`"]
pub struct PAD1_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1_DS1_W<'a> {
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
#[doc = "Pad 0 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD0_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD0_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD0_SR`"]
pub type PAD0_SR_R = crate::R<bool, PAD0_SR_A>;
impl PAD0_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD0_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD0_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD0_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD0_SR`"]
pub struct PAD0_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD0_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD0_DS1`"]
pub type PAD0_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD0_DS1`"]
pub struct PAD0_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 3 slew rate selection."]
    #[inline(always)]
    pub fn pad3_sr(&self) -> PAD3_SR_R {
        PAD3_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad3_ds1(&self) -> PAD3_DS1_R {
        PAD3_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 2 slew rate selection."]
    #[inline(always)]
    pub fn pad2_sr(&self) -> PAD2_SR_R {
        PAD2_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad2_ds1(&self) -> PAD2_DS1_R {
        PAD2_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 1 slew rate selection."]
    #[inline(always)]
    pub fn pad1_sr(&self) -> PAD1_SR_R {
        PAD1_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad1_ds1(&self) -> PAD1_DS1_R {
        PAD1_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 0 slew rate selection."]
    #[inline(always)]
    pub fn pad0_sr(&self) -> PAD0_SR_R {
        PAD0_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad0_ds1(&self) -> PAD0_DS1_R {
        PAD0_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 3 slew rate selection."]
    #[inline(always)]
    pub fn pad3_sr(&mut self) -> PAD3_SR_W {
        PAD3_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad3_ds1(&mut self) -> PAD3_DS1_W {
        PAD3_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 2 slew rate selection."]
    #[inline(always)]
    pub fn pad2_sr(&mut self) -> PAD2_SR_W {
        PAD2_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad2_ds1(&mut self) -> PAD2_DS1_W {
        PAD2_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 1 slew rate selection."]
    #[inline(always)]
    pub fn pad1_sr(&mut self) -> PAD1_SR_W {
        PAD1_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad1_ds1(&mut self) -> PAD1_DS1_W {
        PAD1_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 0 slew rate selection."]
    #[inline(always)]
    pub fn pad0_sr(&mut self) -> PAD0_SR_W {
        PAD0_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad0_ds1(&mut self) -> PAD0_DS1_W {
        PAD0_DS1_W { w: self }
    }
}
