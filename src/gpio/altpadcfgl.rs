#[doc = "Reader of register ALTPADCFGL"]
pub type R = crate::R<u32, super::ALTPADCFGL>;
#[doc = "Writer for register ALTPADCFGL"]
pub type W = crate::W<u32, super::ALTPADCFGL>;
#[doc = "Register ALTPADCFGL `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 47 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD47_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD47_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD47_SR`"]
pub type PAD47_SR_R = crate::R<bool, PAD47_SR_A>;
impl PAD47_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD47_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD47_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD47_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD47_SR`"]
pub struct PAD47_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD47_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD47_DS1`"]
pub type PAD47_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD47_DS1`"]
pub struct PAD47_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47_DS1_W<'a> {
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
#[doc = "Pad 46 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD46_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD46_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD46_SR`"]
pub type PAD46_SR_R = crate::R<bool, PAD46_SR_A>;
impl PAD46_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD46_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD46_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD46_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD46_SR`"]
pub struct PAD46_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD46_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD46_DS1`"]
pub type PAD46_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD46_DS1`"]
pub struct PAD46_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46_DS1_W<'a> {
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
#[doc = "Pad 45 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD45_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD45_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD45_SR`"]
pub type PAD45_SR_R = crate::R<bool, PAD45_SR_A>;
impl PAD45_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD45_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD45_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD45_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD45_SR`"]
pub struct PAD45_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD45_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD45_DS1`"]
pub type PAD45_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD45_DS1`"]
pub struct PAD45_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45_DS1_W<'a> {
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
#[doc = "Pad 44 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD44_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD44_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD44_SR`"]
pub type PAD44_SR_R = crate::R<bool, PAD44_SR_A>;
impl PAD44_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD44_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD44_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD44_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD44_SR`"]
pub struct PAD44_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD44_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD44_DS1`"]
pub type PAD44_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD44_DS1`"]
pub struct PAD44_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 47 slew rate selection."]
    #[inline(always)]
    pub fn pad47_sr(&self) -> PAD47_SR_R {
        PAD47_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad47_ds1(&self) -> PAD47_DS1_R {
        PAD47_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 46 slew rate selection."]
    #[inline(always)]
    pub fn pad46_sr(&self) -> PAD46_SR_R {
        PAD46_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad46_ds1(&self) -> PAD46_DS1_R {
        PAD46_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 45 slew rate selection."]
    #[inline(always)]
    pub fn pad45_sr(&self) -> PAD45_SR_R {
        PAD45_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad45_ds1(&self) -> PAD45_DS1_R {
        PAD45_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 44 slew rate selection."]
    #[inline(always)]
    pub fn pad44_sr(&self) -> PAD44_SR_R {
        PAD44_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad44_ds1(&self) -> PAD44_DS1_R {
        PAD44_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 47 slew rate selection."]
    #[inline(always)]
    pub fn pad47_sr(&mut self) -> PAD47_SR_W {
        PAD47_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad47_ds1(&mut self) -> PAD47_DS1_W {
        PAD47_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 46 slew rate selection."]
    #[inline(always)]
    pub fn pad46_sr(&mut self) -> PAD46_SR_W {
        PAD46_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad46_ds1(&mut self) -> PAD46_DS1_W {
        PAD46_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 45 slew rate selection."]
    #[inline(always)]
    pub fn pad45_sr(&mut self) -> PAD45_SR_W {
        PAD45_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad45_ds1(&mut self) -> PAD45_DS1_W {
        PAD45_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 44 slew rate selection."]
    #[inline(always)]
    pub fn pad44_sr(&mut self) -> PAD44_SR_W {
        PAD44_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad44_ds1(&mut self) -> PAD44_DS1_W {
        PAD44_DS1_W { w: self }
    }
}
