#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEST_A {
    #[doc = "1: The I/O state machine is in the idle state. value."]
    IDLE = 1,
}
impl From<IDLEST_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLEST`"]
pub type IDLEST_R = crate::R<bool, IDLEST_A>;
impl IDLEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, IDLEST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(IDLEST_A::IDLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLEST_A::IDLE
    }
}
#[doc = "Write proxy for field `IDLEST`"]
pub struct IDLEST_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The I/O state machine is in the idle state. value."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(IDLEST_A::IDLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDACT_A {
    #[doc = "1: An I/O command is active.  Indicates the active module has an active command and is processing this.   De-asserted when the command is completed. value."]
    ACTIVE = 1,
}
impl From<CMDACT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDACT`"]
pub type CMDACT_R = crate::R<bool, CMDACT_A>;
impl CMDACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CMDACT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CMDACT_A::ACTIVE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CMDACT_A::ACTIVE
    }
}
#[doc = "Write proxy for field `CMDACT`"]
pub struct CMDACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An I/O command is active. Indicates the active module has an active command and is processing this. De-asserted when the command is completed. value."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(CMDACT_A::ACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Bit has been deprecated. Please refer to the other error indicators. This will always return 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "1: Bit has been deprecated and will always return 0. value."]
    ERROR = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, ERR_A>;
impl ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ERR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ERR_A::ERROR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERR_A::ERROR
    }
}
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit has been deprecated and will always return 0. value."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(ERR_A::ERROR)
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
    #[doc = "Bit 2 - indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
    #[inline(always)]
    pub fn idlest(&self) -> IDLEST_R {
        IDLEST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
    #[inline(always)]
    pub fn idlest(&mut self) -> IDLEST_W {
        IDLEST_W { w: self }
    }
    #[doc = "Bit 1 - Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
    #[inline(always)]
    pub fn cmdact(&mut self) -> CMDACT_W {
        CMDACT_W { w: self }
    }
    #[doc = "Bit 0 - Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
}
