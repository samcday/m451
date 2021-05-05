#[doc = "Register `PWM_CNTEN` reader"]
pub struct R(crate::R<PWM_CNTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CNTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CNTEN_SPEC>> for R {
    fn from(reader: crate::R<PWM_CNTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CNTEN` writer"]
pub struct W(crate::W<PWM_CNTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CNTEN_SPEC>;
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
impl core::convert::From<crate::W<PWM_CNTEN_SPEC>> for W {
    fn from(writer: crate::W<PWM_CNTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Counter Enable Bits\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTENN_A {
    #[doc = "0: PWM Counter and clock prescaler Stop Running"]
    _0 = 0,
    #[doc = "1: PWM Counter and clock prescaler Start Running"]
    _1 = 1,
}
impl From<CNTENN_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTENn` reader - PWM Counter Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct CNTENN_R(crate::FieldReader<u8, CNTENN_A>);
impl CNTENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTENN_A> {
        match self.bits {
            0 => Some(CNTENN_A::_0),
            1 => Some(CNTENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTENN_A::_1
    }
}
impl core::ops::Deref for CNTENN_R {
    type Target = crate::FieldReader<u8, CNTENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTENn` writer - PWM Counter Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct CNTENN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM Counter and clock prescaler Stop Running"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTENN_A::_0)
    }
    #[doc = "PWM Counter and clock prescaler Start Running"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Counter Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn cntenn(&self) -> CNTENN_R {
        CNTENN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Counter Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn cntenn(&mut self) -> CNTENN_W {
        CNTENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Counter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cnten](index.html) module"]
pub struct PWM_CNTEN_SPEC;
impl crate::RegisterSpec for PWM_CNTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cnten::R](R) reader structure"]
impl crate::Readable for PWM_CNTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cnten::W](W) writer structure"]
impl crate::Writable for PWM_CNTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CNTEN to value 0"]
impl crate::Resettable for PWM_CNTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
