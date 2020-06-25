#[doc = "Reader of register CACHECFG"]
pub type R = crate::R<u32, super::CACHECFG>;
#[doc = "Writer for register CACHECFG"]
pub type W = crate::W<u32, super::CACHECFG>;
#[doc = "Register CACHECFG `reset()`'s with value 0x0010_0c50"]
impl crate::ResetValue for super::CACHECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0c50
    }
}
#[doc = "Reader of field `ENABLE_MONITOR`"]
pub type ENABLE_MONITOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_MONITOR`"]
pub struct ENABLE_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_MONITOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DATA_CLKGATE`"]
pub type DATA_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_CLKGATE`"]
pub struct DATA_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CACHE_LS`"]
pub type CACHE_LS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_LS`"]
pub struct CACHE_LS_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_LS_W<'a> {
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
#[doc = "Reader of field `CACHE_CLKGATE`"]
pub type CACHE_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_CLKGATE`"]
pub struct CACHE_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_CLKGATE_W<'a> {
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
#[doc = "Reader of field `DCACHE_ENABLE`"]
pub type DCACHE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCACHE_ENABLE`"]
pub struct DCACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_ENABLE_W<'a> {
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
#[doc = "Reader of field `ICACHE_ENABLE`"]
pub type ICACHE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHE_ENABLE`"]
pub struct ICACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_ENABLE_W<'a> {
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
#[doc = "Sets the cache configuration\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONFIG_A {
    #[doc = "4: Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active) value."]
    W1_128B_512E = 4,
    #[doc = "5: Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active) value."]
    W2_128B_512E = 5,
    #[doc = "8: Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active) value."]
    W1_128B_1024E = 8,
}
impl From<CONFIG_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONFIG`"]
pub type CONFIG_R = crate::R<u8, CONFIG_A>;
impl CONFIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONFIG_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(CONFIG_A::W1_128B_512E),
            5 => Val(CONFIG_A::W2_128B_512E),
            8 => Val(CONFIG_A::W1_128B_1024E),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `W1_128B_512E`"]
    #[inline(always)]
    pub fn is_w1_128b_512e(&self) -> bool {
        *self == CONFIG_A::W1_128B_512E
    }
    #[doc = "Checks if the value of the field is `W2_128B_512E`"]
    #[inline(always)]
    pub fn is_w2_128b_512e(&self) -> bool {
        *self == CONFIG_A::W2_128B_512E
    }
    #[doc = "Checks if the value of the field is `W1_128B_1024E`"]
    #[inline(always)]
    pub fn is_w1_128b_1024e(&self) -> bool {
        *self == CONFIG_A::W1_128B_1024E
    }
}
#[doc = "Write proxy for field `CONFIG`"]
pub struct CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONFIG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active) value."]
    #[inline(always)]
    pub fn w1_128b_512e(self) -> &'a mut W {
        self.variant(CONFIG_A::W1_128B_512E)
    }
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active) value."]
    #[inline(always)]
    pub fn w2_128b_512e(self) -> &'a mut W {
        self.variant(CONFIG_A::W2_128B_512E)
    }
    #[doc = "Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active) value."]
    #[inline(always)]
    pub fn w1_128b_1024e(self) -> &'a mut W {
        self.variant(CONFIG_A::W1_128B_1024E)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_NC1`"]
pub type ENABLE_NC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_NC1`"]
pub struct ENABLE_NC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_NC1_W<'a> {
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
#[doc = "Reader of field `ENABLE_NC0`"]
pub type ENABLE_NC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_NC0`"]
pub struct ENABLE_NC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_NC0_W<'a> {
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
#[doc = "Reader of field `LRU`"]
pub type LRU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LRU`"]
pub struct LRU_W<'a> {
    w: &'a mut W,
}
impl<'a> LRU_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline(always)]
    pub fn enable_monitor(&self) -> ENABLE_MONITOR_R {
        ENABLE_MONITOR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline(always)]
    pub fn data_clkgate(&self) -> DATA_CLKGATE_R {
        DATA_CLKGATE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline(always)]
    pub fn cache_ls(&self) -> CACHE_LS_R {
        CACHE_LS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline(always)]
    pub fn cache_clkgate(&self) -> CACHE_CLKGATE_R {
        CACHE_CLKGATE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Flash Data Caching."]
    #[inline(always)]
    pub fn dcache_enable(&self) -> DCACHE_ENABLE_R {
        DCACHE_ENABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Flash Instruction Caching"]
    #[inline(always)]
    pub fn icache_enable(&self) -> ICACHE_ENABLE_R {
        ICACHE_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Sets the cache configuration"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline(always)]
    pub fn enable_nc1(&self) -> ENABLE_NC1_R {
        ENABLE_NC1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline(always)]
    pub fn enable_nc0(&self) -> ENABLE_NC0_R {
        ENABLE_NC0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline(always)]
    pub fn lru(&self) -> LRU_R {
        LRU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline(always)]
    pub fn enable_monitor(&mut self) -> ENABLE_MONITOR_W {
        ENABLE_MONITOR_W { w: self }
    }
    #[doc = "Bit 20 - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline(always)]
    pub fn data_clkgate(&mut self) -> DATA_CLKGATE_W {
        DATA_CLKGATE_W { w: self }
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline(always)]
    pub fn cache_ls(&mut self) -> CACHE_LS_W {
        CACHE_LS_W { w: self }
    }
    #[doc = "Bit 10 - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline(always)]
    pub fn cache_clkgate(&mut self) -> CACHE_CLKGATE_W {
        CACHE_CLKGATE_W { w: self }
    }
    #[doc = "Bit 9 - Enable Flash Data Caching."]
    #[inline(always)]
    pub fn dcache_enable(&mut self) -> DCACHE_ENABLE_W {
        DCACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Enable Flash Instruction Caching"]
    #[inline(always)]
    pub fn icache_enable(&mut self) -> ICACHE_ENABLE_W {
        ICACHE_ENABLE_W { w: self }
    }
    #[doc = "Bits 4:7 - Sets the cache configuration"]
    #[inline(always)]
    pub fn config(&mut self) -> CONFIG_W {
        CONFIG_W { w: self }
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline(always)]
    pub fn enable_nc1(&mut self) -> ENABLE_NC1_W {
        ENABLE_NC1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline(always)]
    pub fn enable_nc0(&mut self) -> ENABLE_NC0_W {
        ENABLE_NC0_W { w: self }
    }
    #[doc = "Bit 1 - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline(always)]
    pub fn lru(&mut self) -> LRU_W {
        LRU_W { w: self }
    }
    #[doc = "Bit 0 - Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
