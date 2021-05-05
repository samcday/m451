#[doc = "Register `PDMA_TOUTIEN` reader"]
pub struct R(crate::R<PDMA_TOUTIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TOUTIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_TOUTIEN_SPEC>> for R {
    fn from(reader: crate::R<PDMA_TOUTIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_TOUTIEN` writer"]
pub struct W(crate::W<PDMA_TOUTIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_TOUTIEN_SPEC>;
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
impl core::convert::From<crate::W<PDMA_TOUTIEN_SPEC>> for W {
    fn from(writer: crate::W<PDMA_TOUTIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Time-out Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOUTIENN_A {
    #[doc = "0: PDMA Channel n time-out interrupt Disable"]
    _0 = 0,
    #[doc = "1: PDMA Channel n time-out interrupt Enable"]
    _1 = 1,
}
impl From<TOUTIENN_A> for u8 {
    #[inline(always)]
    fn from(variant: TOUTIENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOUTIENn` reader - PDMA Time-out Interrupt Enable Bits"]
pub struct TOUTIENN_R(crate::FieldReader<u8, TOUTIENN_A>);
impl TOUTIENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOUTIENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOUTIENN_A> {
        match self.bits {
            0 => Some(TOUTIENN_A::_0),
            1 => Some(TOUTIENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTIENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTIENN_A::_1
    }
}
impl core::ops::Deref for TOUTIENN_R {
    type Target = crate::FieldReader<u8, TOUTIENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTIENn` writer - PDMA Time-out Interrupt Enable Bits"]
pub struct TOUTIENN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTIENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTIENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDMA Channel n time-out interrupt Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTIENN_A::_0)
    }
    #[doc = "PDMA Channel n time-out interrupt Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTIENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PDMA Time-out Interrupt Enable Bits"]
    #[inline(always)]
    pub fn toutienn(&self) -> TOUTIENN_R {
        TOUTIENN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDMA Time-out Interrupt Enable Bits"]
    #[inline(always)]
    pub fn toutienn(&mut self) -> TOUTIENN_W {
        TOUTIENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Time-out Interrupt Enable Register (M45xD/M45xC Only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_toutien](index.html) module"]
pub struct PDMA_TOUTIEN_SPEC;
impl crate::RegisterSpec for PDMA_TOUTIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_toutien::R](R) reader structure"]
impl crate::Readable for PDMA_TOUTIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_toutien::W](W) writer structure"]
impl crate::Writable for PDMA_TOUTIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_TOUTIEN to value 0"]
impl crate::Resettable for PDMA_TOUTIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
