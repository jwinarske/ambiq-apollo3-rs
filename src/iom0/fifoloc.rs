#[doc = "Reader of register FIFOLOC"]
pub type R = crate::R<u32, super::FIFOLOC>;
#[doc = "Writer for register FIFOLOC"]
pub type W = crate::W<u32, super::FIFOLOC>;
#[doc = "Register FIFOLOC `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOLOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFORPTR`"]
pub type FIFORPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFORPTR`"]
pub struct FIFORPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFORPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FIFOWPTR`"]
pub type FIFOWPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOWPTR`"]
pub struct FIFOWPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOWPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
    #[inline(always)]
    pub fn fiforptr(&self) -> FIFORPTR_R {
        FIFORPTR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
    #[inline(always)]
    pub fn fifowptr(&self) -> FIFOWPTR_R {
        FIFOWPTR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
    #[inline(always)]
    pub fn fiforptr(&mut self) -> FIFORPTR_W {
        FIFORPTR_W { w: self }
    }
    #[doc = "Bits 0:3 - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
    #[inline(always)]
    pub fn fifowptr(&mut self) -> FIFOWPTR_W {
        FIFOWPTR_W { w: self }
    }
}
