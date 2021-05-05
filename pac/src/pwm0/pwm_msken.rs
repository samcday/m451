#[doc = "Register `PWM_MSKEN` reader"]
pub struct R(crate::R<PWM_MSKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_MSKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_MSKEN_SPEC>> for R {
    fn from(reader: crate::R<PWM_MSKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_MSKEN` writer"]
pub struct W(crate::W<PWM_MSKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_MSKEN_SPEC>;
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
impl core::convert::From<crate::W<PWM_MSKEN_SPEC>> for W {
    fn from(writer: crate::W<PWM_MSKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Mask Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSKENN_A {
    #[doc = "0: PWM output signal is non-masked"]
    _0 = 0,
    #[doc = "1: PWM output signal is masked and output MSKDATn data"]
    _1 = 1,
}
impl From<MSKENN_A> for u8 {
    #[inline(always)]
    fn from(variant: MSKENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSKENn` reader - PWM Mask Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKENN_R(crate::FieldReader<u8, MSKENN_A>);
impl MSKENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSKENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSKENN_A> {
        match self.bits {
            0 => Some(MSKENN_A::_0),
            1 => Some(MSKENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKENN_A::_1
    }
}
impl core::ops::Deref for MSKENN_R {
    type Target = crate::FieldReader<u8, MSKENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKENn` writer - PWM Mask Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKENN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM output signal is non-masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKENN_A::_0)
    }
    #[doc = "PWM output signal is masked and output MSKDATn data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Mask Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn mskenn(&self) -> MSKENN_R {
        MSKENN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Mask Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn mskenn(&mut self) -> MSKENN_W {
        MSKENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Mask Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_msken](index.html) module"]
pub struct PWM_MSKEN_SPEC;
impl crate::RegisterSpec for PWM_MSKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_msken::R](R) reader structure"]
impl crate::Readable for PWM_MSKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_msken::W](W) writer structure"]
impl crate::Writable for PWM_MSKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_MSKEN to value 0"]
impl crate::Resettable for PWM_MSKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
