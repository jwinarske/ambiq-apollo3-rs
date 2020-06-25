#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - Software POI Reset"]
    pub swpoi: SWPOI,
    #[doc = "0x08 - Software POR Reset"]
    pub swpor: SWPOR,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - TPIU reset"]
    pub tpiurst: TPIURST,
    _reserved4: [u8; 488usize],
    #[doc = "0x200 - Reset Interrupt register: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - Reset Interrupt register: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - Reset Interrupt register: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - Reset Interrupt register: Set"]
    pub intset: INTSET,
    _reserved8: [u8; 268430832usize],
    #[doc = "0xffff000 - Status Register (SBL)"]
    pub stat: STAT,
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "Software POI Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpoi](swpoi) module"]
pub type SWPOI = crate::Reg<u32, _SWPOI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPOI;
#[doc = "`read()` method returns [swpoi::R](swpoi::R) reader structure"]
impl crate::Readable for SWPOI {}
#[doc = "`write(|w| ..)` method takes [swpoi::W](swpoi::W) writer structure"]
impl crate::Writable for SWPOI {}
#[doc = "Software POI Reset"]
pub mod swpoi;
#[doc = "Software POR Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpor](swpor) module"]
pub type SWPOR = crate::Reg<u32, _SWPOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPOR;
#[doc = "`read()` method returns [swpor::R](swpor::R) reader structure"]
impl crate::Readable for SWPOR {}
#[doc = "`write(|w| ..)` method takes [swpor::W](swpor::W) writer structure"]
impl crate::Writable for SWPOR {}
#[doc = "Software POR Reset"]
pub mod swpor;
#[doc = "TPIU reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpiurst](tpiurst) module"]
pub type TPIURST = crate::Reg<u32, _TPIURST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPIURST;
#[doc = "`read()` method returns [tpiurst::R](tpiurst::R) reader structure"]
impl crate::Readable for TPIURST {}
#[doc = "`write(|w| ..)` method takes [tpiurst::W](tpiurst::W) writer structure"]
impl crate::Writable for TPIURST {}
#[doc = "TPIU reset"]
pub mod tpiurst;
#[doc = "Reset Interrupt register: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Reset Interrupt register: Enable"]
pub mod inten;
#[doc = "Reset Interrupt register: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "Reset Interrupt register: Status"]
pub mod intstat;
#[doc = "Reset Interrupt register: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "Reset Interrupt register: Clear"]
pub mod intclr;
#[doc = "Reset Interrupt register: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "Reset Interrupt register: Set"]
pub mod intset;
#[doc = "Status Register (SBL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Status Register (SBL)"]
pub mod stat;
