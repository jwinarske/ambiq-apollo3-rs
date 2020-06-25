#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pad Configuration Register A (Pads 0-3)"]
    pub padrega: PADREGA,
    #[doc = "0x04 - Pad Configuration Register B (Pads 4-7)"]
    pub padregb: PADREGB,
    #[doc = "0x08 - Pad Configuration Register C (Pads 8-11)"]
    pub padregc: PADREGC,
    #[doc = "0x0c - Pad Configuration Register D (Pads 12-15)"]
    pub padregd: PADREGD,
    #[doc = "0x10 - Pad Configuration Register E (Pads 16-19)"]
    pub padrege: PADREGE,
    #[doc = "0x14 - Pad Configuration Register F (Pads 20-23)"]
    pub padregf: PADREGF,
    #[doc = "0x18 - Pad Configuration Register G (Pads 24-27)"]
    pub padregg: PADREGG,
    #[doc = "0x1c - Pad Configuration Register H (Pads 28-31)"]
    pub padregh: PADREGH,
    #[doc = "0x20 - Pad Configuration Register I (Pads 32-25)"]
    pub padregi: PADREGI,
    #[doc = "0x24 - Pad Configuration Register J (Pads 36-39)"]
    pub padregj: PADREGJ,
    #[doc = "0x28 - Pad Configuration Register K (Pads 40-43)"]
    pub padregk: PADREGK,
    #[doc = "0x2c - Pad Configuration Register L (Pads 44-47)"]
    pub padregl: PADREGL,
    #[doc = "0x30 - Pad Configuration Register M (Pads 47-48)"]
    pub padregm: PADREGM,
    _reserved13: [u8; 12usize],
    #[doc = "0x40 - GPIO Configuration Register A (Pads 0-7)"]
    pub cfga: CFGA,
    #[doc = "0x44 - GPIO Configuration Register B (Pads 8-15)"]
    pub cfgb: CFGB,
    #[doc = "0x48 - GPIO Configuration Register C (Pads 16-23)"]
    pub cfgc: CFGC,
    #[doc = "0x4c - GPIO Configuration Register D (Pads 24-31)"]
    pub cfgd: CFGD,
    #[doc = "0x50 - GPIO Configuration Register E (Pads 32-39)"]
    pub cfge: CFGE,
    #[doc = "0x54 - GPIO Configuration Register F (Pads 40 -47)"]
    pub cfgf: CFGF,
    #[doc = "0x58 - GPIO Configuration Register G (Pads 48-49)"]
    pub cfgg: CFGG,
    _reserved20: [u8; 4usize],
    #[doc = "0x60 - Key Register for all pad configuration registers"]
    pub padkey: PADKEY,
    _reserved21: [u8; 28usize],
    #[doc = "0x80 - GPIO Input Register A"]
    pub rda: RDA,
    #[doc = "0x84 - GPIO Input Register B"]
    pub rdb: RDB,
    #[doc = "0x88 - GPIO Output Register A"]
    pub wta: WTA,
    #[doc = "0x8c - GPIO Output Register B"]
    pub wtb: WTB,
    #[doc = "0x90 - GPIO Output Register A Set"]
    pub wtsa: WTSA,
    #[doc = "0x94 - GPIO Output Register B Set"]
    pub wtsb: WTSB,
    #[doc = "0x98 - GPIO Output Register A Clear"]
    pub wtca: WTCA,
    #[doc = "0x9c - GPIO Output Register B Clear"]
    pub wtcb: WTCB,
    #[doc = "0xa0 - GPIO Enable Register A"]
    pub ena: ENA,
    #[doc = "0xa4 - GPIO Enable Register B"]
    pub enb: ENB,
    #[doc = "0xa8 - GPIO Enable Register A Set"]
    pub ensa: ENSA,
    #[doc = "0xac - GPIO Enable Register B Set"]
    pub ensb: ENSB,
    _reserved33: [u8; 4usize],
    #[doc = "0xb4 - GPIO Enable Register A Clear"]
    pub enca: ENCA,
    #[doc = "0xb8 - GPIO Enable Register B Clear"]
    pub encb: ENCB,
    #[doc = "0xbc - STIMER Capture Control"]
    pub stmrcap: STMRCAP,
    #[doc = "0xc0 - IOM0 Flow Control IRQ Select"]
    pub iom0irq: IOM0IRQ,
    #[doc = "0xc4 - IOM1 Flow Control IRQ Select"]
    pub iom1irq: IOM1IRQ,
    #[doc = "0xc8 - IOM2 Flow Control IRQ Select"]
    pub iom2irq: IOM2IRQ,
    #[doc = "0xcc - IOM3 Flow Control IRQ Select"]
    pub iom3irq: IOM3IRQ,
    #[doc = "0xd0 - IOM4 Flow Control IRQ Select"]
    pub iom4irq: IOM4IRQ,
    #[doc = "0xd4 - IOM5 Flow Control IRQ Select"]
    pub iom5irq: IOM5IRQ,
    #[doc = "0xd8 - BLEIF Flow Control IRQ Select"]
    pub bleifirq: BLEIFIRQ,
    #[doc = "0xdc - GPIO Observation Mode Sample register"]
    pub gpioobs: GPIOOBS,
    #[doc = "0xe0 - Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
    pub altpadcfga: ALTPADCFGA,
    #[doc = "0xe4 - Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
    pub altpadcfgb: ALTPADCFGB,
    #[doc = "0xe8 - Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
    pub altpadcfgc: ALTPADCFGC,
    #[doc = "0xec - Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
    pub altpadcfgd: ALTPADCFGD,
    #[doc = "0xf0 - Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
    pub altpadcfge: ALTPADCFGE,
    #[doc = "0xf4 - Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
    pub altpadcfgf: ALTPADCFGF,
    #[doc = "0xf8 - Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
    pub altpadcfgg: ALTPADCFGG,
    #[doc = "0xfc - Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
    pub altpadcfgh: ALTPADCFGH,
    #[doc = "0x100 - Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
    pub altpadcfgi: ALTPADCFGI,
    #[doc = "0x104 - Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
    pub altpadcfgj: ALTPADCFGJ,
    #[doc = "0x108 - Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
    pub altpadcfgk: ALTPADCFGK,
    #[doc = "0x10c - Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
    pub altpadcfgl: ALTPADCFGL,
    #[doc = "0x110 - Alternate Pad Configuration reg12 (Pads 49,48)"]
    pub altpadcfgm: ALTPADCFGM,
    #[doc = "0x114 - SCARD Card Detect select"]
    pub scdet: SCDET,
    #[doc = "0x118 - Counter/Timer Enable Config"]
    pub ctencfg: CTENCFG,
    _reserved59: [u8; 228usize],
    #[doc = "0x200 - GPIO Interrupt Registers 31-0: Enable"]
    pub int0en: INT0EN,
    #[doc = "0x204 - GPIO Interrupt Registers 31-0: Status"]
    pub int0stat: INT0STAT,
    #[doc = "0x208 - GPIO Interrupt Registers 31-0: Clear"]
    pub int0clr: INT0CLR,
    #[doc = "0x20c - GPIO Interrupt Registers 31-0: Set"]
    pub int0set: INT0SET,
    #[doc = "0x210 - GPIO Interrupt Registers 49-32: Enable"]
    pub int1en: INT1EN,
    #[doc = "0x214 - GPIO Interrupt Registers 49-32: Status"]
    pub int1stat: INT1STAT,
    #[doc = "0x218 - GPIO Interrupt Registers 49-32: Clear"]
    pub int1clr: INT1CLR,
    #[doc = "0x21c - GPIO Interrupt Registers 49-32: Set"]
    pub int1set: INT1SET,
}
#[doc = "Pad Configuration Register A (Pads 0-3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padrega](padrega) module"]
pub type PADREGA = crate::Reg<u32, _PADREGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGA;
#[doc = "`read()` method returns [padrega::R](padrega::R) reader structure"]
impl crate::Readable for PADREGA {}
#[doc = "`write(|w| ..)` method takes [padrega::W](padrega::W) writer structure"]
impl crate::Writable for PADREGA {}
#[doc = "Pad Configuration Register A (Pads 0-3)"]
pub mod padrega;
#[doc = "Pad Configuration Register B (Pads 4-7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregb](padregb) module"]
pub type PADREGB = crate::Reg<u32, _PADREGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGB;
#[doc = "`read()` method returns [padregb::R](padregb::R) reader structure"]
impl crate::Readable for PADREGB {}
#[doc = "`write(|w| ..)` method takes [padregb::W](padregb::W) writer structure"]
impl crate::Writable for PADREGB {}
#[doc = "Pad Configuration Register B (Pads 4-7)"]
pub mod padregb;
#[doc = "Pad Configuration Register C (Pads 8-11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregc](padregc) module"]
pub type PADREGC = crate::Reg<u32, _PADREGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGC;
#[doc = "`read()` method returns [padregc::R](padregc::R) reader structure"]
impl crate::Readable for PADREGC {}
#[doc = "`write(|w| ..)` method takes [padregc::W](padregc::W) writer structure"]
impl crate::Writable for PADREGC {}
#[doc = "Pad Configuration Register C (Pads 8-11)"]
pub mod padregc;
#[doc = "Pad Configuration Register D (Pads 12-15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregd](padregd) module"]
pub type PADREGD = crate::Reg<u32, _PADREGD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGD;
#[doc = "`read()` method returns [padregd::R](padregd::R) reader structure"]
impl crate::Readable for PADREGD {}
#[doc = "`write(|w| ..)` method takes [padregd::W](padregd::W) writer structure"]
impl crate::Writable for PADREGD {}
#[doc = "Pad Configuration Register D (Pads 12-15)"]
pub mod padregd;
#[doc = "Pad Configuration Register E (Pads 16-19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padrege](padrege) module"]
pub type PADREGE = crate::Reg<u32, _PADREGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGE;
#[doc = "`read()` method returns [padrege::R](padrege::R) reader structure"]
impl crate::Readable for PADREGE {}
#[doc = "`write(|w| ..)` method takes [padrege::W](padrege::W) writer structure"]
impl crate::Writable for PADREGE {}
#[doc = "Pad Configuration Register E (Pads 16-19)"]
pub mod padrege;
#[doc = "Pad Configuration Register F (Pads 20-23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregf](padregf) module"]
pub type PADREGF = crate::Reg<u32, _PADREGF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGF;
#[doc = "`read()` method returns [padregf::R](padregf::R) reader structure"]
impl crate::Readable for PADREGF {}
#[doc = "`write(|w| ..)` method takes [padregf::W](padregf::W) writer structure"]
impl crate::Writable for PADREGF {}
#[doc = "Pad Configuration Register F (Pads 20-23)"]
pub mod padregf;
#[doc = "Pad Configuration Register G (Pads 24-27)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregg](padregg) module"]
pub type PADREGG = crate::Reg<u32, _PADREGG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGG;
#[doc = "`read()` method returns [padregg::R](padregg::R) reader structure"]
impl crate::Readable for PADREGG {}
#[doc = "`write(|w| ..)` method takes [padregg::W](padregg::W) writer structure"]
impl crate::Writable for PADREGG {}
#[doc = "Pad Configuration Register G (Pads 24-27)"]
pub mod padregg;
#[doc = "Pad Configuration Register H (Pads 28-31)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregh](padregh) module"]
pub type PADREGH = crate::Reg<u32, _PADREGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGH;
#[doc = "`read()` method returns [padregh::R](padregh::R) reader structure"]
impl crate::Readable for PADREGH {}
#[doc = "`write(|w| ..)` method takes [padregh::W](padregh::W) writer structure"]
impl crate::Writable for PADREGH {}
#[doc = "Pad Configuration Register H (Pads 28-31)"]
pub mod padregh;
#[doc = "Pad Configuration Register I (Pads 32-25)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregi](padregi) module"]
pub type PADREGI = crate::Reg<u32, _PADREGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGI;
#[doc = "`read()` method returns [padregi::R](padregi::R) reader structure"]
impl crate::Readable for PADREGI {}
#[doc = "`write(|w| ..)` method takes [padregi::W](padregi::W) writer structure"]
impl crate::Writable for PADREGI {}
#[doc = "Pad Configuration Register I (Pads 32-25)"]
pub mod padregi;
#[doc = "Pad Configuration Register J (Pads 36-39)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregj](padregj) module"]
pub type PADREGJ = crate::Reg<u32, _PADREGJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGJ;
#[doc = "`read()` method returns [padregj::R](padregj::R) reader structure"]
impl crate::Readable for PADREGJ {}
#[doc = "`write(|w| ..)` method takes [padregj::W](padregj::W) writer structure"]
impl crate::Writable for PADREGJ {}
#[doc = "Pad Configuration Register J (Pads 36-39)"]
pub mod padregj;
#[doc = "Pad Configuration Register K (Pads 40-43)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregk](padregk) module"]
pub type PADREGK = crate::Reg<u32, _PADREGK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGK;
#[doc = "`read()` method returns [padregk::R](padregk::R) reader structure"]
impl crate::Readable for PADREGK {}
#[doc = "`write(|w| ..)` method takes [padregk::W](padregk::W) writer structure"]
impl crate::Writable for PADREGK {}
#[doc = "Pad Configuration Register K (Pads 40-43)"]
pub mod padregk;
#[doc = "Pad Configuration Register L (Pads 44-47)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregl](padregl) module"]
pub type PADREGL = crate::Reg<u32, _PADREGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGL;
#[doc = "`read()` method returns [padregl::R](padregl::R) reader structure"]
impl crate::Readable for PADREGL {}
#[doc = "`write(|w| ..)` method takes [padregl::W](padregl::W) writer structure"]
impl crate::Writable for PADREGL {}
#[doc = "Pad Configuration Register L (Pads 44-47)"]
pub mod padregl;
#[doc = "Pad Configuration Register M (Pads 47-48)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregm](padregm) module"]
pub type PADREGM = crate::Reg<u32, _PADREGM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGM;
#[doc = "`read()` method returns [padregm::R](padregm::R) reader structure"]
impl crate::Readable for PADREGM {}
#[doc = "`write(|w| ..)` method takes [padregm::W](padregm::W) writer structure"]
impl crate::Writable for PADREGM {}
#[doc = "Pad Configuration Register M (Pads 47-48)"]
pub mod padregm;
#[doc = "GPIO Configuration Register A (Pads 0-7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfga](cfga) module"]
pub type CFGA = crate::Reg<u32, _CFGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGA;
#[doc = "`read()` method returns [cfga::R](cfga::R) reader structure"]
impl crate::Readable for CFGA {}
#[doc = "`write(|w| ..)` method takes [cfga::W](cfga::W) writer structure"]
impl crate::Writable for CFGA {}
#[doc = "GPIO Configuration Register A (Pads 0-7)"]
pub mod cfga;
#[doc = "GPIO Configuration Register B (Pads 8-15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgb](cfgb) module"]
pub type CFGB = crate::Reg<u32, _CFGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGB;
#[doc = "`read()` method returns [cfgb::R](cfgb::R) reader structure"]
impl crate::Readable for CFGB {}
#[doc = "`write(|w| ..)` method takes [cfgb::W](cfgb::W) writer structure"]
impl crate::Writable for CFGB {}
#[doc = "GPIO Configuration Register B (Pads 8-15)"]
pub mod cfgb;
#[doc = "GPIO Configuration Register C (Pads 16-23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgc](cfgc) module"]
pub type CFGC = crate::Reg<u32, _CFGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGC;
#[doc = "`read()` method returns [cfgc::R](cfgc::R) reader structure"]
impl crate::Readable for CFGC {}
#[doc = "`write(|w| ..)` method takes [cfgc::W](cfgc::W) writer structure"]
impl crate::Writable for CFGC {}
#[doc = "GPIO Configuration Register C (Pads 16-23)"]
pub mod cfgc;
#[doc = "GPIO Configuration Register D (Pads 24-31)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgd](cfgd) module"]
pub type CFGD = crate::Reg<u32, _CFGD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGD;
#[doc = "`read()` method returns [cfgd::R](cfgd::R) reader structure"]
impl crate::Readable for CFGD {}
#[doc = "`write(|w| ..)` method takes [cfgd::W](cfgd::W) writer structure"]
impl crate::Writable for CFGD {}
#[doc = "GPIO Configuration Register D (Pads 24-31)"]
pub mod cfgd;
#[doc = "GPIO Configuration Register E (Pads 32-39)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfge](cfge) module"]
pub type CFGE = crate::Reg<u32, _CFGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGE;
#[doc = "`read()` method returns [cfge::R](cfge::R) reader structure"]
impl crate::Readable for CFGE {}
#[doc = "`write(|w| ..)` method takes [cfge::W](cfge::W) writer structure"]
impl crate::Writable for CFGE {}
#[doc = "GPIO Configuration Register E (Pads 32-39)"]
pub mod cfge;
#[doc = "GPIO Configuration Register F (Pads 40 -47)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgf](cfgf) module"]
pub type CFGF = crate::Reg<u32, _CFGF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGF;
#[doc = "`read()` method returns [cfgf::R](cfgf::R) reader structure"]
impl crate::Readable for CFGF {}
#[doc = "`write(|w| ..)` method takes [cfgf::W](cfgf::W) writer structure"]
impl crate::Writable for CFGF {}
#[doc = "GPIO Configuration Register F (Pads 40 -47)"]
pub mod cfgf;
#[doc = "GPIO Configuration Register G (Pads 48-49)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgg](cfgg) module"]
pub type CFGG = crate::Reg<u32, _CFGG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGG;
#[doc = "`read()` method returns [cfgg::R](cfgg::R) reader structure"]
impl crate::Readable for CFGG {}
#[doc = "`write(|w| ..)` method takes [cfgg::W](cfgg::W) writer structure"]
impl crate::Writable for CFGG {}
#[doc = "GPIO Configuration Register G (Pads 48-49)"]
pub mod cfgg;
#[doc = "Key Register for all pad configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padkey](padkey) module"]
pub type PADKEY = crate::Reg<u32, _PADKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADKEY;
#[doc = "`read()` method returns [padkey::R](padkey::R) reader structure"]
impl crate::Readable for PADKEY {}
#[doc = "`write(|w| ..)` method takes [padkey::W](padkey::W) writer structure"]
impl crate::Writable for PADKEY {}
#[doc = "Key Register for all pad configuration registers"]
pub mod padkey;
#[doc = "GPIO Input Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rda](rda) module"]
pub type RDA = crate::Reg<u32, _RDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDA;
#[doc = "`read()` method returns [rda::R](rda::R) reader structure"]
impl crate::Readable for RDA {}
#[doc = "`write(|w| ..)` method takes [rda::W](rda::W) writer structure"]
impl crate::Writable for RDA {}
#[doc = "GPIO Input Register A"]
pub mod rda;
#[doc = "GPIO Input Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdb](rdb) module"]
pub type RDB = crate::Reg<u32, _RDB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDB;
#[doc = "`read()` method returns [rdb::R](rdb::R) reader structure"]
impl crate::Readable for RDB {}
#[doc = "`write(|w| ..)` method takes [rdb::W](rdb::W) writer structure"]
impl crate::Writable for RDB {}
#[doc = "GPIO Input Register B"]
pub mod rdb;
#[doc = "GPIO Output Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wta](wta) module"]
pub type WTA = crate::Reg<u32, _WTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTA;
#[doc = "`read()` method returns [wta::R](wta::R) reader structure"]
impl crate::Readable for WTA {}
#[doc = "`write(|w| ..)` method takes [wta::W](wta::W) writer structure"]
impl crate::Writable for WTA {}
#[doc = "GPIO Output Register A"]
pub mod wta;
#[doc = "GPIO Output Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtb](wtb) module"]
pub type WTB = crate::Reg<u32, _WTB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTB;
#[doc = "`read()` method returns [wtb::R](wtb::R) reader structure"]
impl crate::Readable for WTB {}
#[doc = "`write(|w| ..)` method takes [wtb::W](wtb::W) writer structure"]
impl crate::Writable for WTB {}
#[doc = "GPIO Output Register B"]
pub mod wtb;
#[doc = "GPIO Output Register A Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtsa](wtsa) module"]
pub type WTSA = crate::Reg<u32, _WTSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTSA;
#[doc = "`read()` method returns [wtsa::R](wtsa::R) reader structure"]
impl crate::Readable for WTSA {}
#[doc = "`write(|w| ..)` method takes [wtsa::W](wtsa::W) writer structure"]
impl crate::Writable for WTSA {}
#[doc = "GPIO Output Register A Set"]
pub mod wtsa;
#[doc = "GPIO Output Register B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtsb](wtsb) module"]
pub type WTSB = crate::Reg<u32, _WTSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTSB;
#[doc = "`read()` method returns [wtsb::R](wtsb::R) reader structure"]
impl crate::Readable for WTSB {}
#[doc = "`write(|w| ..)` method takes [wtsb::W](wtsb::W) writer structure"]
impl crate::Writable for WTSB {}
#[doc = "GPIO Output Register B Set"]
pub mod wtsb;
#[doc = "GPIO Output Register A Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtca](wtca) module"]
pub type WTCA = crate::Reg<u32, _WTCA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTCA;
#[doc = "`read()` method returns [wtca::R](wtca::R) reader structure"]
impl crate::Readable for WTCA {}
#[doc = "`write(|w| ..)` method takes [wtca::W](wtca::W) writer structure"]
impl crate::Writable for WTCA {}
#[doc = "GPIO Output Register A Clear"]
pub mod wtca;
#[doc = "GPIO Output Register B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtcb](wtcb) module"]
pub type WTCB = crate::Reg<u32, _WTCB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTCB;
#[doc = "`read()` method returns [wtcb::R](wtcb::R) reader structure"]
impl crate::Readable for WTCB {}
#[doc = "`write(|w| ..)` method takes [wtcb::W](wtcb::W) writer structure"]
impl crate::Writable for WTCB {}
#[doc = "GPIO Output Register B Clear"]
pub mod wtcb;
#[doc = "GPIO Enable Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ena](ena) module"]
pub type ENA = crate::Reg<u32, _ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENA;
#[doc = "`read()` method returns [ena::R](ena::R) reader structure"]
impl crate::Readable for ENA {}
#[doc = "`write(|w| ..)` method takes [ena::W](ena::W) writer structure"]
impl crate::Writable for ENA {}
#[doc = "GPIO Enable Register A"]
pub mod ena;
#[doc = "GPIO Enable Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enb](enb) module"]
pub type ENB = crate::Reg<u32, _ENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENB;
#[doc = "`read()` method returns [enb::R](enb::R) reader structure"]
impl crate::Readable for ENB {}
#[doc = "`write(|w| ..)` method takes [enb::W](enb::W) writer structure"]
impl crate::Writable for ENB {}
#[doc = "GPIO Enable Register B"]
pub mod enb;
#[doc = "GPIO Enable Register A Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ensa](ensa) module"]
pub type ENSA = crate::Reg<u32, _ENSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENSA;
#[doc = "`read()` method returns [ensa::R](ensa::R) reader structure"]
impl crate::Readable for ENSA {}
#[doc = "`write(|w| ..)` method takes [ensa::W](ensa::W) writer structure"]
impl crate::Writable for ENSA {}
#[doc = "GPIO Enable Register A Set"]
pub mod ensa;
#[doc = "GPIO Enable Register B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ensb](ensb) module"]
pub type ENSB = crate::Reg<u32, _ENSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENSB;
#[doc = "`read()` method returns [ensb::R](ensb::R) reader structure"]
impl crate::Readable for ENSB {}
#[doc = "`write(|w| ..)` method takes [ensb::W](ensb::W) writer structure"]
impl crate::Writable for ENSB {}
#[doc = "GPIO Enable Register B Set"]
pub mod ensb;
#[doc = "GPIO Enable Register A Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enca](enca) module"]
pub type ENCA = crate::Reg<u32, _ENCA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENCA;
#[doc = "`read()` method returns [enca::R](enca::R) reader structure"]
impl crate::Readable for ENCA {}
#[doc = "`write(|w| ..)` method takes [enca::W](enca::W) writer structure"]
impl crate::Writable for ENCA {}
#[doc = "GPIO Enable Register A Clear"]
pub mod enca;
#[doc = "GPIO Enable Register B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [encb](encb) module"]
pub type ENCB = crate::Reg<u32, _ENCB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENCB;
#[doc = "`read()` method returns [encb::R](encb::R) reader structure"]
impl crate::Readable for ENCB {}
#[doc = "`write(|w| ..)` method takes [encb::W](encb::W) writer structure"]
impl crate::Writable for ENCB {}
#[doc = "GPIO Enable Register B Clear"]
pub mod encb;
#[doc = "STIMER Capture Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmrcap](stmrcap) module"]
pub type STMRCAP = crate::Reg<u32, _STMRCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMRCAP;
#[doc = "`read()` method returns [stmrcap::R](stmrcap::R) reader structure"]
impl crate::Readable for STMRCAP {}
#[doc = "`write(|w| ..)` method takes [stmrcap::W](stmrcap::W) writer structure"]
impl crate::Writable for STMRCAP {}
#[doc = "STIMER Capture Control"]
pub mod stmrcap;
#[doc = "IOM0 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom0irq](iom0irq) module"]
pub type IOM0IRQ = crate::Reg<u32, _IOM0IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOM0IRQ;
#[doc = "`read()` method returns [iom0irq::R](iom0irq::R) reader structure"]
impl crate::Readable for IOM0IRQ {}
#[doc = "`write(|w| ..)` method takes [iom0irq::W](iom0irq::W) writer structure"]
impl crate::Writable for IOM0IRQ {}
#[doc = "IOM0 Flow Control IRQ Select"]
pub mod iom0irq;
#[doc = "IOM1 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom1irq](iom1irq) module"]
pub type IOM1IRQ = crate::Reg<u32, _IOM1IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOM1IRQ;
#[doc = "`read()` method returns [iom1irq::R](iom1irq::R) reader structure"]
impl crate::Readable for IOM1IRQ {}
#[doc = "`write(|w| ..)` method takes [iom1irq::W](iom1irq::W) writer structure"]
impl crate::Writable for IOM1IRQ {}
#[doc = "IOM1 Flow Control IRQ Select"]
pub mod iom1irq;
#[doc = "IOM2 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom2irq](iom2irq) module"]
pub type IOM2IRQ = crate::Reg<u32, _IOM2IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOM2IRQ;
#[doc = "`read()` method returns [iom2irq::R](iom2irq::R) reader structure"]
impl crate::Readable for IOM2IRQ {}
#[doc = "`write(|w| ..)` method takes [iom2irq::W](iom2irq::W) writer structure"]
impl crate::Writable for IOM2IRQ {}
#[doc = "IOM2 Flow Control IRQ Select"]
pub mod iom2irq;
#[doc = "IOM3 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom3irq](iom3irq) module"]
pub type IOM3IRQ = crate::Reg<u32, _IOM3IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOM3IRQ;
#[doc = "`read()` method returns [iom3irq::R](iom3irq::R) reader structure"]
impl crate::Readable for IOM3IRQ {}
#[doc = "`write(|w| ..)` method takes [iom3irq::W](iom3irq::W) writer structure"]
impl crate::Writable for IOM3IRQ {}
#[doc = "IOM3 Flow Control IRQ Select"]
pub mod iom3irq;
#[doc = "IOM4 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom4irq](iom4irq) module"]
pub type IOM4IRQ = crate::Reg<u32, _IOM4IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOM4IRQ;
#[doc = "`read()` method returns [iom4irq::R](iom4irq::R) reader structure"]
impl crate::Readable for IOM4IRQ {}
#[doc = "`write(|w| ..)` method takes [iom4irq::W](iom4irq::W) writer structure"]
impl crate::Writable for IOM4IRQ {}
#[doc = "IOM4 Flow Control IRQ Select"]
pub mod iom4irq;
#[doc = "IOM5 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom5irq](iom5irq) module"]
pub type IOM5IRQ = crate::Reg<u32, _IOM5IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOM5IRQ;
#[doc = "`read()` method returns [iom5irq::R](iom5irq::R) reader structure"]
impl crate::Readable for IOM5IRQ {}
#[doc = "`write(|w| ..)` method takes [iom5irq::W](iom5irq::W) writer structure"]
impl crate::Writable for IOM5IRQ {}
#[doc = "IOM5 Flow Control IRQ Select"]
pub mod iom5irq;
#[doc = "BLEIF Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bleifirq](bleifirq) module"]
pub type BLEIFIRQ = crate::Reg<u32, _BLEIFIRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLEIFIRQ;
#[doc = "`read()` method returns [bleifirq::R](bleifirq::R) reader structure"]
impl crate::Readable for BLEIFIRQ {}
#[doc = "`write(|w| ..)` method takes [bleifirq::W](bleifirq::W) writer structure"]
impl crate::Writable for BLEIFIRQ {}
#[doc = "BLEIF Flow Control IRQ Select"]
pub mod bleifirq;
#[doc = "GPIO Observation Mode Sample register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioobs](gpioobs) module"]
pub type GPIOOBS = crate::Reg<u32, _GPIOOBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOOBS;
#[doc = "`read()` method returns [gpioobs::R](gpioobs::R) reader structure"]
impl crate::Readable for GPIOOBS {}
#[doc = "`write(|w| ..)` method takes [gpioobs::W](gpioobs::W) writer structure"]
impl crate::Writable for GPIOOBS {}
#[doc = "GPIO Observation Mode Sample register"]
pub mod gpioobs;
#[doc = "Alternate Pad Configuration reg0 (Pads 3,2,1,0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfga](altpadcfga) module"]
pub type ALTPADCFGA = crate::Reg<u32, _ALTPADCFGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGA;
#[doc = "`read()` method returns [altpadcfga::R](altpadcfga::R) reader structure"]
impl crate::Readable for ALTPADCFGA {}
#[doc = "`write(|w| ..)` method takes [altpadcfga::W](altpadcfga::W) writer structure"]
impl crate::Writable for ALTPADCFGA {}
#[doc = "Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
pub mod altpadcfga;
#[doc = "Alternate Pad Configuration reg1 (Pads 7,6,5,4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgb](altpadcfgb) module"]
pub type ALTPADCFGB = crate::Reg<u32, _ALTPADCFGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGB;
#[doc = "`read()` method returns [altpadcfgb::R](altpadcfgb::R) reader structure"]
impl crate::Readable for ALTPADCFGB {}
#[doc = "`write(|w| ..)` method takes [altpadcfgb::W](altpadcfgb::W) writer structure"]
impl crate::Writable for ALTPADCFGB {}
#[doc = "Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
pub mod altpadcfgb;
#[doc = "Alternate Pad Configuration reg2 (Pads 11,10,9,8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgc](altpadcfgc) module"]
pub type ALTPADCFGC = crate::Reg<u32, _ALTPADCFGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGC;
#[doc = "`read()` method returns [altpadcfgc::R](altpadcfgc::R) reader structure"]
impl crate::Readable for ALTPADCFGC {}
#[doc = "`write(|w| ..)` method takes [altpadcfgc::W](altpadcfgc::W) writer structure"]
impl crate::Writable for ALTPADCFGC {}
#[doc = "Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
pub mod altpadcfgc;
#[doc = "Alternate Pad Configuration reg3 (Pads 15,14,13,12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgd](altpadcfgd) module"]
pub type ALTPADCFGD = crate::Reg<u32, _ALTPADCFGD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGD;
#[doc = "`read()` method returns [altpadcfgd::R](altpadcfgd::R) reader structure"]
impl crate::Readable for ALTPADCFGD {}
#[doc = "`write(|w| ..)` method takes [altpadcfgd::W](altpadcfgd::W) writer structure"]
impl crate::Writable for ALTPADCFGD {}
#[doc = "Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
pub mod altpadcfgd;
#[doc = "Alternate Pad Configuration reg4 (Pads 19,18,17,16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfge](altpadcfge) module"]
pub type ALTPADCFGE = crate::Reg<u32, _ALTPADCFGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGE;
#[doc = "`read()` method returns [altpadcfge::R](altpadcfge::R) reader structure"]
impl crate::Readable for ALTPADCFGE {}
#[doc = "`write(|w| ..)` method takes [altpadcfge::W](altpadcfge::W) writer structure"]
impl crate::Writable for ALTPADCFGE {}
#[doc = "Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
pub mod altpadcfge;
#[doc = "Alternate Pad Configuration reg5 (Pads 23,22,21,20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgf](altpadcfgf) module"]
pub type ALTPADCFGF = crate::Reg<u32, _ALTPADCFGF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGF;
#[doc = "`read()` method returns [altpadcfgf::R](altpadcfgf::R) reader structure"]
impl crate::Readable for ALTPADCFGF {}
#[doc = "`write(|w| ..)` method takes [altpadcfgf::W](altpadcfgf::W) writer structure"]
impl crate::Writable for ALTPADCFGF {}
#[doc = "Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
pub mod altpadcfgf;
#[doc = "Alternate Pad Configuration reg6 (Pads 27,26,25,24)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgg](altpadcfgg) module"]
pub type ALTPADCFGG = crate::Reg<u32, _ALTPADCFGG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGG;
#[doc = "`read()` method returns [altpadcfgg::R](altpadcfgg::R) reader structure"]
impl crate::Readable for ALTPADCFGG {}
#[doc = "`write(|w| ..)` method takes [altpadcfgg::W](altpadcfgg::W) writer structure"]
impl crate::Writable for ALTPADCFGG {}
#[doc = "Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
pub mod altpadcfgg;
#[doc = "Alternate Pad Configuration reg7 (Pads 31,30,29,28)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgh](altpadcfgh) module"]
pub type ALTPADCFGH = crate::Reg<u32, _ALTPADCFGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGH;
#[doc = "`read()` method returns [altpadcfgh::R](altpadcfgh::R) reader structure"]
impl crate::Readable for ALTPADCFGH {}
#[doc = "`write(|w| ..)` method takes [altpadcfgh::W](altpadcfgh::W) writer structure"]
impl crate::Writable for ALTPADCFGH {}
#[doc = "Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
pub mod altpadcfgh;
#[doc = "Alternate Pad Configuration reg8 (Pads 35,34,33,32)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgi](altpadcfgi) module"]
pub type ALTPADCFGI = crate::Reg<u32, _ALTPADCFGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGI;
#[doc = "`read()` method returns [altpadcfgi::R](altpadcfgi::R) reader structure"]
impl crate::Readable for ALTPADCFGI {}
#[doc = "`write(|w| ..)` method takes [altpadcfgi::W](altpadcfgi::W) writer structure"]
impl crate::Writable for ALTPADCFGI {}
#[doc = "Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
pub mod altpadcfgi;
#[doc = "Alternate Pad Configuration reg9 (Pads 39,38,37,36)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgj](altpadcfgj) module"]
pub type ALTPADCFGJ = crate::Reg<u32, _ALTPADCFGJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGJ;
#[doc = "`read()` method returns [altpadcfgj::R](altpadcfgj::R) reader structure"]
impl crate::Readable for ALTPADCFGJ {}
#[doc = "`write(|w| ..)` method takes [altpadcfgj::W](altpadcfgj::W) writer structure"]
impl crate::Writable for ALTPADCFGJ {}
#[doc = "Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
pub mod altpadcfgj;
#[doc = "Alternate Pad Configuration reg10 (Pads 43,42,41,40)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgk](altpadcfgk) module"]
pub type ALTPADCFGK = crate::Reg<u32, _ALTPADCFGK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGK;
#[doc = "`read()` method returns [altpadcfgk::R](altpadcfgk::R) reader structure"]
impl crate::Readable for ALTPADCFGK {}
#[doc = "`write(|w| ..)` method takes [altpadcfgk::W](altpadcfgk::W) writer structure"]
impl crate::Writable for ALTPADCFGK {}
#[doc = "Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
pub mod altpadcfgk;
#[doc = "Alternate Pad Configuration reg11 (Pads 47,46,45,44)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgl](altpadcfgl) module"]
pub type ALTPADCFGL = crate::Reg<u32, _ALTPADCFGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGL;
#[doc = "`read()` method returns [altpadcfgl::R](altpadcfgl::R) reader structure"]
impl crate::Readable for ALTPADCFGL {}
#[doc = "`write(|w| ..)` method takes [altpadcfgl::W](altpadcfgl::W) writer structure"]
impl crate::Writable for ALTPADCFGL {}
#[doc = "Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
pub mod altpadcfgl;
#[doc = "Alternate Pad Configuration reg12 (Pads 49,48)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgm](altpadcfgm) module"]
pub type ALTPADCFGM = crate::Reg<u32, _ALTPADCFGM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTPADCFGM;
#[doc = "`read()` method returns [altpadcfgm::R](altpadcfgm::R) reader structure"]
impl crate::Readable for ALTPADCFGM {}
#[doc = "`write(|w| ..)` method takes [altpadcfgm::W](altpadcfgm::W) writer structure"]
impl crate::Writable for ALTPADCFGM {}
#[doc = "Alternate Pad Configuration reg12 (Pads 49,48)"]
pub mod altpadcfgm;
#[doc = "SCARD Card Detect select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scdet](scdet) module"]
pub type SCDET = crate::Reg<u32, _SCDET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCDET;
#[doc = "`read()` method returns [scdet::R](scdet::R) reader structure"]
impl crate::Readable for SCDET {}
#[doc = "`write(|w| ..)` method takes [scdet::W](scdet::W) writer structure"]
impl crate::Writable for SCDET {}
#[doc = "SCARD Card Detect select"]
pub mod scdet;
#[doc = "Counter/Timer Enable Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctencfg](ctencfg) module"]
pub type CTENCFG = crate::Reg<u32, _CTENCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTENCFG;
#[doc = "`read()` method returns [ctencfg::R](ctencfg::R) reader structure"]
impl crate::Readable for CTENCFG {}
#[doc = "`write(|w| ..)` method takes [ctencfg::W](ctencfg::W) writer structure"]
impl crate::Writable for CTENCFG {}
#[doc = "Counter/Timer Enable Config"]
pub mod ctencfg;
#[doc = "GPIO Interrupt Registers 31-0: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0en](int0en) module"]
pub type INT0EN = crate::Reg<u32, _INT0EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT0EN;
#[doc = "`read()` method returns [int0en::R](int0en::R) reader structure"]
impl crate::Readable for INT0EN {}
#[doc = "`write(|w| ..)` method takes [int0en::W](int0en::W) writer structure"]
impl crate::Writable for INT0EN {}
#[doc = "GPIO Interrupt Registers 31-0: Enable"]
pub mod int0en;
#[doc = "GPIO Interrupt Registers 31-0: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0stat](int0stat) module"]
pub type INT0STAT = crate::Reg<u32, _INT0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT0STAT;
#[doc = "`read()` method returns [int0stat::R](int0stat::R) reader structure"]
impl crate::Readable for INT0STAT {}
#[doc = "`write(|w| ..)` method takes [int0stat::W](int0stat::W) writer structure"]
impl crate::Writable for INT0STAT {}
#[doc = "GPIO Interrupt Registers 31-0: Status"]
pub mod int0stat;
#[doc = "GPIO Interrupt Registers 31-0: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0clr](int0clr) module"]
pub type INT0CLR = crate::Reg<u32, _INT0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT0CLR;
#[doc = "`read()` method returns [int0clr::R](int0clr::R) reader structure"]
impl crate::Readable for INT0CLR {}
#[doc = "`write(|w| ..)` method takes [int0clr::W](int0clr::W) writer structure"]
impl crate::Writable for INT0CLR {}
#[doc = "GPIO Interrupt Registers 31-0: Clear"]
pub mod int0clr;
#[doc = "GPIO Interrupt Registers 31-0: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0set](int0set) module"]
pub type INT0SET = crate::Reg<u32, _INT0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT0SET;
#[doc = "`read()` method returns [int0set::R](int0set::R) reader structure"]
impl crate::Readable for INT0SET {}
#[doc = "`write(|w| ..)` method takes [int0set::W](int0set::W) writer structure"]
impl crate::Writable for INT0SET {}
#[doc = "GPIO Interrupt Registers 31-0: Set"]
pub mod int0set;
#[doc = "GPIO Interrupt Registers 49-32: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1en](int1en) module"]
pub type INT1EN = crate::Reg<u32, _INT1EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1EN;
#[doc = "`read()` method returns [int1en::R](int1en::R) reader structure"]
impl crate::Readable for INT1EN {}
#[doc = "`write(|w| ..)` method takes [int1en::W](int1en::W) writer structure"]
impl crate::Writable for INT1EN {}
#[doc = "GPIO Interrupt Registers 49-32: Enable"]
pub mod int1en;
#[doc = "GPIO Interrupt Registers 49-32: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1stat](int1stat) module"]
pub type INT1STAT = crate::Reg<u32, _INT1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1STAT;
#[doc = "`read()` method returns [int1stat::R](int1stat::R) reader structure"]
impl crate::Readable for INT1STAT {}
#[doc = "`write(|w| ..)` method takes [int1stat::W](int1stat::W) writer structure"]
impl crate::Writable for INT1STAT {}
#[doc = "GPIO Interrupt Registers 49-32: Status"]
pub mod int1stat;
#[doc = "GPIO Interrupt Registers 49-32: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1clr](int1clr) module"]
pub type INT1CLR = crate::Reg<u32, _INT1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1CLR;
#[doc = "`read()` method returns [int1clr::R](int1clr::R) reader structure"]
impl crate::Readable for INT1CLR {}
#[doc = "`write(|w| ..)` method takes [int1clr::W](int1clr::W) writer structure"]
impl crate::Writable for INT1CLR {}
#[doc = "GPIO Interrupt Registers 49-32: Clear"]
pub mod int1clr;
#[doc = "GPIO Interrupt Registers 49-32: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1set](int1set) module"]
pub type INT1SET = crate::Reg<u32, _INT1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1SET;
#[doc = "`read()` method returns [int1set::R](int1set::R) reader structure"]
impl crate::Readable for INT1SET {}
#[doc = "`write(|w| ..)` method takes [int1set::W](int1set::W) writer structure"]
impl crate::Writable for INT1SET {}
#[doc = "GPIO Interrupt Registers 49-32: Set"]
pub mod int1set;
