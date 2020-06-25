#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip Information Register"]
    pub chippn: CHIPPN,
    #[doc = "0x04 - Unique Chip ID 0"]
    pub chipid0: CHIPID0,
    #[doc = "0x08 - Unique Chip ID 1"]
    pub chipid1: CHIPID1,
    #[doc = "0x0c - Chip Revision"]
    pub chiprev: CHIPREV,
    #[doc = "0x10 - Unique Vendor ID"]
    pub vendorid: VENDORID,
    #[doc = "0x14 - Unique Chip SKU"]
    pub sku: SKU,
    #[doc = "0x18 - Feature Enable on Burst and BLE"]
    pub featureenable: FEATUREENABLE,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Debugger Control"]
    pub debugger: DEBUGGER,
    _reserved8: [u8; 220usize],
    #[doc = "0x100 - BOD control Register"]
    pub bodctrl: BODCTRL,
    #[doc = "0x104 - ADC Power Up Delay Control"]
    pub adcpwrdly: ADCPWRDLY,
    _reserved10: [u8; 4usize],
    #[doc = "0x10c - ADC Calibration Control"]
    pub adccal: ADCCAL,
    #[doc = "0x110 - ADC Battery Load Enable"]
    pub adcbattload: ADCBATTLOAD,
    _reserved12: [u8; 4usize],
    #[doc = "0x118 - ADC Trims"]
    pub adctrim: ADCTRIM,
    #[doc = "0x11c - ADC Referece Keeper and Comparator Control"]
    pub adcrefcomp: ADCREFCOMP,
    #[doc = "0x120 - XTAL Oscillator Control"]
    pub xtalctrl: XTALCTRL,
    #[doc = "0x124 - XTAL Oscillator General Control"]
    pub xtalgenctrl: XTALGENCTRL,
    _reserved16: [u8; 112usize],
    #[doc = "0x198 - Miscellaneous control register."]
    pub miscctrl: MISCCTRL,
    _reserved17: [u8; 4usize],
    #[doc = "0x1a0 - Bootloader and secure boot functions"]
    pub bootloader: BOOTLOADER,
    #[doc = "0x1a4 - Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
    pub shadowvalid: SHADOWVALID,
    _reserved19: [u8; 8usize],
    #[doc = "0x1b0 - Scratch register that is not reset by any reset"]
    pub scratch0: SCRATCH0,
    #[doc = "0x1b4 - Scratch register that is not reset by any reset"]
    pub scratch1: SCRATCH1,
    _reserved21: [u8; 8usize],
    #[doc = "0x1c0 - ICODE bus address which was present when a bus fault occurred."]
    pub icodefaultaddr: ICODEFAULTADDR,
    #[doc = "0x1c4 - DCODE bus address which was present when a bus fault occurred."]
    pub dcodefaultaddr: DCODEFAULTADDR,
    #[doc = "0x1c8 - System bus address which was present when a bus fault occurred."]
    pub sysfaultaddr: SYSFAULTADDR,
    #[doc = "0x1cc - Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
    pub faultstatus: FAULTSTATUS,
    #[doc = "0x1d0 - Enable the fault capture registers"]
    pub faultcaptureen: FAULTCAPTUREEN,
    _reserved26: [u8; 44usize],
    #[doc = "0x200 - Read-only debug register 1"]
    pub dbgr1: DBGR1,
    #[doc = "0x204 - Read-only debug register 2"]
    pub dbgr2: DBGR2,
    _reserved28: [u8; 24usize],
    #[doc = "0x220 - Control bit to enable/disable the PMU"]
    pub pmuenable: PMUENABLE,
    _reserved29: [u8; 44usize],
    #[doc = "0x250 - TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
    pub tpiuctrl: TPIUCTRL,
    _reserved30: [u8; 16usize],
    #[doc = "0x264 - OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
    pub otapointer: OTAPOINTER,
    _reserved31: [u8; 24usize],
    #[doc = "0x280 - DMA Control Register. Determines misc settings for DMA operation"]
    pub apbdmactrl: APBDMACTRL,
    #[doc = "0x284 - SRAM Controller mode bits"]
    pub srammode: SRAMMODE,
    _reserved33: [u8; 192usize],
    #[doc = "0x348 - Key Register to enable the use of external clock selects via the EXTCLKSEL reg"]
    pub kextclksel: KEXTCLKSEL,
    _reserved34: [u8; 16usize],
    #[doc = "0x35c - SIMO Buck Control Reg1"]
    pub simobuck4: SIMOBUCK4,
    _reserved35: [u8; 8usize],
    #[doc = "0x368 - BLEBUCK2 Control Reg"]
    pub blebuck2: BLEBUCK2,
    _reserved36: [u8; 52usize],
    #[doc = "0x3a0 - Flash Write Protection Bits"]
    pub flashwprot0: FLASHWPROT0,
    #[doc = "0x3a4 - Flash Write Protection Bits"]
    pub flashwprot1: FLASHWPROT1,
    _reserved38: [u8; 8usize],
    #[doc = "0x3b0 - Flash Read Protection Bits"]
    pub flashrprot0: FLASHRPROT0,
    #[doc = "0x3b4 - Flash Read Protection Bits"]
    pub flashrprot1: FLASHRPROT1,
    _reserved40: [u8; 8usize],
    #[doc = "0x3c0 - SRAM write-protection bits."]
    pub dmasramwriteprotect0: DMASRAMWRITEPROTECT0,
    #[doc = "0x3c4 - SRAM write-protection bits."]
    pub dmasramwriteprotect1: DMASRAMWRITEPROTECT1,
    _reserved42: [u8; 8usize],
    #[doc = "0x3d0 - SRAM read-protection bits."]
    pub dmasramreadprotect0: DMASRAMREADPROTECT0,
    #[doc = "0x3d4 - SRAM read-protection bits."]
    pub dmasramreadprotect1: DMASRAMREADPROTECT1,
}
#[doc = "Chip Information Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chippn](chippn) module"]
pub type CHIPPN = crate::Reg<u32, _CHIPPN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPPN;
#[doc = "`read()` method returns [chippn::R](chippn::R) reader structure"]
impl crate::Readable for CHIPPN {}
#[doc = "`write(|w| ..)` method takes [chippn::W](chippn::W) writer structure"]
impl crate::Writable for CHIPPN {}
#[doc = "Chip Information Register"]
pub mod chippn;
#[doc = "Unique Chip ID 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid0](chipid0) module"]
pub type CHIPID0 = crate::Reg<u32, _CHIPID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPID0;
#[doc = "`read()` method returns [chipid0::R](chipid0::R) reader structure"]
impl crate::Readable for CHIPID0 {}
#[doc = "`write(|w| ..)` method takes [chipid0::W](chipid0::W) writer structure"]
impl crate::Writable for CHIPID0 {}
#[doc = "Unique Chip ID 0"]
pub mod chipid0;
#[doc = "Unique Chip ID 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid1](chipid1) module"]
pub type CHIPID1 = crate::Reg<u32, _CHIPID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPID1;
#[doc = "`read()` method returns [chipid1::R](chipid1::R) reader structure"]
impl crate::Readable for CHIPID1 {}
#[doc = "`write(|w| ..)` method takes [chipid1::W](chipid1::W) writer structure"]
impl crate::Writable for CHIPID1 {}
#[doc = "Unique Chip ID 1"]
pub mod chipid1;
#[doc = "Chip Revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chiprev](chiprev) module"]
pub type CHIPREV = crate::Reg<u32, _CHIPREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPREV;
#[doc = "`read()` method returns [chiprev::R](chiprev::R) reader structure"]
impl crate::Readable for CHIPREV {}
#[doc = "`write(|w| ..)` method takes [chiprev::W](chiprev::W) writer structure"]
impl crate::Writable for CHIPREV {}
#[doc = "Chip Revision"]
pub mod chiprev;
#[doc = "Unique Vendor ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vendorid](vendorid) module"]
pub type VENDORID = crate::Reg<u32, _VENDORID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VENDORID;
#[doc = "`read()` method returns [vendorid::R](vendorid::R) reader structure"]
impl crate::Readable for VENDORID {}
#[doc = "`write(|w| ..)` method takes [vendorid::W](vendorid::W) writer structure"]
impl crate::Writable for VENDORID {}
#[doc = "Unique Vendor ID"]
pub mod vendorid;
#[doc = "Unique Chip SKU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sku](sku) module"]
pub type SKU = crate::Reg<u32, _SKU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SKU;
#[doc = "`read()` method returns [sku::R](sku::R) reader structure"]
impl crate::Readable for SKU {}
#[doc = "`write(|w| ..)` method takes [sku::W](sku::W) writer structure"]
impl crate::Writable for SKU {}
#[doc = "Unique Chip SKU"]
pub mod sku;
#[doc = "Feature Enable on Burst and BLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [featureenable](featureenable) module"]
pub type FEATUREENABLE = crate::Reg<u32, _FEATUREENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEATUREENABLE;
#[doc = "`read()` method returns [featureenable::R](featureenable::R) reader structure"]
impl crate::Readable for FEATUREENABLE {}
#[doc = "`write(|w| ..)` method takes [featureenable::W](featureenable::W) writer structure"]
impl crate::Writable for FEATUREENABLE {}
#[doc = "Feature Enable on Burst and BLE"]
pub mod featureenable;
#[doc = "Debugger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debugger](debugger) module"]
pub type DEBUGGER = crate::Reg<u32, _DEBUGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUGGER;
#[doc = "`read()` method returns [debugger::R](debugger::R) reader structure"]
impl crate::Readable for DEBUGGER {}
#[doc = "`write(|w| ..)` method takes [debugger::W](debugger::W) writer structure"]
impl crate::Writable for DEBUGGER {}
#[doc = "Debugger Control"]
pub mod debugger;
#[doc = "BOD control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodctrl](bodctrl) module"]
pub type BODCTRL = crate::Reg<u32, _BODCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODCTRL;
#[doc = "`read()` method returns [bodctrl::R](bodctrl::R) reader structure"]
impl crate::Readable for BODCTRL {}
#[doc = "`write(|w| ..)` method takes [bodctrl::W](bodctrl::W) writer structure"]
impl crate::Writable for BODCTRL {}
#[doc = "BOD control Register"]
pub mod bodctrl;
#[doc = "ADC Power Up Delay Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcpwrdly](adcpwrdly) module"]
pub type ADCPWRDLY = crate::Reg<u32, _ADCPWRDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCPWRDLY;
#[doc = "`read()` method returns [adcpwrdly::R](adcpwrdly::R) reader structure"]
impl crate::Readable for ADCPWRDLY {}
#[doc = "`write(|w| ..)` method takes [adcpwrdly::W](adcpwrdly::W) writer structure"]
impl crate::Writable for ADCPWRDLY {}
#[doc = "ADC Power Up Delay Control"]
pub mod adcpwrdly;
#[doc = "ADC Calibration Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adccal](adccal) module"]
pub type ADCCAL = crate::Reg<u32, _ADCCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCAL;
#[doc = "`read()` method returns [adccal::R](adccal::R) reader structure"]
impl crate::Readable for ADCCAL {}
#[doc = "`write(|w| ..)` method takes [adccal::W](adccal::W) writer structure"]
impl crate::Writable for ADCCAL {}
#[doc = "ADC Calibration Control"]
pub mod adccal;
#[doc = "ADC Battery Load Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcbattload](adcbattload) module"]
pub type ADCBATTLOAD = crate::Reg<u32, _ADCBATTLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCBATTLOAD;
#[doc = "`read()` method returns [adcbattload::R](adcbattload::R) reader structure"]
impl crate::Readable for ADCBATTLOAD {}
#[doc = "`write(|w| ..)` method takes [adcbattload::W](adcbattload::W) writer structure"]
impl crate::Writable for ADCBATTLOAD {}
#[doc = "ADC Battery Load Enable"]
pub mod adcbattload;
#[doc = "ADC Trims\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctrim](adctrim) module"]
pub type ADCTRIM = crate::Reg<u32, _ADCTRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCTRIM;
#[doc = "`read()` method returns [adctrim::R](adctrim::R) reader structure"]
impl crate::Readable for ADCTRIM {}
#[doc = "`write(|w| ..)` method takes [adctrim::W](adctrim::W) writer structure"]
impl crate::Writable for ADCTRIM {}
#[doc = "ADC Trims"]
pub mod adctrim;
#[doc = "ADC Referece Keeper and Comparator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcrefcomp](adcrefcomp) module"]
pub type ADCREFCOMP = crate::Reg<u32, _ADCREFCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCREFCOMP;
#[doc = "`read()` method returns [adcrefcomp::R](adcrefcomp::R) reader structure"]
impl crate::Readable for ADCREFCOMP {}
#[doc = "`write(|w| ..)` method takes [adcrefcomp::W](adcrefcomp::W) writer structure"]
impl crate::Writable for ADCREFCOMP {}
#[doc = "ADC Referece Keeper and Comparator Control"]
pub mod adcrefcomp;
#[doc = "XTAL Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalctrl](xtalctrl) module"]
pub type XTALCTRL = crate::Reg<u32, _XTALCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTALCTRL;
#[doc = "`read()` method returns [xtalctrl::R](xtalctrl::R) reader structure"]
impl crate::Readable for XTALCTRL {}
#[doc = "`write(|w| ..)` method takes [xtalctrl::W](xtalctrl::W) writer structure"]
impl crate::Writable for XTALCTRL {}
#[doc = "XTAL Oscillator Control"]
pub mod xtalctrl;
#[doc = "XTAL Oscillator General Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalgenctrl](xtalgenctrl) module"]
pub type XTALGENCTRL = crate::Reg<u32, _XTALGENCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTALGENCTRL;
#[doc = "`read()` method returns [xtalgenctrl::R](xtalgenctrl::R) reader structure"]
impl crate::Readable for XTALGENCTRL {}
#[doc = "`write(|w| ..)` method takes [xtalgenctrl::W](xtalgenctrl::W) writer structure"]
impl crate::Writable for XTALGENCTRL {}
#[doc = "XTAL Oscillator General Control"]
pub mod xtalgenctrl;
#[doc = "Miscellaneous control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miscctrl](miscctrl) module"]
pub type MISCCTRL = crate::Reg<u32, _MISCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISCCTRL;
#[doc = "`read()` method returns [miscctrl::R](miscctrl::R) reader structure"]
impl crate::Readable for MISCCTRL {}
#[doc = "`write(|w| ..)` method takes [miscctrl::W](miscctrl::W) writer structure"]
impl crate::Writable for MISCCTRL {}
#[doc = "Miscellaneous control register."]
pub mod miscctrl;
#[doc = "Bootloader and secure boot functions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootloader](bootloader) module"]
pub type BOOTLOADER = crate::Reg<u32, _BOOTLOADER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOTLOADER;
#[doc = "`read()` method returns [bootloader::R](bootloader::R) reader structure"]
impl crate::Readable for BOOTLOADER {}
#[doc = "`write(|w| ..)` method takes [bootloader::W](bootloader::W) writer structure"]
impl crate::Writable for BOOTLOADER {}
#[doc = "Bootloader and secure boot functions"]
pub mod bootloader;
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shadowvalid](shadowvalid) module"]
pub type SHADOWVALID = crate::Reg<u32, _SHADOWVALID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHADOWVALID;
#[doc = "`read()` method returns [shadowvalid::R](shadowvalid::R) reader structure"]
impl crate::Readable for SHADOWVALID {}
#[doc = "`write(|w| ..)` method takes [shadowvalid::W](shadowvalid::W) writer structure"]
impl crate::Writable for SHADOWVALID {}
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
pub mod shadowvalid;
#[doc = "Scratch register that is not reset by any reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch0](scratch0) module"]
pub type SCRATCH0 = crate::Reg<u32, _SCRATCH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH0;
#[doc = "`read()` method returns [scratch0::R](scratch0::R) reader structure"]
impl crate::Readable for SCRATCH0 {}
#[doc = "`write(|w| ..)` method takes [scratch0::W](scratch0::W) writer structure"]
impl crate::Writable for SCRATCH0 {}
#[doc = "Scratch register that is not reset by any reset"]
pub mod scratch0;
#[doc = "Scratch register that is not reset by any reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch1](scratch1) module"]
pub type SCRATCH1 = crate::Reg<u32, _SCRATCH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH1;
#[doc = "`read()` method returns [scratch1::R](scratch1::R) reader structure"]
impl crate::Readable for SCRATCH1 {}
#[doc = "`write(|w| ..)` method takes [scratch1::W](scratch1::W) writer structure"]
impl crate::Writable for SCRATCH1 {}
#[doc = "Scratch register that is not reset by any reset"]
pub mod scratch1;
#[doc = "ICODE bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icodefaultaddr](icodefaultaddr) module"]
pub type ICODEFAULTADDR = crate::Reg<u32, _ICODEFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICODEFAULTADDR;
#[doc = "`read()` method returns [icodefaultaddr::R](icodefaultaddr::R) reader structure"]
impl crate::Readable for ICODEFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [icodefaultaddr::W](icodefaultaddr::W) writer structure"]
impl crate::Writable for ICODEFAULTADDR {}
#[doc = "ICODE bus address which was present when a bus fault occurred."]
pub mod icodefaultaddr;
#[doc = "DCODE bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcodefaultaddr](dcodefaultaddr) module"]
pub type DCODEFAULTADDR = crate::Reg<u32, _DCODEFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCODEFAULTADDR;
#[doc = "`read()` method returns [dcodefaultaddr::R](dcodefaultaddr::R) reader structure"]
impl crate::Readable for DCODEFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [dcodefaultaddr::W](dcodefaultaddr::W) writer structure"]
impl crate::Writable for DCODEFAULTADDR {}
#[doc = "DCODE bus address which was present when a bus fault occurred."]
pub mod dcodefaultaddr;
#[doc = "System bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysfaultaddr](sysfaultaddr) module"]
pub type SYSFAULTADDR = crate::Reg<u32, _SYSFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSFAULTADDR;
#[doc = "`read()` method returns [sysfaultaddr::R](sysfaultaddr::R) reader structure"]
impl crate::Readable for SYSFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [sysfaultaddr::W](sysfaultaddr::W) writer structure"]
impl crate::Writable for SYSFAULTADDR {}
#[doc = "System bus address which was present when a bus fault occurred."]
pub mod sysfaultaddr;
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultstatus](faultstatus) module"]
pub type FAULTSTATUS = crate::Reg<u32, _FAULTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULTSTATUS;
#[doc = "`read()` method returns [faultstatus::R](faultstatus::R) reader structure"]
impl crate::Readable for FAULTSTATUS {}
#[doc = "`write(|w| ..)` method takes [faultstatus::W](faultstatus::W) writer structure"]
impl crate::Writable for FAULTSTATUS {}
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
pub mod faultstatus;
#[doc = "Enable the fault capture registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultcaptureen](faultcaptureen) module"]
pub type FAULTCAPTUREEN = crate::Reg<u32, _FAULTCAPTUREEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULTCAPTUREEN;
#[doc = "`read()` method returns [faultcaptureen::R](faultcaptureen::R) reader structure"]
impl crate::Readable for FAULTCAPTUREEN {}
#[doc = "`write(|w| ..)` method takes [faultcaptureen::W](faultcaptureen::W) writer structure"]
impl crate::Writable for FAULTCAPTUREEN {}
#[doc = "Enable the fault capture registers"]
pub mod faultcaptureen;
#[doc = "Read-only debug register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgr1](dbgr1) module"]
pub type DBGR1 = crate::Reg<u32, _DBGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGR1;
#[doc = "`read()` method returns [dbgr1::R](dbgr1::R) reader structure"]
impl crate::Readable for DBGR1 {}
#[doc = "`write(|w| ..)` method takes [dbgr1::W](dbgr1::W) writer structure"]
impl crate::Writable for DBGR1 {}
#[doc = "Read-only debug register 1"]
pub mod dbgr1;
#[doc = "Read-only debug register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgr2](dbgr2) module"]
pub type DBGR2 = crate::Reg<u32, _DBGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGR2;
#[doc = "`read()` method returns [dbgr2::R](dbgr2::R) reader structure"]
impl crate::Readable for DBGR2 {}
#[doc = "`write(|w| ..)` method takes [dbgr2::W](dbgr2::W) writer structure"]
impl crate::Writable for DBGR2 {}
#[doc = "Read-only debug register 2"]
pub mod dbgr2;
#[doc = "Control bit to enable/disable the PMU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmuenable](pmuenable) module"]
pub type PMUENABLE = crate::Reg<u32, _PMUENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUENABLE;
#[doc = "`read()` method returns [pmuenable::R](pmuenable::R) reader structure"]
impl crate::Readable for PMUENABLE {}
#[doc = "`write(|w| ..)` method takes [pmuenable::W](pmuenable::W) writer structure"]
impl crate::Writable for PMUENABLE {}
#[doc = "Control bit to enable/disable the PMU"]
pub mod pmuenable;
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpiuctrl](tpiuctrl) module"]
pub type TPIUCTRL = crate::Reg<u32, _TPIUCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPIUCTRL;
#[doc = "`read()` method returns [tpiuctrl::R](tpiuctrl::R) reader structure"]
impl crate::Readable for TPIUCTRL {}
#[doc = "`write(|w| ..)` method takes [tpiuctrl::W](tpiuctrl::W) writer structure"]
impl crate::Writable for TPIUCTRL {}
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
pub mod tpiuctrl;
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otapointer](otapointer) module"]
pub type OTAPOINTER = crate::Reg<u32, _OTAPOINTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTAPOINTER;
#[doc = "`read()` method returns [otapointer::R](otapointer::R) reader structure"]
impl crate::Readable for OTAPOINTER {}
#[doc = "`write(|w| ..)` method takes [otapointer::W](otapointer::W) writer structure"]
impl crate::Writable for OTAPOINTER {}
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
pub mod otapointer;
#[doc = "DMA Control Register. Determines misc settings for DMA operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbdmactrl](apbdmactrl) module"]
pub type APBDMACTRL = crate::Reg<u32, _APBDMACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBDMACTRL;
#[doc = "`read()` method returns [apbdmactrl::R](apbdmactrl::R) reader structure"]
impl crate::Readable for APBDMACTRL {}
#[doc = "`write(|w| ..)` method takes [apbdmactrl::W](apbdmactrl::W) writer structure"]
impl crate::Writable for APBDMACTRL {}
#[doc = "DMA Control Register. Determines misc settings for DMA operation"]
pub mod apbdmactrl;
#[doc = "SRAM Controller mode bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srammode](srammode) module"]
pub type SRAMMODE = crate::Reg<u32, _SRAMMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMMODE;
#[doc = "`read()` method returns [srammode::R](srammode::R) reader structure"]
impl crate::Readable for SRAMMODE {}
#[doc = "`write(|w| ..)` method takes [srammode::W](srammode::W) writer structure"]
impl crate::Writable for SRAMMODE {}
#[doc = "SRAM Controller mode bits"]
pub mod srammode;
#[doc = "Key Register to enable the use of external clock selects via the EXTCLKSEL reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kextclksel](kextclksel) module"]
pub type KEXTCLKSEL = crate::Reg<u32, _KEXTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEXTCLKSEL;
#[doc = "`read()` method returns [kextclksel::R](kextclksel::R) reader structure"]
impl crate::Readable for KEXTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [kextclksel::W](kextclksel::W) writer structure"]
impl crate::Writable for KEXTCLKSEL {}
#[doc = "Key Register to enable the use of external clock selects via the EXTCLKSEL reg"]
pub mod kextclksel;
#[doc = "SIMO Buck Control Reg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simobuck4](simobuck4) module"]
pub type SIMOBUCK4 = crate::Reg<u32, _SIMOBUCK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIMOBUCK4;
#[doc = "`read()` method returns [simobuck4::R](simobuck4::R) reader structure"]
impl crate::Readable for SIMOBUCK4 {}
#[doc = "`write(|w| ..)` method takes [simobuck4::W](simobuck4::W) writer structure"]
impl crate::Writable for SIMOBUCK4 {}
#[doc = "SIMO Buck Control Reg1"]
pub mod simobuck4;
#[doc = "BLEBUCK2 Control Reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blebuck2](blebuck2) module"]
pub type BLEBUCK2 = crate::Reg<u32, _BLEBUCK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLEBUCK2;
#[doc = "`read()` method returns [blebuck2::R](blebuck2::R) reader structure"]
impl crate::Readable for BLEBUCK2 {}
#[doc = "`write(|w| ..)` method takes [blebuck2::W](blebuck2::W) writer structure"]
impl crate::Writable for BLEBUCK2 {}
#[doc = "BLEBUCK2 Control Reg"]
pub mod blebuck2;
#[doc = "Flash Write Protection Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashwprot0](flashwprot0) module"]
pub type FLASHWPROT0 = crate::Reg<u32, _FLASHWPROT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHWPROT0;
#[doc = "`read()` method returns [flashwprot0::R](flashwprot0::R) reader structure"]
impl crate::Readable for FLASHWPROT0 {}
#[doc = "`write(|w| ..)` method takes [flashwprot0::W](flashwprot0::W) writer structure"]
impl crate::Writable for FLASHWPROT0 {}
#[doc = "Flash Write Protection Bits"]
pub mod flashwprot0;
#[doc = "Flash Write Protection Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashwprot1](flashwprot1) module"]
pub type FLASHWPROT1 = crate::Reg<u32, _FLASHWPROT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHWPROT1;
#[doc = "`read()` method returns [flashwprot1::R](flashwprot1::R) reader structure"]
impl crate::Readable for FLASHWPROT1 {}
#[doc = "`write(|w| ..)` method takes [flashwprot1::W](flashwprot1::W) writer structure"]
impl crate::Writable for FLASHWPROT1 {}
#[doc = "Flash Write Protection Bits"]
pub mod flashwprot1;
#[doc = "Flash Read Protection Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashrprot0](flashrprot0) module"]
pub type FLASHRPROT0 = crate::Reg<u32, _FLASHRPROT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHRPROT0;
#[doc = "`read()` method returns [flashrprot0::R](flashrprot0::R) reader structure"]
impl crate::Readable for FLASHRPROT0 {}
#[doc = "`write(|w| ..)` method takes [flashrprot0::W](flashrprot0::W) writer structure"]
impl crate::Writable for FLASHRPROT0 {}
#[doc = "Flash Read Protection Bits"]
pub mod flashrprot0;
#[doc = "Flash Read Protection Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashrprot1](flashrprot1) module"]
pub type FLASHRPROT1 = crate::Reg<u32, _FLASHRPROT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHRPROT1;
#[doc = "`read()` method returns [flashrprot1::R](flashrprot1::R) reader structure"]
impl crate::Readable for FLASHRPROT1 {}
#[doc = "`write(|w| ..)` method takes [flashrprot1::W](flashrprot1::W) writer structure"]
impl crate::Writable for FLASHRPROT1 {}
#[doc = "Flash Read Protection Bits"]
pub mod flashrprot1;
#[doc = "SRAM write-protection bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasramwriteprotect0](dmasramwriteprotect0) module"]
pub type DMASRAMWRITEPROTECT0 = crate::Reg<u32, _DMASRAMWRITEPROTECT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASRAMWRITEPROTECT0;
#[doc = "`read()` method returns [dmasramwriteprotect0::R](dmasramwriteprotect0::R) reader structure"]
impl crate::Readable for DMASRAMWRITEPROTECT0 {}
#[doc = "`write(|w| ..)` method takes [dmasramwriteprotect0::W](dmasramwriteprotect0::W) writer structure"]
impl crate::Writable for DMASRAMWRITEPROTECT0 {}
#[doc = "SRAM write-protection bits."]
pub mod dmasramwriteprotect0;
#[doc = "SRAM write-protection bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasramwriteprotect1](dmasramwriteprotect1) module"]
pub type DMASRAMWRITEPROTECT1 = crate::Reg<u32, _DMASRAMWRITEPROTECT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASRAMWRITEPROTECT1;
#[doc = "`read()` method returns [dmasramwriteprotect1::R](dmasramwriteprotect1::R) reader structure"]
impl crate::Readable for DMASRAMWRITEPROTECT1 {}
#[doc = "`write(|w| ..)` method takes [dmasramwriteprotect1::W](dmasramwriteprotect1::W) writer structure"]
impl crate::Writable for DMASRAMWRITEPROTECT1 {}
#[doc = "SRAM write-protection bits."]
pub mod dmasramwriteprotect1;
#[doc = "SRAM read-protection bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasramreadprotect0](dmasramreadprotect0) module"]
pub type DMASRAMREADPROTECT0 = crate::Reg<u32, _DMASRAMREADPROTECT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASRAMREADPROTECT0;
#[doc = "`read()` method returns [dmasramreadprotect0::R](dmasramreadprotect0::R) reader structure"]
impl crate::Readable for DMASRAMREADPROTECT0 {}
#[doc = "`write(|w| ..)` method takes [dmasramreadprotect0::W](dmasramreadprotect0::W) writer structure"]
impl crate::Writable for DMASRAMREADPROTECT0 {}
#[doc = "SRAM read-protection bits."]
pub mod dmasramreadprotect0;
#[doc = "SRAM read-protection bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasramreadprotect1](dmasramreadprotect1) module"]
pub type DMASRAMREADPROTECT1 = crate::Reg<u32, _DMASRAMREADPROTECT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASRAMREADPROTECT1;
#[doc = "`read()` method returns [dmasramreadprotect1::R](dmasramreadprotect1::R) reader structure"]
impl crate::Readable for DMASRAMREADPROTECT1 {}
#[doc = "`write(|w| ..)` method takes [dmasramreadprotect1::W](dmasramreadprotect1::W) writer structure"]
impl crate::Writable for DMASRAMREADPROTECT1 {}
#[doc = "SRAM read-protection bits."]
pub mod dmasramreadprotect1;
