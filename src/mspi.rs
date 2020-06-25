#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSPI PIO Transfer Control/Status Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - MSPI Transfer Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - MSPI Transfer Address Register"]
    pub addr: ADDR,
    #[doc = "0x0c - MSPI Transfer Instruction"]
    pub instr: INSTR,
    #[doc = "0x10 - TX Data FIFO"]
    pub txfifo: TXFIFO,
    #[doc = "0x14 - RX Data FIFO"]
    pub rxfifo: RXFIFO,
    #[doc = "0x18 - TX FIFO Entries"]
    pub txentries: TXENTRIES,
    #[doc = "0x1c - RX FIFO Entries"]
    pub rxentries: RXENTRIES,
    #[doc = "0x20 - TX/RX FIFO Threshhold Levels"]
    pub threshold: THRESHOLD,
    _reserved9: [u8; 220usize],
    #[doc = "0x100 - MSPI Module Configuration"]
    pub mspicfg: MSPICFG,
    #[doc = "0x104 - MSPI Output Pad Configuration"]
    pub padcfg: PADCFG,
    #[doc = "0x108 - MSPI Output Enable Pad Configuration"]
    pub padouten: PADOUTEN,
    #[doc = "0x10c - Configuration for XIP/DMA support of SPI flash modules."]
    pub flash: FLASH,
    _reserved13: [u8; 16usize],
    #[doc = "0x120 - External Flash Scrambling Controls"]
    pub scrambling: SCRAMBLING,
    _reserved14: [u8; 220usize],
    #[doc = "0x200 - MSPI Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - MSPI Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - MSPI Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - MSPI Master Interrupts: Set"]
    pub intset: INTSET,
    _reserved18: [u8; 64usize],
    #[doc = "0x250 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    #[doc = "0x254 - DMA Status Register"]
    pub dmastat: DMASTAT,
    #[doc = "0x258 - DMA Target Address Register"]
    pub dmatargaddr: DMATARGADDR,
    #[doc = "0x25c - DMA Device Address Register"]
    pub dmadevaddr: DMADEVADDR,
    #[doc = "0x260 - DMA Total Transfer Count"]
    pub dmatotcount: DMATOTCOUNT,
    #[doc = "0x264 - DMA BYTE Transfer Count"]
    pub dmabcount: DMABCOUNT,
    _reserved24: [u8; 16usize],
    #[doc = "0x278 - DMA Transmit Trigger Threshhold"]
    pub dmathresh: DMATHRESH,
    _reserved25: [u8; 36usize],
    #[doc = "0x2a0 - Command Queue Configuration Register"]
    pub cqcfg: CQCFG,
    _reserved26: [u8; 4usize],
    #[doc = "0x2a8 - CQ Target Read Address Register"]
    pub cqaddr: CQADDR,
    #[doc = "0x2ac - Command Queue Status Register"]
    pub cqstat: CQSTAT,
    #[doc = "0x2b0 - Command Queue Flag Register"]
    pub cqflags: CQFLAGS,
    #[doc = "0x2b4 - Command Queue Flag Set/Clear Register"]
    pub cqsetclear: CQSETCLEAR,
    #[doc = "0x2b8 - Command Queue Pause Mask Register"]
    pub cqpause: CQPAUSE,
    _reserved31: [u8; 4usize],
    #[doc = "0x2c0 - Command Queue Current Index"]
    pub cqcuridx: CQCURIDX,
    #[doc = "0x2c4 - Command Queue End Index"]
    pub cqendidx: CQENDIDX,
}
#[doc = "MSPI PIO Transfer Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "MSPI PIO Transfer Control/Status Register"]
pub mod ctrl;
#[doc = "MSPI Transfer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "MSPI Transfer Configuration Register"]
pub mod cfg;
#[doc = "MSPI Transfer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "MSPI Transfer Address Register"]
pub mod addr;
#[doc = "MSPI Transfer Instruction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr](instr) module"]
pub type INSTR = crate::Reg<u32, _INSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR;
#[doc = "`read()` method returns [instr::R](instr::R) reader structure"]
impl crate::Readable for INSTR {}
#[doc = "`write(|w| ..)` method takes [instr::W](instr::W) writer structure"]
impl crate::Writable for INSTR {}
#[doc = "MSPI Transfer Instruction"]
pub mod instr;
#[doc = "TX Data FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifo](txfifo) module"]
pub type TXFIFO = crate::Reg<u32, _TXFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFIFO;
#[doc = "`read()` method returns [txfifo::R](txfifo::R) reader structure"]
impl crate::Readable for TXFIFO {}
#[doc = "`write(|w| ..)` method takes [txfifo::W](txfifo::W) writer structure"]
impl crate::Writable for TXFIFO {}
#[doc = "TX Data FIFO"]
pub mod txfifo;
#[doc = "RX Data FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo](rxfifo) module"]
pub type RXFIFO = crate::Reg<u32, _RXFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIFO;
#[doc = "`read()` method returns [rxfifo::R](rxfifo::R) reader structure"]
impl crate::Readable for RXFIFO {}
#[doc = "`write(|w| ..)` method takes [rxfifo::W](rxfifo::W) writer structure"]
impl crate::Writable for RXFIFO {}
#[doc = "RX Data FIFO"]
pub mod rxfifo;
#[doc = "TX FIFO Entries\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txentries](txentries) module"]
pub type TXENTRIES = crate::Reg<u32, _TXENTRIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXENTRIES;
#[doc = "`read()` method returns [txentries::R](txentries::R) reader structure"]
impl crate::Readable for TXENTRIES {}
#[doc = "`write(|w| ..)` method takes [txentries::W](txentries::W) writer structure"]
impl crate::Writable for TXENTRIES {}
#[doc = "TX FIFO Entries"]
pub mod txentries;
#[doc = "RX FIFO Entries\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxentries](rxentries) module"]
pub type RXENTRIES = crate::Reg<u32, _RXENTRIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXENTRIES;
#[doc = "`read()` method returns [rxentries::R](rxentries::R) reader structure"]
impl crate::Readable for RXENTRIES {}
#[doc = "`write(|w| ..)` method takes [rxentries::W](rxentries::W) writer structure"]
impl crate::Writable for RXENTRIES {}
#[doc = "RX FIFO Entries"]
pub mod rxentries;
#[doc = "TX/RX FIFO Threshhold Levels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [threshold](threshold) module"]
pub type THRESHOLD = crate::Reg<u32, _THRESHOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THRESHOLD;
#[doc = "`read()` method returns [threshold::R](threshold::R) reader structure"]
impl crate::Readable for THRESHOLD {}
#[doc = "`write(|w| ..)` method takes [threshold::W](threshold::W) writer structure"]
impl crate::Writable for THRESHOLD {}
#[doc = "TX/RX FIFO Threshhold Levels"]
pub mod threshold;
#[doc = "MSPI Module Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspicfg](mspicfg) module"]
pub type MSPICFG = crate::Reg<u32, _MSPICFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSPICFG;
#[doc = "`read()` method returns [mspicfg::R](mspicfg::R) reader structure"]
impl crate::Readable for MSPICFG {}
#[doc = "`write(|w| ..)` method takes [mspicfg::W](mspicfg::W) writer structure"]
impl crate::Writable for MSPICFG {}
#[doc = "MSPI Module Configuration"]
pub mod mspicfg;
#[doc = "MSPI Output Pad Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg](padcfg) module"]
pub type PADCFG = crate::Reg<u32, _PADCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADCFG;
#[doc = "`read()` method returns [padcfg::R](padcfg::R) reader structure"]
impl crate::Readable for PADCFG {}
#[doc = "`write(|w| ..)` method takes [padcfg::W](padcfg::W) writer structure"]
impl crate::Writable for PADCFG {}
#[doc = "MSPI Output Pad Configuration"]
pub mod padcfg;
#[doc = "MSPI Output Enable Pad Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padouten](padouten) module"]
pub type PADOUTEN = crate::Reg<u32, _PADOUTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADOUTEN;
#[doc = "`read()` method returns [padouten::R](padouten::R) reader structure"]
impl crate::Readable for PADOUTEN {}
#[doc = "`write(|w| ..)` method takes [padouten::W](padouten::W) writer structure"]
impl crate::Writable for PADOUTEN {}
#[doc = "MSPI Output Enable Pad Configuration"]
pub mod padouten;
#[doc = "Configuration for XIP/DMA support of SPI flash modules.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash](flash) module"]
pub type FLASH = crate::Reg<u32, _FLASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH;
#[doc = "`read()` method returns [flash::R](flash::R) reader structure"]
impl crate::Readable for FLASH {}
#[doc = "`write(|w| ..)` method takes [flash::W](flash::W) writer structure"]
impl crate::Writable for FLASH {}
#[doc = "Configuration for XIP/DMA support of SPI flash modules."]
pub mod flash;
#[doc = "External Flash Scrambling Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scrambling](scrambling) module"]
pub type SCRAMBLING = crate::Reg<u32, _SCRAMBLING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRAMBLING;
#[doc = "`read()` method returns [scrambling::R](scrambling::R) reader structure"]
impl crate::Readable for SCRAMBLING {}
#[doc = "`write(|w| ..)` method takes [scrambling::W](scrambling::W) writer structure"]
impl crate::Writable for SCRAMBLING {}
#[doc = "External Flash Scrambling Controls"]
pub mod scrambling;
#[doc = "MSPI Master Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "MSPI Master Interrupts: Enable"]
pub mod inten;
#[doc = "MSPI Master Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "MSPI Master Interrupts: Status"]
pub mod intstat;
#[doc = "MSPI Master Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "MSPI Master Interrupts: Clear"]
pub mod intclr;
#[doc = "MSPI Master Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "MSPI Master Interrupts: Set"]
pub mod intset;
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
#[doc = "DMA Device Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmadevaddr](dmadevaddr) module"]
pub type DMADEVADDR = crate::Reg<u32, _DMADEVADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMADEVADDR;
#[doc = "`read()` method returns [dmadevaddr::R](dmadevaddr::R) reader structure"]
impl crate::Readable for DMADEVADDR {}
#[doc = "`write(|w| ..)` method takes [dmadevaddr::W](dmadevaddr::W) writer structure"]
impl crate::Writable for DMADEVADDR {}
#[doc = "DMA Device Address Register"]
pub mod dmadevaddr;
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
#[doc = "DMA BYTE Transfer Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabcount](dmabcount) module"]
pub type DMABCOUNT = crate::Reg<u32, _DMABCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMABCOUNT;
#[doc = "`read()` method returns [dmabcount::R](dmabcount::R) reader structure"]
impl crate::Readable for DMABCOUNT {}
#[doc = "`write(|w| ..)` method takes [dmabcount::W](dmabcount::W) writer structure"]
impl crate::Writable for DMABCOUNT {}
#[doc = "DMA BYTE Transfer Count"]
pub mod dmabcount;
#[doc = "DMA Transmit Trigger Threshhold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmathresh](dmathresh) module"]
pub type DMATHRESH = crate::Reg<u32, _DMATHRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATHRESH;
#[doc = "`read()` method returns [dmathresh::R](dmathresh::R) reader structure"]
impl crate::Readable for DMATHRESH {}
#[doc = "`write(|w| ..)` method takes [dmathresh::W](dmathresh::W) writer structure"]
impl crate::Writable for DMATHRESH {}
#[doc = "DMA Transmit Trigger Threshhold"]
pub mod dmathresh;
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
#[doc = "Command Queue Pause Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqpause](cqpause) module"]
pub type CQPAUSE = crate::Reg<u32, _CQPAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQPAUSE;
#[doc = "`read()` method returns [cqpause::R](cqpause::R) reader structure"]
impl crate::Readable for CQPAUSE {}
#[doc = "`write(|w| ..)` method takes [cqpause::W](cqpause::W) writer structure"]
impl crate::Writable for CQPAUSE {}
#[doc = "Command Queue Pause Mask Register"]
pub mod cqpause;
#[doc = "Command Queue Current Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcuridx](cqcuridx) module"]
pub type CQCURIDX = crate::Reg<u32, _CQCURIDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCURIDX;
#[doc = "`read()` method returns [cqcuridx::R](cqcuridx::R) reader structure"]
impl crate::Readable for CQCURIDX {}
#[doc = "`write(|w| ..)` method takes [cqcuridx::W](cqcuridx::W) writer structure"]
impl crate::Writable for CQCURIDX {}
#[doc = "Command Queue Current Index"]
pub mod cqcuridx;
#[doc = "Command Queue End Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqendidx](cqendidx) module"]
pub type CQENDIDX = crate::Reg<u32, _CQENDIDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQENDIDX;
#[doc = "`read()` method returns [cqendidx::R](cqendidx::R) reader structure"]
impl crate::Readable for CQENDIDX {}
#[doc = "`write(|w| ..)` method takes [cqendidx::W](cqendidx::W) writer structure"]
impl crate::Writable for CQENDIDX {}
#[doc = "Command Queue End Index"]
pub mod cqendidx;
