#[doc = "Reader of register FIFOPTR"]
pub type R = crate::R<u32, super::FIFOPTR>;
#[doc = "Writer for register FIFOPTR"]
pub type W = crate::W<u32, super::FIFOPTR>;
#[doc = "Register FIFOPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFO1REM`"]
pub type FIFO1REM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO1REM`"]
pub struct FIFO1REM_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO1REM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `FIFO1SIZ`"]
pub type FIFO1SIZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO1SIZ`"]
pub struct FIFO1SIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO1SIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `FIFO0REM`"]
pub type FIFO0REM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO0REM`"]
pub struct FIFO0REM_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO0REM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FIFO0SIZ`"]
pub type FIFO0SIZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO0SIZ`"]
pub struct FIFO0SIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO0SIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1rem(&self) -> FIFO1REM_R {
        FIFO1REM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1siz(&self) -> FIFO1SIZ_R {
        FIFO1SIZ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0rem(&self) -> FIFO0REM_R {
        FIFO0REM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0siz(&self) -> FIFO0SIZ_R {
        FIFO0SIZ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1rem(&mut self) -> FIFO1REM_W {
        FIFO1REM_W { w: self }
    }
    #[doc = "Bits 16:23 - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1siz(&mut self) -> FIFO1SIZ_W {
        FIFO1SIZ_W { w: self }
    }
    #[doc = "Bits 8:15 - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0rem(&mut self) -> FIFO0REM_W {
        FIFO0REM_W { w: self }
    }
    #[doc = "Bits 0:7 - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0siz(&mut self) -> FIFO0SIZ_W {
        FIFO0SIZ_W { w: self }
    }
}
