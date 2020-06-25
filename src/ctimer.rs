#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Counter/Timer Register"]
    pub tmr0: TMR0,
    #[doc = "0x04 - Counter/Timer A0 Compare Registers"]
    pub cmpra0: CMPRA0,
    #[doc = "0x08 - Counter/Timer B0 Compare Registers"]
    pub cmprb0: CMPRB0,
    #[doc = "0x0c - Counter/Timer Control"]
    pub ctrl0: CTRL0,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - Counter/Timer A0 Compare Registers"]
    pub cmprauxa0: CMPRAUXA0,
    #[doc = "0x18 - Counter/Timer B0 Compare Registers"]
    pub cmprauxb0: CMPRAUXB0,
    #[doc = "0x1c - Counter/Timer Auxiliary"]
    pub aux0: AUX0,
    #[doc = "0x20 - Counter/Timer Register"]
    pub tmr1: TMR1,
    #[doc = "0x24 - Counter/Timer A1 Compare Registers"]
    pub cmpra1: CMPRA1,
    #[doc = "0x28 - Counter/Timer B1 Compare Registers"]
    pub cmprb1: CMPRB1,
    #[doc = "0x2c - Counter/Timer Control"]
    pub ctrl1: CTRL1,
    _reserved11: [u8; 4usize],
    #[doc = "0x34 - Counter/Timer A1 Compare Registers"]
    pub cmprauxa1: CMPRAUXA1,
    #[doc = "0x38 - Counter/Timer B1 Compare Registers"]
    pub cmprauxb1: CMPRAUXB1,
    #[doc = "0x3c - Counter/Timer Auxiliary"]
    pub aux1: AUX1,
    #[doc = "0x40 - Counter/Timer Register"]
    pub tmr2: TMR2,
    #[doc = "0x44 - Counter/Timer A2 Compare Registers"]
    pub cmpra2: CMPRA2,
    #[doc = "0x48 - Counter/Timer B2 Compare Registers"]
    pub cmprb2: CMPRB2,
    #[doc = "0x4c - Counter/Timer Control"]
    pub ctrl2: CTRL2,
    _reserved18: [u8; 4usize],
    #[doc = "0x54 - Counter/Timer A2 Compare Registers"]
    pub cmprauxa2: CMPRAUXA2,
    #[doc = "0x58 - Counter/Timer B2 Compare Registers"]
    pub cmprauxb2: CMPRAUXB2,
    #[doc = "0x5c - Counter/Timer Auxiliary"]
    pub aux2: AUX2,
    #[doc = "0x60 - Counter/Timer Register"]
    pub tmr3: TMR3,
    #[doc = "0x64 - Counter/Timer A3 Compare Registers"]
    pub cmpra3: CMPRA3,
    #[doc = "0x68 - Counter/Timer B3 Compare Registers"]
    pub cmprb3: CMPRB3,
    #[doc = "0x6c - Counter/Timer Control"]
    pub ctrl3: CTRL3,
    _reserved25: [u8; 4usize],
    #[doc = "0x74 - Counter/Timer A3 Compare Registers"]
    pub cmprauxa3: CMPRAUXA3,
    #[doc = "0x78 - Counter/Timer B3 Compare Registers"]
    pub cmprauxb3: CMPRAUXB3,
    #[doc = "0x7c - Counter/Timer Auxiliary"]
    pub aux3: AUX3,
    #[doc = "0x80 - Counter/Timer Register"]
    pub tmr4: TMR4,
    #[doc = "0x84 - Counter/Timer A4 Compare Registers"]
    pub cmpra4: CMPRA4,
    #[doc = "0x88 - Counter/Timer B4 Compare Registers"]
    pub cmprb4: CMPRB4,
    #[doc = "0x8c - Counter/Timer Control"]
    pub ctrl4: CTRL4,
    _reserved32: [u8; 4usize],
    #[doc = "0x94 - Counter/Timer A4 Compare Registers"]
    pub cmprauxa4: CMPRAUXA4,
    #[doc = "0x98 - Counter/Timer B4 Compare Registers"]
    pub cmprauxb4: CMPRAUXB4,
    #[doc = "0x9c - Counter/Timer Auxiliary"]
    pub aux4: AUX4,
    #[doc = "0xa0 - Counter/Timer Register"]
    pub tmr5: TMR5,
    #[doc = "0xa4 - Counter/Timer A5 Compare Registers"]
    pub cmpra5: CMPRA5,
    #[doc = "0xa8 - Counter/Timer B5 Compare Registers"]
    pub cmprb5: CMPRB5,
    #[doc = "0xac - Counter/Timer Control"]
    pub ctrl5: CTRL5,
    _reserved39: [u8; 4usize],
    #[doc = "0xb4 - Counter/Timer A5 Compare Registers"]
    pub cmprauxa5: CMPRAUXA5,
    #[doc = "0xb8 - Counter/Timer B5 Compare Registers"]
    pub cmprauxb5: CMPRAUXB5,
    #[doc = "0xbc - Counter/Timer Auxiliary"]
    pub aux5: AUX5,
    #[doc = "0xc0 - Counter/Timer Register"]
    pub tmr6: TMR6,
    #[doc = "0xc4 - Counter/Timer A6 Compare Registers"]
    pub cmpra6: CMPRA6,
    #[doc = "0xc8 - Counter/Timer B6 Compare Registers"]
    pub cmprb6: CMPRB6,
    #[doc = "0xcc - Counter/Timer Control"]
    pub ctrl6: CTRL6,
    _reserved46: [u8; 4usize],
    #[doc = "0xd4 - Counter/Timer A6 Compare Registers"]
    pub cmprauxa6: CMPRAUXA6,
    #[doc = "0xd8 - Counter/Timer B6 Compare Registers"]
    pub cmprauxb6: CMPRAUXB6,
    #[doc = "0xdc - Counter/Timer Auxiliary"]
    pub aux6: AUX6,
    #[doc = "0xe0 - Counter/Timer Register"]
    pub tmr7: TMR7,
    #[doc = "0xe4 - Counter/Timer A7 Compare Registers"]
    pub cmpra7: CMPRA7,
    #[doc = "0xe8 - Counter/Timer B7 Compare Registers"]
    pub cmprb7: CMPRB7,
    #[doc = "0xec - Counter/Timer Control"]
    pub ctrl7: CTRL7,
    _reserved53: [u8; 4usize],
    #[doc = "0xf4 - Counter/Timer A7 Compare Registers"]
    pub cmprauxa7: CMPRAUXA7,
    #[doc = "0xf8 - Counter/Timer B7 Compare Registers"]
    pub cmprauxb7: CMPRAUXB7,
    #[doc = "0xfc - Counter/Timer Auxiliary"]
    pub aux7: AUX7,
    #[doc = "0x100 - Counter/Timer Global Enable"]
    pub globen: GLOBEN,
    #[doc = "0x104 - Counter/Timer Output Config 0"]
    pub outcfg0: OUTCFG0,
    #[doc = "0x108 - Counter/Timer Output Config 1"]
    pub outcfg1: OUTCFG1,
    #[doc = "0x10c - Counter/Timer Output Config 2"]
    pub outcfg2: OUTCFG2,
    _reserved60: [u8; 4usize],
    #[doc = "0x114 - Counter/Timer Output Config 3"]
    pub outcfg3: OUTCFG3,
    #[doc = "0x118 - Counter/Timer Input Config"]
    pub incfg: INCFG,
    _reserved62: [u8; 36usize],
    #[doc = "0x140 - Configuration Register"]
    pub stcfg: STCFG,
    #[doc = "0x144 - System Timer Count Register (Real Time Counter)"]
    pub sttmr: STTMR,
    #[doc = "0x148 - Capture Control Register"]
    pub capturecontrol: CAPTURECONTROL,
    _reserved65: [u8; 4usize],
    #[doc = "0x150 - Compare Register A"]
    pub scmpr0: SCMPR0,
    #[doc = "0x154 - Compare Register B"]
    pub scmpr1: SCMPR1,
    #[doc = "0x158 - Compare Register C"]
    pub scmpr2: SCMPR2,
    #[doc = "0x15c - Compare Register D"]
    pub scmpr3: SCMPR3,
    #[doc = "0x160 - Compare Register E"]
    pub scmpr4: SCMPR4,
    #[doc = "0x164 - Compare Register F"]
    pub scmpr5: SCMPR5,
    #[doc = "0x168 - Compare Register G"]
    pub scmpr6: SCMPR6,
    #[doc = "0x16c - Compare Register H"]
    pub scmpr7: SCMPR7,
    _reserved73: [u8; 112usize],
    #[doc = "0x1e0 - Capture Register A"]
    pub scapt0: SCAPT0,
    #[doc = "0x1e4 - Capture Register B"]
    pub scapt1: SCAPT1,
    #[doc = "0x1e8 - Capture Register C"]
    pub scapt2: SCAPT2,
    #[doc = "0x1ec - Capture Register D"]
    pub scapt3: SCAPT3,
    #[doc = "0x1f0 - System Timer NVRAM_A Register"]
    pub snvr0: SNVR0,
    #[doc = "0x1f4 - System Timer NVRAM_B Register"]
    pub snvr1: SNVR1,
    #[doc = "0x1f8 - System Timer NVRAM_C Register"]
    pub snvr2: SNVR2,
    #[doc = "0x1fc - System Timer NVRAM_D Register"]
    pub snvr3: SNVR3,
    #[doc = "0x200 - Counter/Timer Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - Counter/Timer Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - Counter/Timer Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - Counter/Timer Interrupts: Set"]
    pub intset: INTSET,
    _reserved85: [u8; 240usize],
    #[doc = "0x300 - STIMER Interrupt registers: Enable"]
    pub stminten: STMINTEN,
    #[doc = "0x304 - STIMER Interrupt registers: Status"]
    pub stmintstat: STMINTSTAT,
    #[doc = "0x308 - STIMER Interrupt registers: Clear"]
    pub stmintclr: STMINTCLR,
    #[doc = "0x30c - STIMER Interrupt registers: Set"]
    pub stmintset: STMINTSET,
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr0](tmr0) module"]
pub type TMR0 = crate::Reg<u32, _TMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR0;
#[doc = "`read()` method returns [tmr0::R](tmr0::R) reader structure"]
impl crate::Readable for TMR0 {}
#[doc = "`write(|w| ..)` method takes [tmr0::W](tmr0::W) writer structure"]
impl crate::Writable for TMR0 {}
#[doc = "Counter/Timer Register"]
pub mod tmr0;
#[doc = "Counter/Timer A0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra0](cmpra0) module"]
pub type CMPRA0 = crate::Reg<u32, _CMPRA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA0;
#[doc = "`read()` method returns [cmpra0::R](cmpra0::R) reader structure"]
impl crate::Readable for CMPRA0 {}
#[doc = "`write(|w| ..)` method takes [cmpra0::W](cmpra0::W) writer structure"]
impl crate::Writable for CMPRA0 {}
#[doc = "Counter/Timer A0 Compare Registers"]
pub mod cmpra0;
#[doc = "Counter/Timer B0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb0](cmprb0) module"]
pub type CMPRB0 = crate::Reg<u32, _CMPRB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB0;
#[doc = "`read()` method returns [cmprb0::R](cmprb0::R) reader structure"]
impl crate::Readable for CMPRB0 {}
#[doc = "`write(|w| ..)` method takes [cmprb0::W](cmprb0::W) writer structure"]
impl crate::Writable for CMPRB0 {}
#[doc = "Counter/Timer B0 Compare Registers"]
pub mod cmprb0;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](ctrl0) module"]
pub type CTRL0 = crate::Reg<u32, _CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL0;
#[doc = "`read()` method returns [ctrl0::R](ctrl0::R) reader structure"]
impl crate::Readable for CTRL0 {}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](ctrl0::W) writer structure"]
impl crate::Writable for CTRL0 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl0;
#[doc = "Counter/Timer A0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa0](cmprauxa0) module"]
pub type CMPRAUXA0 = crate::Reg<u32, _CMPRAUXA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXA0;
#[doc = "`read()` method returns [cmprauxa0::R](cmprauxa0::R) reader structure"]
impl crate::Readable for CMPRAUXA0 {}
#[doc = "`write(|w| ..)` method takes [cmprauxa0::W](cmprauxa0::W) writer structure"]
impl crate::Writable for CMPRAUXA0 {}
#[doc = "Counter/Timer A0 Compare Registers"]
pub mod cmprauxa0;
#[doc = "Counter/Timer B0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb0](cmprauxb0) module"]
pub type CMPRAUXB0 = crate::Reg<u32, _CMPRAUXB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXB0;
#[doc = "`read()` method returns [cmprauxb0::R](cmprauxb0::R) reader structure"]
impl crate::Readable for CMPRAUXB0 {}
#[doc = "`write(|w| ..)` method takes [cmprauxb0::W](cmprauxb0::W) writer structure"]
impl crate::Writable for CMPRAUXB0 {}
#[doc = "Counter/Timer B0 Compare Registers"]
pub mod cmprauxb0;
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux0](aux0) module"]
pub type AUX0 = crate::Reg<u32, _AUX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUX0;
#[doc = "`read()` method returns [aux0::R](aux0::R) reader structure"]
impl crate::Readable for AUX0 {}
#[doc = "`write(|w| ..)` method takes [aux0::W](aux0::W) writer structure"]
impl crate::Writable for AUX0 {}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux0;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr1](tmr1) module"]
pub type TMR1 = crate::Reg<u32, _TMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR1;
#[doc = "`read()` method returns [tmr1::R](tmr1::R) reader structure"]
impl crate::Readable for TMR1 {}
#[doc = "`write(|w| ..)` method takes [tmr1::W](tmr1::W) writer structure"]
impl crate::Writable for TMR1 {}
#[doc = "Counter/Timer Register"]
pub mod tmr1;
#[doc = "Counter/Timer A1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra1](cmpra1) module"]
pub type CMPRA1 = crate::Reg<u32, _CMPRA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA1;
#[doc = "`read()` method returns [cmpra1::R](cmpra1::R) reader structure"]
impl crate::Readable for CMPRA1 {}
#[doc = "`write(|w| ..)` method takes [cmpra1::W](cmpra1::W) writer structure"]
impl crate::Writable for CMPRA1 {}
#[doc = "Counter/Timer A1 Compare Registers"]
pub mod cmpra1;
#[doc = "Counter/Timer B1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb1](cmprb1) module"]
pub type CMPRB1 = crate::Reg<u32, _CMPRB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB1;
#[doc = "`read()` method returns [cmprb1::R](cmprb1::R) reader structure"]
impl crate::Readable for CMPRB1 {}
#[doc = "`write(|w| ..)` method takes [cmprb1::W](cmprb1::W) writer structure"]
impl crate::Writable for CMPRB1 {}
#[doc = "Counter/Timer B1 Compare Registers"]
pub mod cmprb1;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl1;
#[doc = "Counter/Timer A1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa1](cmprauxa1) module"]
pub type CMPRAUXA1 = crate::Reg<u32, _CMPRAUXA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXA1;
#[doc = "`read()` method returns [cmprauxa1::R](cmprauxa1::R) reader structure"]
impl crate::Readable for CMPRAUXA1 {}
#[doc = "`write(|w| ..)` method takes [cmprauxa1::W](cmprauxa1::W) writer structure"]
impl crate::Writable for CMPRAUXA1 {}
#[doc = "Counter/Timer A1 Compare Registers"]
pub mod cmprauxa1;
#[doc = "Counter/Timer B1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb1](cmprauxb1) module"]
pub type CMPRAUXB1 = crate::Reg<u32, _CMPRAUXB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXB1;
#[doc = "`read()` method returns [cmprauxb1::R](cmprauxb1::R) reader structure"]
impl crate::Readable for CMPRAUXB1 {}
#[doc = "`write(|w| ..)` method takes [cmprauxb1::W](cmprauxb1::W) writer structure"]
impl crate::Writable for CMPRAUXB1 {}
#[doc = "Counter/Timer B1 Compare Registers"]
pub mod cmprauxb1;
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux1](aux1) module"]
pub type AUX1 = crate::Reg<u32, _AUX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUX1;
#[doc = "`read()` method returns [aux1::R](aux1::R) reader structure"]
impl crate::Readable for AUX1 {}
#[doc = "`write(|w| ..)` method takes [aux1::W](aux1::W) writer structure"]
impl crate::Writable for AUX1 {}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux1;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2](tmr2) module"]
pub type TMR2 = crate::Reg<u32, _TMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2;
#[doc = "`read()` method returns [tmr2::R](tmr2::R) reader structure"]
impl crate::Readable for TMR2 {}
#[doc = "`write(|w| ..)` method takes [tmr2::W](tmr2::W) writer structure"]
impl crate::Writable for TMR2 {}
#[doc = "Counter/Timer Register"]
pub mod tmr2;
#[doc = "Counter/Timer A2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra2](cmpra2) module"]
pub type CMPRA2 = crate::Reg<u32, _CMPRA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA2;
#[doc = "`read()` method returns [cmpra2::R](cmpra2::R) reader structure"]
impl crate::Readable for CMPRA2 {}
#[doc = "`write(|w| ..)` method takes [cmpra2::W](cmpra2::W) writer structure"]
impl crate::Writable for CMPRA2 {}
#[doc = "Counter/Timer A2 Compare Registers"]
pub mod cmpra2;
#[doc = "Counter/Timer B2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb2](cmprb2) module"]
pub type CMPRB2 = crate::Reg<u32, _CMPRB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB2;
#[doc = "`read()` method returns [cmprb2::R](cmprb2::R) reader structure"]
impl crate::Readable for CMPRB2 {}
#[doc = "`write(|w| ..)` method takes [cmprb2::W](cmprb2::W) writer structure"]
impl crate::Writable for CMPRB2 {}
#[doc = "Counter/Timer B2 Compare Registers"]
pub mod cmprb2;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl2;
#[doc = "Counter/Timer A2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa2](cmprauxa2) module"]
pub type CMPRAUXA2 = crate::Reg<u32, _CMPRAUXA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXA2;
#[doc = "`read()` method returns [cmprauxa2::R](cmprauxa2::R) reader structure"]
impl crate::Readable for CMPRAUXA2 {}
#[doc = "`write(|w| ..)` method takes [cmprauxa2::W](cmprauxa2::W) writer structure"]
impl crate::Writable for CMPRAUXA2 {}
#[doc = "Counter/Timer A2 Compare Registers"]
pub mod cmprauxa2;
#[doc = "Counter/Timer B2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb2](cmprauxb2) module"]
pub type CMPRAUXB2 = crate::Reg<u32, _CMPRAUXB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXB2;
#[doc = "`read()` method returns [cmprauxb2::R](cmprauxb2::R) reader structure"]
impl crate::Readable for CMPRAUXB2 {}
#[doc = "`write(|w| ..)` method takes [cmprauxb2::W](cmprauxb2::W) writer structure"]
impl crate::Writable for CMPRAUXB2 {}
#[doc = "Counter/Timer B2 Compare Registers"]
pub mod cmprauxb2;
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux2](aux2) module"]
pub type AUX2 = crate::Reg<u32, _AUX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUX2;
#[doc = "`read()` method returns [aux2::R](aux2::R) reader structure"]
impl crate::Readable for AUX2 {}
#[doc = "`write(|w| ..)` method takes [aux2::W](aux2::W) writer structure"]
impl crate::Writable for AUX2 {}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux2;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3](tmr3) module"]
pub type TMR3 = crate::Reg<u32, _TMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3;
#[doc = "`read()` method returns [tmr3::R](tmr3::R) reader structure"]
impl crate::Readable for TMR3 {}
#[doc = "`write(|w| ..)` method takes [tmr3::W](tmr3::W) writer structure"]
impl crate::Writable for TMR3 {}
#[doc = "Counter/Timer Register"]
pub mod tmr3;
#[doc = "Counter/Timer A3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra3](cmpra3) module"]
pub type CMPRA3 = crate::Reg<u32, _CMPRA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA3;
#[doc = "`read()` method returns [cmpra3::R](cmpra3::R) reader structure"]
impl crate::Readable for CMPRA3 {}
#[doc = "`write(|w| ..)` method takes [cmpra3::W](cmpra3::W) writer structure"]
impl crate::Writable for CMPRA3 {}
#[doc = "Counter/Timer A3 Compare Registers"]
pub mod cmpra3;
#[doc = "Counter/Timer B3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb3](cmprb3) module"]
pub type CMPRB3 = crate::Reg<u32, _CMPRB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB3;
#[doc = "`read()` method returns [cmprb3::R](cmprb3::R) reader structure"]
impl crate::Readable for CMPRB3 {}
#[doc = "`write(|w| ..)` method takes [cmprb3::W](cmprb3::W) writer structure"]
impl crate::Writable for CMPRB3 {}
#[doc = "Counter/Timer B3 Compare Registers"]
pub mod cmprb3;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl3](ctrl3) module"]
pub type CTRL3 = crate::Reg<u32, _CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL3;
#[doc = "`read()` method returns [ctrl3::R](ctrl3::R) reader structure"]
impl crate::Readable for CTRL3 {}
#[doc = "`write(|w| ..)` method takes [ctrl3::W](ctrl3::W) writer structure"]
impl crate::Writable for CTRL3 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl3;
#[doc = "Counter/Timer A3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa3](cmprauxa3) module"]
pub type CMPRAUXA3 = crate::Reg<u32, _CMPRAUXA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXA3;
#[doc = "`read()` method returns [cmprauxa3::R](cmprauxa3::R) reader structure"]
impl crate::Readable for CMPRAUXA3 {}
#[doc = "`write(|w| ..)` method takes [cmprauxa3::W](cmprauxa3::W) writer structure"]
impl crate::Writable for CMPRAUXA3 {}
#[doc = "Counter/Timer A3 Compare Registers"]
pub mod cmprauxa3;
#[doc = "Counter/Timer B3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb3](cmprauxb3) module"]
pub type CMPRAUXB3 = crate::Reg<u32, _CMPRAUXB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXB3;
#[doc = "`read()` method returns [cmprauxb3::R](cmprauxb3::R) reader structure"]
impl crate::Readable for CMPRAUXB3 {}
#[doc = "`write(|w| ..)` method takes [cmprauxb3::W](cmprauxb3::W) writer structure"]
impl crate::Writable for CMPRAUXB3 {}
#[doc = "Counter/Timer B3 Compare Registers"]
pub mod cmprauxb3;
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux3](aux3) module"]
pub type AUX3 = crate::Reg<u32, _AUX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUX3;
#[doc = "`read()` method returns [aux3::R](aux3::R) reader structure"]
impl crate::Readable for AUX3 {}
#[doc = "`write(|w| ..)` method takes [aux3::W](aux3::W) writer structure"]
impl crate::Writable for AUX3 {}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux3;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr4](tmr4) module"]
pub type TMR4 = crate::Reg<u32, _TMR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR4;
#[doc = "`read()` method returns [tmr4::R](tmr4::R) reader structure"]
impl crate::Readable for TMR4 {}
#[doc = "`write(|w| ..)` method takes [tmr4::W](tmr4::W) writer structure"]
impl crate::Writable for TMR4 {}
#[doc = "Counter/Timer Register"]
pub mod tmr4;
#[doc = "Counter/Timer A4 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra4](cmpra4) module"]
pub type CMPRA4 = crate::Reg<u32, _CMPRA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA4;
#[doc = "`read()` method returns [cmpra4::R](cmpra4::R) reader structure"]
impl crate::Readable for CMPRA4 {}
#[doc = "`write(|w| ..)` method takes [cmpra4::W](cmpra4::W) writer structure"]
impl crate::Writable for CMPRA4 {}
#[doc = "Counter/Timer A4 Compare Registers"]
pub mod cmpra4;
#[doc = "Counter/Timer B4 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb4](cmprb4) module"]
pub type CMPRB4 = crate::Reg<u32, _CMPRB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB4;
#[doc = "`read()` method returns [cmprb4::R](cmprb4::R) reader structure"]
impl crate::Readable for CMPRB4 {}
#[doc = "`write(|w| ..)` method takes [cmprb4::W](cmprb4::W) writer structure"]
impl crate::Writable for CMPRB4 {}
#[doc = "Counter/Timer B4 Compare Registers"]
pub mod cmprb4;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl4](ctrl4) module"]
pub type CTRL4 = crate::Reg<u32, _CTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL4;
#[doc = "`read()` method returns [ctrl4::R](ctrl4::R) reader structure"]
impl crate::Readable for CTRL4 {}
#[doc = "`write(|w| ..)` method takes [ctrl4::W](ctrl4::W) writer structure"]
impl crate::Writable for CTRL4 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl4;
#[doc = "Counter/Timer A4 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa4](cmprauxa4) module"]
pub type CMPRAUXA4 = crate::Reg<u32, _CMPRAUXA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXA4;
#[doc = "`read()` method returns [cmprauxa4::R](cmprauxa4::R) reader structure"]
impl crate::Readable for CMPRAUXA4 {}
#[doc = "`write(|w| ..)` method takes [cmprauxa4::W](cmprauxa4::W) writer structure"]
impl crate::Writable for CMPRAUXA4 {}
#[doc = "Counter/Timer A4 Compare Registers"]
pub mod cmprauxa4;
#[doc = "Counter/Timer B4 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb4](cmprauxb4) module"]
pub type CMPRAUXB4 = crate::Reg<u32, _CMPRAUXB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXB4;
#[doc = "`read()` method returns [cmprauxb4::R](cmprauxb4::R) reader structure"]
impl crate::Readable for CMPRAUXB4 {}
#[doc = "`write(|w| ..)` method takes [cmprauxb4::W](cmprauxb4::W) writer structure"]
impl crate::Writable for CMPRAUXB4 {}
#[doc = "Counter/Timer B4 Compare Registers"]
pub mod cmprauxb4;
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux4](aux4) module"]
pub type AUX4 = crate::Reg<u32, _AUX4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUX4;
#[doc = "`read()` method returns [aux4::R](aux4::R) reader structure"]
impl crate::Readable for AUX4 {}
#[doc = "`write(|w| ..)` method takes [aux4::W](aux4::W) writer structure"]
impl crate::Writable for AUX4 {}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux4;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr5](tmr5) module"]
pub type TMR5 = crate::Reg<u32, _TMR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR5;
#[doc = "`read()` method returns [tmr5::R](tmr5::R) reader structure"]
impl crate::Readable for TMR5 {}
#[doc = "`write(|w| ..)` method takes [tmr5::W](tmr5::W) writer structure"]
impl crate::Writable for TMR5 {}
#[doc = "Counter/Timer Register"]
pub mod tmr5;
#[doc = "Counter/Timer A5 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra5](cmpra5) module"]
pub type CMPRA5 = crate::Reg<u32, _CMPRA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA5;
#[doc = "`read()` method returns [cmpra5::R](cmpra5::R) reader structure"]
impl crate::Readable for CMPRA5 {}
#[doc = "`write(|w| ..)` method takes [cmpra5::W](cmpra5::W) writer structure"]
impl crate::Writable for CMPRA5 {}
#[doc = "Counter/Timer A5 Compare Registers"]
pub mod cmpra5;
#[doc = "Counter/Timer B5 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb5](cmprb5) module"]
pub type CMPRB5 = crate::Reg<u32, _CMPRB5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB5;
#[doc = "`read()` method returns [cmprb5::R](cmprb5::R) reader structure"]
impl crate::Readable for CMPRB5 {}
#[doc = "`write(|w| ..)` method takes [cmprb5::W](cmprb5::W) writer structure"]
impl crate::Writable for CMPRB5 {}
#[doc = "Counter/Timer B5 Compare Registers"]
pub mod cmprb5;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl5](ctrl5) module"]
pub type CTRL5 = crate::Reg<u32, _CTRL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL5;
#[doc = "`read()` method returns [ctrl5::R](ctrl5::R) reader structure"]
impl crate::Readable for CTRL5 {}
#[doc = "`write(|w| ..)` method takes [ctrl5::W](ctrl5::W) writer structure"]
impl crate::Writable for CTRL5 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl5;
#[doc = "Counter/Timer A5 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa5](cmprauxa5) module"]
pub type CMPRAUXA5 = crate::Reg<u32, _CMPRAUXA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXA5;
#[doc = "`read()` method returns [cmprauxa5::R](cmprauxa5::R) reader structure"]
impl crate::Readable for CMPRAUXA5 {}
#[doc = "`write(|w| ..)` method takes [cmprauxa5::W](cmprauxa5::W) writer structure"]
impl crate::Writable for CMPRAUXA5 {}
#[doc = "Counter/Timer A5 Compare Registers"]
pub mod cmprauxa5;
#[doc = "Counter/Timer B5 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb5](cmprauxb5) module"]
pub type CMPRAUXB5 = crate::Reg<u32, _CMPRAUXB5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXB5;
#[doc = "`read()` method returns [cmprauxb5::R](cmprauxb5::R) reader structure"]
impl crate::Readable for CMPRAUXB5 {}
#[doc = "`write(|w| ..)` method takes [cmprauxb5::W](cmprauxb5::W) writer structure"]
impl crate::Writable for CMPRAUXB5 {}
#[doc = "Counter/Timer B5 Compare Registers"]
pub mod cmprauxb5;
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux5](aux5) module"]
pub type AUX5 = crate::Reg<u32, _AUX5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUX5;
#[doc = "`read()` method returns [aux5::R](aux5::R) reader structure"]
impl crate::Readable for AUX5 {}
#[doc = "`write(|w| ..)` method takes [aux5::W](aux5::W) writer structure"]
impl crate::Writable for AUX5 {}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux5;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr6](tmr6) module"]
pub type TMR6 = crate::Reg<u32, _TMR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR6;
#[doc = "`read()` method returns [tmr6::R](tmr6::R) reader structure"]
impl crate::Readable for TMR6 {}
#[doc = "`write(|w| ..)` method takes [tmr6::W](tmr6::W) writer structure"]
impl crate::Writable for TMR6 {}
#[doc = "Counter/Timer Register"]
pub mod tmr6;
#[doc = "Counter/Timer A6 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra6](cmpra6) module"]
pub type CMPRA6 = crate::Reg<u32, _CMPRA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA6;
#[doc = "`read()` method returns [cmpra6::R](cmpra6::R) reader structure"]
impl crate::Readable for CMPRA6 {}
#[doc = "`write(|w| ..)` method takes [cmpra6::W](cmpra6::W) writer structure"]
impl crate::Writable for CMPRA6 {}
#[doc = "Counter/Timer A6 Compare Registers"]
pub mod cmpra6;
#[doc = "Counter/Timer B6 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb6](cmprb6) module"]
pub type CMPRB6 = crate::Reg<u32, _CMPRB6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB6;
#[doc = "`read()` method returns [cmprb6::R](cmprb6::R) reader structure"]
impl crate::Readable for CMPRB6 {}
#[doc = "`write(|w| ..)` method takes [cmprb6::W](cmprb6::W) writer structure"]
impl crate::Writable for CMPRB6 {}
#[doc = "Counter/Timer B6 Compare Registers"]
pub mod cmprb6;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl6](ctrl6) module"]
pub type CTRL6 = crate::Reg<u32, _CTRL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL6;
#[doc = "`read()` method returns [ctrl6::R](ctrl6::R) reader structure"]
impl crate::Readable for CTRL6 {}
#[doc = "`write(|w| ..)` method takes [ctrl6::W](ctrl6::W) writer structure"]
impl crate::Writable for CTRL6 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl6;
#[doc = "Counter/Timer A6 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa6](cmprauxa6) module"]
pub type CMPRAUXA6 = crate::Reg<u32, _CMPRAUXA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXA6;
#[doc = "`read()` method returns [cmprauxa6::R](cmprauxa6::R) reader structure"]
impl crate::Readable for CMPRAUXA6 {}
#[doc = "`write(|w| ..)` method takes [cmprauxa6::W](cmprauxa6::W) writer structure"]
impl crate::Writable for CMPRAUXA6 {}
#[doc = "Counter/Timer A6 Compare Registers"]
pub mod cmprauxa6;
#[doc = "Counter/Timer B6 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb6](cmprauxb6) module"]
pub type CMPRAUXB6 = crate::Reg<u32, _CMPRAUXB6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXB6;
#[doc = "`read()` method returns [cmprauxb6::R](cmprauxb6::R) reader structure"]
impl crate::Readable for CMPRAUXB6 {}
#[doc = "`write(|w| ..)` method takes [cmprauxb6::W](cmprauxb6::W) writer structure"]
impl crate::Writable for CMPRAUXB6 {}
#[doc = "Counter/Timer B6 Compare Registers"]
pub mod cmprauxb6;
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux6](aux6) module"]
pub type AUX6 = crate::Reg<u32, _AUX6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUX6;
#[doc = "`read()` method returns [aux6::R](aux6::R) reader structure"]
impl crate::Readable for AUX6 {}
#[doc = "`write(|w| ..)` method takes [aux6::W](aux6::W) writer structure"]
impl crate::Writable for AUX6 {}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux6;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr7](tmr7) module"]
pub type TMR7 = crate::Reg<u32, _TMR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR7;
#[doc = "`read()` method returns [tmr7::R](tmr7::R) reader structure"]
impl crate::Readable for TMR7 {}
#[doc = "`write(|w| ..)` method takes [tmr7::W](tmr7::W) writer structure"]
impl crate::Writable for TMR7 {}
#[doc = "Counter/Timer Register"]
pub mod tmr7;
#[doc = "Counter/Timer A7 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra7](cmpra7) module"]
pub type CMPRA7 = crate::Reg<u32, _CMPRA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA7;
#[doc = "`read()` method returns [cmpra7::R](cmpra7::R) reader structure"]
impl crate::Readable for CMPRA7 {}
#[doc = "`write(|w| ..)` method takes [cmpra7::W](cmpra7::W) writer structure"]
impl crate::Writable for CMPRA7 {}
#[doc = "Counter/Timer A7 Compare Registers"]
pub mod cmpra7;
#[doc = "Counter/Timer B7 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb7](cmprb7) module"]
pub type CMPRB7 = crate::Reg<u32, _CMPRB7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB7;
#[doc = "`read()` method returns [cmprb7::R](cmprb7::R) reader structure"]
impl crate::Readable for CMPRB7 {}
#[doc = "`write(|w| ..)` method takes [cmprb7::W](cmprb7::W) writer structure"]
impl crate::Writable for CMPRB7 {}
#[doc = "Counter/Timer B7 Compare Registers"]
pub mod cmprb7;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl7](ctrl7) module"]
pub type CTRL7 = crate::Reg<u32, _CTRL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL7;
#[doc = "`read()` method returns [ctrl7::R](ctrl7::R) reader structure"]
impl crate::Readable for CTRL7 {}
#[doc = "`write(|w| ..)` method takes [ctrl7::W](ctrl7::W) writer structure"]
impl crate::Writable for CTRL7 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl7;
#[doc = "Counter/Timer A7 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa7](cmprauxa7) module"]
pub type CMPRAUXA7 = crate::Reg<u32, _CMPRAUXA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXA7;
#[doc = "`read()` method returns [cmprauxa7::R](cmprauxa7::R) reader structure"]
impl crate::Readable for CMPRAUXA7 {}
#[doc = "`write(|w| ..)` method takes [cmprauxa7::W](cmprauxa7::W) writer structure"]
impl crate::Writable for CMPRAUXA7 {}
#[doc = "Counter/Timer A7 Compare Registers"]
pub mod cmprauxa7;
#[doc = "Counter/Timer B7 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb7](cmprauxb7) module"]
pub type CMPRAUXB7 = crate::Reg<u32, _CMPRAUXB7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRAUXB7;
#[doc = "`read()` method returns [cmprauxb7::R](cmprauxb7::R) reader structure"]
impl crate::Readable for CMPRAUXB7 {}
#[doc = "`write(|w| ..)` method takes [cmprauxb7::W](cmprauxb7::W) writer structure"]
impl crate::Writable for CMPRAUXB7 {}
#[doc = "Counter/Timer B7 Compare Registers"]
pub mod cmprauxb7;
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux7](aux7) module"]
pub type AUX7 = crate::Reg<u32, _AUX7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUX7;
#[doc = "`read()` method returns [aux7::R](aux7::R) reader structure"]
impl crate::Readable for AUX7 {}
#[doc = "`write(|w| ..)` method takes [aux7::W](aux7::W) writer structure"]
impl crate::Writable for AUX7 {}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux7;
#[doc = "Counter/Timer Global Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globen](globen) module"]
pub type GLOBEN = crate::Reg<u32, _GLOBEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBEN;
#[doc = "`read()` method returns [globen::R](globen::R) reader structure"]
impl crate::Readable for GLOBEN {}
#[doc = "`write(|w| ..)` method takes [globen::W](globen::W) writer structure"]
impl crate::Writable for GLOBEN {}
#[doc = "Counter/Timer Global Enable"]
pub mod globen;
#[doc = "Counter/Timer Output Config 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcfg0](outcfg0) module"]
pub type OUTCFG0 = crate::Reg<u32, _OUTCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCFG0;
#[doc = "`read()` method returns [outcfg0::R](outcfg0::R) reader structure"]
impl crate::Readable for OUTCFG0 {}
#[doc = "`write(|w| ..)` method takes [outcfg0::W](outcfg0::W) writer structure"]
impl crate::Writable for OUTCFG0 {}
#[doc = "Counter/Timer Output Config 0"]
pub mod outcfg0;
#[doc = "Counter/Timer Output Config 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcfg1](outcfg1) module"]
pub type OUTCFG1 = crate::Reg<u32, _OUTCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCFG1;
#[doc = "`read()` method returns [outcfg1::R](outcfg1::R) reader structure"]
impl crate::Readable for OUTCFG1 {}
#[doc = "`write(|w| ..)` method takes [outcfg1::W](outcfg1::W) writer structure"]
impl crate::Writable for OUTCFG1 {}
#[doc = "Counter/Timer Output Config 1"]
pub mod outcfg1;
#[doc = "Counter/Timer Output Config 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcfg2](outcfg2) module"]
pub type OUTCFG2 = crate::Reg<u32, _OUTCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCFG2;
#[doc = "`read()` method returns [outcfg2::R](outcfg2::R) reader structure"]
impl crate::Readable for OUTCFG2 {}
#[doc = "`write(|w| ..)` method takes [outcfg2::W](outcfg2::W) writer structure"]
impl crate::Writable for OUTCFG2 {}
#[doc = "Counter/Timer Output Config 2"]
pub mod outcfg2;
#[doc = "Counter/Timer Output Config 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcfg3](outcfg3) module"]
pub type OUTCFG3 = crate::Reg<u32, _OUTCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCFG3;
#[doc = "`read()` method returns [outcfg3::R](outcfg3::R) reader structure"]
impl crate::Readable for OUTCFG3 {}
#[doc = "`write(|w| ..)` method takes [outcfg3::W](outcfg3::W) writer structure"]
impl crate::Writable for OUTCFG3 {}
#[doc = "Counter/Timer Output Config 3"]
pub mod outcfg3;
#[doc = "Counter/Timer Input Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [incfg](incfg) module"]
pub type INCFG = crate::Reg<u32, _INCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INCFG;
#[doc = "`read()` method returns [incfg::R](incfg::R) reader structure"]
impl crate::Readable for INCFG {}
#[doc = "`write(|w| ..)` method takes [incfg::W](incfg::W) writer structure"]
impl crate::Writable for INCFG {}
#[doc = "Counter/Timer Input Config"]
pub mod incfg;
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcfg](stcfg) module"]
pub type STCFG = crate::Reg<u32, _STCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCFG;
#[doc = "`read()` method returns [stcfg::R](stcfg::R) reader structure"]
impl crate::Readable for STCFG {}
#[doc = "`write(|w| ..)` method takes [stcfg::W](stcfg::W) writer structure"]
impl crate::Writable for STCFG {}
#[doc = "Configuration Register"]
pub mod stcfg;
#[doc = "System Timer Count Register (Real Time Counter)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sttmr](sttmr) module"]
pub type STTMR = crate::Reg<u32, _STTMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STTMR;
#[doc = "`read()` method returns [sttmr::R](sttmr::R) reader structure"]
impl crate::Readable for STTMR {}
#[doc = "`write(|w| ..)` method takes [sttmr::W](sttmr::W) writer structure"]
impl crate::Writable for STTMR {}
#[doc = "System Timer Count Register (Real Time Counter)"]
pub mod sttmr;
#[doc = "Capture Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capturecontrol](capturecontrol) module"]
pub type CAPTURECONTROL = crate::Reg<u32, _CAPTURECONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTURECONTROL;
#[doc = "`read()` method returns [capturecontrol::R](capturecontrol::R) reader structure"]
impl crate::Readable for CAPTURECONTROL {}
#[doc = "`write(|w| ..)` method takes [capturecontrol::W](capturecontrol::W) writer structure"]
impl crate::Writable for CAPTURECONTROL {}
#[doc = "Capture Control Register"]
pub mod capturecontrol;
#[doc = "Compare Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr0](scmpr0) module"]
pub type SCMPR0 = crate::Reg<u32, _SCMPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR0;
#[doc = "`read()` method returns [scmpr0::R](scmpr0::R) reader structure"]
impl crate::Readable for SCMPR0 {}
#[doc = "`write(|w| ..)` method takes [scmpr0::W](scmpr0::W) writer structure"]
impl crate::Writable for SCMPR0 {}
#[doc = "Compare Register A"]
pub mod scmpr0;
#[doc = "Compare Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr1](scmpr1) module"]
pub type SCMPR1 = crate::Reg<u32, _SCMPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR1;
#[doc = "`read()` method returns [scmpr1::R](scmpr1::R) reader structure"]
impl crate::Readable for SCMPR1 {}
#[doc = "`write(|w| ..)` method takes [scmpr1::W](scmpr1::W) writer structure"]
impl crate::Writable for SCMPR1 {}
#[doc = "Compare Register B"]
pub mod scmpr1;
#[doc = "Compare Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr2](scmpr2) module"]
pub type SCMPR2 = crate::Reg<u32, _SCMPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR2;
#[doc = "`read()` method returns [scmpr2::R](scmpr2::R) reader structure"]
impl crate::Readable for SCMPR2 {}
#[doc = "`write(|w| ..)` method takes [scmpr2::W](scmpr2::W) writer structure"]
impl crate::Writable for SCMPR2 {}
#[doc = "Compare Register C"]
pub mod scmpr2;
#[doc = "Compare Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr3](scmpr3) module"]
pub type SCMPR3 = crate::Reg<u32, _SCMPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR3;
#[doc = "`read()` method returns [scmpr3::R](scmpr3::R) reader structure"]
impl crate::Readable for SCMPR3 {}
#[doc = "`write(|w| ..)` method takes [scmpr3::W](scmpr3::W) writer structure"]
impl crate::Writable for SCMPR3 {}
#[doc = "Compare Register D"]
pub mod scmpr3;
#[doc = "Compare Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr4](scmpr4) module"]
pub type SCMPR4 = crate::Reg<u32, _SCMPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR4;
#[doc = "`read()` method returns [scmpr4::R](scmpr4::R) reader structure"]
impl crate::Readable for SCMPR4 {}
#[doc = "`write(|w| ..)` method takes [scmpr4::W](scmpr4::W) writer structure"]
impl crate::Writable for SCMPR4 {}
#[doc = "Compare Register E"]
pub mod scmpr4;
#[doc = "Compare Register F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr5](scmpr5) module"]
pub type SCMPR5 = crate::Reg<u32, _SCMPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR5;
#[doc = "`read()` method returns [scmpr5::R](scmpr5::R) reader structure"]
impl crate::Readable for SCMPR5 {}
#[doc = "`write(|w| ..)` method takes [scmpr5::W](scmpr5::W) writer structure"]
impl crate::Writable for SCMPR5 {}
#[doc = "Compare Register F"]
pub mod scmpr5;
#[doc = "Compare Register G\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr6](scmpr6) module"]
pub type SCMPR6 = crate::Reg<u32, _SCMPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR6;
#[doc = "`read()` method returns [scmpr6::R](scmpr6::R) reader structure"]
impl crate::Readable for SCMPR6 {}
#[doc = "`write(|w| ..)` method takes [scmpr6::W](scmpr6::W) writer structure"]
impl crate::Writable for SCMPR6 {}
#[doc = "Compare Register G"]
pub mod scmpr6;
#[doc = "Compare Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr7](scmpr7) module"]
pub type SCMPR7 = crate::Reg<u32, _SCMPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR7;
#[doc = "`read()` method returns [scmpr7::R](scmpr7::R) reader structure"]
impl crate::Readable for SCMPR7 {}
#[doc = "`write(|w| ..)` method takes [scmpr7::W](scmpr7::W) writer structure"]
impl crate::Writable for SCMPR7 {}
#[doc = "Compare Register H"]
pub mod scmpr7;
#[doc = "Capture Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt0](scapt0) module"]
pub type SCAPT0 = crate::Reg<u32, _SCAPT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAPT0;
#[doc = "`read()` method returns [scapt0::R](scapt0::R) reader structure"]
impl crate::Readable for SCAPT0 {}
#[doc = "`write(|w| ..)` method takes [scapt0::W](scapt0::W) writer structure"]
impl crate::Writable for SCAPT0 {}
#[doc = "Capture Register A"]
pub mod scapt0;
#[doc = "Capture Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt1](scapt1) module"]
pub type SCAPT1 = crate::Reg<u32, _SCAPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAPT1;
#[doc = "`read()` method returns [scapt1::R](scapt1::R) reader structure"]
impl crate::Readable for SCAPT1 {}
#[doc = "`write(|w| ..)` method takes [scapt1::W](scapt1::W) writer structure"]
impl crate::Writable for SCAPT1 {}
#[doc = "Capture Register B"]
pub mod scapt1;
#[doc = "Capture Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt2](scapt2) module"]
pub type SCAPT2 = crate::Reg<u32, _SCAPT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAPT2;
#[doc = "`read()` method returns [scapt2::R](scapt2::R) reader structure"]
impl crate::Readable for SCAPT2 {}
#[doc = "`write(|w| ..)` method takes [scapt2::W](scapt2::W) writer structure"]
impl crate::Writable for SCAPT2 {}
#[doc = "Capture Register C"]
pub mod scapt2;
#[doc = "Capture Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt3](scapt3) module"]
pub type SCAPT3 = crate::Reg<u32, _SCAPT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAPT3;
#[doc = "`read()` method returns [scapt3::R](scapt3::R) reader structure"]
impl crate::Readable for SCAPT3 {}
#[doc = "`write(|w| ..)` method takes [scapt3::W](scapt3::W) writer structure"]
impl crate::Writable for SCAPT3 {}
#[doc = "Capture Register D"]
pub mod scapt3;
#[doc = "System Timer NVRAM_A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr0](snvr0) module"]
pub type SNVR0 = crate::Reg<u32, _SNVR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNVR0;
#[doc = "`read()` method returns [snvr0::R](snvr0::R) reader structure"]
impl crate::Readable for SNVR0 {}
#[doc = "`write(|w| ..)` method takes [snvr0::W](snvr0::W) writer structure"]
impl crate::Writable for SNVR0 {}
#[doc = "System Timer NVRAM_A Register"]
pub mod snvr0;
#[doc = "System Timer NVRAM_B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr1](snvr1) module"]
pub type SNVR1 = crate::Reg<u32, _SNVR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNVR1;
#[doc = "`read()` method returns [snvr1::R](snvr1::R) reader structure"]
impl crate::Readable for SNVR1 {}
#[doc = "`write(|w| ..)` method takes [snvr1::W](snvr1::W) writer structure"]
impl crate::Writable for SNVR1 {}
#[doc = "System Timer NVRAM_B Register"]
pub mod snvr1;
#[doc = "System Timer NVRAM_C Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr2](snvr2) module"]
pub type SNVR2 = crate::Reg<u32, _SNVR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNVR2;
#[doc = "`read()` method returns [snvr2::R](snvr2::R) reader structure"]
impl crate::Readable for SNVR2 {}
#[doc = "`write(|w| ..)` method takes [snvr2::W](snvr2::W) writer structure"]
impl crate::Writable for SNVR2 {}
#[doc = "System Timer NVRAM_C Register"]
pub mod snvr2;
#[doc = "System Timer NVRAM_D Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr3](snvr3) module"]
pub type SNVR3 = crate::Reg<u32, _SNVR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNVR3;
#[doc = "`read()` method returns [snvr3::R](snvr3::R) reader structure"]
impl crate::Readable for SNVR3 {}
#[doc = "`write(|w| ..)` method takes [snvr3::W](snvr3::W) writer structure"]
impl crate::Writable for SNVR3 {}
#[doc = "System Timer NVRAM_D Register"]
pub mod snvr3;
#[doc = "Counter/Timer Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Counter/Timer Interrupts: Enable"]
pub mod inten;
#[doc = "Counter/Timer Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "Counter/Timer Interrupts: Status"]
pub mod intstat;
#[doc = "Counter/Timer Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "Counter/Timer Interrupts: Clear"]
pub mod intclr;
#[doc = "Counter/Timer Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "Counter/Timer Interrupts: Set"]
pub mod intset;
#[doc = "STIMER Interrupt registers: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stminten](stminten) module"]
pub type STMINTEN = crate::Reg<u32, _STMINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMINTEN;
#[doc = "`read()` method returns [stminten::R](stminten::R) reader structure"]
impl crate::Readable for STMINTEN {}
#[doc = "`write(|w| ..)` method takes [stminten::W](stminten::W) writer structure"]
impl crate::Writable for STMINTEN {}
#[doc = "STIMER Interrupt registers: Enable"]
pub mod stminten;
#[doc = "STIMER Interrupt registers: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmintstat](stmintstat) module"]
pub type STMINTSTAT = crate::Reg<u32, _STMINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMINTSTAT;
#[doc = "`read()` method returns [stmintstat::R](stmintstat::R) reader structure"]
impl crate::Readable for STMINTSTAT {}
#[doc = "`write(|w| ..)` method takes [stmintstat::W](stmintstat::W) writer structure"]
impl crate::Writable for STMINTSTAT {}
#[doc = "STIMER Interrupt registers: Status"]
pub mod stmintstat;
#[doc = "STIMER Interrupt registers: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmintclr](stmintclr) module"]
pub type STMINTCLR = crate::Reg<u32, _STMINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMINTCLR;
#[doc = "`read()` method returns [stmintclr::R](stmintclr::R) reader structure"]
impl crate::Readable for STMINTCLR {}
#[doc = "`write(|w| ..)` method takes [stmintclr::W](stmintclr::W) writer structure"]
impl crate::Writable for STMINTCLR {}
#[doc = "STIMER Interrupt registers: Clear"]
pub mod stmintclr;
#[doc = "STIMER Interrupt registers: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmintset](stmintset) module"]
pub type STMINTSET = crate::Reg<u32, _STMINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMINTSET;
#[doc = "`read()` method returns [stmintset::R](stmintset::R) reader structure"]
impl crate::Readable for STMINTSET {}
#[doc = "`write(|w| ..)` method takes [stmintset::W](stmintset::W) writer structure"]
impl crate::Writable for STMINTSET {}
#[doc = "STIMER Interrupt registers: Set"]
pub mod stmintset;
