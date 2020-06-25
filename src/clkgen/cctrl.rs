#[doc = "Reader of register CCTRL"]
pub type R = crate::R<u32, super::CCTRL>;
#[doc = "Writer for register CCTRL"]
pub type W = crate::W<u32, super::CCTRL>;
#[doc = "Register CCTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Core Clock divisor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORESEL_A {
    #[doc = "0: Core Clock is HFRC value."]
    HFRC = 0,
    #[doc = "1: Core Clock is HFRC / 2 value."]
    HFRC_DIV2 = 1,
}
impl From<CORESEL_A> for bool {
    #[inline(always)]
    fn from(variant: CORESEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CORESEL`"]
pub type CORESEL_R = crate::R<bool, CORESEL_A>;
impl CORESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORESEL_A {
        match self.bits {
            false => CORESEL_A::HFRC,
            true => CORESEL_A::HFRC_DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == CORESEL_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2`"]
    #[inline(always)]
    pub fn is_hfrc_div2(&self) -> bool {
        *self == CORESEL_A::HFRC_DIV2
    }
}
#[doc = "Write proxy for field `CORESEL`"]
pub struct CORESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CORESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORESEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Core Clock is HFRC value."]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC)
    }
    #[doc = "Core Clock is HFRC / 2 value."]
    #[inline(always)]
    pub fn hfrc_div2(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Core Clock divisor"]
    #[inline(always)]
    pub fn coresel(&self) -> CORESEL_R {
        CORESEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Clock divisor"]
    #[inline(always)]
    pub fn coresel(&mut self) -> CORESEL_W {
        CORESEL_W { w: self }
    }
}
