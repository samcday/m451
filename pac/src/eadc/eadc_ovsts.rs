#[doc = "Register `EADC_OVSTS` reader"]
pub struct R(crate::R<EADC_OVSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_OVSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_OVSTS_SPEC>> for R {
    fn from(reader: crate::R<EADC_OVSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADC_OVSTS` writer"]
pub struct W(crate::W<EADC_OVSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_OVSTS_SPEC>;
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
impl core::convert::From<crate::W<EADC_OVSTS_SPEC>> for W {
    fn from(writer: crate::W<EADC_OVSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D SAMPLE0~18 Overrun Flag\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SPOVF_A {
    #[doc = "0: No sample module event overrun"]
    _0 = 0,
    #[doc = "1: Indicates a new sample module event is generated while an old one event is pending"]
    _1 = 1,
}
impl From<SPOVF_A> for u32 {
    #[inline(always)]
    fn from(variant: SPOVF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPOVF` reader - A/D SAMPLE0~18 Overrun Flag\\nNote: This bit is cleared by writing 1 to it."]
pub struct SPOVF_R(crate::FieldReader<u32, SPOVF_A>);
impl SPOVF_R {
    pub(crate) fn new(bits: u32) -> Self {
        SPOVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPOVF_A> {
        match self.bits {
            0 => Some(SPOVF_A::_0),
            1 => Some(SPOVF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPOVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPOVF_A::_1
    }
}
impl core::ops::Deref for SPOVF_R {
    type Target = crate::FieldReader<u32, SPOVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPOVF` writer - A/D SAMPLE0~18 Overrun Flag\\nNote: This bit is cleared by writing 1 to it."]
pub struct SPOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOVF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No sample module event overrun"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPOVF_A::_0)
    }
    #[doc = "Indicates a new sample module event is generated while an old one event is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPOVF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - A/D SAMPLE0~18 Overrun Flag\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn spovf(&self) -> SPOVF_R {
        SPOVF_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - A/D SAMPLE0~18 Overrun Flag\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn spovf(&mut self) -> SPOVF_W {
        SPOVF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Sample Module Start of Conversion Overrun Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_ovsts](index.html) module"]
pub struct EADC_OVSTS_SPEC;
impl crate::RegisterSpec for EADC_OVSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_ovsts::R](R) reader structure"]
impl crate::Readable for EADC_OVSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eadc_ovsts::W](W) writer structure"]
impl crate::Writable for EADC_OVSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_OVSTS to value 0"]
impl crate::Resettable for EADC_OVSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
