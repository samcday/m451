#[doc = "Register `FMC_MPDAT2` reader"]
pub struct R(crate::R<FMC_MPDAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_MPDAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_MPDAT2_SPEC>> for R {
    fn from(reader: crate::R<FMC_MPDAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_MPDAT2` writer"]
pub struct W(crate::W<FMC_MPDAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_MPDAT2_SPEC>;
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
impl core::convert::From<crate::W<FMC_MPDAT2_SPEC>> for W {
    fn from(writer: crate::W<FMC_MPDAT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISPDAT2` reader - ISP Data 2\\nThis register is the third 32-bit data for multi-word programming."]
pub struct ISPDAT2_R(crate::FieldReader<u32, u32>);
impl ISPDAT2_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISPDAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISPDAT2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPDAT2` writer - ISP Data 2\\nThis register is the third 32-bit data for multi-word programming."]
pub struct ISPDAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPDAT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISP Data 2\\nThis register is the third 32-bit data for multi-word programming."]
    #[inline(always)]
    pub fn ispdat2(&self) -> ISPDAT2_R {
        ISPDAT2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISP Data 2\\nThis register is the third 32-bit data for multi-word programming."]
    #[inline(always)]
    pub fn ispdat2(&mut self) -> ISPDAT2_W {
        ISPDAT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP Data2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_mpdat2](index.html) module"]
pub struct FMC_MPDAT2_SPEC;
impl crate::RegisterSpec for FMC_MPDAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_mpdat2::R](R) reader structure"]
impl crate::Readable for FMC_MPDAT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_mpdat2::W](W) writer structure"]
impl crate::Writable for FMC_MPDAT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_MPDAT2 to value 0"]
impl crate::Resettable for FMC_MPDAT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
