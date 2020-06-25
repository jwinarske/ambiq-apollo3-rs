#[doc = "Reader of register DEVPWRSTATUS"]
pub type R = crate::R<u32, super::DEVPWRSTATUS>;
#[doc = "Writer for register DEVPWRSTATUS"]
pub type W = crate::W<u32, super::DEVPWRSTATUS>;
#[doc = "Register DEVPWRSTATUS `reset()`'s with value 0x03"]
impl crate::ResetValue for super::DEVPWRSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `SYSDEEPSLEEP`"]
pub type SYSDEEPSLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSDEEPSLEEP`"]
pub struct SYSDEEPSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSDEEPSLEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `COREDEEPSLEEP`"]
pub type COREDEEPSLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COREDEEPSLEEP`"]
pub struct COREDEEPSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> COREDEEPSLEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CORESLEEP`"]
pub type CORESLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CORESLEEP`"]
pub struct CORESLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CORESLEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `BLEH`"]
pub type BLEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEH`"]
pub struct BLEH_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `BLEL`"]
pub type BLEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEL`"]
pub struct BLEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PWRPDM`"]
pub type PWRPDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRPDM`"]
pub struct PWRPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRPDM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PWRMSPI`"]
pub type PWRMSPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRMSPI`"]
pub struct PWRMSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRMSPI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PWRADC`"]
pub type PWRADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRADC`"]
pub struct PWRADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRADC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `HCPC`"]
pub type HCPC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCPC`"]
pub struct HCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `HCPB`"]
pub type HCPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCPB`"]
pub struct HCPB_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `HCPA`"]
pub type HCPA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCPA`"]
pub struct HCPA_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MCUH`"]
pub type MCUH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCUH`"]
pub struct MCUH_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MCUL`"]
pub type MCUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCUL`"]
pub struct MCUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - This bit is 1 if SYSTEM has been in Deep Sleep. Write '1' to this bit to clear it."]
    #[inline(always)]
    pub fn sysdeepsleep(&self) -> SYSDEEPSLEEP_R {
        SYSDEEPSLEEP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit is 1 if CORE has been in Deep Sleep. Write '1' to this bit to clear it."]
    #[inline(always)]
    pub fn coredeepsleep(&self) -> COREDEEPSLEEP_R {
        COREDEEPSLEEP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This bit is 1 if CORE has been in SLEEP State. Write '1' to this bit to clear it."]
    #[inline(always)]
    pub fn coresleep(&self) -> CORESLEEP_R {
        CORESLEEP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit is 1 if power is supplied to BLEH"]
    #[inline(always)]
    pub fn bleh(&self) -> BLEH_R {
        BLEH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to BLEL"]
    #[inline(always)]
    pub fn blel(&self) -> BLEL_R {
        BLEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to PDM"]
    #[inline(always)]
    pub fn pwrpdm(&self) -> PWRPDM_R {
        PWRPDM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to MSPI"]
    #[inline(always)]
    pub fn pwrmspi(&self) -> PWRMSPI_R {
        PWRMSPI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to ADC"]
    #[inline(always)]
    pub fn pwradc(&self) -> PWRADC_R {
        PWRADC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to HCPC domain (IO MASTER4, 5, 6)"]
    #[inline(always)]
    pub fn hcpc(&self) -> HCPC_R {
        HCPC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to HCPB domain (IO MASTER 0, 1, 2)"]
    #[inline(always)]
    pub fn hcpb(&self) -> HCPB_R {
        HCPB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to HCPA domain (IO SLAVE, UART0, UART1, SCARD)"]
    #[inline(always)]
    pub fn hcpa(&self) -> HCPA_R {
        HCPA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to MCUH"]
    #[inline(always)]
    pub fn mcuh(&self) -> MCUH_R {
        MCUH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit is 1 if power is supplied to MCUL"]
    #[inline(always)]
    pub fn mcul(&self) -> MCUL_R {
        MCUL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - This bit is 1 if SYSTEM has been in Deep Sleep. Write '1' to this bit to clear it."]
    #[inline(always)]
    pub fn sysdeepsleep(&mut self) -> SYSDEEPSLEEP_W {
        SYSDEEPSLEEP_W { w: self }
    }
    #[doc = "Bit 30 - This bit is 1 if CORE has been in Deep Sleep. Write '1' to this bit to clear it."]
    #[inline(always)]
    pub fn coredeepsleep(&mut self) -> COREDEEPSLEEP_W {
        COREDEEPSLEEP_W { w: self }
    }
    #[doc = "Bit 29 - This bit is 1 if CORE has been in SLEEP State. Write '1' to this bit to clear it."]
    #[inline(always)]
    pub fn coresleep(&mut self) -> CORESLEEP_W {
        CORESLEEP_W { w: self }
    }
    #[doc = "Bit 9 - This bit is 1 if power is supplied to BLEH"]
    #[inline(always)]
    pub fn bleh(&mut self) -> BLEH_W {
        BLEH_W { w: self }
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to BLEL"]
    #[inline(always)]
    pub fn blel(&mut self) -> BLEL_W {
        BLEL_W { w: self }
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to PDM"]
    #[inline(always)]
    pub fn pwrpdm(&mut self) -> PWRPDM_W {
        PWRPDM_W { w: self }
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to MSPI"]
    #[inline(always)]
    pub fn pwrmspi(&mut self) -> PWRMSPI_W {
        PWRMSPI_W { w: self }
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to ADC"]
    #[inline(always)]
    pub fn pwradc(&mut self) -> PWRADC_W {
        PWRADC_W { w: self }
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to HCPC domain (IO MASTER4, 5, 6)"]
    #[inline(always)]
    pub fn hcpc(&mut self) -> HCPC_W {
        HCPC_W { w: self }
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to HCPB domain (IO MASTER 0, 1, 2)"]
    #[inline(always)]
    pub fn hcpb(&mut self) -> HCPB_W {
        HCPB_W { w: self }
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to HCPA domain (IO SLAVE, UART0, UART1, SCARD)"]
    #[inline(always)]
    pub fn hcpa(&mut self) -> HCPA_W {
        HCPA_W { w: self }
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to MCUH"]
    #[inline(always)]
    pub fn mcuh(&mut self) -> MCUH_W {
        MCUH_W { w: self }
    }
    #[doc = "Bit 0 - This bit is 1 if power is supplied to MCUL"]
    #[inline(always)]
    pub fn mcul(&mut self) -> MCUL_W {
        MCUL_W { w: self }
    }
}
