#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - XT Oscillator Control"]
    pub calxt: CALXT,
    #[doc = "0x04 - RC Oscillator Control"]
    pub calrc: CALRC,
    #[doc = "0x08 - Autocalibration Counter"]
    pub acalctr: ACALCTR,
    #[doc = "0x0c - Oscillator Control"]
    pub octrl: OCTRL,
    #[doc = "0x10 - CLKOUT Frequency Select"]
    pub clkout: CLKOUT,
    #[doc = "0x14 - Key Register for Clock Control Register"]
    pub clkkey: CLKKEY,
    #[doc = "0x18 - HFRC Clock Control"]
    pub cctrl: CCTRL,
    #[doc = "0x1c - Clock Generator Status"]
    pub status: STATUS,
    #[doc = "0x20 - HFRC Adjustment"]
    pub hfadj: HFADJ,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - Clock Enable Status"]
    pub clockenstat: CLOCKENSTAT,
    #[doc = "0x2c - Clock Enable Status"]
    pub clocken2stat: CLOCKEN2STAT,
    #[doc = "0x30 - Clock Enable Status"]
    pub clocken3stat: CLOCKEN3STAT,
    #[doc = "0x34 - HFRC Frequency Control register"]
    pub freqctrl: FREQCTRL,
    _reserved13: [u8; 4usize],
    #[doc = "0x3c - BLE BUCK TON ADJUST"]
    pub blebucktonadj: BLEBUCKTONADJ,
    _reserved14: [u8; 192usize],
    #[doc = "0x100 - CLKGEN Interrupt Register: Enable"]
    pub intrpten: INTRPTEN,
    #[doc = "0x104 - CLKGEN Interrupt Register: Status"]
    pub intrptstat: INTRPTSTAT,
    #[doc = "0x108 - CLKGEN Interrupt Register: Clear"]
    pub intrptclr: INTRPTCLR,
    #[doc = "0x10c - CLKGEN Interrupt Register: Set"]
    pub intrptset: INTRPTSET,
}
#[doc = "XT Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calxt](calxt) module"]
pub type CALXT = crate::Reg<u32, _CALXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALXT;
#[doc = "`read()` method returns [calxt::R](calxt::R) reader structure"]
impl crate::Readable for CALXT {}
#[doc = "`write(|w| ..)` method takes [calxt::W](calxt::W) writer structure"]
impl crate::Writable for CALXT {}
#[doc = "XT Oscillator Control"]
pub mod calxt;
#[doc = "RC Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calrc](calrc) module"]
pub type CALRC = crate::Reg<u32, _CALRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALRC;
#[doc = "`read()` method returns [calrc::R](calrc::R) reader structure"]
impl crate::Readable for CALRC {}
#[doc = "`write(|w| ..)` method takes [calrc::W](calrc::W) writer structure"]
impl crate::Writable for CALRC {}
#[doc = "RC Oscillator Control"]
pub mod calrc;
#[doc = "Autocalibration Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acalctr](acalctr) module"]
pub type ACALCTR = crate::Reg<u32, _ACALCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACALCTR;
#[doc = "`read()` method returns [acalctr::R](acalctr::R) reader structure"]
impl crate::Readable for ACALCTR {}
#[doc = "`write(|w| ..)` method takes [acalctr::W](acalctr::W) writer structure"]
impl crate::Writable for ACALCTR {}
#[doc = "Autocalibration Counter"]
pub mod acalctr;
#[doc = "Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octrl](octrl) module"]
pub type OCTRL = crate::Reg<u32, _OCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTRL;
#[doc = "`read()` method returns [octrl::R](octrl::R) reader structure"]
impl crate::Readable for OCTRL {}
#[doc = "`write(|w| ..)` method takes [octrl::W](octrl::W) writer structure"]
impl crate::Writable for OCTRL {}
#[doc = "Oscillator Control"]
pub mod octrl;
#[doc = "CLKOUT Frequency Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkout](clkout) module"]
pub type CLKOUT = crate::Reg<u32, _CLKOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUT;
#[doc = "`read()` method returns [clkout::R](clkout::R) reader structure"]
impl crate::Readable for CLKOUT {}
#[doc = "`write(|w| ..)` method takes [clkout::W](clkout::W) writer structure"]
impl crate::Writable for CLKOUT {}
#[doc = "CLKOUT Frequency Select"]
pub mod clkout;
#[doc = "Key Register for Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkkey](clkkey) module"]
pub type CLKKEY = crate::Reg<u32, _CLKKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKKEY;
#[doc = "`read()` method returns [clkkey::R](clkkey::R) reader structure"]
impl crate::Readable for CLKKEY {}
#[doc = "`write(|w| ..)` method takes [clkkey::W](clkkey::W) writer structure"]
impl crate::Writable for CLKKEY {}
#[doc = "Key Register for Clock Control Register"]
pub mod clkkey;
#[doc = "HFRC Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cctrl](cctrl) module"]
pub type CCTRL = crate::Reg<u32, _CCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCTRL;
#[doc = "`read()` method returns [cctrl::R](cctrl::R) reader structure"]
impl crate::Readable for CCTRL {}
#[doc = "`write(|w| ..)` method takes [cctrl::W](cctrl::W) writer structure"]
impl crate::Writable for CCTRL {}
#[doc = "HFRC Clock Control"]
pub mod cctrl;
#[doc = "Clock Generator Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Clock Generator Status"]
pub mod status;
#[doc = "HFRC Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfadj](hfadj) module"]
pub type HFADJ = crate::Reg<u32, _HFADJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFADJ;
#[doc = "`read()` method returns [hfadj::R](hfadj::R) reader structure"]
impl crate::Readable for HFADJ {}
#[doc = "`write(|w| ..)` method takes [hfadj::W](hfadj::W) writer structure"]
impl crate::Writable for HFADJ {}
#[doc = "HFRC Adjustment"]
pub mod hfadj;
#[doc = "Clock Enable Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockenstat](clockenstat) module"]
pub type CLOCKENSTAT = crate::Reg<u32, _CLOCKENSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCKENSTAT;
#[doc = "`read()` method returns [clockenstat::R](clockenstat::R) reader structure"]
impl crate::Readable for CLOCKENSTAT {}
#[doc = "`write(|w| ..)` method takes [clockenstat::W](clockenstat::W) writer structure"]
impl crate::Writable for CLOCKENSTAT {}
#[doc = "Clock Enable Status"]
pub mod clockenstat;
#[doc = "Clock Enable Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clocken2stat](clocken2stat) module"]
pub type CLOCKEN2STAT = crate::Reg<u32, _CLOCKEN2STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCKEN2STAT;
#[doc = "`read()` method returns [clocken2stat::R](clocken2stat::R) reader structure"]
impl crate::Readable for CLOCKEN2STAT {}
#[doc = "`write(|w| ..)` method takes [clocken2stat::W](clocken2stat::W) writer structure"]
impl crate::Writable for CLOCKEN2STAT {}
#[doc = "Clock Enable Status"]
pub mod clocken2stat;
#[doc = "Clock Enable Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clocken3stat](clocken3stat) module"]
pub type CLOCKEN3STAT = crate::Reg<u32, _CLOCKEN3STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCKEN3STAT;
#[doc = "`read()` method returns [clocken3stat::R](clocken3stat::R) reader structure"]
impl crate::Readable for CLOCKEN3STAT {}
#[doc = "`write(|w| ..)` method takes [clocken3stat::W](clocken3stat::W) writer structure"]
impl crate::Writable for CLOCKEN3STAT {}
#[doc = "Clock Enable Status"]
pub mod clocken3stat;
#[doc = "HFRC Frequency Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqctrl](freqctrl) module"]
pub type FREQCTRL = crate::Reg<u32, _FREQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQCTRL;
#[doc = "`read()` method returns [freqctrl::R](freqctrl::R) reader structure"]
impl crate::Readable for FREQCTRL {}
#[doc = "`write(|w| ..)` method takes [freqctrl::W](freqctrl::W) writer structure"]
impl crate::Writable for FREQCTRL {}
#[doc = "HFRC Frequency Control register"]
pub mod freqctrl;
#[doc = "BLE BUCK TON ADJUST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blebucktonadj](blebucktonadj) module"]
pub type BLEBUCKTONADJ = crate::Reg<u32, _BLEBUCKTONADJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLEBUCKTONADJ;
#[doc = "`read()` method returns [blebucktonadj::R](blebucktonadj::R) reader structure"]
impl crate::Readable for BLEBUCKTONADJ {}
#[doc = "`write(|w| ..)` method takes [blebucktonadj::W](blebucktonadj::W) writer structure"]
impl crate::Writable for BLEBUCKTONADJ {}
#[doc = "BLE BUCK TON ADJUST"]
pub mod blebucktonadj;
#[doc = "CLKGEN Interrupt Register: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrpten](intrpten) module"]
pub type INTRPTEN = crate::Reg<u32, _INTRPTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRPTEN;
#[doc = "`read()` method returns [intrpten::R](intrpten::R) reader structure"]
impl crate::Readable for INTRPTEN {}
#[doc = "`write(|w| ..)` method takes [intrpten::W](intrpten::W) writer structure"]
impl crate::Writable for INTRPTEN {}
#[doc = "CLKGEN Interrupt Register: Enable"]
pub mod intrpten;
#[doc = "CLKGEN Interrupt Register: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrptstat](intrptstat) module"]
pub type INTRPTSTAT = crate::Reg<u32, _INTRPTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRPTSTAT;
#[doc = "`read()` method returns [intrptstat::R](intrptstat::R) reader structure"]
impl crate::Readable for INTRPTSTAT {}
#[doc = "`write(|w| ..)` method takes [intrptstat::W](intrptstat::W) writer structure"]
impl crate::Writable for INTRPTSTAT {}
#[doc = "CLKGEN Interrupt Register: Status"]
pub mod intrptstat;
#[doc = "CLKGEN Interrupt Register: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrptclr](intrptclr) module"]
pub type INTRPTCLR = crate::Reg<u32, _INTRPTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRPTCLR;
#[doc = "`read()` method returns [intrptclr::R](intrptclr::R) reader structure"]
impl crate::Readable for INTRPTCLR {}
#[doc = "`write(|w| ..)` method takes [intrptclr::W](intrptclr::W) writer structure"]
impl crate::Writable for INTRPTCLR {}
#[doc = "CLKGEN Interrupt Register: Clear"]
pub mod intrptclr;
#[doc = "CLKGEN Interrupt Register: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrptset](intrptset) module"]
pub type INTRPTSET = crate::Reg<u32, _INTRPTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRPTSET;
#[doc = "`read()` method returns [intrptset::R](intrptset::R) reader structure"]
impl crate::Readable for INTRPTSET {}
#[doc = "`write(|w| ..)` method takes [intrptset::W](intrptset::W) writer structure"]
impl crate::Writable for INTRPTSET {}
#[doc = "CLKGEN Interrupt Register: Set"]
pub mod intrptset;
