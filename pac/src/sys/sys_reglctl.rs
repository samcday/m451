#[doc = "Register `SYS_REGLCTL` reader"]
pub struct R(crate::R<SYS_REGLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_REGLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_REGLCTL_SPEC>> for R {
    fn from(reader: crate::R<SYS_REGLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_REGLCTL` writer"]
pub struct W(crate::W<SYS_REGLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_REGLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<SYS_REGLCTL_SPEC>> for W {
    fn from(writer: crate::W<SYS_REGLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Register Lock Control Code (Write Only)\\nSome registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write.\\nRegister Lock Control Disable Index (Read Only)\\nThe Protected registers are:\\nSYS_IPRST0: address 0x4000_0008\\nSYS_BODCTL: address 0x4000_0018\\nSYS_PORCTL: address 0x4000_0024\\nSYS_VREFCTL: address 0x4000_0028\\nSYS_USBPHY: address 0x4000_002C\\nCLK_PWRCTL: address 0x4000_0200 (bit\\[6\\]
is not protected for power-down wake-up interrupt clear)\\nSYS_SRAM_BISTCTL: address 0x4000_00D0\\nCLK_APBCLK0 \\[0\\]: address 0x4000_0208 (bit\\[0\\]
is watchdog clock enable)\\nCLK_CLKSEL0: address 0x4000_0210 (for HCLK and CPU STCLK clock source select)\\nCLK_CLKSEL1 \\[1:0\\]: address 0x4000_0214 (for watchdog clock source select)\\nCLK_CLKSEL1 \\[31:30\\]: address 0x4000_0214 (for window watchdog clock source select)\\nCLK_CLKDSTS: address 0x4000_0274\\nNMIEN: address 0x4000_0300\\nFMC_ISPCTL: address 0x4000_C000 (Flash ISP Control register)\\nFMC_ISPTRG: address 0x4000_C010 (ISP Trigger Control register)\\nFMC_ISPSTS: address 0x4000_C040\\nWDT_CTL: address 0x4004_0000\\nFMC_FTCTL: address 0x4000_5018\\nSYS_AHBMCTL: address 0x40000400\\nCLK_PLLCTL: address 0x40000240\\nPWM_CTL0: address 0x4005_8000 \\nPWM_CTL0: address 0x4005_9000\\nPWM_DTCTL0_1: address 0x4005_8070\\nPWM_DTCTL0_1: address 0x4005_9070\\nPWM_DTCTL2_3: address 0x4005_8074\\nPWM_DTCTL2_3: address 0x4005_9074\\nPWM_DTCTL4_5: address 0x4005_8078\\nPWM_DTCTL4_5: address 0x4005_9078\\nPWM_BRKCTL0_1: address 0x4005_80C8\\nPWM_BRKCTL0_1: address 0x4005_90C8\\nPWM_BRKCTL2_3: address 0x4005_80CC\\nPWM_BRKCTL2_3: address 0x4005_90CC\\nPWM_BRKCTL4_5: address 0x4005_80D0\\nPWM_BRKCTL4_5: address 0x4005_90D0\\nPWM_INTEN1: address 0x4005_80E4\\nPWM_INTEN1: address 0x4005_90E4\\nPWM_INTSTS1: address 0x4005_80EC\\nPWM_INTSTS1: address 0x4005_90EC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REGLCTL_A {
    #[doc = "0: Write-protection Enabled for writing protected registers. Any write to the protected register is ignored"]
    _0 = 0,
    #[doc = "1: Write-protection Disabled for writing protected registers"]
    _1 = 1,
}
impl From<REGLCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: REGLCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REGLCTL` reader - Register Lock Control Code (Write Only)\\nSome registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write.\\nRegister Lock Control Disable Index (Read Only)\\nThe Protected registers are:\\nSYS_IPRST0: address 0x4000_0008\\nSYS_BODCTL: address 0x4000_0018\\nSYS_PORCTL: address 0x4000_0024\\nSYS_VREFCTL: address 0x4000_0028\\nSYS_USBPHY: address 0x4000_002C\\nCLK_PWRCTL: address 0x4000_0200 (bit\\[6\\]
is not protected for power-down wake-up interrupt clear)\\nSYS_SRAM_BISTCTL: address 0x4000_00D0\\nCLK_APBCLK0 \\[0\\]: address 0x4000_0208 (bit\\[0\\]
is watchdog clock enable)\\nCLK_CLKSEL0: address 0x4000_0210 (for HCLK and CPU STCLK clock source select)\\nCLK_CLKSEL1 \\[1:0\\]: address 0x4000_0214 (for watchdog clock source select)\\nCLK_CLKSEL1 \\[31:30\\]: address 0x4000_0214 (for window watchdog clock source select)\\nCLK_CLKDSTS: address 0x4000_0274\\nNMIEN: address 0x4000_0300\\nFMC_ISPCTL: address 0x4000_C000 (Flash ISP Control register)\\nFMC_ISPTRG: address 0x4000_C010 (ISP Trigger Control register)\\nFMC_ISPSTS: address 0x4000_C040\\nWDT_CTL: address 0x4004_0000\\nFMC_FTCTL: address 0x4000_5018\\nSYS_AHBMCTL: address 0x40000400\\nCLK_PLLCTL: address 0x40000240\\nPWM_CTL0: address 0x4005_8000 \\nPWM_CTL0: address 0x4005_9000\\nPWM_DTCTL0_1: address 0x4005_8070\\nPWM_DTCTL0_1: address 0x4005_9070\\nPWM_DTCTL2_3: address 0x4005_8074\\nPWM_DTCTL2_3: address 0x4005_9074\\nPWM_DTCTL4_5: address 0x4005_8078\\nPWM_DTCTL4_5: address 0x4005_9078\\nPWM_BRKCTL0_1: address 0x4005_80C8\\nPWM_BRKCTL0_1: address 0x4005_90C8\\nPWM_BRKCTL2_3: address 0x4005_80CC\\nPWM_BRKCTL2_3: address 0x4005_90CC\\nPWM_BRKCTL4_5: address 0x4005_80D0\\nPWM_BRKCTL4_5: address 0x4005_90D0\\nPWM_INTEN1: address 0x4005_80E4\\nPWM_INTEN1: address 0x4005_90E4\\nPWM_INTSTS1: address 0x4005_80EC\\nPWM_INTSTS1: address 0x4005_90EC"]
pub struct REGLCTL_R(crate::FieldReader<u8, REGLCTL_A>);
impl REGLCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGLCTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REGLCTL_A> {
        match self.bits {
            0 => Some(REGLCTL_A::_0),
            1 => Some(REGLCTL_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REGLCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REGLCTL_A::_1
    }
}
impl core::ops::Deref for REGLCTL_R {
    type Target = crate::FieldReader<u8, REGLCTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGLCTL` writer - Register Lock Control Code (Write Only)\\nSome registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write.\\nRegister Lock Control Disable Index (Read Only)\\nThe Protected registers are:\\nSYS_IPRST0: address 0x4000_0008\\nSYS_BODCTL: address 0x4000_0018\\nSYS_PORCTL: address 0x4000_0024\\nSYS_VREFCTL: address 0x4000_0028\\nSYS_USBPHY: address 0x4000_002C\\nCLK_PWRCTL: address 0x4000_0200 (bit\\[6\\]
is not protected for power-down wake-up interrupt clear)\\nSYS_SRAM_BISTCTL: address 0x4000_00D0\\nCLK_APBCLK0 \\[0\\]: address 0x4000_0208 (bit\\[0\\]
is watchdog clock enable)\\nCLK_CLKSEL0: address 0x4000_0210 (for HCLK and CPU STCLK clock source select)\\nCLK_CLKSEL1 \\[1:0\\]: address 0x4000_0214 (for watchdog clock source select)\\nCLK_CLKSEL1 \\[31:30\\]: address 0x4000_0214 (for window watchdog clock source select)\\nCLK_CLKDSTS: address 0x4000_0274\\nNMIEN: address 0x4000_0300\\nFMC_ISPCTL: address 0x4000_C000 (Flash ISP Control register)\\nFMC_ISPTRG: address 0x4000_C010 (ISP Trigger Control register)\\nFMC_ISPSTS: address 0x4000_C040\\nWDT_CTL: address 0x4004_0000\\nFMC_FTCTL: address 0x4000_5018\\nSYS_AHBMCTL: address 0x40000400\\nCLK_PLLCTL: address 0x40000240\\nPWM_CTL0: address 0x4005_8000 \\nPWM_CTL0: address 0x4005_9000\\nPWM_DTCTL0_1: address 0x4005_8070\\nPWM_DTCTL0_1: address 0x4005_9070\\nPWM_DTCTL2_3: address 0x4005_8074\\nPWM_DTCTL2_3: address 0x4005_9074\\nPWM_DTCTL4_5: address 0x4005_8078\\nPWM_DTCTL4_5: address 0x4005_9078\\nPWM_BRKCTL0_1: address 0x4005_80C8\\nPWM_BRKCTL0_1: address 0x4005_90C8\\nPWM_BRKCTL2_3: address 0x4005_80CC\\nPWM_BRKCTL2_3: address 0x4005_90CC\\nPWM_BRKCTL4_5: address 0x4005_80D0\\nPWM_BRKCTL4_5: address 0x4005_90D0\\nPWM_INTEN1: address 0x4005_80E4\\nPWM_INTEN1: address 0x4005_90E4\\nPWM_INTSTS1: address 0x4005_80EC\\nPWM_INTSTS1: address 0x4005_90EC"]
pub struct REGLCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGLCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGLCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Write-protection Enabled for writing protected registers. Any write to the protected register is ignored"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REGLCTL_A::_0)
    }
    #[doc = "Write-protection Disabled for writing protected registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REGLCTL_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Register Lock Control Code (Write Only)\\nSome registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write.\\nRegister Lock Control Disable Index (Read Only)\\nThe Protected registers are:\\nSYS_IPRST0: address 0x4000_0008\\nSYS_BODCTL: address 0x4000_0018\\nSYS_PORCTL: address 0x4000_0024\\nSYS_VREFCTL: address 0x4000_0028\\nSYS_USBPHY: address 0x4000_002C\\nCLK_PWRCTL: address 0x4000_0200 (bit\\[6\\]
is not protected for power-down wake-up interrupt clear)\\nSYS_SRAM_BISTCTL: address 0x4000_00D0\\nCLK_APBCLK0 \\[0\\]: address 0x4000_0208 (bit\\[0\\]
is watchdog clock enable)\\nCLK_CLKSEL0: address 0x4000_0210 (for HCLK and CPU STCLK clock source select)\\nCLK_CLKSEL1 \\[1:0\\]: address 0x4000_0214 (for watchdog clock source select)\\nCLK_CLKSEL1 \\[31:30\\]: address 0x4000_0214 (for window watchdog clock source select)\\nCLK_CLKDSTS: address 0x4000_0274\\nNMIEN: address 0x4000_0300\\nFMC_ISPCTL: address 0x4000_C000 (Flash ISP Control register)\\nFMC_ISPTRG: address 0x4000_C010 (ISP Trigger Control register)\\nFMC_ISPSTS: address 0x4000_C040\\nWDT_CTL: address 0x4004_0000\\nFMC_FTCTL: address 0x4000_5018\\nSYS_AHBMCTL: address 0x40000400\\nCLK_PLLCTL: address 0x40000240\\nPWM_CTL0: address 0x4005_8000 \\nPWM_CTL0: address 0x4005_9000\\nPWM_DTCTL0_1: address 0x4005_8070\\nPWM_DTCTL0_1: address 0x4005_9070\\nPWM_DTCTL2_3: address 0x4005_8074\\nPWM_DTCTL2_3: address 0x4005_9074\\nPWM_DTCTL4_5: address 0x4005_8078\\nPWM_DTCTL4_5: address 0x4005_9078\\nPWM_BRKCTL0_1: address 0x4005_80C8\\nPWM_BRKCTL0_1: address 0x4005_90C8\\nPWM_BRKCTL2_3: address 0x4005_80CC\\nPWM_BRKCTL2_3: address 0x4005_90CC\\nPWM_BRKCTL4_5: address 0x4005_80D0\\nPWM_BRKCTL4_5: address 0x4005_90D0\\nPWM_INTEN1: address 0x4005_80E4\\nPWM_INTEN1: address 0x4005_90E4\\nPWM_INTSTS1: address 0x4005_80EC\\nPWM_INTSTS1: address 0x4005_90EC"]
    #[inline(always)]
    pub fn reglctl(&self) -> REGLCTL_R {
        REGLCTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register Lock Control Code (Write Only)\\nSome registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write.\\nRegister Lock Control Disable Index (Read Only)\\nThe Protected registers are:\\nSYS_IPRST0: address 0x4000_0008\\nSYS_BODCTL: address 0x4000_0018\\nSYS_PORCTL: address 0x4000_0024\\nSYS_VREFCTL: address 0x4000_0028\\nSYS_USBPHY: address 0x4000_002C\\nCLK_PWRCTL: address 0x4000_0200 (bit\\[6\\]
is not protected for power-down wake-up interrupt clear)\\nSYS_SRAM_BISTCTL: address 0x4000_00D0\\nCLK_APBCLK0 \\[0\\]: address 0x4000_0208 (bit\\[0\\]
is watchdog clock enable)\\nCLK_CLKSEL0: address 0x4000_0210 (for HCLK and CPU STCLK clock source select)\\nCLK_CLKSEL1 \\[1:0\\]: address 0x4000_0214 (for watchdog clock source select)\\nCLK_CLKSEL1 \\[31:30\\]: address 0x4000_0214 (for window watchdog clock source select)\\nCLK_CLKDSTS: address 0x4000_0274\\nNMIEN: address 0x4000_0300\\nFMC_ISPCTL: address 0x4000_C000 (Flash ISP Control register)\\nFMC_ISPTRG: address 0x4000_C010 (ISP Trigger Control register)\\nFMC_ISPSTS: address 0x4000_C040\\nWDT_CTL: address 0x4004_0000\\nFMC_FTCTL: address 0x4000_5018\\nSYS_AHBMCTL: address 0x40000400\\nCLK_PLLCTL: address 0x40000240\\nPWM_CTL0: address 0x4005_8000 \\nPWM_CTL0: address 0x4005_9000\\nPWM_DTCTL0_1: address 0x4005_8070\\nPWM_DTCTL0_1: address 0x4005_9070\\nPWM_DTCTL2_3: address 0x4005_8074\\nPWM_DTCTL2_3: address 0x4005_9074\\nPWM_DTCTL4_5: address 0x4005_8078\\nPWM_DTCTL4_5: address 0x4005_9078\\nPWM_BRKCTL0_1: address 0x4005_80C8\\nPWM_BRKCTL0_1: address 0x4005_90C8\\nPWM_BRKCTL2_3: address 0x4005_80CC\\nPWM_BRKCTL2_3: address 0x4005_90CC\\nPWM_BRKCTL4_5: address 0x4005_80D0\\nPWM_BRKCTL4_5: address 0x4005_90D0\\nPWM_INTEN1: address 0x4005_80E4\\nPWM_INTEN1: address 0x4005_90E4\\nPWM_INTSTS1: address 0x4005_80EC\\nPWM_INTSTS1: address 0x4005_90EC"]
    #[inline(always)]
    pub fn reglctl(&mut self) -> REGLCTL_W {
        REGLCTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register Lock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reglctl](index.html) module"]
pub struct SYS_REGLCTL_SPEC;
impl crate::RegisterSpec for SYS_REGLCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_reglctl::R](R) reader structure"]
impl crate::Readable for SYS_REGLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_reglctl::W](W) writer structure"]
impl crate::Writable for SYS_REGLCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_REGLCTL to value 0"]
impl crate::Resettable for SYS_REGLCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
