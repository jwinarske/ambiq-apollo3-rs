#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub bbvalue: BBVALUE,
    #[doc = "0x04 - Set/Clear Register"]
    pub bbsetclear: BBSETCLEAR,
    #[doc = "0x08 - PIO Input Values"]
    pub bbinput: BBINPUT,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - PIO Input Values"]
    pub debugdata: DEBUGDATA,
    _reserved4: [u8; 28usize],
    #[doc = "0x40 - PIO Input Values"]
    pub debug: DEBUG,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbvalue](bbvalue) module"]
pub type BBVALUE = crate::Reg<u32, _BBVALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBVALUE;
#[doc = "`read()` method returns [bbvalue::R](bbvalue::R) reader structure"]
impl crate::Readable for BBVALUE {}
#[doc = "`write(|w| ..)` method takes [bbvalue::W](bbvalue::W) writer structure"]
impl crate::Writable for BBVALUE {}
#[doc = "Control Register"]
pub mod bbvalue;
#[doc = "Set/Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbsetclear](bbsetclear) module"]
pub type BBSETCLEAR = crate::Reg<u32, _BBSETCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBSETCLEAR;
#[doc = "`read()` method returns [bbsetclear::R](bbsetclear::R) reader structure"]
impl crate::Readable for BBSETCLEAR {}
#[doc = "`write(|w| ..)` method takes [bbsetclear::W](bbsetclear::W) writer structure"]
impl crate::Writable for BBSETCLEAR {}
#[doc = "Set/Clear Register"]
pub mod bbsetclear;
#[doc = "PIO Input Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbinput](bbinput) module"]
pub type BBINPUT = crate::Reg<u32, _BBINPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBINPUT;
#[doc = "`read()` method returns [bbinput::R](bbinput::R) reader structure"]
impl crate::Readable for BBINPUT {}
#[doc = "`write(|w| ..)` method takes [bbinput::W](bbinput::W) writer structure"]
impl crate::Writable for BBINPUT {}
#[doc = "PIO Input Values"]
pub mod bbinput;
#[doc = "PIO Input Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debugdata](debugdata) module"]
pub type DEBUGDATA = crate::Reg<u32, _DEBUGDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUGDATA;
#[doc = "`read()` method returns [debugdata::R](debugdata::R) reader structure"]
impl crate::Readable for DEBUGDATA {}
#[doc = "`write(|w| ..)` method takes [debugdata::W](debugdata::W) writer structure"]
impl crate::Writable for DEBUGDATA {}
#[doc = "PIO Input Values"]
pub mod debugdata;
#[doc = "PIO Input Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug](debug) module"]
pub type DEBUG = crate::Reg<u32, _DEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG;
#[doc = "`read()` method returns [debug::R](debug::R) reader structure"]
impl crate::Readable for DEBUG {}
#[doc = "`write(|w| ..)` method takes [debug::W](debug::W) writer structure"]
impl crate::Writable for DEBUG {}
#[doc = "PIO Input Values"]
pub mod debug;
