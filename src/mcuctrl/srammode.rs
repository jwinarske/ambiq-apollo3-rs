#[doc = "Reader of register SRAMMODE"]
pub type R = crate::R<u32, super::SRAMMODE>;
#[doc = "Writer for register SRAMMODE"]
pub type W = crate::W<u32, super::SRAMMODE>;
#[doc = "Register SRAMMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAMMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPREFETCH_CACHE`"]
pub type DPREFETCH_CACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPREFETCH_CACHE`"]
pub struct DPREFETCH_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPREFETCH_CACHE_W<'a> {
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
#[doc = "Reader of field `DPREFETCH`"]
pub type DPREFETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPREFETCH`"]
pub struct DPREFETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DPREFETCH_W<'a> {
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
#[doc = "Reader of field `IPREFETCH_CACHE`"]
pub type IPREFETCH_CACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPREFETCH_CACHE`"]
pub struct IPREFETCH_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREFETCH_CACHE_W<'a> {
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
#[doc = "Reader of field `IPREFETCH`"]
pub type IPREFETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPREFETCH`"]
pub struct IPREFETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREFETCH_W<'a> {
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
    #[doc = "Bit 5 - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires DPREFETCH to be set)."]
    #[inline(always)]
    pub fn dprefetch_cache(&self) -> DPREFETCH_CACHE_R {
        DPREFETCH_CACHE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When set, data bus accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Use of this mode bit is only recommended if the work flow has a large number of sequential accesses."]
    #[inline(always)]
    pub fn dprefetch(&self) -> DPREFETCH_R {
        DPREFETCH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires IPREFETCH to be set)."]
    #[inline(always)]
    pub fn iprefetch_cache(&self) -> IPREFETCH_CACHE_R {
        IPREFETCH_CACHE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - When set, instruction accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Generally, this mode bit should be set for improved performance when executing instructions from SRAM."]
    #[inline(always)]
    pub fn iprefetch(&self) -> IPREFETCH_R {
        IPREFETCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires DPREFETCH to be set)."]
    #[inline(always)]
    pub fn dprefetch_cache(&mut self) -> DPREFETCH_CACHE_W {
        DPREFETCH_CACHE_W { w: self }
    }
    #[doc = "Bit 4 - When set, data bus accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Use of this mode bit is only recommended if the work flow has a large number of sequential accesses."]
    #[inline(always)]
    pub fn dprefetch(&mut self) -> DPREFETCH_W {
        DPREFETCH_W { w: self }
    }
    #[doc = "Bit 1 - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires IPREFETCH to be set)."]
    #[inline(always)]
    pub fn iprefetch_cache(&mut self) -> IPREFETCH_CACHE_W {
        IPREFETCH_CACHE_W { w: self }
    }
    #[doc = "Bit 0 - When set, instruction accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Generally, this mode bit should be set for improved performance when executing instructions from SRAM."]
    #[inline(always)]
    pub fn iprefetch(&mut self) -> IPREFETCH_W {
        IPREFETCH_W { w: self }
    }
}
