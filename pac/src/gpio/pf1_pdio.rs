#[doc = "Register `PF1_PDIO` reader"]
pub struct R(crate::R<PF1_PDIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF1_PDIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PF1_PDIO_SPEC>> for R {
    fn from(reader: crate::R<PF1_PDIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF1_PDIO` writer"]
pub struct W(crate::W<PF1_PDIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF1_PDIO_SPEC>;
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
impl core::convert::From<crate::W<PF1_PDIO_SPEC>> for W {
    fn from(writer: crate::W<PF1_PDIO_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PF.n Pin Data Input/Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf1_pdio](index.html) module"]
pub struct PF1_PDIO_SPEC;
impl crate::RegisterSpec for PF1_PDIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf1_pdio::R](R) reader structure"]
impl crate::Readable for PF1_PDIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf1_pdio::W](W) writer structure"]
impl crate::Writable for PF1_PDIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF1_PDIO to value 0"]
impl crate::Resettable for PF1_PDIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
