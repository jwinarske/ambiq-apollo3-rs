#[doc = "Reader of register CMDSTAT"]
pub type R = crate::R<u32, super::CMDSTAT>;
#[doc = "Writer for register CMDSTAT"]
pub type W = crate::W<u32, super::CMDSTAT>;
#[doc = "Register CMDSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTSIZE`"]
pub type CTSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTSIZE`"]
pub struct CTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | (((value as u32) & 0x0fff) << 8);
        self.w
    }
}
#[doc = "The current status of the command execution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDSTAT_A {
    #[doc = "1: Error encountered with command value."]
    ERR = 1,
    #[doc = "2: Actively processing command value."]
    ACTIVE = 2,
    #[doc = "4: Idle state, no active command, no error value."]
    IDLE = 4,
    #[doc = "6: Command in progress, but waiting on data from host value."]
    WAIT = 6,
}
impl From<CMDSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMDSTAT`"]
pub type CMDSTAT_R = crate::R<u8, CMDSTAT_A>;
impl CMDSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMDSTAT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CMDSTAT_A::ERR),
            2 => Val(CMDSTAT_A::ACTIVE),
            4 => Val(CMDSTAT_A::IDLE),
            6 => Val(CMDSTAT_A::WAIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == CMDSTAT_A::ERR
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CMDSTAT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == CMDSTAT_A::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == CMDSTAT_A::WAIT
    }
}
#[doc = "Write proxy for field `CMDSTAT`"]
pub struct CMDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDSTAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Error encountered with command value."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(CMDSTAT_A::ERR)
    }
    #[doc = "Actively processing command value."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(CMDSTAT_A::ACTIVE)
    }
    #[doc = "Idle state, no active command, no error value."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(CMDSTAT_A::IDLE)
    }
    #[doc = "Command in progress, but waiting on data from host value."]
    #[inline(always)]
    pub fn wait(self) -> &'a mut W {
        self.variant(CMDSTAT_A::WAIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `CCMD`"]
pub type CCMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCMD`"]
pub struct CCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:19 - The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 5:7 - The current status of the command execution."]
    #[inline(always)]
    pub fn cmdstat(&self) -> CMDSTAT_R {
        CMDSTAT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - current command that is being executed"]
    #[inline(always)]
    pub fn ccmd(&self) -> CCMD_R {
        CCMD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:19 - The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline(always)]
    pub fn ctsize(&mut self) -> CTSIZE_W {
        CTSIZE_W { w: self }
    }
    #[doc = "Bits 5:7 - The current status of the command execution."]
    #[inline(always)]
    pub fn cmdstat(&mut self) -> CMDSTAT_W {
        CMDSTAT_W { w: self }
    }
    #[doc = "Bits 0:4 - current command that is being executed"]
    #[inline(always)]
    pub fn ccmd(&mut self) -> CCMD_W {
        CCMD_W { w: self }
    }
}
