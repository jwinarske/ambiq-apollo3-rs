#[doc = "Reader of register PADOUTEN"]
pub type R = crate::R<u32, super::PADOUTEN>;
#[doc = "Writer for register PADOUTEN"]
pub type W = crate::W<u32, super::PADOUTEN>;
#[doc = "Register PADOUTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::PADOUTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data, \\[7:4\\]
are Quad1 data, and \\[8\\]
is clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum OUTEN_A {
    #[doc = "271: Quad0 (4 data + 1 clock) value."]
    QUAD0 = 271,
    #[doc = "496: Quad1 (4 data + 1 clock) value."]
    QUAD1 = 496,
    #[doc = "511: Octal (8 data + 1 clock) value."]
    OCTAL = 511,
    #[doc = "259: Serial (2 data + 1 clock) value."]
    SERIAL0 = 259,
}
impl From<OUTEN_A> for u16 {
    #[inline(always)]
    fn from(variant: OUTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTEN`"]
pub type OUTEN_R = crate::R<u16, OUTEN_A>;
impl OUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, OUTEN_A> {
        use crate::Variant::*;
        match self.bits {
            271 => Val(OUTEN_A::QUAD0),
            496 => Val(OUTEN_A::QUAD1),
            511 => Val(OUTEN_A::OCTAL),
            259 => Val(OUTEN_A::SERIAL0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `QUAD0`"]
    #[inline(always)]
    pub fn is_quad0(&self) -> bool {
        *self == OUTEN_A::QUAD0
    }
    #[doc = "Checks if the value of the field is `QUAD1`"]
    #[inline(always)]
    pub fn is_quad1(&self) -> bool {
        *self == OUTEN_A::QUAD1
    }
    #[doc = "Checks if the value of the field is `OCTAL`"]
    #[inline(always)]
    pub fn is_octal(&self) -> bool {
        *self == OUTEN_A::OCTAL
    }
    #[doc = "Checks if the value of the field is `SERIAL0`"]
    #[inline(always)]
    pub fn is_serial0(&self) -> bool {
        *self == OUTEN_A::SERIAL0
    }
}
#[doc = "Write proxy for field `OUTEN`"]
pub struct OUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Quad0 (4 data + 1 clock) value."]
    #[inline(always)]
    pub fn quad0(self) -> &'a mut W {
        self.variant(OUTEN_A::QUAD0)
    }
    #[doc = "Quad1 (4 data + 1 clock) value."]
    #[inline(always)]
    pub fn quad1(self) -> &'a mut W {
        self.variant(OUTEN_A::QUAD1)
    }
    #[doc = "Octal (8 data + 1 clock) value."]
    #[inline(always)]
    pub fn octal(self) -> &'a mut W {
        self.variant(OUTEN_A::OCTAL)
    }
    #[doc = "Serial (2 data + 1 clock) value."]
    #[inline(always)]
    pub fn serial0(self) -> &'a mut W {
        self.variant(OUTEN_A::SERIAL0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data, \\[7:4\\]
are Quad1 data, and \\[8\\]
is clock."]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data, \\[7:4\\]
are Quad1 data, and \\[8\\]
is clock."]
    #[inline(always)]
    pub fn outen(&mut self) -> OUTEN_W {
        OUTEN_W { w: self }
    }
}
