#[doc = "Reader of register FIFOTHR"]
pub type R = crate::R<u32, super::FIFOTHR>;
#[doc = "Writer for register FIFOTHR"]
pub type W = crate::W<u32, super::FIFOTHR>;
#[doc = "Register FIFOTHR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOTHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOWTHR`"]
pub type FIFOWTHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOWTHR`"]
pub struct FIFOWTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOWTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FIFORTHR`"]
pub type FIFORTHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFORTHR`"]
pub struct FIFORTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFORTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline(always)]
    pub fn fifowthr(&self) -> FIFOWTHR_R {
        FIFOWTHR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline(always)]
    pub fn fiforthr(&self) -> FIFORTHR_R {
        FIFORTHR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline(always)]
    pub fn fifowthr(&mut self) -> FIFOWTHR_W {
        FIFOWTHR_W { w: self }
    }
    #[doc = "Bits 0:5 - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline(always)]
    pub fn fiforthr(&mut self) -> FIFORTHR_W {
        FIFORTHR_W { w: self }
    }
}
