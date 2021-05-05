#[doc = "Register `ACMP_VREF` reader"]
pub struct R(crate::R<ACMP_VREF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMP_VREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACMP_VREF_SPEC>> for R {
    fn from(reader: crate::R<ACMP_VREF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMP_VREF` writer"]
pub struct W(crate::W<ACMP_VREF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMP_VREF_SPEC>;
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
impl core::convert::From<crate::W<ACMP_VREF_SPEC>> for W {
    fn from(writer: crate::W<ACMP_VREF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRVCTL` reader - Comparator Reference Voltage Setting"]
pub struct CRVCTL_R(crate::FieldReader<u8, u8>);
impl CRVCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRVCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRVCTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRVCTL` writer - Comparator Reference Voltage Setting"]
pub struct CRVCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRVCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "CRV Source Voltage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRVSSEL_A {
    #[doc = "0: AVDD is selected as CRV source voltage"]
    _0 = 0,
    #[doc = "1: The reference voltage defined by SYS_VREFCTL register is selected as CRV source voltage"]
    _1 = 1,
}
impl From<CRVSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CRVSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRVSSEL` reader - CRV Source Voltage Selection"]
pub struct CRVSSEL_R(crate::FieldReader<bool, CRVSSEL_A>);
impl CRVSSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRVSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRVSSEL_A {
        match self.bits {
            false => CRVSSEL_A::_0,
            true => CRVSSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRVSSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRVSSEL_A::_1
    }
}
impl core::ops::Deref for CRVSSEL_R {
    type Target = crate::FieldReader<bool, CRVSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRVSSEL` writer - CRV Source Voltage Selection"]
pub struct CRVSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRVSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRVSSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "AVDD is selected as CRV source voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRVSSEL_A::_0)
    }
    #[doc = "The reference voltage defined by SYS_VREFCTL register is selected as CRV source voltage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRVSSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Comparator Reference Voltage Setting"]
    #[inline(always)]
    pub fn crvctl(&self) -> CRVCTL_R {
        CRVCTL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - CRV Source Voltage Selection"]
    #[inline(always)]
    pub fn crvssel(&self) -> CRVSSEL_R {
        CRVSSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comparator Reference Voltage Setting"]
    #[inline(always)]
    pub fn crvctl(&mut self) -> CRVCTL_W {
        CRVCTL_W { w: self }
    }
    #[doc = "Bit 6 - CRV Source Voltage Selection"]
    #[inline(always)]
    pub fn crvssel(&mut self) -> CRVSSEL_W {
        CRVSSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Reference Voltage Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmp_vref](index.html) module"]
pub struct ACMP_VREF_SPEC;
impl crate::RegisterSpec for ACMP_VREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmp_vref::R](R) reader structure"]
impl crate::Readable for ACMP_VREF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmp_vref::W](W) writer structure"]
impl crate::Writable for ACMP_VREF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACMP_VREF to value 0"]
impl crate::Resettable for ACMP_VREF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
