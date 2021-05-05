#[doc = "Register `PWM_CAPINEN` reader"]
pub struct R(crate::R<PWM_CAPINEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CAPINEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CAPINEN_SPEC>> for R {
    fn from(reader: crate::R<PWM_CAPINEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CAPINEN` writer"]
pub struct W(crate::W<PWM_CAPINEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CAPINEN_SPEC>;
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
impl core::convert::From<crate::W<PWM_CAPINEN_SPEC>> for W {
    fn from(writer: crate::W<PWM_CAPINEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture Input Enable Bits\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPINENN_A {
    #[doc = "0: PWM Channel capture input path Disabled. The input of PWM channel capture function is always regarded as 0"]
    _0 = 0,
    #[doc = "1: PWM Channel capture input path Enabled. The input of PWM channel capture function comes from correlative multifunction pin"]
    _1 = 1,
}
impl From<CAPINENN_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPINENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPINENn` reader - Capture Input Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct CAPINENN_R(crate::FieldReader<u8, CAPINENN_A>);
impl CAPINENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPINENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPINENN_A> {
        match self.bits {
            0 => Some(CAPINENN_A::_0),
            1 => Some(CAPINENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINENN_A::_1
    }
}
impl core::ops::Deref for CAPINENN_R {
    type Target = crate::FieldReader<u8, CAPINENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINENn` writer - Capture Input Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct CAPINENN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM Channel capture input path Disabled. The input of PWM channel capture function is always regarded as 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINENN_A::_0)
    }
    #[doc = "PWM Channel capture input path Enabled. The input of PWM channel capture function comes from correlative multifunction pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Capture Input Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn capinenn(&self) -> CAPINENN_R {
        CAPINENN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Capture Input Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn capinenn(&mut self) -> CAPINENN_W {
        CAPINENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Capture Input Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_capinen](index.html) module"]
pub struct PWM_CAPINEN_SPEC;
impl crate::RegisterSpec for PWM_CAPINEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_capinen::R](R) reader structure"]
impl crate::Readable for PWM_CAPINEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_capinen::W](W) writer structure"]
impl crate::Writable for PWM_CAPINEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CAPINEN to value 0"]
impl crate::Resettable for PWM_CAPINEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
