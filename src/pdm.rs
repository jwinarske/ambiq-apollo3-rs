#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDM Configuration Register"]
    pub pcfg: PCFG,
    #[doc = "0x04 - Voice Configuration Register"]
    pub vcfg: VCFG,
    #[doc = "0x08 - Voice Status Register"]
    pub voicestat: VOICESTAT,
    #[doc = "0x0c - FIFO Read"]
    pub fiforead: FIFOREAD,
    #[doc = "0x10 - FIFO Flush"]
    pub fifoflush: FIFOFLUSH,
    #[doc = "0x14 - FIFO Threshold"]
    pub fifothr: FIFOTHR,
    _reserved6: [u8; 488usize],
    #[doc = "0x200 - IO Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - IO Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - IO Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - IO Master Interrupts: Set"]
    pub intset: INTSET,
    _reserved10: [u8; 48usize],
    #[doc = "0x240 - DMA Trigger Enable Register"]
    pub dmatrigen: DMATRIGEN,
    #[doc = "0x244 - DMA Trigger Status Register"]
    pub dmatrigstat: DMATRIGSTAT,
    _reserved12: [u8; 56usize],
    #[doc = "0x280 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    _reserved13: [u8; 4usize],
    #[doc = "0x288 - DMA Total Transfer Count"]
    pub dmatotcount: DMATOTCOUNT,
    #[doc = "0x28c - DMA Target Address Register"]
    pub dmatargaddr: DMATARGADDR,
    #[doc = "0x290 - DMA Status Register"]
    pub dmastat: DMASTAT,
}
#[doc = "PDM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg](pcfg) module"]
pub type PCFG = crate::Reg<u32, _PCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCFG;
#[doc = "`read()` method returns [pcfg::R](pcfg::R) reader structure"]
impl crate::Readable for PCFG {}
#[doc = "`write(|w| ..)` method takes [pcfg::W](pcfg::W) writer structure"]
impl crate::Writable for PCFG {}
#[doc = "PDM Configuration Register"]
pub mod pcfg;
#[doc = "Voice Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcfg](vcfg) module"]
pub type VCFG = crate::Reg<u32, _VCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCFG;
#[doc = "`read()` method returns [vcfg::R](vcfg::R) reader structure"]
impl crate::Readable for VCFG {}
#[doc = "`write(|w| ..)` method takes [vcfg::W](vcfg::W) writer structure"]
impl crate::Writable for VCFG {}
#[doc = "Voice Configuration Register"]
pub mod vcfg;
#[doc = "Voice Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voicestat](voicestat) module"]
pub type VOICESTAT = crate::Reg<u32, _VOICESTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VOICESTAT;
#[doc = "`read()` method returns [voicestat::R](voicestat::R) reader structure"]
impl crate::Readable for VOICESTAT {}
#[doc = "`write(|w| ..)` method takes [voicestat::W](voicestat::W) writer structure"]
impl crate::Writable for VOICESTAT {}
#[doc = "Voice Status Register"]
pub mod voicestat;
#[doc = "FIFO Read\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiforead](fiforead) module"]
pub type FIFOREAD = crate::Reg<u32, _FIFOREAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOREAD;
#[doc = "`read()` method returns [fiforead::R](fiforead::R) reader structure"]
impl crate::Readable for FIFOREAD {}
#[doc = "`write(|w| ..)` method takes [fiforead::W](fiforead::W) writer structure"]
impl crate::Writable for FIFOREAD {}
#[doc = "FIFO Read"]
pub mod fiforead;
#[doc = "FIFO Flush\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoflush](fifoflush) module"]
pub type FIFOFLUSH = crate::Reg<u32, _FIFOFLUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOFLUSH;
#[doc = "`read()` method returns [fifoflush::R](fifoflush::R) reader structure"]
impl crate::Readable for FIFOFLUSH {}
#[doc = "`write(|w| ..)` method takes [fifoflush::W](fifoflush::W) writer structure"]
impl crate::Writable for FIFOFLUSH {}
#[doc = "FIFO Flush"]
pub mod fifoflush;
#[doc = "FIFO Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifothr](fifothr) module"]
pub type FIFOTHR = crate::Reg<u32, _FIFOTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOTHR;
#[doc = "`read()` method returns [fifothr::R](fifothr::R) reader structure"]
impl crate::Readable for FIFOTHR {}
#[doc = "`write(|w| ..)` method takes [fifothr::W](fifothr::W) writer structure"]
impl crate::Writable for FIFOTHR {}
#[doc = "FIFO Threshold"]
pub mod fifothr;
#[doc = "IO Master Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "IO Master Interrupts: Enable"]
pub mod inten;
#[doc = "IO Master Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "IO Master Interrupts: Status"]
pub mod intstat;
#[doc = "IO Master Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "IO Master Interrupts: Clear"]
pub mod intclr;
#[doc = "IO Master Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "IO Master Interrupts: Set"]
pub mod intset;
#[doc = "DMA Trigger Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatrigen](dmatrigen) module"]
pub type DMATRIGEN = crate::Reg<u32, _DMATRIGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATRIGEN;
#[doc = "`read()` method returns [dmatrigen::R](dmatrigen::R) reader structure"]
impl crate::Readable for DMATRIGEN {}
#[doc = "`write(|w| ..)` method takes [dmatrigen::W](dmatrigen::W) writer structure"]
impl crate::Writable for DMATRIGEN {}
#[doc = "DMA Trigger Enable Register"]
pub mod dmatrigen;
#[doc = "DMA Trigger Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatrigstat](dmatrigstat) module"]
pub type DMATRIGSTAT = crate::Reg<u32, _DMATRIGSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATRIGSTAT;
#[doc = "`read()` method returns [dmatrigstat::R](dmatrigstat::R) reader structure"]
impl crate::Readable for DMATRIGSTAT {}
#[doc = "`write(|w| ..)` method takes [dmatrigstat::W](dmatrigstat::W) writer structure"]
impl crate::Writable for DMATRIGSTAT {}
#[doc = "DMA Trigger Status Register"]
pub mod dmatrigstat;
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfg](dmacfg) module"]
pub type DMACFG = crate::Reg<u32, _DMACFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACFG;
#[doc = "`read()` method returns [dmacfg::R](dmacfg::R) reader structure"]
impl crate::Readable for DMACFG {}
#[doc = "`write(|w| ..)` method takes [dmacfg::W](dmacfg::W) writer structure"]
impl crate::Writable for DMACFG {}
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "DMA Total Transfer Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatotcount](dmatotcount) module"]
pub type DMATOTCOUNT = crate::Reg<u32, _DMATOTCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATOTCOUNT;
#[doc = "`read()` method returns [dmatotcount::R](dmatotcount::R) reader structure"]
impl crate::Readable for DMATOTCOUNT {}
#[doc = "`write(|w| ..)` method takes [dmatotcount::W](dmatotcount::W) writer structure"]
impl crate::Writable for DMATOTCOUNT {}
#[doc = "DMA Total Transfer Count"]
pub mod dmatotcount;
#[doc = "DMA Target Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatargaddr](dmatargaddr) module"]
pub type DMATARGADDR = crate::Reg<u32, _DMATARGADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATARGADDR;
#[doc = "`read()` method returns [dmatargaddr::R](dmatargaddr::R) reader structure"]
impl crate::Readable for DMATARGADDR {}
#[doc = "`write(|w| ..)` method takes [dmatargaddr::W](dmatargaddr::W) writer structure"]
impl crate::Writable for DMATARGADDR {}
#[doc = "DMA Target Address Register"]
pub mod dmatargaddr;
#[doc = "DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastat](dmastat) module"]
pub type DMASTAT = crate::Reg<u32, _DMASTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASTAT;
#[doc = "`read()` method returns [dmastat::R](dmastat::R) reader structure"]
impl crate::Readable for DMASTAT {}
#[doc = "`write(|w| ..)` method takes [dmastat::W](dmastat::W) writer structure"]
impl crate::Writable for DMASTAT {}
#[doc = "DMA Status Register"]
pub mod dmastat;
