#[doc = "Register `FMC_MPDAT0` reader"]
pub struct R(crate::R<FMC_MPDAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_MPDAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_MPDAT0_SPEC>> for R {
    fn from(reader: crate::R<FMC_MPDAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_MPDAT0` writer"]
pub struct W(crate::W<FMC_MPDAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_MPDAT0_SPEC>;
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
impl core::convert::From<crate::W<FMC_MPDAT0_SPEC>> for W {
    fn from(writer: crate::W<FMC_MPDAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISPDAT0` reader - ISP Data 0\\nThis register is the first 32-bit data for 32-bit/64-bit/multi-word programming, and it is also the mirror of FMC_ISPDAT, both registers keep the same data."]
pub struct ISPDAT0_R(crate::FieldReader<u32, u32>);
impl ISPDAT0_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISPDAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISPDAT0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPDAT0` writer - ISP Data 0\\nThis register is the first 32-bit data for 32-bit/64-bit/multi-word programming, and it is also the mirror of FMC_ISPDAT, both registers keep the same data."]
pub struct ISPDAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPDAT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISP Data 0\\nThis register is the first 32-bit data for 32-bit/64-bit/multi-word programming, and it is also the mirror of FMC_ISPDAT, both registers keep the same data."]
    #[inline(always)]
    pub fn ispdat0(&self) -> ISPDAT0_R {
        ISPDAT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISP Data 0\\nThis register is the first 32-bit data for 32-bit/64-bit/multi-word programming, and it is also the mirror of FMC_ISPDAT, both registers keep the same data."]
    #[inline(always)]
    pub fn ispdat0(&mut self) -> ISPDAT0_W {
        ISPDAT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP Data0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_mpdat0](index.html) module"]
pub struct FMC_MPDAT0_SPEC;
impl crate::RegisterSpec for FMC_MPDAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_mpdat0::R](R) reader structure"]
impl crate::Readable for FMC_MPDAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_mpdat0::W](W) writer structure"]
impl crate::Writable for FMC_MPDAT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_MPDAT0 to value 0"]
impl crate::Resettable for FMC_MPDAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
