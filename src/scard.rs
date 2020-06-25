#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISO7816 interrupt status"]
    pub sr: SR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - ISO7816 data"]
    pub dr: DR,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - ISO7816 interrupt status 1"]
    pub sr1: SR1,
    _reserved3: [u8; 20usize],
    #[doc = "0x38 - ISO7816 resent count inquiry"]
    pub retxcntrmi: RETXCNTRMI,
    _reserved4: [u8; 196usize],
    #[doc = "0x100 - Clock Control"]
    pub clkctrl: CLKCTRL,
}
#[doc = "ISO7816 interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "ISO7816 interrupt status"]
pub mod sr;
#[doc = "ISO7816 data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "ISO7816 data"]
pub mod dr;
#[doc = "ISO7816 interrupt status 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](sr1) module"]
pub type SR1 = crate::Reg<u32, _SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR1;
#[doc = "`read()` method returns [sr1::R](sr1::R) reader structure"]
impl crate::Readable for SR1 {}
#[doc = "`write(|w| ..)` method takes [sr1::W](sr1::W) writer structure"]
impl crate::Writable for SR1 {}
#[doc = "ISO7816 interrupt status 1"]
pub mod sr1;
#[doc = "ISO7816 resent count inquiry\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retxcntrmi](retxcntrmi) module"]
pub type RETXCNTRMI = crate::Reg<u32, _RETXCNTRMI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RETXCNTRMI;
#[doc = "`read()` method returns [retxcntrmi::R](retxcntrmi::R) reader structure"]
impl crate::Readable for RETXCNTRMI {}
#[doc = "`write(|w| ..)` method takes [retxcntrmi::W](retxcntrmi::W) writer structure"]
impl crate::Writable for RETXCNTRMI {}
#[doc = "ISO7816 resent count inquiry"]
pub mod retxcntrmi;
#[doc = "Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](clkctrl) module"]
pub type CLKCTRL = crate::Reg<u32, _CLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCTRL;
#[doc = "`read()` method returns [clkctrl::R](clkctrl::R) reader structure"]
impl crate::Readable for CLKCTRL {}
#[doc = "`write(|w| ..)` method takes [clkctrl::W](clkctrl::W) writer structure"]
impl crate::Writable for CLKCTRL {}
#[doc = "Clock Control"]
pub mod clkctrl;
