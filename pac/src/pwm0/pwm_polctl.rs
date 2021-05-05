#[doc = "Register `PWM_POLCTL` reader"]
pub struct R(crate::R<PWM_POLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_POLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_POLCTL_SPEC>> for R {
    fn from(reader: crate::R<PWM_POLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_POLCTL` writer"]
pub struct W(crate::W<PWM_POLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_POLCTL_SPEC>;
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
impl core::convert::From<crate::W<PWM_POLCTL_SPEC>> for W {
    fn from(writer: crate::W<PWM_POLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM PIN Polar Inverse Control Bits\\nThe register controls polarity state of PWM output. Each bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINVN_A {
    #[doc = "0: PWM output polar inverse Disabled"]
    _0 = 0,
    #[doc = "1: PWM output polar inverse Enabled"]
    _1 = 1,
}
impl From<PINVN_A> for u8 {
    #[inline(always)]
    fn from(variant: PINVN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PINVn` reader - PWM PIN Polar Inverse Control Bits\\nThe register controls polarity state of PWM output. Each bit n controls the corresponding PWM channel n."]
pub struct PINVN_R(crate::FieldReader<u8, PINVN_A>);
impl PINVN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PINVN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PINVN_A> {
        match self.bits {
            0 => Some(PINVN_A::_0),
            1 => Some(PINVN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINVN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINVN_A::_1
    }
}
impl core::ops::Deref for PINVN_R {
    type Target = crate::FieldReader<u8, PINVN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINVn` writer - PWM PIN Polar Inverse Control Bits\\nThe register controls polarity state of PWM output. Each bit n controls the corresponding PWM channel n."]
pub struct PINVN_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINVN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM output polar inverse Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINVN_A::_0)
    }
    #[doc = "PWM output polar inverse Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINVN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM PIN Polar Inverse Control Bits\\nThe register controls polarity state of PWM output. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn pinvn(&self) -> PINVN_R {
        PINVN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM PIN Polar Inverse Control Bits\\nThe register controls polarity state of PWM output. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn pinvn(&mut self) -> PINVN_W {
        PINVN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Pin Polar Inverse Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_polctl](index.html) module"]
pub struct PWM_POLCTL_SPEC;
impl crate::RegisterSpec for PWM_POLCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_polctl::R](R) reader structure"]
impl crate::Readable for PWM_POLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_polctl::W](W) writer structure"]
impl crate::Writable for PWM_POLCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_POLCTL to value 0"]
impl crate::Resettable for PWM_POLCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
