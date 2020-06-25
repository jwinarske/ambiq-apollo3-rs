#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FIFO Access Port"]
    pub fifo: FIFO,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - FIFO size and remaining slots open values"]
    pub fifoptr: FIFOPTR,
    #[doc = "0x104 - FIFO Threshold Configuration"]
    pub fifothr: FIFOTHR,
    #[doc = "0x108 - FIFO POP register"]
    pub fifopop: FIFOPOP,
    #[doc = "0x10c - FIFO PUSH register"]
    pub fifopush: FIFOPUSH,
    #[doc = "0x110 - FIFO Control Register"]
    pub fifoctrl: FIFOCTRL,
    #[doc = "0x114 - FIFO Pointers"]
    pub fifoloc: FIFOLOC,
    _reserved7: [u8; 232usize],
    #[doc = "0x200 - I/O Clock Configuration"]
    pub clkcfg: CLKCFG,
    _reserved8: [u8; 8usize],
    #[doc = "0x20c - Command and offset Register"]
    pub cmd: CMD,
    #[doc = "0x210 - Command Repeat Register"]
    pub cmdrpt: CMDRPT,
    #[doc = "0x214 - High order offset bytes"]
    pub offsethi: OFFSETHI,
    #[doc = "0x218 - Command status"]
    pub cmdstat: CMDSTAT,
    _reserved12: [u8; 4usize],
    #[doc = "0x220 - IO Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x224 - IO Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x228 - IO Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x22c - IO Master Interrupts: Set"]
    pub intset: INTSET,
    #[doc = "0x230 - DMA Trigger Enable Register"]
    pub dmatrigen: DMATRIGEN,
    #[doc = "0x234 - DMA Trigger Status Register"]
    pub dmatrigstat: DMATRIGSTAT,
    #[doc = "0x238 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    #[doc = "0x23c - DMA Total Transfer Count"]
    pub dmatotcount: DMATOTCOUNT,
    #[doc = "0x240 - DMA Target Address Register"]
    pub dmatargaddr: DMATARGADDR,
    #[doc = "0x244 - DMA Status Register"]
    pub dmastat: DMASTAT,
    #[doc = "0x248 - Command Queue Configuration Register"]
    pub cqcfg: CQCFG,
    #[doc = "0x24c - CQ Target Read Address Register"]
    pub cqaddr: CQADDR,
    #[doc = "0x250 - Command Queue Status Register"]
    pub cqstat: CQSTAT,
    #[doc = "0x254 - Command Queue Flag Register"]
    pub cqflags: CQFLAGS,
    #[doc = "0x258 - Command Queue Flag Set/Clear Register"]
    pub cqsetclear: CQSETCLEAR,
    #[doc = "0x25c - Command Queue Pause Enable Register"]
    pub cqpauseen: CQPAUSEEN,
    #[doc = "0x260 - IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue"]
    pub cqcuridx: CQCURIDX,
    #[doc = "0x264 - IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue"]
    pub cqendidx: CQENDIDX,
    #[doc = "0x268 - IOM Module Status Register"]
    pub status: STATUS,
    _reserved31: [u8; 148usize],
    #[doc = "0x300 - SPI module master configuration"]
    pub mspicfg: MSPICFG,
    #[doc = "0x304 - BLE Core Control"]
    pub blecfg: BLECFG,
    #[doc = "0x308 - BLE Power command interface"]
    pub pwrcmd: PWRCMD,
    #[doc = "0x30c - BLE Core status"]
    pub bstatus: BSTATUS,
    _reserved35: [u8; 256usize],
    #[doc = "0x410 - BLEIF Master Debug Register"]
    pub bledbg: BLEDBG,
}
#[doc = "FIFO Access Port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "FIFO Access Port"]
pub mod fifo;
#[doc = "FIFO size and remaining slots open values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoptr](fifoptr) module"]
pub type FIFOPTR = crate::Reg<u32, _FIFOPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOPTR;
#[doc = "`read()` method returns [fifoptr::R](fifoptr::R) reader structure"]
impl crate::Readable for FIFOPTR {}
#[doc = "`write(|w| ..)` method takes [fifoptr::W](fifoptr::W) writer structure"]
impl crate::Writable for FIFOPTR {}
#[doc = "FIFO size and remaining slots open values"]
pub mod fifoptr;
#[doc = "FIFO Threshold Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifothr](fifothr) module"]
pub type FIFOTHR = crate::Reg<u32, _FIFOTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOTHR;
#[doc = "`read()` method returns [fifothr::R](fifothr::R) reader structure"]
impl crate::Readable for FIFOTHR {}
#[doc = "`write(|w| ..)` method takes [fifothr::W](fifothr::W) writer structure"]
impl crate::Writable for FIFOTHR {}
#[doc = "FIFO Threshold Configuration"]
pub mod fifothr;
#[doc = "FIFO POP register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifopop](fifopop) module"]
pub type FIFOPOP = crate::Reg<u32, _FIFOPOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOPOP;
#[doc = "`read()` method returns [fifopop::R](fifopop::R) reader structure"]
impl crate::Readable for FIFOPOP {}
#[doc = "`write(|w| ..)` method takes [fifopop::W](fifopop::W) writer structure"]
impl crate::Writable for FIFOPOP {}
#[doc = "FIFO POP register"]
pub mod fifopop;
#[doc = "FIFO PUSH register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifopush](fifopush) module"]
pub type FIFOPUSH = crate::Reg<u32, _FIFOPUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOPUSH;
#[doc = "`read()` method returns [fifopush::R](fifopush::R) reader structure"]
impl crate::Readable for FIFOPUSH {}
#[doc = "`write(|w| ..)` method takes [fifopush::W](fifopush::W) writer structure"]
impl crate::Writable for FIFOPUSH {}
#[doc = "FIFO PUSH register"]
pub mod fifopush;
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoctrl](fifoctrl) module"]
pub type FIFOCTRL = crate::Reg<u32, _FIFOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCTRL;
#[doc = "`read()` method returns [fifoctrl::R](fifoctrl::R) reader structure"]
impl crate::Readable for FIFOCTRL {}
#[doc = "`write(|w| ..)` method takes [fifoctrl::W](fifoctrl::W) writer structure"]
impl crate::Writable for FIFOCTRL {}
#[doc = "FIFO Control Register"]
pub mod fifoctrl;
#[doc = "FIFO Pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoloc](fifoloc) module"]
pub type FIFOLOC = crate::Reg<u32, _FIFOLOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOLOC;
#[doc = "`read()` method returns [fifoloc::R](fifoloc::R) reader structure"]
impl crate::Readable for FIFOLOC {}
#[doc = "`write(|w| ..)` method takes [fifoloc::W](fifoloc::W) writer structure"]
impl crate::Writable for FIFOLOC {}
#[doc = "FIFO Pointers"]
pub mod fifoloc;
#[doc = "I/O Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcfg](clkcfg) module"]
pub type CLKCFG = crate::Reg<u32, _CLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCFG;
#[doc = "`read()` method returns [clkcfg::R](clkcfg::R) reader structure"]
impl crate::Readable for CLKCFG {}
#[doc = "`write(|w| ..)` method takes [clkcfg::W](clkcfg::W) writer structure"]
impl crate::Writable for CLKCFG {}
#[doc = "I/O Clock Configuration"]
pub mod clkcfg;
#[doc = "Command and offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command and offset Register"]
pub mod cmd;
#[doc = "Command Repeat Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdrpt](cmdrpt) module"]
pub type CMDRPT = crate::Reg<u32, _CMDRPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDRPT;
#[doc = "`read()` method returns [cmdrpt::R](cmdrpt::R) reader structure"]
impl crate::Readable for CMDRPT {}
#[doc = "`write(|w| ..)` method takes [cmdrpt::W](cmdrpt::W) writer structure"]
impl crate::Writable for CMDRPT {}
#[doc = "Command Repeat Register"]
pub mod cmdrpt;
#[doc = "High order offset bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offsethi](offsethi) module"]
pub type OFFSETHI = crate::Reg<u32, _OFFSETHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFFSETHI;
#[doc = "`read()` method returns [offsethi::R](offsethi::R) reader structure"]
impl crate::Readable for OFFSETHI {}
#[doc = "`write(|w| ..)` method takes [offsethi::W](offsethi::W) writer structure"]
impl crate::Writable for OFFSETHI {}
#[doc = "High order offset bytes"]
pub mod offsethi;
#[doc = "Command status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdstat](cmdstat) module"]
pub type CMDSTAT = crate::Reg<u32, _CMDSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDSTAT;
#[doc = "`read()` method returns [cmdstat::R](cmdstat::R) reader structure"]
impl crate::Readable for CMDSTAT {}
#[doc = "`write(|w| ..)` method takes [cmdstat::W](cmdstat::W) writer structure"]
impl crate::Writable for CMDSTAT {}
#[doc = "Command status"]
pub mod cmdstat;
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
#[doc = "Command Queue Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcfg](cqcfg) module"]
pub type CQCFG = crate::Reg<u32, _CQCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCFG;
#[doc = "`read()` method returns [cqcfg::R](cqcfg::R) reader structure"]
impl crate::Readable for CQCFG {}
#[doc = "`write(|w| ..)` method takes [cqcfg::W](cqcfg::W) writer structure"]
impl crate::Writable for CQCFG {}
#[doc = "Command Queue Configuration Register"]
pub mod cqcfg;
#[doc = "CQ Target Read Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqaddr](cqaddr) module"]
pub type CQADDR = crate::Reg<u32, _CQADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQADDR;
#[doc = "`read()` method returns [cqaddr::R](cqaddr::R) reader structure"]
impl crate::Readable for CQADDR {}
#[doc = "`write(|w| ..)` method takes [cqaddr::W](cqaddr::W) writer structure"]
impl crate::Writable for CQADDR {}
#[doc = "CQ Target Read Address Register"]
pub mod cqaddr;
#[doc = "Command Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqstat](cqstat) module"]
pub type CQSTAT = crate::Reg<u32, _CQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQSTAT;
#[doc = "`read()` method returns [cqstat::R](cqstat::R) reader structure"]
impl crate::Readable for CQSTAT {}
#[doc = "`write(|w| ..)` method takes [cqstat::W](cqstat::W) writer structure"]
impl crate::Writable for CQSTAT {}
#[doc = "Command Queue Status Register"]
pub mod cqstat;
#[doc = "Command Queue Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqflags](cqflags) module"]
pub type CQFLAGS = crate::Reg<u32, _CQFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQFLAGS;
#[doc = "`read()` method returns [cqflags::R](cqflags::R) reader structure"]
impl crate::Readable for CQFLAGS {}
#[doc = "`write(|w| ..)` method takes [cqflags::W](cqflags::W) writer structure"]
impl crate::Writable for CQFLAGS {}
#[doc = "Command Queue Flag Register"]
pub mod cqflags;
#[doc = "Command Queue Flag Set/Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqsetclear](cqsetclear) module"]
pub type CQSETCLEAR = crate::Reg<u32, _CQSETCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQSETCLEAR;
#[doc = "`read()` method returns [cqsetclear::R](cqsetclear::R) reader structure"]
impl crate::Readable for CQSETCLEAR {}
#[doc = "`write(|w| ..)` method takes [cqsetclear::W](cqsetclear::W) writer structure"]
impl crate::Writable for CQSETCLEAR {}
#[doc = "Command Queue Flag Set/Clear Register"]
pub mod cqsetclear;
#[doc = "Command Queue Pause Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqpauseen](cqpauseen) module"]
pub type CQPAUSEEN = crate::Reg<u32, _CQPAUSEEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQPAUSEEN;
#[doc = "`read()` method returns [cqpauseen::R](cqpauseen::R) reader structure"]
impl crate::Readable for CQPAUSEEN {}
#[doc = "`write(|w| ..)` method takes [cqpauseen::W](cqpauseen::W) writer structure"]
impl crate::Writable for CQPAUSEEN {}
#[doc = "Command Queue Pause Enable Register"]
pub mod cqpauseen;
#[doc = "IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcuridx](cqcuridx) module"]
pub type CQCURIDX = crate::Reg<u32, _CQCURIDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCURIDX;
#[doc = "`read()` method returns [cqcuridx::R](cqcuridx::R) reader structure"]
impl crate::Readable for CQCURIDX {}
#[doc = "`write(|w| ..)` method takes [cqcuridx::W](cqcuridx::W) writer structure"]
impl crate::Writable for CQCURIDX {}
#[doc = "IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue"]
pub mod cqcuridx;
#[doc = "IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqendidx](cqendidx) module"]
pub type CQENDIDX = crate::Reg<u32, _CQENDIDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQENDIDX;
#[doc = "`read()` method returns [cqendidx::R](cqendidx::R) reader structure"]
impl crate::Readable for CQENDIDX {}
#[doc = "`write(|w| ..)` method takes [cqendidx::W](cqendidx::W) writer structure"]
impl crate::Writable for CQENDIDX {}
#[doc = "IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue"]
pub mod cqendidx;
#[doc = "IOM Module Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "IOM Module Status Register"]
pub mod status;
#[doc = "SPI module master configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspicfg](mspicfg) module"]
pub type MSPICFG = crate::Reg<u32, _MSPICFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSPICFG;
#[doc = "`read()` method returns [mspicfg::R](mspicfg::R) reader structure"]
impl crate::Readable for MSPICFG {}
#[doc = "`write(|w| ..)` method takes [mspicfg::W](mspicfg::W) writer structure"]
impl crate::Writable for MSPICFG {}
#[doc = "SPI module master configuration"]
pub mod mspicfg;
#[doc = "BLE Core Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blecfg](blecfg) module"]
pub type BLECFG = crate::Reg<u32, _BLECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLECFG;
#[doc = "`read()` method returns [blecfg::R](blecfg::R) reader structure"]
impl crate::Readable for BLECFG {}
#[doc = "`write(|w| ..)` method takes [blecfg::W](blecfg::W) writer structure"]
impl crate::Writable for BLECFG {}
#[doc = "BLE Core Control"]
pub mod blecfg;
#[doc = "BLE Power command interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcmd](pwrcmd) module"]
pub type PWRCMD = crate::Reg<u32, _PWRCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCMD;
#[doc = "`read()` method returns [pwrcmd::R](pwrcmd::R) reader structure"]
impl crate::Readable for PWRCMD {}
#[doc = "`write(|w| ..)` method takes [pwrcmd::W](pwrcmd::W) writer structure"]
impl crate::Writable for PWRCMD {}
#[doc = "BLE Power command interface"]
pub mod pwrcmd;
#[doc = "BLE Core status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bstatus](bstatus) module"]
pub type BSTATUS = crate::Reg<u32, _BSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSTATUS;
#[doc = "`read()` method returns [bstatus::R](bstatus::R) reader structure"]
impl crate::Readable for BSTATUS {}
#[doc = "`write(|w| ..)` method takes [bstatus::W](bstatus::W) writer structure"]
impl crate::Writable for BSTATUS {}
#[doc = "BLE Core status"]
pub mod bstatus;
#[doc = "BLEIF Master Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bledbg](bledbg) module"]
pub type BLEDBG = crate::Reg<u32, _BLEDBG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLEDBG;
#[doc = "`read()` method returns [bledbg::R](bledbg::R) reader structure"]
impl crate::Readable for BLEDBG {}
#[doc = "`write(|w| ..)` method takes [bledbg::W](bledbg::W) writer structure"]
impl crate::Writable for BLEDBG {}
#[doc = "BLEIF Master Debug Register"]
pub mod bledbg;
