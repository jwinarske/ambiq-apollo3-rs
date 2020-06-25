#[doc = "Reader of register CQPAUSEEN"]
pub type R = crate::R<u32, super::CQPAUSEEN>;
#[doc = "Writer for register CQPAUSEEN"]
pub type W = crate::W<u32, super::CQPAUSEEN>;
#[doc = "Register CQPAUSEEN `reset()`'s with value 0"]
impl crate::ResetValue for super::CQPAUSEEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enables the specified event to pause command processing when active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CQPEN_A {
    #[doc = "32768: Pauses command queue processing when HWCNT matches SWCNT value."]
    CNTEQ = 32768,
    #[doc = "16384: Pause command queue when input BLE bit XORed with SWFLAG4 is '1' value."]
    BLEXOREN = 16384,
    #[doc = "8192: Pause command queue when input IOM bit XORed with SWFLAG3 is '1' value."]
    IOMXOREN = 8192,
    #[doc = "4096: Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1' value."]
    GPIOXOREN = 4096,
    #[doc = "2048: Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1' value."]
    MSPI1XNOREN = 2048,
    #[doc = "1024: Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1' value."]
    MSPI0XNOREN = 1024,
    #[doc = "512: Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1' value."]
    MSPI1XOREN = 512,
    #[doc = "256: Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1' value."]
    MSPI0XOREN = 256,
    #[doc = "128: Pause the command queue when software flag bit 7 is '1'. value."]
    SWFLAGEN7 = 128,
    #[doc = "64: Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN6 = 64,
    #[doc = "32: Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN5 = 32,
    #[doc = "16: Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN4 = 16,
    #[doc = "8: Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN3 = 8,
    #[doc = "4: Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN2 = 4,
    #[doc = "2: Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN1 = 2,
    #[doc = "1: Pause the command queue when software flag bit 7 is '1' value."]
    SWFLGEN0 = 1,
}
impl From<CQPEN_A> for u16 {
    #[inline(always)]
    fn from(variant: CQPEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CQPEN`"]
pub type CQPEN_R = crate::R<u16, CQPEN_A>;
impl CQPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CQPEN_A> {
        use crate::Variant::*;
        match self.bits {
            32768 => Val(CQPEN_A::CNTEQ),
            16384 => Val(CQPEN_A::BLEXOREN),
            8192 => Val(CQPEN_A::IOMXOREN),
            4096 => Val(CQPEN_A::GPIOXOREN),
            2048 => Val(CQPEN_A::MSPI1XNOREN),
            1024 => Val(CQPEN_A::MSPI0XNOREN),
            512 => Val(CQPEN_A::MSPI1XOREN),
            256 => Val(CQPEN_A::MSPI0XOREN),
            128 => Val(CQPEN_A::SWFLAGEN7),
            64 => Val(CQPEN_A::SWFLAGEN6),
            32 => Val(CQPEN_A::SWFLAGEN5),
            16 => Val(CQPEN_A::SWFLAGEN4),
            8 => Val(CQPEN_A::SWFLAGEN3),
            4 => Val(CQPEN_A::SWFLAGEN2),
            2 => Val(CQPEN_A::SWFLAGEN1),
            1 => Val(CQPEN_A::SWFLGEN0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CNTEQ`"]
    #[inline(always)]
    pub fn is_cnteq(&self) -> bool {
        *self == CQPEN_A::CNTEQ
    }
    #[doc = "Checks if the value of the field is `BLEXOREN`"]
    #[inline(always)]
    pub fn is_blexoren(&self) -> bool {
        *self == CQPEN_A::BLEXOREN
    }
    #[doc = "Checks if the value of the field is `IOMXOREN`"]
    #[inline(always)]
    pub fn is_iomxoren(&self) -> bool {
        *self == CQPEN_A::IOMXOREN
    }
    #[doc = "Checks if the value of the field is `GPIOXOREN`"]
    #[inline(always)]
    pub fn is_gpioxoren(&self) -> bool {
        *self == CQPEN_A::GPIOXOREN
    }
    #[doc = "Checks if the value of the field is `MSPI1XNOREN`"]
    #[inline(always)]
    pub fn is_mspi1xnoren(&self) -> bool {
        *self == CQPEN_A::MSPI1XNOREN
    }
    #[doc = "Checks if the value of the field is `MSPI0XNOREN`"]
    #[inline(always)]
    pub fn is_mspi0xnoren(&self) -> bool {
        *self == CQPEN_A::MSPI0XNOREN
    }
    #[doc = "Checks if the value of the field is `MSPI1XOREN`"]
    #[inline(always)]
    pub fn is_mspi1xoren(&self) -> bool {
        *self == CQPEN_A::MSPI1XOREN
    }
    #[doc = "Checks if the value of the field is `MSPI0XOREN`"]
    #[inline(always)]
    pub fn is_mspi0xoren(&self) -> bool {
        *self == CQPEN_A::MSPI0XOREN
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN7`"]
    #[inline(always)]
    pub fn is_swflagen7(&self) -> bool {
        *self == CQPEN_A::SWFLAGEN7
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN6`"]
    #[inline(always)]
    pub fn is_swflagen6(&self) -> bool {
        *self == CQPEN_A::SWFLAGEN6
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN5`"]
    #[inline(always)]
    pub fn is_swflagen5(&self) -> bool {
        *self == CQPEN_A::SWFLAGEN5
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN4`"]
    #[inline(always)]
    pub fn is_swflagen4(&self) -> bool {
        *self == CQPEN_A::SWFLAGEN4
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN3`"]
    #[inline(always)]
    pub fn is_swflagen3(&self) -> bool {
        *self == CQPEN_A::SWFLAGEN3
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN2`"]
    #[inline(always)]
    pub fn is_swflagen2(&self) -> bool {
        *self == CQPEN_A::SWFLAGEN2
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN1`"]
    #[inline(always)]
    pub fn is_swflagen1(&self) -> bool {
        *self == CQPEN_A::SWFLAGEN1
    }
    #[doc = "Checks if the value of the field is `SWFLGEN0`"]
    #[inline(always)]
    pub fn is_swflgen0(&self) -> bool {
        *self == CQPEN_A::SWFLGEN0
    }
}
#[doc = "Write proxy for field `CQPEN`"]
pub struct CQPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQPEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pauses command queue processing when HWCNT matches SWCNT value."]
    #[inline(always)]
    pub fn cnteq(self) -> &'a mut W {
        self.variant(CQPEN_A::CNTEQ)
    }
    #[doc = "Pause command queue when input BLE bit XORed with SWFLAG4 is '1' value."]
    #[inline(always)]
    pub fn blexoren(self) -> &'a mut W {
        self.variant(CQPEN_A::BLEXOREN)
    }
    #[doc = "Pause command queue when input IOM bit XORed with SWFLAG3 is '1' value."]
    #[inline(always)]
    pub fn iomxoren(self) -> &'a mut W {
        self.variant(CQPEN_A::IOMXOREN)
    }
    #[doc = "Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1' value."]
    #[inline(always)]
    pub fn gpioxoren(self) -> &'a mut W {
        self.variant(CQPEN_A::GPIOXOREN)
    }
    #[doc = "Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1' value."]
    #[inline(always)]
    pub fn mspi1xnoren(self) -> &'a mut W {
        self.variant(CQPEN_A::MSPI1XNOREN)
    }
    #[doc = "Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1' value."]
    #[inline(always)]
    pub fn mspi0xnoren(self) -> &'a mut W {
        self.variant(CQPEN_A::MSPI0XNOREN)
    }
    #[doc = "Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1' value."]
    #[inline(always)]
    pub fn mspi1xoren(self) -> &'a mut W {
        self.variant(CQPEN_A::MSPI1XOREN)
    }
    #[doc = "Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1' value."]
    #[inline(always)]
    pub fn mspi0xoren(self) -> &'a mut W {
        self.variant(CQPEN_A::MSPI0XOREN)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1'. value."]
    #[inline(always)]
    pub fn swflagen7(self) -> &'a mut W {
        self.variant(CQPEN_A::SWFLAGEN7)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline(always)]
    pub fn swflagen6(self) -> &'a mut W {
        self.variant(CQPEN_A::SWFLAGEN6)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline(always)]
    pub fn swflagen5(self) -> &'a mut W {
        self.variant(CQPEN_A::SWFLAGEN5)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline(always)]
    pub fn swflagen4(self) -> &'a mut W {
        self.variant(CQPEN_A::SWFLAGEN4)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline(always)]
    pub fn swflagen3(self) -> &'a mut W {
        self.variant(CQPEN_A::SWFLAGEN3)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline(always)]
    pub fn swflagen2(self) -> &'a mut W {
        self.variant(CQPEN_A::SWFLAGEN2)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline(always)]
    pub fn swflagen1(self) -> &'a mut W {
        self.variant(CQPEN_A::SWFLAGEN1)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline(always)]
    pub fn swflgen0(self) -> &'a mut W {
        self.variant(CQPEN_A::SWFLGEN0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Enables the specified event to pause command processing when active"]
    #[inline(always)]
    pub fn cqpen(&self) -> CQPEN_R {
        CQPEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Enables the specified event to pause command processing when active"]
    #[inline(always)]
    pub fn cqpen(&mut self) -> CQPEN_W {
        CQPEN_W { w: self }
    }
}
