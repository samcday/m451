#[doc = "Register `SC_EGT` reader"]
pub struct R(crate::R<SC_EGT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_EGT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_EGT_SPEC>> for R {
    fn from(reader: crate::R<SC_EGT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_EGT` writer"]
pub struct W(crate::W<SC_EGT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_EGT_SPEC>;
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
impl core::convert::From<crate::W<SC_EGT_SPEC>> for W {
    fn from(writer: crate::W<SC_EGT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EGT` reader - Extended Guard Time\\nThis field indicates the extended guard timer value.\\nNote: The counter is ETU base and the real extended guard time is EGT."]
pub struct EGT_R(crate::FieldReader<u8, u8>);
impl EGT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EGT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EGT` writer - Extended Guard Time\\nThis field indicates the extended guard timer value.\\nNote: The counter is ETU base and the real extended guard time is EGT."]
pub struct EGT_W<'a> {
    w: &'a mut W,
}
impl<'a> EGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Extended Guard Time\\nThis field indicates the extended guard timer value.\\nNote: The counter is ETU base and the real extended guard time is EGT."]
    #[inline(always)]
    pub fn egt(&self) -> EGT_R {
        EGT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Extended Guard Time\\nThis field indicates the extended guard timer value.\\nNote: The counter is ETU base and the real extended guard time is EGT."]
    #[inline(always)]
    pub fn egt(&mut self) -> EGT_W {
        EGT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Extend Guard Time Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_egt](index.html) module"]
pub struct SC_EGT_SPEC;
impl crate::RegisterSpec for SC_EGT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_egt::R](R) reader structure"]
impl crate::Readable for SC_EGT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_egt::W](W) writer structure"]
impl crate::Writable for SC_EGT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_EGT to value 0"]
impl crate::Resettable for SC_EGT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
