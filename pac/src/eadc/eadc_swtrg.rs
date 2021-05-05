#[doc = "Register `EADC_SWTRG` writer"]
pub struct W(crate::W<EADC_SWTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_SWTRG_SPEC>;
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
impl core::convert::From<crate::W<EADC_SWTRG_SPEC>> for W {
    fn from(writer: crate::W<EADC_SWTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Sample Module 0~18 Software Force to Start ADC Conversion\\nNote: After write this register to start ADC conversion, the EADC_PENDSTS register will show which sample module will conversion. If user want to disable the conversion of the sample module, user can write EADC_PENDSTS register to clear it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SWTRG_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Cause an ADC conversion when the priority is given to sample module"]
    _1 = 1,
}
impl From<SWTRG_AW> for u32 {
    #[inline(always)]
    fn from(variant: SWTRG_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SWTRG` writer - A/D Sample Module 0~18 Software Force to Start ADC Conversion\\nNote: After write this register to start ADC conversion, the EADC_PENDSTS register will show which sample module will conversion. If user want to disable the conversion of the sample module, user can write EADC_PENDSTS register to clear it."]
pub struct SWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRG_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWTRG_AW::_0)
    }
    #[doc = "Cause an ADC conversion when the priority is given to sample module"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWTRG_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:18 - A/D Sample Module 0~18 Software Force to Start ADC Conversion\\nNote: After write this register to start ADC conversion, the EADC_PENDSTS register will show which sample module will conversion. If user want to disable the conversion of the sample module, user can write EADC_PENDSTS register to clear it."]
    #[inline(always)]
    pub fn swtrg(&mut self) -> SWTRG_W {
        SWTRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Sample Module Software Start Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_swtrg](index.html) module"]
pub struct EADC_SWTRG_SPEC;
impl crate::RegisterSpec for EADC_SWTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eadc_swtrg::W](W) writer structure"]
impl crate::Writable for EADC_SWTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_SWTRG to value 0"]
impl crate::Resettable for EADC_SWTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
