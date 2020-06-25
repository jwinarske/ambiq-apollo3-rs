#[doc = "Reader of register CQFLAGS"]
pub type R = crate::R<u32, super::CQFLAGS>;
#[doc = "Writer for register CQFLAGS"]
pub type W = crate::W<u32, super::CQFLAGS>;
#[doc = "Register CQFLAGS `reset()`'s with value 0"]
impl crate::ResetValue for super::CQFLAGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CQFLAGS_A {
    #[doc = "32768: CQ Stop Flag.  When set, CQ processing will complete. value."]
    STOP = 32768,
    #[doc = "16384: CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    CQIDX = 16384,
    #[doc = "2048: DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    DMACPL = 2048,
    #[doc = "1024: PIO Operation completed (STATUS bit in CTRL register) value."]
    CMDCPL = 1024,
    #[doc = "512: IOM Buffer 1 Ready Status (from selected IOM).  This status is the result of XOR'ing the IOM0START with the incoming status from the IOM.  When high, MSPI can send to the buffer. value."]
    IOM1READY = 512,
    #[doc = "256: IOM Buffer 0 Ready Status (from selected IOM).  This status is the result of XOR'ing the IOM0START with the incoming status from the IOM.  When high, MSPI can send to the buffer. value."]
    IOM0READY = 256,
    #[doc = "128: Software flag 7.  Can be used by software to start/pause operations value."]
    SWFLAG7 = 128,
    #[doc = "64: Software flag 6. Can be used by software to start/pause operatoins value."]
    SWFLAG6 = 64,
    #[doc = "32: Software flag 5.  Can be used by software to start/pause operations value."]
    SWFLAG5 = 32,
    #[doc = "16: Software flag 4. Can be used by software to start/pause operatoins value."]
    SWFLAG4 = 16,
    #[doc = "8: Software flag 3.  Can be used by software to start/pause operations value."]
    SWFLAG3 = 8,
    #[doc = "4: Software flag 2. Can be used by software to start/pause operatoins value."]
    SWFLAG2 = 4,
    #[doc = "2: Software flag 1.  Can be used by software to start/pause operations value."]
    SWFLAG1 = 2,
    #[doc = "1: Software flag 0. Can be used by software to start/pause operatoins value."]
    SWFLAG0 = 1,
}
impl From<CQFLAGS_A> for u16 {
    #[inline(always)]
    fn from(variant: CQFLAGS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CQFLAGS`"]
pub type CQFLAGS_R = crate::R<u16, CQFLAGS_A>;
impl CQFLAGS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CQFLAGS_A> {
        use crate::Variant::*;
        match self.bits {
            32768 => Val(CQFLAGS_A::STOP),
            16384 => Val(CQFLAGS_A::CQIDX),
            2048 => Val(CQFLAGS_A::DMACPL),
            1024 => Val(CQFLAGS_A::CMDCPL),
            512 => Val(CQFLAGS_A::IOM1READY),
            256 => Val(CQFLAGS_A::IOM0READY),
            128 => Val(CQFLAGS_A::SWFLAG7),
            64 => Val(CQFLAGS_A::SWFLAG6),
            32 => Val(CQFLAGS_A::SWFLAG5),
            16 => Val(CQFLAGS_A::SWFLAG4),
            8 => Val(CQFLAGS_A::SWFLAG3),
            4 => Val(CQFLAGS_A::SWFLAG2),
            2 => Val(CQFLAGS_A::SWFLAG1),
            1 => Val(CQFLAGS_A::SWFLAG0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CQFLAGS_A::STOP
    }
    #[doc = "Checks if the value of the field is `CQIDX`"]
    #[inline(always)]
    pub fn is_cqidx(&self) -> bool {
        *self == CQFLAGS_A::CQIDX
    }
    #[doc = "Checks if the value of the field is `DMACPL`"]
    #[inline(always)]
    pub fn is_dmacpl(&self) -> bool {
        *self == CQFLAGS_A::DMACPL
    }
    #[doc = "Checks if the value of the field is `CMDCPL`"]
    #[inline(always)]
    pub fn is_cmdcpl(&self) -> bool {
        *self == CQFLAGS_A::CMDCPL
    }
    #[doc = "Checks if the value of the field is `IOM1READY`"]
    #[inline(always)]
    pub fn is_iom1ready(&self) -> bool {
        *self == CQFLAGS_A::IOM1READY
    }
    #[doc = "Checks if the value of the field is `IOM0READY`"]
    #[inline(always)]
    pub fn is_iom0ready(&self) -> bool {
        *self == CQFLAGS_A::IOM0READY
    }
    #[doc = "Checks if the value of the field is `SWFLAG7`"]
    #[inline(always)]
    pub fn is_swflag7(&self) -> bool {
        *self == CQFLAGS_A::SWFLAG7
    }
    #[doc = "Checks if the value of the field is `SWFLAG6`"]
    #[inline(always)]
    pub fn is_swflag6(&self) -> bool {
        *self == CQFLAGS_A::SWFLAG6
    }
    #[doc = "Checks if the value of the field is `SWFLAG5`"]
    #[inline(always)]
    pub fn is_swflag5(&self) -> bool {
        *self == CQFLAGS_A::SWFLAG5
    }
    #[doc = "Checks if the value of the field is `SWFLAG4`"]
    #[inline(always)]
    pub fn is_swflag4(&self) -> bool {
        *self == CQFLAGS_A::SWFLAG4
    }
    #[doc = "Checks if the value of the field is `SWFLAG3`"]
    #[inline(always)]
    pub fn is_swflag3(&self) -> bool {
        *self == CQFLAGS_A::SWFLAG3
    }
    #[doc = "Checks if the value of the field is `SWFLAG2`"]
    #[inline(always)]
    pub fn is_swflag2(&self) -> bool {
        *self == CQFLAGS_A::SWFLAG2
    }
    #[doc = "Checks if the value of the field is `SWFLAG1`"]
    #[inline(always)]
    pub fn is_swflag1(&self) -> bool {
        *self == CQFLAGS_A::SWFLAG1
    }
    #[doc = "Checks if the value of the field is `SWFLAG0`"]
    #[inline(always)]
    pub fn is_swflag0(&self) -> bool {
        *self == CQFLAGS_A::SWFLAG0
    }
}
#[doc = "Write proxy for field `CQFLAGS`"]
pub struct CQFLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFLAGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQFLAGS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CQ Stop Flag. When set, CQ processing will complete. value."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CQFLAGS_A::STOP)
    }
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    #[inline(always)]
    pub fn cqidx(self) -> &'a mut W {
        self.variant(CQFLAGS_A::CQIDX)
    }
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    #[inline(always)]
    pub fn dmacpl(self) -> &'a mut W {
        self.variant(CQFLAGS_A::DMACPL)
    }
    #[doc = "PIO Operation completed (STATUS bit in CTRL register) value."]
    #[inline(always)]
    pub fn cmdcpl(self) -> &'a mut W {
        self.variant(CQFLAGS_A::CMDCPL)
    }
    #[doc = "IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    #[inline(always)]
    pub fn iom1ready(self) -> &'a mut W {
        self.variant(CQFLAGS_A::IOM1READY)
    }
    #[doc = "IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    #[inline(always)]
    pub fn iom0ready(self) -> &'a mut W {
        self.variant(CQFLAGS_A::IOM0READY)
    }
    #[doc = "Software flag 7. Can be used by software to start/pause operations value."]
    #[inline(always)]
    pub fn swflag7(self) -> &'a mut W {
        self.variant(CQFLAGS_A::SWFLAG7)
    }
    #[doc = "Software flag 6. Can be used by software to start/pause operatoins value."]
    #[inline(always)]
    pub fn swflag6(self) -> &'a mut W {
        self.variant(CQFLAGS_A::SWFLAG6)
    }
    #[doc = "Software flag 5. Can be used by software to start/pause operations value."]
    #[inline(always)]
    pub fn swflag5(self) -> &'a mut W {
        self.variant(CQFLAGS_A::SWFLAG5)
    }
    #[doc = "Software flag 4. Can be used by software to start/pause operatoins value."]
    #[inline(always)]
    pub fn swflag4(self) -> &'a mut W {
        self.variant(CQFLAGS_A::SWFLAG4)
    }
    #[doc = "Software flag 3. Can be used by software to start/pause operations value."]
    #[inline(always)]
    pub fn swflag3(self) -> &'a mut W {
        self.variant(CQFLAGS_A::SWFLAG3)
    }
    #[doc = "Software flag 2. Can be used by software to start/pause operatoins value."]
    #[inline(always)]
    pub fn swflag2(self) -> &'a mut W {
        self.variant(CQFLAGS_A::SWFLAG2)
    }
    #[doc = "Software flag 1. Can be used by software to start/pause operations value."]
    #[inline(always)]
    pub fn swflag1(self) -> &'a mut W {
        self.variant(CQFLAGS_A::SWFLAG1)
    }
    #[doc = "Software flag 0. Can be used by software to start/pause operatoins value."]
    #[inline(always)]
    pub fn swflag0(self) -> &'a mut W {
        self.variant(CQFLAGS_A::SWFLAG0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    pub fn cqflags(&self) -> CQFLAGS_R {
        CQFLAGS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    pub fn cqflags(&mut self) -> CQFLAGS_W {
        CQFLAGS_W { w: self }
    }
}
