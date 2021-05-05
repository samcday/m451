#[doc = "Register `EADC_PENDSTS` reader"]
pub struct R(crate::R<EADC_PENDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_PENDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_PENDSTS_SPEC>> for R {
    fn from(reader: crate::R<EADC_PENDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADC_PENDSTS` writer"]
pub struct W(crate::W<EADC_PENDSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_PENDSTS_SPEC>;
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
impl core::convert::From<crate::W<EADC_PENDSTS_SPEC>> for W {
    fn from(writer: crate::W<EADC_PENDSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Sample Module 0~18 Start of Conversion Pending Flag\\nRead:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum STPF_A {
    #[doc = "0: There is no pending conversion for sample module"]
    _0 = 0,
    #[doc = "1: Sample module ADC start of conversion is pending.\\nClear pending flag and stop conversion for corresponding sample module"]
    _1 = 1,
}
impl From<STPF_A> for u32 {
    #[inline(always)]
    fn from(variant: STPF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STPF` reader - A/D Sample Module 0~18 Start of Conversion Pending Flag\\nRead:"]
pub struct STPF_R(crate::FieldReader<u32, STPF_A>);
impl STPF_R {
    pub(crate) fn new(bits: u32) -> Self {
        STPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STPF_A> {
        match self.bits {
            0 => Some(STPF_A::_0),
            1 => Some(STPF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STPF_A::_1
    }
}
impl core::ops::Deref for STPF_R {
    type Target = crate::FieldReader<u32, STPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPF` writer - A/D Sample Module 0~18 Start of Conversion Pending Flag\\nRead:"]
pub struct STPF_W<'a> {
    w: &'a mut W,
}
impl<'a> STPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "There is no pending conversion for sample module"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPF_A::_0)
    }
    #[doc = "Sample module ADC start of conversion is pending.\\nClear pending flag and stop conversion for corresponding sample module"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - A/D Sample Module 0~18 Start of Conversion Pending Flag\\nRead:"]
    #[inline(always)]
    pub fn stpf(&self) -> STPF_R {
        STPF_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - A/D Sample Module 0~18 Start of Conversion Pending Flag\\nRead:"]
    #[inline(always)]
    pub fn stpf(&mut self) -> STPF_W {
        STPF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Start of Conversion Pending Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_pendsts](index.html) module"]
pub struct EADC_PENDSTS_SPEC;
impl crate::RegisterSpec for EADC_PENDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_pendsts::R](R) reader structure"]
impl crate::Readable for EADC_PENDSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eadc_pendsts::W](W) writer structure"]
impl crate::Writable for EADC_PENDSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_PENDSTS to value 0"]
impl crate::Resettable for EADC_PENDSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
