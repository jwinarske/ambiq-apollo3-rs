#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Cache Control Register"]
    pub cachecfg: CACHECFG,
    #[doc = "0x04 - Flash Control Register"]
    pub flashcfg: FLASHCFG,
    #[doc = "0x08 - Cache Control"]
    pub ctrl: CTRL,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Flash Cache Noncachable Region 0 Start"]
    pub ncr0start: NCR0START,
    #[doc = "0x14 - Flash Cache Noncachable Region 0 End"]
    pub ncr0end: NCR0END,
    #[doc = "0x18 - Flash Cache Noncachable Region 1 Start"]
    pub ncr1start: NCR1START,
    #[doc = "0x1c - Flash Cache Noncachable Region 1 End"]
    pub ncr1end: NCR1END,
    _reserved7: [u8; 32usize],
    #[doc = "0x40 - Data Cache Total Accesses"]
    pub dmon0: DMON0,
    #[doc = "0x44 - Data Cache Tag Lookups"]
    pub dmon1: DMON1,
    #[doc = "0x48 - Data Cache Hits"]
    pub dmon2: DMON2,
    #[doc = "0x4c - Data Cache Line Hits"]
    pub dmon3: DMON3,
    #[doc = "0x50 - Instruction Cache Total Accesses"]
    pub imon0: IMON0,
    #[doc = "0x54 - Instruction Cache Tag Lookups"]
    pub imon1: IMON1,
    #[doc = "0x58 - Instruction Cache Hits"]
    pub imon2: IMON2,
    #[doc = "0x5c - Instruction Cache Line Hits"]
    pub imon3: IMON3,
}
#[doc = "Flash Cache Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachecfg](cachecfg) module"]
pub type CACHECFG = crate::Reg<u32, _CACHECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHECFG;
#[doc = "`read()` method returns [cachecfg::R](cachecfg::R) reader structure"]
impl crate::Readable for CACHECFG {}
#[doc = "`write(|w| ..)` method takes [cachecfg::W](cachecfg::W) writer structure"]
impl crate::Writable for CACHECFG {}
#[doc = "Flash Cache Control Register"]
pub mod cachecfg;
#[doc = "Flash Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](flashcfg) module"]
pub type FLASHCFG = crate::Reg<u32, _FLASHCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCFG;
#[doc = "`read()` method returns [flashcfg::R](flashcfg::R) reader structure"]
impl crate::Readable for FLASHCFG {}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](flashcfg::W) writer structure"]
impl crate::Writable for FLASHCFG {}
#[doc = "Flash Control Register"]
pub mod flashcfg;
#[doc = "Cache Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Cache Control"]
pub mod ctrl;
#[doc = "Flash Cache Noncachable Region 0 Start\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr0start](ncr0start) module"]
pub type NCR0START = crate::Reg<u32, _NCR0START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCR0START;
#[doc = "`read()` method returns [ncr0start::R](ncr0start::R) reader structure"]
impl crate::Readable for NCR0START {}
#[doc = "`write(|w| ..)` method takes [ncr0start::W](ncr0start::W) writer structure"]
impl crate::Writable for NCR0START {}
#[doc = "Flash Cache Noncachable Region 0 Start"]
pub mod ncr0start;
#[doc = "Flash Cache Noncachable Region 0 End\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr0end](ncr0end) module"]
pub type NCR0END = crate::Reg<u32, _NCR0END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCR0END;
#[doc = "`read()` method returns [ncr0end::R](ncr0end::R) reader structure"]
impl crate::Readable for NCR0END {}
#[doc = "`write(|w| ..)` method takes [ncr0end::W](ncr0end::W) writer structure"]
impl crate::Writable for NCR0END {}
#[doc = "Flash Cache Noncachable Region 0 End"]
pub mod ncr0end;
#[doc = "Flash Cache Noncachable Region 1 Start\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr1start](ncr1start) module"]
pub type NCR1START = crate::Reg<u32, _NCR1START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCR1START;
#[doc = "`read()` method returns [ncr1start::R](ncr1start::R) reader structure"]
impl crate::Readable for NCR1START {}
#[doc = "`write(|w| ..)` method takes [ncr1start::W](ncr1start::W) writer structure"]
impl crate::Writable for NCR1START {}
#[doc = "Flash Cache Noncachable Region 1 Start"]
pub mod ncr1start;
#[doc = "Flash Cache Noncachable Region 1 End\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr1end](ncr1end) module"]
pub type NCR1END = crate::Reg<u32, _NCR1END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCR1END;
#[doc = "`read()` method returns [ncr1end::R](ncr1end::R) reader structure"]
impl crate::Readable for NCR1END {}
#[doc = "`write(|w| ..)` method takes [ncr1end::W](ncr1end::W) writer structure"]
impl crate::Writable for NCR1END {}
#[doc = "Flash Cache Noncachable Region 1 End"]
pub mod ncr1end;
#[doc = "Data Cache Total Accesses\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmon0](dmon0) module"]
pub type DMON0 = crate::Reg<u32, _DMON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMON0;
#[doc = "`read()` method returns [dmon0::R](dmon0::R) reader structure"]
impl crate::Readable for DMON0 {}
#[doc = "`write(|w| ..)` method takes [dmon0::W](dmon0::W) writer structure"]
impl crate::Writable for DMON0 {}
#[doc = "Data Cache Total Accesses"]
pub mod dmon0;
#[doc = "Data Cache Tag Lookups\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmon1](dmon1) module"]
pub type DMON1 = crate::Reg<u32, _DMON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMON1;
#[doc = "`read()` method returns [dmon1::R](dmon1::R) reader structure"]
impl crate::Readable for DMON1 {}
#[doc = "`write(|w| ..)` method takes [dmon1::W](dmon1::W) writer structure"]
impl crate::Writable for DMON1 {}
#[doc = "Data Cache Tag Lookups"]
pub mod dmon1;
#[doc = "Data Cache Hits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmon2](dmon2) module"]
pub type DMON2 = crate::Reg<u32, _DMON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMON2;
#[doc = "`read()` method returns [dmon2::R](dmon2::R) reader structure"]
impl crate::Readable for DMON2 {}
#[doc = "`write(|w| ..)` method takes [dmon2::W](dmon2::W) writer structure"]
impl crate::Writable for DMON2 {}
#[doc = "Data Cache Hits"]
pub mod dmon2;
#[doc = "Data Cache Line Hits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmon3](dmon3) module"]
pub type DMON3 = crate::Reg<u32, _DMON3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMON3;
#[doc = "`read()` method returns [dmon3::R](dmon3::R) reader structure"]
impl crate::Readable for DMON3 {}
#[doc = "`write(|w| ..)` method takes [dmon3::W](dmon3::W) writer structure"]
impl crate::Writable for DMON3 {}
#[doc = "Data Cache Line Hits"]
pub mod dmon3;
#[doc = "Instruction Cache Total Accesses\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imon0](imon0) module"]
pub type IMON0 = crate::Reg<u32, _IMON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMON0;
#[doc = "`read()` method returns [imon0::R](imon0::R) reader structure"]
impl crate::Readable for IMON0 {}
#[doc = "`write(|w| ..)` method takes [imon0::W](imon0::W) writer structure"]
impl crate::Writable for IMON0 {}
#[doc = "Instruction Cache Total Accesses"]
pub mod imon0;
#[doc = "Instruction Cache Tag Lookups\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imon1](imon1) module"]
pub type IMON1 = crate::Reg<u32, _IMON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMON1;
#[doc = "`read()` method returns [imon1::R](imon1::R) reader structure"]
impl crate::Readable for IMON1 {}
#[doc = "`write(|w| ..)` method takes [imon1::W](imon1::W) writer structure"]
impl crate::Writable for IMON1 {}
#[doc = "Instruction Cache Tag Lookups"]
pub mod imon1;
#[doc = "Instruction Cache Hits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imon2](imon2) module"]
pub type IMON2 = crate::Reg<u32, _IMON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMON2;
#[doc = "`read()` method returns [imon2::R](imon2::R) reader structure"]
impl crate::Readable for IMON2 {}
#[doc = "`write(|w| ..)` method takes [imon2::W](imon2::W) writer structure"]
impl crate::Writable for IMON2 {}
#[doc = "Instruction Cache Hits"]
pub mod imon2;
#[doc = "Instruction Cache Line Hits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imon3](imon3) module"]
pub type IMON3 = crate::Reg<u32, _IMON3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMON3;
#[doc = "`read()` method returns [imon3::R](imon3::R) reader structure"]
impl crate::Readable for IMON3 {}
#[doc = "`write(|w| ..)` method takes [imon3::W](imon3::W) writer structure"]
impl crate::Writable for IMON3 {}
#[doc = "Instruction Cache Line Hits"]
pub mod imon3;
