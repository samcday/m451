#[doc = "Register `PWM_POEN` reader"]
pub struct R(crate::R<PWM_POEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_POEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_POEN_SPEC>> for R {
    fn from(reader: crate::R<PWM_POEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_POEN` writer"]
pub struct W(crate::W<PWM_POEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_POEN_SPEC>;
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
impl core::convert::From<crate::W<PWM_POEN_SPEC>> for W {
    fn from(writer: crate::W<PWM_POEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Pin Output Enable Bits\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POENN_A {
    #[doc = "0: PWM pin at tri-state"]
    _0 = 0,
    #[doc = "1: PWM pin in output mode"]
    _1 = 1,
}
impl From<POENN_A> for u8 {
    #[inline(always)]
    fn from(variant: POENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POENn` reader - PWM Pin Output Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct POENN_R(crate::FieldReader<u8, POENN_A>);
impl POENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        POENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POENN_A> {
        match self.bits {
            0 => Some(POENN_A::_0),
            1 => Some(POENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POENN_A::_1
    }
}
impl core::ops::Deref for POENN_R {
    type Target = crate::FieldReader<u8, POENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POENn` writer - PWM Pin Output Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct POENN_W<'a> {
    w: &'a mut W,
}
impl<'a> POENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM pin at tri-state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POENN_A::_0)
    }
    #[doc = "PWM pin in output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Pin Output Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn poenn(&self) -> POENN_R {
        POENN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Pin Output Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn poenn(&mut self) -> POENN_W {
        POENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_poen](index.html) module"]
pub struct PWM_POEN_SPEC;
impl crate::RegisterSpec for PWM_POEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_poen::R](R) reader structure"]
impl crate::Readable for PWM_POEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_poen::W](W) writer structure"]
impl crate::Writable for PWM_POEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_POEN to value 0"]
impl crate::Resettable for PWM_POEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
