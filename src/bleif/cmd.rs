#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSETLO`"]
pub type OFFSETLO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSETLO`"]
pub struct OFFSETLO_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `CMDSEL`"]
pub type CMDSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDSEL`"]
pub struct CMDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `TSIZE`"]
pub type TSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSIZE`"]
pub struct TSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | (((value as u32) & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
#[doc = "Reader of field `OFFSETCNT`"]
pub type OFFSETCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSETCNT`"]
pub struct OFFSETCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Command for submodule.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "1: Write command using count of offset bytes specified in the OFFSETCNT field value."]
    WRITE = 1,
    #[doc = "2: Read command using count of offset bytes specified in the OFFSETCNT field value."]
    READ = 2,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, CMD_A>;
impl CMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMD_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CMD_A::WRITE),
            2 => Val(CMD_A::READ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == CMD_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == CMD_A::READ
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Write command using count of offset bytes specified in the OFFSETCNT field value."]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_A::WRITE)
    }
    #[doc = "Read command using count of offset bytes specified in the OFFSETCNT field value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_A::READ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command. Offset bytes are transferred starting from the highest byte first."]
    #[inline(always)]
    pub fn offsetlo(&self) -> OFFSETLO_R {
        OFFSETLO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - Command Specific selection information"]
    #[inline(always)]
    pub fn cmdsel(&self) -> CMDSEL_R {
        CMDSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 8:19 - Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 7 - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted first, then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline(always)]
    pub fn offsetcnt(&self) -> OFFSETCNT_R {
        OFFSETCNT_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - Command for submodule."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command. Offset bytes are transferred starting from the highest byte first."]
    #[inline(always)]
    pub fn offsetlo(&mut self) -> OFFSETLO_W {
        OFFSETLO_W { w: self }
    }
    #[doc = "Bits 20:21 - Command Specific selection information"]
    #[inline(always)]
    pub fn cmdsel(&mut self) -> CMDSEL_W {
        CMDSEL_W { w: self }
    }
    #[doc = "Bits 8:19 - Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline(always)]
    pub fn tsize(&mut self) -> TSIZE_W {
        TSIZE_W { w: self }
    }
    #[doc = "Bit 7 - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bits 5:6 - Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted first, then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline(always)]
    pub fn offsetcnt(&mut self) -> OFFSETCNT_W {
        OFFSETCNT_W { w: self }
    }
    #[doc = "Bits 0:4 - Command for submodule."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}
