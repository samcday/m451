#[doc = "Register `PWM_CNTCLR` reader"]
pub struct R(crate::R<PWM_CNTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CNTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CNTCLR_SPEC>> for R {
    fn from(reader: crate::R<PWM_CNTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CNTCLR` writer"]
pub struct W(crate::W<PWM_CNTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CNTCLR_SPEC>;
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
impl core::convert::From<crate::W<PWM_CNTCLR_SPEC>> for W {
    fn from(writer: crate::W<PWM_CNTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear PWM Counter Control Bit\\nIt is automatically cleared by hardware. Each bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTCLRN_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear 16-bit PWM counter to 0000H"]
    _1 = 1,
}
impl From<CNTCLRN_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTCLRN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTCLRn` reader - Clear PWM Counter Control Bit\\nIt is automatically cleared by hardware. Each bit n controls the corresponding PWM channel n."]
pub struct CNTCLRN_R(crate::FieldReader<u8, CNTCLRN_A>);
impl CNTCLRN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTCLRN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTCLRN_A> {
        match self.bits {
            0 => Some(CNTCLRN_A::_0),
            1 => Some(CNTCLRN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTCLRN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTCLRN_A::_1
    }
}
impl core::ops::Deref for CNTCLRN_R {
    type Target = crate::FieldReader<u8, CNTCLRN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTCLRn` writer - Clear PWM Counter Control Bit\\nIt is automatically cleared by hardware. Each bit n controls the corresponding PWM channel n."]
pub struct CNTCLRN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCLRN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTCLRN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTCLRN_A::_0)
    }
    #[doc = "Clear 16-bit PWM counter to 0000H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTCLRN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Clear PWM Counter Control Bit\\nIt is automatically cleared by hardware. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn cntclrn(&self) -> CNTCLRN_R {
        CNTCLRN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear PWM Counter Control Bit\\nIt is automatically cleared by hardware. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn cntclrn(&mut self) -> CNTCLRN_W {
        CNTCLRN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Clear Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cntclr](index.html) module"]
pub struct PWM_CNTCLR_SPEC;
impl crate::RegisterSpec for PWM_CNTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cntclr::R](R) reader structure"]
impl crate::Readable for PWM_CNTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cntclr::W](W) writer structure"]
impl crate::Writable for PWM_CNTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CNTCLR to value 0"]
impl crate::Resettable for PWM_CNTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
