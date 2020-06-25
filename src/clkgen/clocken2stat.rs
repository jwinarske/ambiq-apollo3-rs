#[doc = "Reader of register CLOCKEN2STAT"]
pub type R = crate::R<u32, super::CLOCKEN2STAT>;
#[doc = "Writer for register CLOCKEN2STAT"]
pub type W = crate::W<u32, super::CLOCKEN2STAT>;
#[doc = "Register CLOCKEN2STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCKEN2STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock enable status 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLOCKEN2STAT_A {
    #[doc = "1: Clock enable for the IO MASTER 1 IFC INTERFACE value."]
    IOMSTRIFC1_CLKEN = 1,
    #[doc = "2: Clock enable for the IO MASTER 2 IFC INTERFACE value."]
    IOMSTRIFC2_CLKEN = 2,
    #[doc = "4: Clock enable for the IO MASTER 3 IFC INTERFACE value."]
    IOMSTRIFC3_CLKEN = 4,
    #[doc = "8: Clock enable for the IO MASTER 4 IFC INTERFACE value."]
    IOMSTRIFC4_CLKEN = 8,
    #[doc = "16: Clock enable for the IO MASTER 5 IFC INTERFACE value."]
    IOMSTRIFC5_CLKEN = 16,
    #[doc = "32: Clock enable for the PDM value."]
    PDM_CLKEN = 32,
    #[doc = "64: Clock enable for the PDM INTERFACE value."]
    PDMIFC_CLKEN = 64,
    #[doc = "128: Clock enable for the PWRCTRL value."]
    PWRCTRL_CLKEN = 128,
    #[doc = "256: Clock enable for the RSTGEN value."]
    RSTGEN_CLKEN = 256,
    #[doc = "512: Clock enable for the SCARD value."]
    SCARD_CLKEN = 512,
    #[doc = "1024: Clock enable for the SCARD ALTAPB value."]
    SCARD_ALTAPB_CLKEN = 1024,
    #[doc = "2048: Clock enable for the STIMER_CNT_CLKEN value."]
    STIMER_CNT_CLKEN = 2048,
    #[doc = "4096: Clock enable for the TPIU_CLKEN value."]
    TPIU_CLKEN = 4096,
    #[doc = "8192: Clock enable for the UART0 HF value."]
    UART0HF_CLKEN = 8192,
    #[doc = "16384: Clock enable for the UART1 HF value."]
    UART1HF_CLKEN = 16384,
    #[doc = "1073741824: Clock enable for the XT 32KHZ value."]
    XT_32KHZ_EN = 1073741824,
    #[doc = "2147483648: HFRC is forced on Status. value."]
    FORCEHFRC = 2147483648,
}
impl From<CLOCKEN2STAT_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKEN2STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLOCKEN2STAT`"]
pub type CLOCKEN2STAT_R = crate::R<u32, CLOCKEN2STAT_A>;
impl CLOCKEN2STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLOCKEN2STAT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CLOCKEN2STAT_A::IOMSTRIFC1_CLKEN),
            2 => Val(CLOCKEN2STAT_A::IOMSTRIFC2_CLKEN),
            4 => Val(CLOCKEN2STAT_A::IOMSTRIFC3_CLKEN),
            8 => Val(CLOCKEN2STAT_A::IOMSTRIFC4_CLKEN),
            16 => Val(CLOCKEN2STAT_A::IOMSTRIFC5_CLKEN),
            32 => Val(CLOCKEN2STAT_A::PDM_CLKEN),
            64 => Val(CLOCKEN2STAT_A::PDMIFC_CLKEN),
            128 => Val(CLOCKEN2STAT_A::PWRCTRL_CLKEN),
            256 => Val(CLOCKEN2STAT_A::RSTGEN_CLKEN),
            512 => Val(CLOCKEN2STAT_A::SCARD_CLKEN),
            1024 => Val(CLOCKEN2STAT_A::SCARD_ALTAPB_CLKEN),
            2048 => Val(CLOCKEN2STAT_A::STIMER_CNT_CLKEN),
            4096 => Val(CLOCKEN2STAT_A::TPIU_CLKEN),
            8192 => Val(CLOCKEN2STAT_A::UART0HF_CLKEN),
            16384 => Val(CLOCKEN2STAT_A::UART1HF_CLKEN),
            1073741824 => Val(CLOCKEN2STAT_A::XT_32KHZ_EN),
            2147483648 => Val(CLOCKEN2STAT_A::FORCEHFRC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC1_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc1_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::IOMSTRIFC1_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC2_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc2_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::IOMSTRIFC2_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC3_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc3_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::IOMSTRIFC3_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC4_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc4_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::IOMSTRIFC4_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC5_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc5_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::IOMSTRIFC5_CLKEN
    }
    #[doc = "Checks if the value of the field is `PDM_CLKEN`"]
    #[inline(always)]
    pub fn is_pdm_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::PDM_CLKEN
    }
    #[doc = "Checks if the value of the field is `PDMIFC_CLKEN`"]
    #[inline(always)]
    pub fn is_pdmifc_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::PDMIFC_CLKEN
    }
    #[doc = "Checks if the value of the field is `PWRCTRL_CLKEN`"]
    #[inline(always)]
    pub fn is_pwrctrl_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::PWRCTRL_CLKEN
    }
    #[doc = "Checks if the value of the field is `RSTGEN_CLKEN`"]
    #[inline(always)]
    pub fn is_rstgen_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::RSTGEN_CLKEN
    }
    #[doc = "Checks if the value of the field is `SCARD_CLKEN`"]
    #[inline(always)]
    pub fn is_scard_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::SCARD_CLKEN
    }
    #[doc = "Checks if the value of the field is `SCARD_ALTAPB_CLKEN`"]
    #[inline(always)]
    pub fn is_scard_altapb_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::SCARD_ALTAPB_CLKEN
    }
    #[doc = "Checks if the value of the field is `STIMER_CNT_CLKEN`"]
    #[inline(always)]
    pub fn is_stimer_cnt_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::STIMER_CNT_CLKEN
    }
    #[doc = "Checks if the value of the field is `TPIU_CLKEN`"]
    #[inline(always)]
    pub fn is_tpiu_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::TPIU_CLKEN
    }
    #[doc = "Checks if the value of the field is `UART0HF_CLKEN`"]
    #[inline(always)]
    pub fn is_uart0hf_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::UART0HF_CLKEN
    }
    #[doc = "Checks if the value of the field is `UART1HF_CLKEN`"]
    #[inline(always)]
    pub fn is_uart1hf_clken(&self) -> bool {
        *self == CLOCKEN2STAT_A::UART1HF_CLKEN
    }
    #[doc = "Checks if the value of the field is `XT_32KHZ_EN`"]
    #[inline(always)]
    pub fn is_xt_32khz_en(&self) -> bool {
        *self == CLOCKEN2STAT_A::XT_32KHZ_EN
    }
    #[doc = "Checks if the value of the field is `FORCEHFRC`"]
    #[inline(always)]
    pub fn is_forcehfrc(&self) -> bool {
        *self == CLOCKEN2STAT_A::FORCEHFRC
    }
}
#[doc = "Write proxy for field `CLOCKEN2STAT`"]
pub struct CLOCKEN2STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKEN2STAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKEN2STAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock enable for the IO MASTER 1 IFC INTERFACE value."]
    #[inline(always)]
    pub fn iomstrifc1_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::IOMSTRIFC1_CLKEN)
    }
    #[doc = "Clock enable for the IO MASTER 2 IFC INTERFACE value."]
    #[inline(always)]
    pub fn iomstrifc2_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::IOMSTRIFC2_CLKEN)
    }
    #[doc = "Clock enable for the IO MASTER 3 IFC INTERFACE value."]
    #[inline(always)]
    pub fn iomstrifc3_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::IOMSTRIFC3_CLKEN)
    }
    #[doc = "Clock enable for the IO MASTER 4 IFC INTERFACE value."]
    #[inline(always)]
    pub fn iomstrifc4_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::IOMSTRIFC4_CLKEN)
    }
    #[doc = "Clock enable for the IO MASTER 5 IFC INTERFACE value."]
    #[inline(always)]
    pub fn iomstrifc5_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::IOMSTRIFC5_CLKEN)
    }
    #[doc = "Clock enable for the PDM value."]
    #[inline(always)]
    pub fn pdm_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::PDM_CLKEN)
    }
    #[doc = "Clock enable for the PDM INTERFACE value."]
    #[inline(always)]
    pub fn pdmifc_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::PDMIFC_CLKEN)
    }
    #[doc = "Clock enable for the PWRCTRL value."]
    #[inline(always)]
    pub fn pwrctrl_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::PWRCTRL_CLKEN)
    }
    #[doc = "Clock enable for the RSTGEN value."]
    #[inline(always)]
    pub fn rstgen_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::RSTGEN_CLKEN)
    }
    #[doc = "Clock enable for the SCARD value."]
    #[inline(always)]
    pub fn scard_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::SCARD_CLKEN)
    }
    #[doc = "Clock enable for the SCARD ALTAPB value."]
    #[inline(always)]
    pub fn scard_altapb_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::SCARD_ALTAPB_CLKEN)
    }
    #[doc = "Clock enable for the STIMER_CNT_CLKEN value."]
    #[inline(always)]
    pub fn stimer_cnt_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::STIMER_CNT_CLKEN)
    }
    #[doc = "Clock enable for the TPIU_CLKEN value."]
    #[inline(always)]
    pub fn tpiu_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::TPIU_CLKEN)
    }
    #[doc = "Clock enable for the UART0 HF value."]
    #[inline(always)]
    pub fn uart0hf_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::UART0HF_CLKEN)
    }
    #[doc = "Clock enable for the UART1 HF value."]
    #[inline(always)]
    pub fn uart1hf_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::UART1HF_CLKEN)
    }
    #[doc = "Clock enable for the XT 32KHZ value."]
    #[inline(always)]
    pub fn xt_32khz_en(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::XT_32KHZ_EN)
    }
    #[doc = "HFRC is forced on Status. value."]
    #[inline(always)]
    pub fn forcehfrc(self) -> &'a mut W {
        self.variant(CLOCKEN2STAT_A::FORCEHFRC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status 2"]
    #[inline(always)]
    pub fn clocken2stat(&self) -> CLOCKEN2STAT_R {
        CLOCKEN2STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status 2"]
    #[inline(always)]
    pub fn clocken2stat(&mut self) -> CLOCKEN2STAT_W {
        CLOCKEN2STAT_W { w: self }
    }
}
