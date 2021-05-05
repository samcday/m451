#[doc = "Register `DAC_SWTRG` reader"]
pub struct R(crate::R<DAC_SWTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SWTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DAC_SWTRG_SPEC>> for R {
    fn from(reader: crate::R<DAC_SWTRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_SWTRG` writer"]
pub struct W(crate::W<DAC_SWTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SWTRG_SPEC>;
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
impl core::convert::From<crate::W<DAC_SWTRG_SPEC>> for W {
    fn from(writer: crate::W<DAC_SWTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Trigger\\nUser writes this bit to generate one shot pulse and it is cleared to 0 by hardware automatically; Reading this bit will always get 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRG_A {
    #[doc = "0: Software trigger Disabled"]
    _0 = 0,
    #[doc = "1: Software trigger Enabled"]
    _1 = 1,
}
impl From<SWTRG_A> for bool {
    #[inline(always)]
    fn from(variant: SWTRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRG` reader - Software Trigger\\nUser writes this bit to generate one shot pulse and it is cleared to 0 by hardware automatically; Reading this bit will always get 0."]
pub struct SWTRG_R(crate::FieldReader<bool, SWTRG_A>);
impl SWTRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWTRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWTRG_A {
        match self.bits {
            false => SWTRG_A::_0,
            true => SWTRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWTRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWTRG_A::_1
    }
}
impl core::ops::Deref for SWTRG_R {
    type Target = crate::FieldReader<bool, SWTRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRG` writer - Software Trigger\\nUser writes this bit to generate one shot pulse and it is cleared to 0 by hardware automatically; Reading this bit will always get 0."]
pub struct SWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software trigger Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWTRG_A::_0)
    }
    #[doc = "Software trigger Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWTRG_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Trigger\\nUser writes this bit to generate one shot pulse and it is cleared to 0 by hardware automatically; Reading this bit will always get 0."]
    #[inline(always)]
    pub fn swtrg(&self) -> SWTRG_R {
        SWTRG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Trigger\\nUser writes this bit to generate one shot pulse and it is cleared to 0 by hardware automatically; Reading this bit will always get 0."]
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
#[doc = "DAC Software Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_swtrg](index.html) module"]
pub struct DAC_SWTRG_SPEC;
impl crate::RegisterSpec for DAC_SWTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_swtrg::R](R) reader structure"]
impl crate::Readable for DAC_SWTRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_swtrg::W](W) writer structure"]
impl crate::Writable for DAC_SWTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_SWTRG to value 0"]
impl crate::Resettable for DAC_SWTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
