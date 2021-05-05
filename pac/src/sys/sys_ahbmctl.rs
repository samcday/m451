#[doc = "Register `SYS_AHBMCTL` reader"]
pub struct R(crate::R<SYS_AHBMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_AHBMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_AHBMCTL_SPEC>> for R {
    fn from(reader: crate::R<SYS_AHBMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_AHBMCTL` writer"]
pub struct W(crate::W<SYS_AHBMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_AHBMCTL_SPEC>;
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
impl core::convert::From<crate::W<SYS_AHBMCTL_SPEC>> for W {
    fn from(writer: crate::W<SYS_AHBMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Highest AHB Bus Priority of Cortex M4 Core Enable Bit (Write Protect)\\nEnable Cortex-M4 Core With Highest AHB Bus Priority In AHB Bus Matrix\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTACTEN_A {
    #[doc = "0: Run robin mode"]
    _0 = 0,
    #[doc = "1: Cortex-M4 CPU with highest bus priority when interrupt occusr"]
    _1 = 1,
}
impl From<INTACTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTACTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTACTEN` reader - Highest AHB Bus Priority of Cortex M4 Core Enable Bit (Write Protect)\\nEnable Cortex-M4 Core With Highest AHB Bus Priority In AHB Bus Matrix\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct INTACTEN_R(crate::FieldReader<bool, INTACTEN_A>);
impl INTACTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTACTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTACTEN_A {
        match self.bits {
            false => INTACTEN_A::_0,
            true => INTACTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTACTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTACTEN_A::_1
    }
}
impl core::ops::Deref for INTACTEN_R {
    type Target = crate::FieldReader<bool, INTACTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTACTEN` writer - Highest AHB Bus Priority of Cortex M4 Core Enable Bit (Write Protect)\\nEnable Cortex-M4 Core With Highest AHB Bus Priority In AHB Bus Matrix\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct INTACTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTACTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTACTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Run robin mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTACTEN_A::_0)
    }
    #[doc = "Cortex-M4 CPU with highest bus priority when interrupt occusr"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTACTEN_A::_1)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Highest AHB Bus Priority of Cortex M4 Core Enable Bit (Write Protect)\\nEnable Cortex-M4 Core With Highest AHB Bus Priority In AHB Bus Matrix\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn intacten(&self) -> INTACTEN_R {
        INTACTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Highest AHB Bus Priority of Cortex M4 Core Enable Bit (Write Protect)\\nEnable Cortex-M4 Core With Highest AHB Bus Priority In AHB Bus Matrix\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn intacten(&mut self) -> INTACTEN_W {
        INTACTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Bus Matrix Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ahbmctl](index.html) module"]
pub struct SYS_AHBMCTL_SPEC;
impl crate::RegisterSpec for SYS_AHBMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ahbmctl::R](R) reader structure"]
impl crate::Readable for SYS_AHBMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ahbmctl::W](W) writer structure"]
impl crate::Writable for SYS_AHBMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_AHBMCTL to value 0x01"]
impl crate::Resettable for SYS_AHBMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
