#[doc = "Register `PWM_SSCTL` reader"]
pub struct R(crate::R<PWM_SSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_SSCTL_SPEC>> for R {
    fn from(reader: crate::R<PWM_SSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_SSCTL` writer"]
pub struct W(crate::W<PWM_SSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SSCTL_SPEC>;
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
impl core::convert::From<crate::W<PWM_SSCTL_SPEC>> for W {
    fn from(writer: crate::W<PWM_SSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Synchronous Start Function Enable Bits\\nWhen synchronous start function is enabled, the PWM counter enable register (PWM_CNTEN) can be enabled by writing PWM synchronous start trigger bit (CNTSEN). Each bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSENN_A {
    #[doc = "0: PWM synchronous start function Disabled"]
    _0 = 0,
    #[doc = "1: PWM synchronous start function Enabled"]
    _1 = 1,
}
impl From<SSENN_A> for u8 {
    #[inline(always)]
    fn from(variant: SSENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSENn` reader - PWM Synchronous Start Function Enable Bits\\nWhen synchronous start function is enabled, the PWM counter enable register (PWM_CNTEN) can be enabled by writing PWM synchronous start trigger bit (CNTSEN). Each bit n controls the corresponding PWM channel n."]
pub struct SSENN_R(crate::FieldReader<u8, SSENN_A>);
impl SSENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSENN_A> {
        match self.bits {
            0 => Some(SSENN_A::_0),
            1 => Some(SSENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSENN_A::_1
    }
}
impl core::ops::Deref for SSENN_R {
    type Target = crate::FieldReader<u8, SSENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSENn` writer - PWM Synchronous Start Function Enable Bits\\nWhen synchronous start function is enabled, the PWM counter enable register (PWM_CNTEN) can be enabled by writing PWM synchronous start trigger bit (CNTSEN). Each bit n controls the corresponding PWM channel n."]
pub struct SSENN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM synchronous start function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSENN_A::_0)
    }
    #[doc = "PWM synchronous start function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Synchronous Start Function Enable Bits\\nWhen synchronous start function is enabled, the PWM counter enable register (PWM_CNTEN) can be enabled by writing PWM synchronous start trigger bit (CNTSEN). Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn ssenn(&self) -> SSENN_R {
        SSENN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Synchronous Start Function Enable Bits\\nWhen synchronous start function is enabled, the PWM counter enable register (PWM_CNTEN) can be enabled by writing PWM synchronous start trigger bit (CNTSEN). Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn ssenn(&mut self) -> SSENN_W {
        SSENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Synchronous Start Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ssctl](index.html) module"]
pub struct PWM_SSCTL_SPEC;
impl crate::RegisterSpec for PWM_SSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ssctl::R](R) reader structure"]
impl crate::Readable for PWM_SSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ssctl::W](W) writer structure"]
impl crate::Writable for PWM_SSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SSCTL to value 0"]
impl crate::Resettable for PWM_SSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
