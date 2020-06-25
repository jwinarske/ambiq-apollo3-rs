#[doc = "Reader of register STMINTSET"]
pub type R = crate::R<u32, super::STMINTSET>;
#[doc = "Writer for register STMINTSET"]
pub type W = crate::W<u32, super::STMINTSET>;
#[doc = "Register STMINTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::STMINTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CAPTURE register D has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURED_A {
    #[doc = "1: Capture D interrupt status bit was set. value."]
    CAPD_INT = 1,
}
impl From<CAPTURED_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTURED`"]
pub type CAPTURED_R = crate::R<bool, CAPTURED_A>;
impl CAPTURED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CAPTURED_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CAPTURED_A::CAPD_INT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPD_INT`"]
    #[inline(always)]
    pub fn is_capd_int(&self) -> bool {
        *self == CAPTURED_A::CAPD_INT
    }
}
#[doc = "Write proxy for field `CAPTURED`"]
pub struct CAPTURED_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture D interrupt status bit was set. value."]
    #[inline(always)]
    pub fn capd_int(self) -> &'a mut W {
        self.variant(CAPTURED_A::CAPD_INT)
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
#[doc = "CAPTURE register C has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREC_A {
    #[doc = "1: CAPTURE C interrupt status bit was set. value."]
    CAPC_INT = 1,
}
impl From<CAPTUREC_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTUREC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTUREC`"]
pub type CAPTUREC_R = crate::R<bool, CAPTUREC_A>;
impl CAPTUREC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CAPTUREC_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CAPTUREC_A::CAPC_INT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPC_INT`"]
    #[inline(always)]
    pub fn is_capc_int(&self) -> bool {
        *self == CAPTUREC_A::CAPC_INT
    }
}
#[doc = "Write proxy for field `CAPTUREC`"]
pub struct CAPTUREC_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTUREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTUREC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAPTURE C interrupt status bit was set. value."]
    #[inline(always)]
    pub fn capc_int(self) -> &'a mut W {
        self.variant(CAPTUREC_A::CAPC_INT)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "CAPTURE register B has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREB_A {
    #[doc = "1: CAPTURE B interrupt status bit was set. value."]
    CAPB_INT = 1,
}
impl From<CAPTUREB_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTUREB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTUREB`"]
pub type CAPTUREB_R = crate::R<bool, CAPTUREB_A>;
impl CAPTUREB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CAPTUREB_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CAPTUREB_A::CAPB_INT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPB_INT`"]
    #[inline(always)]
    pub fn is_capb_int(&self) -> bool {
        *self == CAPTUREB_A::CAPB_INT
    }
}
#[doc = "Write proxy for field `CAPTUREB`"]
pub struct CAPTUREB_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTUREB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTUREB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAPTURE B interrupt status bit was set. value."]
    #[inline(always)]
    pub fn capb_int(self) -> &'a mut W {
        self.variant(CAPTUREB_A::CAPB_INT)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "CAPTURE register A has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREA_A {
    #[doc = "1: CAPTURE A interrupt status bit was set. value."]
    CAPA_INT = 1,
}
impl From<CAPTUREA_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTUREA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTUREA`"]
pub type CAPTUREA_R = crate::R<bool, CAPTUREA_A>;
impl CAPTUREA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CAPTUREA_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CAPTUREA_A::CAPA_INT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPA_INT`"]
    #[inline(always)]
    pub fn is_capa_int(&self) -> bool {
        *self == CAPTUREA_A::CAPA_INT
    }
}
#[doc = "Write proxy for field `CAPTUREA`"]
pub struct CAPTUREA_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTUREA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTUREA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAPTURE A interrupt status bit was set. value."]
    #[inline(always)]
    pub fn capa_int(self) -> &'a mut W {
        self.variant(CAPTUREA_A::CAPA_INT)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "1: Overflow interrupt status bit was set. value."]
    OFLOW_INT = 1,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVERFLOW`"]
pub type OVERFLOW_R = crate::R<bool, OVERFLOW_A>;
impl OVERFLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, OVERFLOW_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(OVERFLOW_A::OFLOW_INT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFLOW_INT`"]
    #[inline(always)]
    pub fn is_oflow_int(&self) -> bool {
        *self == OVERFLOW_A::OFLOW_INT
    }
}
#[doc = "Write proxy for field `OVERFLOW`"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Overflow interrupt status bit was set. value."]
    #[inline(always)]
    pub fn oflow_int(self) -> &'a mut W {
        self.variant(OVERFLOW_A::OFLOW_INT)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register H.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREH_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREH_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPAREH`"]
pub type COMPAREH_R = crate::R<bool, COMPAREH_A>;
impl COMPAREH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPAREH_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPAREH_A::COMPARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREH_A::COMPARED
    }
}
#[doc = "Write proxy for field `COMPAREH`"]
pub struct COMPAREH_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREH_A::COMPARED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register G.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREG_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREG_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPAREG`"]
pub type COMPAREG_R = crate::R<bool, COMPAREG_A>;
impl COMPAREG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPAREG_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPAREG_A::COMPARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREG_A::COMPARED
    }
}
#[doc = "Write proxy for field `COMPAREG`"]
pub struct COMPAREG_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREG_A::COMPARED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREF_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREF_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPAREF`"]
pub type COMPAREF_R = crate::R<bool, COMPAREF_A>;
impl COMPAREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPAREF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPAREF_A::COMPARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREF_A::COMPARED
    }
}
#[doc = "Write proxy for field `COMPAREF`"]
pub struct COMPAREF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREF_A::COMPARED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register E.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREE_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREE_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPAREE`"]
pub type COMPAREE_R = crate::R<bool, COMPAREE_A>;
impl COMPAREE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPAREE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPAREE_A::COMPARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREE_A::COMPARED
    }
}
#[doc = "Write proxy for field `COMPAREE`"]
pub struct COMPAREE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREE_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register D.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARED_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPARED_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARED`"]
pub type COMPARED_R = crate::R<bool, COMPARED_A>;
impl COMPARED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPARED_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPARED_A::COMPARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == COMPARED_A::COMPARED
    }
}
#[doc = "Write proxy for field `COMPARED`"]
pub struct COMPARED_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPARED_A::COMPARED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register C.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREC_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREC_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPAREC`"]
pub type COMPAREC_R = crate::R<bool, COMPAREC_A>;
impl COMPAREC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPAREC_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPAREC_A::COMPARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREC_A::COMPARED
    }
}
#[doc = "Write proxy for field `COMPAREC`"]
pub struct COMPAREC_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREC_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register B.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREB_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREB_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPAREB`"]
pub type COMPAREB_R = crate::R<bool, COMPAREB_A>;
impl COMPAREB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPAREB_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPAREB_A::COMPARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREB_A::COMPARED
    }
}
#[doc = "Write proxy for field `COMPAREB`"]
pub struct COMPAREB_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREB_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREA_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREA_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPAREA`"]
pub type COMPAREA_R = crate::R<bool, COMPAREA_A>;
impl COMPAREA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPAREA_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPAREA_A::COMPARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREA_A::COMPARED
    }
}
#[doc = "Write proxy for field `COMPAREA`"]
pub struct COMPAREA_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREA_A::COMPARED)
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
    #[doc = "Bit 12 - CAPTURE register D has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captured(&self) -> CAPTURED_R {
        CAPTURED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CAPTURE register C has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturec(&self) -> CAPTUREC_R {
        CAPTUREC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CAPTURE register B has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captureb(&self) -> CAPTUREB_R {
        CAPTUREB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CAPTURE register A has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturea(&self) -> CAPTUREA_R {
        CAPTUREA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub fn compareh(&self) -> COMPAREH_R {
        COMPAREH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub fn compareg(&self) -> COMPAREG_R {
        COMPAREG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub fn comparef(&self) -> COMPAREF_R {
        COMPAREF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub fn comparee(&self) -> COMPAREE_R {
        COMPAREE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub fn compared(&self) -> COMPARED_R {
        COMPARED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub fn comparec(&self) -> COMPAREC_R {
        COMPAREC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub fn compareb(&self) -> COMPAREB_R {
        COMPAREB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub fn comparea(&self) -> COMPAREA_R {
        COMPAREA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - CAPTURE register D has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captured(&mut self) -> CAPTURED_W {
        CAPTURED_W { w: self }
    }
    #[doc = "Bit 11 - CAPTURE register C has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturec(&mut self) -> CAPTUREC_W {
        CAPTUREC_W { w: self }
    }
    #[doc = "Bit 10 - CAPTURE register B has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captureb(&mut self) -> CAPTUREB_W {
        CAPTUREB_W { w: self }
    }
    #[doc = "Bit 9 - CAPTURE register A has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturea(&mut self) -> CAPTUREA_W {
        CAPTUREA_W { w: self }
    }
    #[doc = "Bit 8 - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 7 - COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub fn compareh(&mut self) -> COMPAREH_W {
        COMPAREH_W { w: self }
    }
    #[doc = "Bit 6 - COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub fn compareg(&mut self) -> COMPAREG_W {
        COMPAREG_W { w: self }
    }
    #[doc = "Bit 5 - COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub fn comparef(&mut self) -> COMPAREF_W {
        COMPAREF_W { w: self }
    }
    #[doc = "Bit 4 - COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub fn comparee(&mut self) -> COMPAREE_W {
        COMPAREE_W { w: self }
    }
    #[doc = "Bit 3 - COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub fn compared(&mut self) -> COMPARED_W {
        COMPARED_W { w: self }
    }
    #[doc = "Bit 2 - COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub fn comparec(&mut self) -> COMPAREC_W {
        COMPAREC_W { w: self }
    }
    #[doc = "Bit 1 - COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub fn compareb(&mut self) -> COMPAREB_W {
        COMPAREB_W { w: self }
    }
    #[doc = "Bit 0 - COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub fn comparea(&mut self) -> COMPAREA_W {
        COMPAREA_W { w: self }
    }
}
