#[doc = "Register `PDMA_DSCT9_SA` reader"]
pub struct R(crate::R<PDMA_DSCT9_SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_DSCT9_SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_DSCT9_SA_SPEC>> for R {
    fn from(reader: crate::R<PDMA_DSCT9_SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_DSCT9_SA` writer"]
pub struct W(crate::W<PDMA_DSCT9_SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_DSCT9_SA_SPEC>;
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
impl core::convert::From<crate::W<PDMA_DSCT9_SA_SPEC>> for W {
    fn from(writer: crate::W<PDMA_DSCT9_SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SA` reader - PDMA Transfer Source Address Register\\nThis field indicates a 32-bit source address of PDMA controller."]
pub struct SA_R(crate::FieldReader<u32, u32>);
impl SA_R {
    pub(crate) fn new(bits: u32) -> Self {
        SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA` writer - PDMA Transfer Source Address Register\\nThis field indicates a 32-bit source address of PDMA controller."]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PDMA Transfer Source Address Register\\nThis field indicates a 32-bit source address of PDMA controller."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PDMA Transfer Source Address Register\\nThis field indicates a 32-bit source address of PDMA controller."]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_dsct9_sa](index.html) module"]
pub struct PDMA_DSCT9_SA_SPEC;
impl crate::RegisterSpec for PDMA_DSCT9_SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_dsct9_sa::R](R) reader structure"]
impl crate::Readable for PDMA_DSCT9_SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_dsct9_sa::W](W) writer structure"]
impl crate::Writable for PDMA_DSCT9_SA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_DSCT9_SA to value 0"]
impl crate::Resettable for PDMA_DSCT9_SA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
