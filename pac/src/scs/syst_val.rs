#[doc = "Register `SYST_VAL` reader"]
pub struct R(crate::R<SYST_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYST_VAL_SPEC>> for R {
    fn from(reader: crate::R<SYST_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_VAL` writer"]
pub struct W(crate::W<SYST_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_VAL_SPEC>;
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
impl core::convert::From<crate::W<SYST_VAL_SPEC>> for W {
    fn from(writer: crate::W<SYST_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRENT` reader - System Tick Current Value\\nCurrent counter value. This is the value of the counter at the time it is sampled. The counter does not provide read-modify-write protection. The register is write-clear. A software write of any value will clear the register to 0. Unsupported bits RAZ (see SysTick Reload Value register)."]
pub struct CURRENT_R(crate::FieldReader<u32, u32>);
impl CURRENT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRENT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURRENT` writer - System Tick Current Value\\nCurrent counter value. This is the value of the counter at the time it is sampled. The counter does not provide read-modify-write protection. The register is write-clear. A software write of any value will clear the register to 0. Unsupported bits RAZ (see SysTick Reload Value register)."]
pub struct CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - System Tick Current Value\\nCurrent counter value. This is the value of the counter at the time it is sampled. The counter does not provide read-modify-write protection. The register is write-clear. A software write of any value will clear the register to 0. Unsupported bits RAZ (see SysTick Reload Value register)."]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - System Tick Current Value\\nCurrent counter value. This is the value of the counter at the time it is sampled. The counter does not provide read-modify-write protection. The register is write-clear. A software write of any value will clear the register to 0. Unsupported bits RAZ (see SysTick Reload Value register)."]
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W {
        CURRENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_val](index.html) module"]
pub struct SYST_VAL_SPEC;
impl crate::RegisterSpec for SYST_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_val::R](R) reader structure"]
impl crate::Readable for SYST_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_val::W](W) writer structure"]
impl crate::Writable for SYST_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYST_VAL to value 0"]
impl crate::Resettable for SYST_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
