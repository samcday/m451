#[doc = "Register `FMC_FTCTL` reader"]
pub struct R(crate::R<FMC_FTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_FTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_FTCTL_SPEC>> for R {
    fn from(reader: crate::R<FMC_FTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_FTCTL` writer"]
pub struct W(crate::W<FMC_FTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_FTCTL_SPEC>;
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
impl core::convert::From<crate::W<FMC_FTCTL_SPEC>> for W {
    fn from(writer: crate::W<FMC_FTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frequency Optimization Mode (Write Protect)\\nThe NuMicro M451 series support adjustable flash access timing to optimize the flash access cycles in different working frequency.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FOM_A {
    #[doc = "1: Frequency   12MHz"]
    _1 = 1,
    #[doc = "2: Frequency   36MHz"]
    _2 = 2,
    #[doc = "4: Frequency   60MHz"]
    _4 = 4,
}
impl From<FOM_A> for u8 {
    #[inline(always)]
    fn from(variant: FOM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FOM` reader - Frequency Optimization Mode (Write Protect)\\nThe NuMicro M451 series support adjustable flash access timing to optimize the flash access cycles in different working frequency.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct FOM_R(crate::FieldReader<u8, FOM_A>);
impl FOM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FOM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FOM_A> {
        match self.bits {
            1 => Some(FOM_A::_1),
            2 => Some(FOM_A::_2),
            4 => Some(FOM_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FOM_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == FOM_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == FOM_A::_4
    }
}
impl core::ops::Deref for FOM_R {
    type Target = crate::FieldReader<u8, FOM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOM` writer - Frequency Optimization Mode (Write Protect)\\nThe NuMicro M451 series support adjustable flash access timing to optimize the flash access cycles in different working frequency.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct FOM_W<'a> {
    w: &'a mut W,
}
impl<'a> FOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Frequency 12MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FOM_A::_1)
    }
    #[doc = "Frequency 36MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FOM_A::_2)
    }
    #[doc = "Frequency 60MHz"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FOM_A::_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Frequency Optimization Mode (Write Protect)\\nThe NuMicro M451 series support adjustable flash access timing to optimize the flash access cycles in different working frequency.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn fom(&self) -> FOM_R {
        FOM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Frequency Optimization Mode (Write Protect)\\nThe NuMicro M451 series support adjustable flash access timing to optimize the flash access cycles in different working frequency.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn fom(&mut self) -> FOM_W {
        FOM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Access Time Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ftctl](index.html) module"]
pub struct FMC_FTCTL_SPEC;
impl crate::RegisterSpec for FMC_FTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ftctl::R](R) reader structure"]
impl crate::Readable for FMC_FTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ftctl::W](W) writer structure"]
impl crate::Writable for FMC_FTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_FTCTL to value 0"]
impl crate::Resettable for FMC_FTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
