#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage Regulator Select Register"]
    pub supplysrc: SUPPLYSRC,
    #[doc = "0x04 - Voltage Regulators status"]
    pub supplystatus: SUPPLYSTATUS,
    #[doc = "0x08 - Device Power Enables"]
    pub devpwren: DEVPWREN,
    #[doc = "0x0c - Powerdown SRAM banks in Deep Sleep mode"]
    pub mempwdinsleep: MEMPWDINSLEEP,
    #[doc = "0x10 - Enables individual banks of the MEMORY array"]
    pub mempwren: MEMPWREN,
    #[doc = "0x14 - Mem Power ON Status"]
    pub mempwrstatus: MEMPWRSTATUS,
    #[doc = "0x18 - Device Power ON Status"]
    pub devpwrstatus: DEVPWRSTATUS,
    #[doc = "0x1c - SRAM Control register"]
    pub sramctrl: SRAMCTRL,
    #[doc = "0x20 - Power Status Register for ADC Block"]
    pub adcstatus: ADCSTATUS,
    #[doc = "0x24 - Power Optimization Control Bits"]
    pub misc: MISC,
    #[doc = "0x28 - Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
    pub devpwreventen: DEVPWREVENTEN,
    #[doc = "0x2c - Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
    pub mempwreventen: MEMPWREVENTEN,
}
#[doc = "Voltage Regulator Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supplysrc](supplysrc) module"]
pub type SUPPLYSRC = crate::Reg<u32, _SUPPLYSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPPLYSRC;
#[doc = "`read()` method returns [supplysrc::R](supplysrc::R) reader structure"]
impl crate::Readable for SUPPLYSRC {}
#[doc = "`write(|w| ..)` method takes [supplysrc::W](supplysrc::W) writer structure"]
impl crate::Writable for SUPPLYSRC {}
#[doc = "Voltage Regulator Select Register"]
pub mod supplysrc;
#[doc = "Voltage Regulators status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supplystatus](supplystatus) module"]
pub type SUPPLYSTATUS = crate::Reg<u32, _SUPPLYSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPPLYSTATUS;
#[doc = "`read()` method returns [supplystatus::R](supplystatus::R) reader structure"]
impl crate::Readable for SUPPLYSTATUS {}
#[doc = "`write(|w| ..)` method takes [supplystatus::W](supplystatus::W) writer structure"]
impl crate::Writable for SUPPLYSTATUS {}
#[doc = "Voltage Regulators status"]
pub mod supplystatus;
#[doc = "Device Power Enables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devpwren](devpwren) module"]
pub type DEVPWREN = crate::Reg<u32, _DEVPWREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVPWREN;
#[doc = "`read()` method returns [devpwren::R](devpwren::R) reader structure"]
impl crate::Readable for DEVPWREN {}
#[doc = "`write(|w| ..)` method takes [devpwren::W](devpwren::W) writer structure"]
impl crate::Writable for DEVPWREN {}
#[doc = "Device Power Enables"]
pub mod devpwren;
#[doc = "Powerdown SRAM banks in Deep Sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mempwdinsleep](mempwdinsleep) module"]
pub type MEMPWDINSLEEP = crate::Reg<u32, _MEMPWDINSLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMPWDINSLEEP;
#[doc = "`read()` method returns [mempwdinsleep::R](mempwdinsleep::R) reader structure"]
impl crate::Readable for MEMPWDINSLEEP {}
#[doc = "`write(|w| ..)` method takes [mempwdinsleep::W](mempwdinsleep::W) writer structure"]
impl crate::Writable for MEMPWDINSLEEP {}
#[doc = "Powerdown SRAM banks in Deep Sleep mode"]
pub mod mempwdinsleep;
#[doc = "Enables individual banks of the MEMORY array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mempwren](mempwren) module"]
pub type MEMPWREN = crate::Reg<u32, _MEMPWREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMPWREN;
#[doc = "`read()` method returns [mempwren::R](mempwren::R) reader structure"]
impl crate::Readable for MEMPWREN {}
#[doc = "`write(|w| ..)` method takes [mempwren::W](mempwren::W) writer structure"]
impl crate::Writable for MEMPWREN {}
#[doc = "Enables individual banks of the MEMORY array"]
pub mod mempwren;
#[doc = "Mem Power ON Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mempwrstatus](mempwrstatus) module"]
pub type MEMPWRSTATUS = crate::Reg<u32, _MEMPWRSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMPWRSTATUS;
#[doc = "`read()` method returns [mempwrstatus::R](mempwrstatus::R) reader structure"]
impl crate::Readable for MEMPWRSTATUS {}
#[doc = "`write(|w| ..)` method takes [mempwrstatus::W](mempwrstatus::W) writer structure"]
impl crate::Writable for MEMPWRSTATUS {}
#[doc = "Mem Power ON Status"]
pub mod mempwrstatus;
#[doc = "Device Power ON Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devpwrstatus](devpwrstatus) module"]
pub type DEVPWRSTATUS = crate::Reg<u32, _DEVPWRSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVPWRSTATUS;
#[doc = "`read()` method returns [devpwrstatus::R](devpwrstatus::R) reader structure"]
impl crate::Readable for DEVPWRSTATUS {}
#[doc = "`write(|w| ..)` method takes [devpwrstatus::W](devpwrstatus::W) writer structure"]
impl crate::Writable for DEVPWRSTATUS {}
#[doc = "Device Power ON Status"]
pub mod devpwrstatus;
#[doc = "SRAM Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramctrl](sramctrl) module"]
pub type SRAMCTRL = crate::Reg<u32, _SRAMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMCTRL;
#[doc = "`read()` method returns [sramctrl::R](sramctrl::R) reader structure"]
impl crate::Readable for SRAMCTRL {}
#[doc = "`write(|w| ..)` method takes [sramctrl::W](sramctrl::W) writer structure"]
impl crate::Writable for SRAMCTRL {}
#[doc = "SRAM Control register"]
pub mod sramctrl;
#[doc = "Power Status Register for ADC Block\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcstatus](adcstatus) module"]
pub type ADCSTATUS = crate::Reg<u32, _ADCSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCSTATUS;
#[doc = "`read()` method returns [adcstatus::R](adcstatus::R) reader structure"]
impl crate::Readable for ADCSTATUS {}
#[doc = "`write(|w| ..)` method takes [adcstatus::W](adcstatus::W) writer structure"]
impl crate::Writable for ADCSTATUS {}
#[doc = "Power Status Register for ADC Block"]
pub mod adcstatus;
#[doc = "Power Optimization Control Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](misc) module"]
pub type MISC = crate::Reg<u32, _MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC;
#[doc = "`read()` method returns [misc::R](misc::R) reader structure"]
impl crate::Readable for MISC {}
#[doc = "`write(|w| ..)` method takes [misc::W](misc::W) writer structure"]
impl crate::Writable for MISC {}
#[doc = "Power Optimization Control Bits"]
pub mod misc;
#[doc = "Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devpwreventen](devpwreventen) module"]
pub type DEVPWREVENTEN = crate::Reg<u32, _DEVPWREVENTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVPWREVENTEN;
#[doc = "`read()` method returns [devpwreventen::R](devpwreventen::R) reader structure"]
impl crate::Readable for DEVPWREVENTEN {}
#[doc = "`write(|w| ..)` method takes [devpwreventen::W](devpwreventen::W) writer structure"]
impl crate::Writable for DEVPWREVENTEN {}
#[doc = "Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
pub mod devpwreventen;
#[doc = "Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mempwreventen](mempwreventen) module"]
pub type MEMPWREVENTEN = crate::Reg<u32, _MEMPWREVENTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMPWREVENTEN;
#[doc = "`read()` method returns [mempwreventen::R](mempwreventen::R) reader structure"]
impl crate::Readable for MEMPWREVENTEN {}
#[doc = "`write(|w| ..)` method takes [mempwreventen::W](mempwreventen::W) writer structure"]
impl crate::Writable for MEMPWREVENTEN {}
#[doc = "Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
pub mod mempwreventen;
