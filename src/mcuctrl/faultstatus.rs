#[doc = "Reader of register FAULTSTATUS"]
pub type R = crate::R<u32, super::FAULTSTATUS>;
#[doc = "Writer for register FAULTSTATUS"]
pub type W = crate::W<u32, super::FAULTSTATUS>;
#[doc = "Register FAULTSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::FAULTSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSFAULT_A {
    #[doc = "0: No bus fault has been detected. value."]
    NOFAULT = 0,
    #[doc = "1: Bus fault detected. value."]
    FAULT = 1,
}
impl From<SYSFAULT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSFAULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSFAULT`"]
pub type SYSFAULT_R = crate::R<bool, SYSFAULT_A>;
impl SYSFAULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSFAULT_A {
        match self.bits {
            false => SYSFAULT_A::NOFAULT,
            true => SYSFAULT_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == SYSFAULT_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == SYSFAULT_A::FAULT
    }
}
#[doc = "Write proxy for field `SYSFAULT`"]
pub struct SYSFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSFAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSFAULT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No bus fault has been detected. value."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(SYSFAULT_A::NOFAULT)
    }
    #[doc = "Bus fault detected. value."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(SYSFAULT_A::FAULT)
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
#[doc = "DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCODEFAULT_A {
    #[doc = "0: No DCODE fault has been detected. value."]
    NOFAULT = 0,
    #[doc = "1: DCODE fault detected. value."]
    FAULT = 1,
}
impl From<DCODEFAULT_A> for bool {
    #[inline(always)]
    fn from(variant: DCODEFAULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCODEFAULT`"]
pub type DCODEFAULT_R = crate::R<bool, DCODEFAULT_A>;
impl DCODEFAULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCODEFAULT_A {
        match self.bits {
            false => DCODEFAULT_A::NOFAULT,
            true => DCODEFAULT_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == DCODEFAULT_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == DCODEFAULT_A::FAULT
    }
}
#[doc = "Write proxy for field `DCODEFAULT`"]
pub struct DCODEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> DCODEFAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCODEFAULT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DCODE fault has been detected. value."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(DCODEFAULT_A::NOFAULT)
    }
    #[doc = "DCODE fault detected. value."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(DCODEFAULT_A::FAULT)
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
#[doc = "The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICODEFAULT_A {
    #[doc = "0: No ICODE fault has been detected. value."]
    NOFAULT = 0,
    #[doc = "1: ICODE fault detected. value."]
    FAULT = 1,
}
impl From<ICODEFAULT_A> for bool {
    #[inline(always)]
    fn from(variant: ICODEFAULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICODEFAULT`"]
pub type ICODEFAULT_R = crate::R<bool, ICODEFAULT_A>;
impl ICODEFAULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICODEFAULT_A {
        match self.bits {
            false => ICODEFAULT_A::NOFAULT,
            true => ICODEFAULT_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == ICODEFAULT_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == ICODEFAULT_A::FAULT
    }
}
#[doc = "Write proxy for field `ICODEFAULT`"]
pub struct ICODEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> ICODEFAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICODEFAULT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ICODE fault has been detected. value."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(ICODEFAULT_A::NOFAULT)
    }
    #[doc = "ICODE fault detected. value."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(ICODEFAULT_A::FAULT)
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
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn sysfault(&self) -> SYSFAULT_R {
        SYSFAULT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn dcodefault(&self) -> DCODEFAULT_R {
        DCODEFAULT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn icodefault(&self) -> ICODEFAULT_R {
        ICODEFAULT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn sysfault(&mut self) -> SYSFAULT_W {
        SYSFAULT_W { w: self }
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn dcodefault(&mut self) -> DCODEFAULT_W {
        DCODEFAULT_W { w: self }
    }
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn icodefault(&mut self) -> ICODEFAULT_W {
        ICODEFAULT_W { w: self }
    }
}
