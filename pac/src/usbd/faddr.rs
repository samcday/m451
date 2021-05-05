#[doc = "Register `FADDR` reader"]
pub struct R(crate::R<FADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FADDR_SPEC>> for R {
    fn from(reader: crate::R<FADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FADDR` writer"]
pub struct W(crate::W<FADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FADDR_SPEC>;
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
impl core::convert::From<crate::W<FADDR_SPEC>> for W {
    fn from(writer: crate::W<FADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADDR` reader - USB Device Function Address"]
pub struct FADDR_R(crate::FieldReader<u8, u8>);
impl FADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR` writer - USB Device Function Address"]
pub struct FADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Device Function Address"]
    #[inline(always)]
    pub fn faddr(&self) -> FADDR_R {
        FADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Device Function Address"]
    #[inline(always)]
    pub fn faddr(&mut self) -> FADDR_W {
        FADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Function Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](index.html) module"]
pub struct FADDR_SPEC;
impl crate::RegisterSpec for FADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [faddr::R](R) reader structure"]
impl crate::Readable for FADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faddr::W](W) writer structure"]
impl crate::Writable for FADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FADDR to value 0"]
impl crate::Resettable for FADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
