#[doc = "Reader of register ALTPADCFGJ"]
pub type R = crate::R<u32, super::ALTPADCFGJ>;
#[doc = "Writer for register ALTPADCFGJ"]
pub type W = crate::W<u32, super::ALTPADCFGJ>;
#[doc = "Register ALTPADCFGJ `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGJ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 39 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD39_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD39_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD39_SR`"]
pub type PAD39_SR_R = crate::R<bool, PAD39_SR_A>;
impl PAD39_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD39_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD39_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD39_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD39_SR`"]
pub struct PAD39_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD39_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD39_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD39_DS1`"]
pub type PAD39_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD39_DS1`"]
pub struct PAD39_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39_DS1_W<'a> {
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
#[doc = "Pad 38 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD38_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD38_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD38_SR`"]
pub type PAD38_SR_R = crate::R<bool, PAD38_SR_A>;
impl PAD38_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD38_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD38_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD38_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD38_SR`"]
pub struct PAD38_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD38_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD38_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD38_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD38_DS1`"]
pub type PAD38_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD38_DS1`"]
pub struct PAD38_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD38_DS1_W<'a> {
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
#[doc = "Pad 37 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD37_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD37_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD37_SR`"]
pub type PAD37_SR_R = crate::R<bool, PAD37_SR_A>;
impl PAD37_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD37_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD37_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD37_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD37_SR`"]
pub struct PAD37_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD37_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD37_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD37_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD37_DS1`"]
pub type PAD37_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD37_DS1`"]
pub struct PAD37_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD37_DS1_W<'a> {
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
#[doc = "Pad 36 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD36_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD36_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD36_SR`"]
pub type PAD36_SR_R = crate::R<bool, PAD36_SR_A>;
impl PAD36_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD36_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD36_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD36_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD36_SR`"]
pub struct PAD36_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD36_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD36_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD36_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD36_DS1`"]
pub type PAD36_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD36_DS1`"]
pub struct PAD36_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD36_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 39 slew rate selection."]
    #[inline(always)]
    pub fn pad39_sr(&self) -> PAD39_SR_R {
        PAD39_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad39_ds1(&self) -> PAD39_DS1_R {
        PAD39_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 38 slew rate selection."]
    #[inline(always)]
    pub fn pad38_sr(&self) -> PAD38_SR_R {
        PAD38_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad38_ds1(&self) -> PAD38_DS1_R {
        PAD38_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 37 slew rate selection."]
    #[inline(always)]
    pub fn pad37_sr(&self) -> PAD37_SR_R {
        PAD37_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad37_ds1(&self) -> PAD37_DS1_R {
        PAD37_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 36 slew rate selection."]
    #[inline(always)]
    pub fn pad36_sr(&self) -> PAD36_SR_R {
        PAD36_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad36_ds1(&self) -> PAD36_DS1_R {
        PAD36_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 39 slew rate selection."]
    #[inline(always)]
    pub fn pad39_sr(&mut self) -> PAD39_SR_W {
        PAD39_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad39_ds1(&mut self) -> PAD39_DS1_W {
        PAD39_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 38 slew rate selection."]
    #[inline(always)]
    pub fn pad38_sr(&mut self) -> PAD38_SR_W {
        PAD38_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad38_ds1(&mut self) -> PAD38_DS1_W {
        PAD38_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 37 slew rate selection."]
    #[inline(always)]
    pub fn pad37_sr(&mut self) -> PAD37_SR_W {
        PAD37_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad37_ds1(&mut self) -> PAD37_DS1_W {
        PAD37_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 36 slew rate selection."]
    #[inline(always)]
    pub fn pad36_sr(&mut self) -> PAD36_SR_W {
        PAD36_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad36_ds1(&mut self) -> PAD36_DS1_W {
        PAD36_DS1_W { w: self }
    }
}
