#[doc = "Register `FMC_ISPADDR` reader"]
pub struct R(crate::R<FMC_ISPADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_ISPADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_ISPADDR_SPEC>> for R {
    fn from(reader: crate::R<FMC_ISPADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_ISPADDR` writer"]
pub struct W(crate::W<FMC_ISPADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_ISPADDR_SPEC>;
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
impl core::convert::From<crate::W<FMC_ISPADDR_SPEC>> for W {
    fn from(writer: crate::W<FMC_ISPADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISPADDR` reader - ISP Address\\nThe NuMicro M451 series is equipped with embedded flash. ISPADDR\\[1:0\\]
must be kept 00 for ISP 32-bit operation. ISPADDR\\[2:0\\]
must be kept 000 for ISP 64-bit operation.\\nFor Checksum Calculation command, this field is the flash starting address for checksum calculation, 2 KB alignment is necessary for checksum calculation."]
pub struct ISPADDR_R(crate::FieldReader<u32, u32>);
impl ISPADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISPADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISPADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPADDR` writer - ISP Address\\nThe NuMicro M451 series is equipped with embedded flash. ISPADDR\\[1:0\\]
must be kept 00 for ISP 32-bit operation. ISPADDR\\[2:0\\]
must be kept 000 for ISP 64-bit operation.\\nFor Checksum Calculation command, this field is the flash starting address for checksum calculation, 2 KB alignment is necessary for checksum calculation."]
pub struct ISPADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISP Address\\nThe NuMicro M451 series is equipped with embedded flash. ISPADDR\\[1:0\\]
must be kept 00 for ISP 32-bit operation. ISPADDR\\[2:0\\]
must be kept 000 for ISP 64-bit operation.\\nFor Checksum Calculation command, this field is the flash starting address for checksum calculation, 2 KB alignment is necessary for checksum calculation."]
    #[inline(always)]
    pub fn ispaddr(&self) -> ISPADDR_R {
        ISPADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISP Address\\nThe NuMicro M451 series is equipped with embedded flash. ISPADDR\\[1:0\\]
must be kept 00 for ISP 32-bit operation. ISPADDR\\[2:0\\]
must be kept 000 for ISP 64-bit operation.\\nFor Checksum Calculation command, this field is the flash starting address for checksum calculation, 2 KB alignment is necessary for checksum calculation."]
    #[inline(always)]
    pub fn ispaddr(&mut self) -> ISPADDR_W {
        ISPADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ispaddr](index.html) module"]
pub struct FMC_ISPADDR_SPEC;
impl crate::RegisterSpec for FMC_ISPADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ispaddr::R](R) reader structure"]
impl crate::Readable for FMC_ISPADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ispaddr::W](W) writer structure"]
impl crate::Writable for FMC_ISPADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_ISPADDR to value 0"]
impl crate::Resettable for FMC_ISPADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
