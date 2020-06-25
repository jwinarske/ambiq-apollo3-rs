#[doc = "Reader of register ALTPADCFGG"]
pub type R = crate::R<u32, super::ALTPADCFGG>;
#[doc = "Writer for register ALTPADCFGG"]
pub type W = crate::W<u32, super::ALTPADCFGG>;
#[doc = "Register ALTPADCFGG `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 27 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD27_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD27_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD27_SR`"]
pub type PAD27_SR_R = crate::R<bool, PAD27_SR_A>;
impl PAD27_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD27_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD27_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD27_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD27_SR`"]
pub struct PAD27_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD27_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD27_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD27_DS1`"]
pub type PAD27_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD27_DS1`"]
pub struct PAD27_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27_DS1_W<'a> {
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
#[doc = "Pad 26 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD26_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD26_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD26_SR`"]
pub type PAD26_SR_R = crate::R<bool, PAD26_SR_A>;
impl PAD26_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD26_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD26_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD26_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD26_SR`"]
pub struct PAD26_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD26_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD26_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD26_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD26_DS1`"]
pub type PAD26_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD26_DS1`"]
pub struct PAD26_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD26_DS1_W<'a> {
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
#[doc = "Pad 25 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD25_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD25_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD25_SR`"]
pub type PAD25_SR_R = crate::R<bool, PAD25_SR_A>;
impl PAD25_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD25_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD25_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD25_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD25_SR`"]
pub struct PAD25_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD25_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD25_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD25_DS1`"]
pub type PAD25_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD25_DS1`"]
pub struct PAD25_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25_DS1_W<'a> {
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
#[doc = "Pad 24 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD24_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD24_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD24_SR`"]
pub type PAD24_SR_R = crate::R<bool, PAD24_SR_A>;
impl PAD24_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD24_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD24_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD24_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD24_SR`"]
pub struct PAD24_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD24_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD24_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD24_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD24_DS1`"]
pub type PAD24_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD24_DS1`"]
pub struct PAD24_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD24_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 27 slew rate selection."]
    #[inline(always)]
    pub fn pad27_sr(&self) -> PAD27_SR_R {
        PAD27_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad27_ds1(&self) -> PAD27_DS1_R {
        PAD27_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 26 slew rate selection."]
    #[inline(always)]
    pub fn pad26_sr(&self) -> PAD26_SR_R {
        PAD26_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad26_ds1(&self) -> PAD26_DS1_R {
        PAD26_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 25 slew rate selection."]
    #[inline(always)]
    pub fn pad25_sr(&self) -> PAD25_SR_R {
        PAD25_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad25_ds1(&self) -> PAD25_DS1_R {
        PAD25_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 24 slew rate selection."]
    #[inline(always)]
    pub fn pad24_sr(&self) -> PAD24_SR_R {
        PAD24_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad24_ds1(&self) -> PAD24_DS1_R {
        PAD24_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 27 slew rate selection."]
    #[inline(always)]
    pub fn pad27_sr(&mut self) -> PAD27_SR_W {
        PAD27_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad27_ds1(&mut self) -> PAD27_DS1_W {
        PAD27_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 26 slew rate selection."]
    #[inline(always)]
    pub fn pad26_sr(&mut self) -> PAD26_SR_W {
        PAD26_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad26_ds1(&mut self) -> PAD26_DS1_W {
        PAD26_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 25 slew rate selection."]
    #[inline(always)]
    pub fn pad25_sr(&mut self) -> PAD25_SR_W {
        PAD25_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad25_ds1(&mut self) -> PAD25_DS1_W {
        PAD25_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 24 slew rate selection."]
    #[inline(always)]
    pub fn pad24_sr(&mut self) -> PAD24_SR_W {
        PAD24_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad24_ds1(&mut self) -> PAD24_DS1_W {
        PAD24_DS1_W { w: self }
    }
}
