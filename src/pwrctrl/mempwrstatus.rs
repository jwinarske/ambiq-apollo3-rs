#[doc = "Reader of register MEMPWRSTATUS"]
pub type R = crate::R<u32, super::MEMPWRSTATUS>;
#[doc = "Writer for register MEMPWRSTATUS"]
pub type W = crate::W<u32, super::MEMPWRSTATUS>;
#[doc = "Register MEMPWRSTATUS `reset()`'s with value 0x7fff"]
impl crate::ResetValue for super::MEMPWRSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff
    }
}
#[doc = "Reader of field `CACHEB2`"]
pub type CACHEB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHEB2`"]
pub struct CACHEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB2_W<'a> {
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
#[doc = "Reader of field `CACHEB0`"]
pub type CACHEB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHEB0`"]
pub struct CACHEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB0_W<'a> {
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
#[doc = "Reader of field `FLASH1`"]
pub type FLASH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH1`"]
pub struct FLASH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_W<'a> {
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
#[doc = "Reader of field `FLASH0`"]
pub type FLASH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH0`"]
pub struct FLASH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_W<'a> {
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
#[doc = "Reader of field `SRAM9`"]
pub type SRAM9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM9`"]
pub struct SRAM9_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM9_W<'a> {
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
#[doc = "Reader of field `SRAM8`"]
pub type SRAM8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM8`"]
pub struct SRAM8_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM8_W<'a> {
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
#[doc = "Reader of field `SRAM7`"]
pub type SRAM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM7`"]
pub struct SRAM7_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM7_W<'a> {
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
#[doc = "Reader of field `SRAM6`"]
pub type SRAM6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM6`"]
pub struct SRAM6_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM6_W<'a> {
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
#[doc = "Reader of field `SRAM5`"]
pub type SRAM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM5`"]
pub struct SRAM5_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM5_W<'a> {
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
#[doc = "Reader of field `SRAM4`"]
pub type SRAM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM4`"]
pub struct SRAM4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4_W<'a> {
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
#[doc = "Reader of field `SRAM3`"]
pub type SRAM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM3`"]
pub struct SRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3_W<'a> {
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
#[doc = "Reader of field `SRAM2`"]
pub type SRAM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM2`"]
pub struct SRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2_W<'a> {
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
#[doc = "Reader of field `SRAM1`"]
pub type SRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM1`"]
pub struct SRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_W<'a> {
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
#[doc = "Reader of field `SRAM0`"]
pub type SRAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM0`"]
pub struct SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0_W<'a> {
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
#[doc = "Reader of field `DTCM1`"]
pub type DTCM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCM1`"]
pub struct DTCM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM1_W<'a> {
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
#[doc = "Reader of field `DTCM01`"]
pub type DTCM01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCM01`"]
pub struct DTCM01_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM01_W<'a> {
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
#[doc = "Reader of field `DTCM00`"]
pub type DTCM00_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCM00`"]
pub struct DTCM00_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM00_W<'a> {
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
    #[doc = "Bit 16 - This bit is 1 if power is supplied to Cache Bank 2"]
    #[inline(always)]
    pub fn cacheb2(&self) -> CACHEB2_R {
        CACHEB2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit is 1 if power is supplied to Cache Bank 0"]
    #[inline(always)]
    pub fn cacheb0(&self) -> CACHEB0_R {
        CACHEB0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is 1 if power is supplied to FLASH 1"]
    #[inline(always)]
    pub fn flash1(&self) -> FLASH1_R {
        FLASH1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is 1 if power is supplied to FLASH 0"]
    #[inline(always)]
    pub fn flash0(&self) -> FLASH0_R {
        FLASH0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is 1 if power is supplied to SRAM GROUP9"]
    #[inline(always)]
    pub fn sram9(&self) -> SRAM9_R {
        SRAM9_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit is 1 if power is supplied to SRAM GROUP8"]
    #[inline(always)]
    pub fn sram8(&self) -> SRAM8_R {
        SRAM8_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit is 1 if power is supplied to SRAM GROUP7"]
    #[inline(always)]
    pub fn sram7(&self) -> SRAM7_R {
        SRAM7_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit is 1 if power is supplied to SRAM GROUP6"]
    #[inline(always)]
    pub fn sram6(&self) -> SRAM6_R {
        SRAM6_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to SRAM GROUP5"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to SRAM GROUP4"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to SRAM GROUP3"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to SRAM GROUP2"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to SRAM GROUP1"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to SRAM GROUP0"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to DTCM GROUP1"]
    #[inline(always)]
    pub fn dtcm1(&self) -> DTCM1_R {
        DTCM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to DTCM GROUP0_1"]
    #[inline(always)]
    pub fn dtcm01(&self) -> DTCM01_R {
        DTCM01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit is 1 if power is supplied to DTCM GROUP0_0"]
    #[inline(always)]
    pub fn dtcm00(&self) -> DTCM00_R {
        DTCM00_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - This bit is 1 if power is supplied to Cache Bank 2"]
    #[inline(always)]
    pub fn cacheb2(&mut self) -> CACHEB2_W {
        CACHEB2_W { w: self }
    }
    #[doc = "Bit 15 - This bit is 1 if power is supplied to Cache Bank 0"]
    #[inline(always)]
    pub fn cacheb0(&mut self) -> CACHEB0_W {
        CACHEB0_W { w: self }
    }
    #[doc = "Bit 14 - This bit is 1 if power is supplied to FLASH 1"]
    #[inline(always)]
    pub fn flash1(&mut self) -> FLASH1_W {
        FLASH1_W { w: self }
    }
    #[doc = "Bit 13 - This bit is 1 if power is supplied to FLASH 0"]
    #[inline(always)]
    pub fn flash0(&mut self) -> FLASH0_W {
        FLASH0_W { w: self }
    }
    #[doc = "Bit 12 - This bit is 1 if power is supplied to SRAM GROUP9"]
    #[inline(always)]
    pub fn sram9(&mut self) -> SRAM9_W {
        SRAM9_W { w: self }
    }
    #[doc = "Bit 11 - This bit is 1 if power is supplied to SRAM GROUP8"]
    #[inline(always)]
    pub fn sram8(&mut self) -> SRAM8_W {
        SRAM8_W { w: self }
    }
    #[doc = "Bit 10 - This bit is 1 if power is supplied to SRAM GROUP7"]
    #[inline(always)]
    pub fn sram7(&mut self) -> SRAM7_W {
        SRAM7_W { w: self }
    }
    #[doc = "Bit 9 - This bit is 1 if power is supplied to SRAM GROUP6"]
    #[inline(always)]
    pub fn sram6(&mut self) -> SRAM6_W {
        SRAM6_W { w: self }
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to SRAM GROUP5"]
    #[inline(always)]
    pub fn sram5(&mut self) -> SRAM5_W {
        SRAM5_W { w: self }
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to SRAM GROUP4"]
    #[inline(always)]
    pub fn sram4(&mut self) -> SRAM4_W {
        SRAM4_W { w: self }
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to SRAM GROUP3"]
    #[inline(always)]
    pub fn sram3(&mut self) -> SRAM3_W {
        SRAM3_W { w: self }
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to SRAM GROUP2"]
    #[inline(always)]
    pub fn sram2(&mut self) -> SRAM2_W {
        SRAM2_W { w: self }
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to SRAM GROUP1"]
    #[inline(always)]
    pub fn sram1(&mut self) -> SRAM1_W {
        SRAM1_W { w: self }
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to SRAM GROUP0"]
    #[inline(always)]
    pub fn sram0(&mut self) -> SRAM0_W {
        SRAM0_W { w: self }
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to DTCM GROUP1"]
    #[inline(always)]
    pub fn dtcm1(&mut self) -> DTCM1_W {
        DTCM1_W { w: self }
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to DTCM GROUP0_1"]
    #[inline(always)]
    pub fn dtcm01(&mut self) -> DTCM01_W {
        DTCM01_W { w: self }
    }
    #[doc = "Bit 0 - This bit is 1 if power is supplied to DTCM GROUP0_0"]
    #[inline(always)]
    pub fn dtcm00(&mut self) -> DTCM00_W {
        DTCM00_W { w: self }
    }
}
