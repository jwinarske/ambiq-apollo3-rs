#[doc = "Reader of register PADCFG"]
pub type R = crate::R<u32, super::PADCFG>;
#[doc = "Writer for register PADCFG"]
pub type W = crate::W<u32, super::PADCFG>;
#[doc = "Register PADCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PADCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REVCS`"]
pub type REVCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REVCS`"]
pub struct REVCS_W<'a> {
    w: &'a mut W,
}
impl<'a> REVCS_W<'a> {
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
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN3`"]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
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
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN2`"]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
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
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN1`"]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
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
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN0`"]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `OUT7`"]
pub type OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT7`"]
pub struct OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT7_W<'a> {
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
#[doc = "Reader of field `OUT6`"]
pub type OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT6`"]
pub struct OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT6_W<'a> {
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
#[doc = "Reader of field `OUT5`"]
pub type OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT5`"]
pub struct OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT5_W<'a> {
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
#[doc = "Reader of field `OUT4`"]
pub type OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT4`"]
pub struct OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT4_W<'a> {
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
#[doc = "Reader of field `OUT3`"]
pub type OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT3`"]
pub struct OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT3_W<'a> {
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
    #[doc = "Bit 21 - Reverse CS connections. Allows CS1 to be associated with lower data lanes and CS0 to be associated with upper data lines"]
    #[inline(always)]
    pub fn revcs(&self) -> REVCS_R {
        REVCS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Input pad 3 pin muxing: 0=pad\\[3\\]
1=pad\\[7\\]"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Data Input pad 2 pin muxing: 0=pad\\[2\\]
1=pad\\[6\\]"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Input pad 1 pin muxing: 0=pad\\[1\\]
1=pad\\[5\\]"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Data Input pad 0 pin muxing: 0=pad\\[0\\]
1=pad\\[4\\]
2=pad\\[1\\]
3=pad\\[5\\]"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Output pad 7 configuration. 0=data\\[7\\]
1=data\\[3\\]"]
    #[inline(always)]
    pub fn out7(&self) -> OUT7_R {
        OUT7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output pad 6 configuration. 0=data\\[6\\]
1=data\\[2\\]"]
    #[inline(always)]
    pub fn out6(&self) -> OUT6_R {
        OUT6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output pad 5 configuration. 0=data\\[5\\]
1=data\\[1\\]"]
    #[inline(always)]
    pub fn out5(&self) -> OUT5_R {
        OUT5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output pad 4 configuration. 0=data\\[4\\]
1=data\\[0\\]"]
    #[inline(always)]
    pub fn out4(&self) -> OUT4_R {
        OUT4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Output pad 3 configuration. 0=data\\[3\\]
1=CLK"]
    #[inline(always)]
    pub fn out3(&self) -> OUT3_R {
        OUT3_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Reverse CS connections. Allows CS1 to be associated with lower data lanes and CS0 to be associated with upper data lines"]
    #[inline(always)]
    pub fn revcs(&mut self) -> REVCS_W {
        REVCS_W { w: self }
    }
    #[doc = "Bit 20 - Data Input pad 3 pin muxing: 0=pad\\[3\\]
1=pad\\[7\\]"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W {
        IN3_W { w: self }
    }
    #[doc = "Bit 19 - Data Input pad 2 pin muxing: 0=pad\\[2\\]
1=pad\\[6\\]"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    #[doc = "Bit 18 - Data Input pad 1 pin muxing: 0=pad\\[1\\]
1=pad\\[5\\]"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    #[doc = "Bits 16:17 - Data Input pad 0 pin muxing: 0=pad\\[0\\]
1=pad\\[4\\]
2=pad\\[1\\]
3=pad\\[5\\]"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W {
        IN0_W { w: self }
    }
    #[doc = "Bit 4 - Output pad 7 configuration. 0=data\\[7\\]
1=data\\[3\\]"]
    #[inline(always)]
    pub fn out7(&mut self) -> OUT7_W {
        OUT7_W { w: self }
    }
    #[doc = "Bit 3 - Output pad 6 configuration. 0=data\\[6\\]
1=data\\[2\\]"]
    #[inline(always)]
    pub fn out6(&mut self) -> OUT6_W {
        OUT6_W { w: self }
    }
    #[doc = "Bit 2 - Output pad 5 configuration. 0=data\\[5\\]
1=data\\[1\\]"]
    #[inline(always)]
    pub fn out5(&mut self) -> OUT5_W {
        OUT5_W { w: self }
    }
    #[doc = "Bit 1 - Output pad 4 configuration. 0=data\\[4\\]
1=data\\[0\\]"]
    #[inline(always)]
    pub fn out4(&mut self) -> OUT4_W {
        OUT4_W { w: self }
    }
    #[doc = "Bit 0 - Output pad 3 configuration. 0=data\\[3\\]
1=CLK"]
    #[inline(always)]
    pub fn out3(&mut self) -> OUT3_W {
        OUT3_W { w: self }
    }
}
