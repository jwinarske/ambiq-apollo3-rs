#[doc = "Reader of register INTCLR"]
pub type R = crate::R<u32, super::INTCLR>;
#[doc = "Writer for register INTCLR"]
pub type W = crate::W<u32, super::INTCLR>;
#[doc = "Register INTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTMRB7C1INT`"]
pub type CTMRB7C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB7C1INT`"]
pub struct CTMRB7C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB7C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `CTMRA7C1INT`"]
pub type CTMRA7C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA7C1INT`"]
pub struct CTMRA7C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA7C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CTMRB6C1INT`"]
pub type CTMRB6C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB6C1INT`"]
pub struct CTMRB6C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB6C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `CTMRA6C1INT`"]
pub type CTMRA6C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA6C1INT`"]
pub struct CTMRA6C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA6C1INT_W<'a> {
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
#[doc = "Reader of field `CTMRB5C1INT`"]
pub type CTMRB5C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB5C1INT`"]
pub struct CTMRB5C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB5C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `CTMRA5C1INT`"]
pub type CTMRA5C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA5C1INT`"]
pub struct CTMRA5C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA5C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CTMRB4C1INT`"]
pub type CTMRB4C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB4C1INT`"]
pub struct CTMRB4C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB4C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `CTMRA4C1INT`"]
pub type CTMRA4C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA4C1INT`"]
pub struct CTMRA4C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA4C1INT_W<'a> {
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
#[doc = "Reader of field `CTMRB3C1INT`"]
pub type CTMRB3C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB3C1INT`"]
pub struct CTMRB3C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB3C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CTMRA3C1INT`"]
pub type CTMRA3C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA3C1INT`"]
pub struct CTMRA3C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA3C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CTMRB2C1INT`"]
pub type CTMRB2C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB2C1INT`"]
pub struct CTMRB2C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB2C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `CTMRA2C1INT`"]
pub type CTMRA2C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA2C1INT`"]
pub struct CTMRA2C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA2C1INT_W<'a> {
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
#[doc = "Reader of field `CTMRB1C1INT`"]
pub type CTMRB1C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB1C1INT`"]
pub struct CTMRB1C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB1C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `CTMRA1C1INT`"]
pub type CTMRA1C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA1C1INT`"]
pub struct CTMRA1C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA1C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CTMRB0C1INT`"]
pub type CTMRB0C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB0C1INT`"]
pub struct CTMRB0C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB0C1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CTMRA0C1INT`"]
pub type CTMRA0C1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA0C1INT`"]
pub struct CTMRA0C1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA0C1INT_W<'a> {
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
#[doc = "Reader of field `CTMRB7C0INT`"]
pub type CTMRB7C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB7C0INT`"]
pub struct CTMRB7C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB7C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CTMRA7C0INT`"]
pub type CTMRA7C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA7C0INT`"]
pub struct CTMRA7C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA7C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CTMRB6C0INT`"]
pub type CTMRB6C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB6C0INT`"]
pub struct CTMRB6C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB6C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CTMRA6C0INT`"]
pub type CTMRA6C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA6C0INT`"]
pub struct CTMRA6C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA6C0INT_W<'a> {
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
#[doc = "Reader of field `CTMRB5C0INT`"]
pub type CTMRB5C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB5C0INT`"]
pub struct CTMRB5C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB5C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CTMRA5C0INT`"]
pub type CTMRA5C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA5C0INT`"]
pub struct CTMRA5C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA5C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CTMRB4C0INT`"]
pub type CTMRB4C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB4C0INT`"]
pub struct CTMRB4C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB4C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CTMRA4C0INT`"]
pub type CTMRA4C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA4C0INT`"]
pub struct CTMRA4C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA4C0INT_W<'a> {
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
#[doc = "Reader of field `CTMRB3C0INT`"]
pub type CTMRB3C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB3C0INT`"]
pub struct CTMRB3C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB3C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CTMRA3C0INT`"]
pub type CTMRA3C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA3C0INT`"]
pub struct CTMRA3C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA3C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CTMRB2C0INT`"]
pub type CTMRB2C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB2C0INT`"]
pub struct CTMRB2C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB2C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CTMRA2C0INT`"]
pub type CTMRA2C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA2C0INT`"]
pub struct CTMRA2C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA2C0INT_W<'a> {
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
#[doc = "Reader of field `CTMRB1C0INT`"]
pub type CTMRB1C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB1C0INT`"]
pub struct CTMRB1C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB1C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CTMRA1C0INT`"]
pub type CTMRA1C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA1C0INT`"]
pub struct CTMRA1C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA1C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CTMRB0C0INT`"]
pub type CTMRB0C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB0C0INT`"]
pub struct CTMRB0C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB0C0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CTMRA0C0INT`"]
pub type CTMRA0C0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA0C0INT`"]
pub struct CTMRA0C0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA0C0INT_W<'a> {
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
    #[doc = "Bit 31 - Counter/Timer B7 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb7c1int(&self) -> CTMRB7C1INT_R {
        CTMRB7C1INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Counter/Timer A7 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra7c1int(&self) -> CTMRA7C1INT_R {
        CTMRA7C1INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Counter/Timer B6 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb6c1int(&self) -> CTMRB6C1INT_R {
        CTMRB6C1INT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer A6 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra6c1int(&self) -> CTMRA6C1INT_R {
        CTMRA6C1INT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B5 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb5c1int(&self) -> CTMRB5C1INT_R {
        CTMRB5C1INT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer A5 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra5c1int(&self) -> CTMRA5C1INT_R {
        CTMRA5C1INT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B4 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb4c1int(&self) -> CTMRB4C1INT_R {
        CTMRB4C1INT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Counter/Timer A4 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra4c1int(&self) -> CTMRA4C1INT_R {
        CTMRA4C1INT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Counter/Timer B3 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb3c1int(&self) -> CTMRB3C1INT_R {
        CTMRB3C1INT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Counter/Timer A3 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra3c1int(&self) -> CTMRA3C1INT_R {
        CTMRA3C1INT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Counter/Timer B2 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb2c1int(&self) -> CTMRB2C1INT_R {
        CTMRB2C1INT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Counter/Timer A2 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra2c1int(&self) -> CTMRA2C1INT_R {
        CTMRA2C1INT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Counter/Timer B1 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb1c1int(&self) -> CTMRB1C1INT_R {
        CTMRB1C1INT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Counter/Timer A1 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra1c1int(&self) -> CTMRA1C1INT_R {
        CTMRA1C1INT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Counter/Timer B0 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb0c1int(&self) -> CTMRB0C1INT_R {
        CTMRB0C1INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Counter/Timer A0 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra0c1int(&self) -> CTMRA0C1INT_R {
        CTMRA0C1INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Counter/Timer B7 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb7c0int(&self) -> CTMRB7C0INT_R {
        CTMRB7C0INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Counter/Timer A7 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra7c0int(&self) -> CTMRA7C0INT_R {
        CTMRA7C0INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer B6 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb6c0int(&self) -> CTMRB6C0INT_R {
        CTMRB6C0INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A6 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra6c0int(&self) -> CTMRA6C0INT_R {
        CTMRA6C0INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer B5 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb5c0int(&self) -> CTMRB5C0INT_R {
        CTMRB5C0INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A5 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra5c0int(&self) -> CTMRA5C0INT_R {
        CTMRA5C0INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer B4 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb4c0int(&self) -> CTMRB4C0INT_R {
        CTMRB4C0INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Counter/Timer A4 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra4c0int(&self) -> CTMRA4C0INT_R {
        CTMRA4C0INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter/Timer B3 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb3c0int(&self) -> CTMRB3C0INT_R {
        CTMRB3C0INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Counter/Timer A3 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra3c0int(&self) -> CTMRA3C0INT_R {
        CTMRA3C0INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter/Timer B2 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb2c0int(&self) -> CTMRB2C0INT_R {
        CTMRB2C0INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Counter/Timer A2 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra2c0int(&self) -> CTMRA2C0INT_R {
        CTMRA2C0INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Counter/Timer B1 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb1c0int(&self) -> CTMRB1C0INT_R {
        CTMRB1C0INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter/Timer A1 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra1c0int(&self) -> CTMRA1C0INT_R {
        CTMRA1C0INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter/Timer B0 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb0c0int(&self) -> CTMRB0C0INT_R {
        CTMRB0C0INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Counter/Timer A0 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra0c0int(&self) -> CTMRA0C0INT_R {
        CTMRA0C0INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer B7 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb7c1int(&mut self) -> CTMRB7C1INT_W {
        CTMRB7C1INT_W { w: self }
    }
    #[doc = "Bit 30 - Counter/Timer A7 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra7c1int(&mut self) -> CTMRA7C1INT_W {
        CTMRA7C1INT_W { w: self }
    }
    #[doc = "Bit 29 - Counter/Timer B6 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb6c1int(&mut self) -> CTMRB6C1INT_W {
        CTMRB6C1INT_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer A6 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra6c1int(&mut self) -> CTMRA6C1INT_W {
        CTMRA6C1INT_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B5 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb5c1int(&mut self) -> CTMRB5C1INT_W {
        CTMRB5C1INT_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer A5 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra5c1int(&mut self) -> CTMRA5C1INT_W {
        CTMRA5C1INT_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B4 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb4c1int(&mut self) -> CTMRB4C1INT_W {
        CTMRB4C1INT_W { w: self }
    }
    #[doc = "Bit 24 - Counter/Timer A4 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra4c1int(&mut self) -> CTMRA4C1INT_W {
        CTMRA4C1INT_W { w: self }
    }
    #[doc = "Bit 23 - Counter/Timer B3 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb3c1int(&mut self) -> CTMRB3C1INT_W {
        CTMRB3C1INT_W { w: self }
    }
    #[doc = "Bit 22 - Counter/Timer A3 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra3c1int(&mut self) -> CTMRA3C1INT_W {
        CTMRA3C1INT_W { w: self }
    }
    #[doc = "Bit 21 - Counter/Timer B2 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb2c1int(&mut self) -> CTMRB2C1INT_W {
        CTMRB2C1INT_W { w: self }
    }
    #[doc = "Bit 20 - Counter/Timer A2 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra2c1int(&mut self) -> CTMRA2C1INT_W {
        CTMRA2C1INT_W { w: self }
    }
    #[doc = "Bit 19 - Counter/Timer B1 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb1c1int(&mut self) -> CTMRB1C1INT_W {
        CTMRB1C1INT_W { w: self }
    }
    #[doc = "Bit 18 - Counter/Timer A1 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra1c1int(&mut self) -> CTMRA1C1INT_W {
        CTMRA1C1INT_W { w: self }
    }
    #[doc = "Bit 17 - Counter/Timer B0 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmrb0c1int(&mut self) -> CTMRB0C1INT_W {
        CTMRB0C1INT_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer A0 interrupt based on COMPR1."]
    #[inline(always)]
    pub fn ctmra0c1int(&mut self) -> CTMRA0C1INT_W {
        CTMRA0C1INT_W { w: self }
    }
    #[doc = "Bit 15 - Counter/Timer B7 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb7c0int(&mut self) -> CTMRB7C0INT_W {
        CTMRB7C0INT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A7 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra7c0int(&mut self) -> CTMRA7C0INT_W {
        CTMRA7C0INT_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer B6 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb6c0int(&mut self) -> CTMRB6C0INT_W {
        CTMRB6C0INT_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A6 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra6c0int(&mut self) -> CTMRA6C0INT_W {
        CTMRA6C0INT_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer B5 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb5c0int(&mut self) -> CTMRB5C0INT_W {
        CTMRB5C0INT_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A5 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra5c0int(&mut self) -> CTMRA5C0INT_W {
        CTMRA5C0INT_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer B4 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb4c0int(&mut self) -> CTMRB4C0INT_W {
        CTMRB4C0INT_W { w: self }
    }
    #[doc = "Bit 8 - Counter/Timer A4 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra4c0int(&mut self) -> CTMRA4C0INT_W {
        CTMRA4C0INT_W { w: self }
    }
    #[doc = "Bit 7 - Counter/Timer B3 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb3c0int(&mut self) -> CTMRB3C0INT_W {
        CTMRB3C0INT_W { w: self }
    }
    #[doc = "Bit 6 - Counter/Timer A3 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra3c0int(&mut self) -> CTMRA3C0INT_W {
        CTMRA3C0INT_W { w: self }
    }
    #[doc = "Bit 5 - Counter/Timer B2 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb2c0int(&mut self) -> CTMRB2C0INT_W {
        CTMRB2C0INT_W { w: self }
    }
    #[doc = "Bit 4 - Counter/Timer A2 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra2c0int(&mut self) -> CTMRA2C0INT_W {
        CTMRA2C0INT_W { w: self }
    }
    #[doc = "Bit 3 - Counter/Timer B1 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb1c0int(&mut self) -> CTMRB1C0INT_W {
        CTMRB1C0INT_W { w: self }
    }
    #[doc = "Bit 2 - Counter/Timer A1 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra1c0int(&mut self) -> CTMRA1C0INT_W {
        CTMRA1C0INT_W { w: self }
    }
    #[doc = "Bit 1 - Counter/Timer B0 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmrb0c0int(&mut self) -> CTMRB0C0INT_W {
        CTMRB0C0INT_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A0 interrupt based on COMPR0."]
    #[inline(always)]
    pub fn ctmra0c0int(&mut self) -> CTMRA0C0INT_W {
        CTMRA0C0INT_W { w: self }
    }
}
