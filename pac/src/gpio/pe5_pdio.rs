#[doc = "Register `PE5_PDIO` reader"]
pub struct R(crate::R<PE5_PDIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE5_PDIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PE5_PDIO_SPEC>> for R {
    fn from(reader: crate::R<PE5_PDIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE5_PDIO` writer"]
pub struct W(crate::W<PE5_PDIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE5_PDIO_SPEC>;
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
impl core::convert::From<crate::W<PE5_PDIO_SPEC>> for W {
    fn from(writer: crate::W<PE5_PDIO_SPEC>) -> Self {
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
#[doc = "GPIO PE.n Pin Data Input/Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe5_pdio](index.html) module"]
pub struct PE5_PDIO_SPEC;
impl crate::RegisterSpec for PE5_PDIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe5_pdio::R](R) reader structure"]
impl crate::Readable for PE5_PDIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe5_pdio::W](W) writer structure"]
impl crate::Writable for PE5_PDIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE5_PDIO to value 0"]
impl crate::Resettable for PE5_PDIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
