#[doc = "Register `PWM_MSK` reader"]
pub struct R(crate::R<PWM_MSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_MSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_MSK_SPEC>> for R {
    fn from(reader: crate::R<PWM_MSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_MSK` writer"]
pub struct W(crate::W<PWM_MSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_MSK_SPEC>;
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
impl core::convert::From<crate::W<PWM_MSK_SPEC>> for W {
    fn from(writer: crate::W<PWM_MSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Mask Data Bit\\nThis data bit control the state of PWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSKDATN_A {
    #[doc = "0: Output logic low to PWMn"]
    _0 = 0,
    #[doc = "1: Output logic high to PWMn"]
    _1 = 1,
}
impl From<MSKDATN_A> for u8 {
    #[inline(always)]
    fn from(variant: MSKDATN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSKDATn` reader - PWM Mask Data Bit\\nThis data bit control the state of PWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding PWM channel n."]
pub struct MSKDATN_R(crate::FieldReader<u8, MSKDATN_A>);
impl MSKDATN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSKDATN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSKDATN_A> {
        match self.bits {
            0 => Some(MSKDATN_A::_0),
            1 => Some(MSKDATN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKDATN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKDATN_A::_1
    }
}
impl core::ops::Deref for MSKDATN_R {
    type Target = crate::FieldReader<u8, MSKDATN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKDATn` writer - PWM Mask Data Bit\\nThis data bit control the state of PWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding PWM channel n."]
pub struct MSKDATN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKDATN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKDATN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output logic low to PWMn"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKDATN_A::_0)
    }
    #[doc = "Output logic high to PWMn"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKDATN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Mask Data Bit\\nThis data bit control the state of PWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn mskdatn(&self) -> MSKDATN_R {
        MSKDATN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Mask Data Bit\\nThis data bit control the state of PWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn mskdatn(&mut self) -> MSKDATN_W {
        MSKDATN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Mask Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_msk](index.html) module"]
pub struct PWM_MSK_SPEC;
impl crate::RegisterSpec for PWM_MSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_msk::R](R) reader structure"]
impl crate::Readable for PWM_MSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_msk::W](W) writer structure"]
impl crate::Writable for PWM_MSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_MSK to value 0"]
impl crate::Resettable for PWM_MSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
