#[doc = "Register `PWM_STATUS` reader"]
pub struct R(crate::R<PWM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_STATUS_SPEC>> for R {
    fn from(reader: crate::R<PWM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_STATUS` writer"]
pub struct W(crate::W<PWM_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_STATUS_SPEC>;
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
impl core::convert::From<crate::W<PWM_STATUS_SPEC>> for W {
    fn from(writer: crate::W<PWM_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time-base Counter Equal to 0xFFFF Latched Flag\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTMAXFN_A {
    #[doc = "0: indicates the time-base counter never reached its maximum value 0xFFFF"]
    _0 = 0,
    #[doc = "1: indicates the time-base counter reached its maximum value, software can write 1 to clear this bit"]
    _1 = 1,
}
impl From<CNTMAXFN_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTMAXFN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTMAXFn` reader - Time-base Counter Equal to 0xFFFF Latched Flag\\nEach bit n controls the corresponding PWM channel n."]
pub struct CNTMAXFN_R(crate::FieldReader<u8, CNTMAXFN_A>);
impl CNTMAXFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTMAXFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTMAXFN_A> {
        match self.bits {
            0 => Some(CNTMAXFN_A::_0),
            1 => Some(CNTMAXFN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTMAXFN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTMAXFN_A::_1
    }
}
impl core::ops::Deref for CNTMAXFN_R {
    type Target = crate::FieldReader<u8, CNTMAXFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTMAXFn` writer - Time-base Counter Equal to 0xFFFF Latched Flag\\nEach bit n controls the corresponding PWM channel n."]
pub struct CNTMAXFN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMAXFN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMAXFN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "indicates the time-base counter never reached its maximum value 0xFFFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMAXFN_A::_0)
    }
    #[doc = "indicates the time-base counter reached its maximum value, software can write 1 to clear this bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMAXFN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Input Synchronization Latched Flag\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCINFN_A {
    #[doc = "0: Indicates no SYNC_IN event has occurred"]
    _0 = 0,
    #[doc = "1: Indicates an SYNC_IN event has occurred, software can write 1 to clear this bit"]
    _1 = 1,
}
impl From<SYNCINFN_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCINFN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYNCINFn` reader - Input Synchronization Latched Flag\\nEach bit n controls the corresponding PWM channel n."]
pub struct SYNCINFN_R(crate::FieldReader<u8, SYNCINFN_A>);
impl SYNCINFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCINFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCINFN_A> {
        match self.bits {
            0 => Some(SYNCINFN_A::_0),
            1 => Some(SYNCINFN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYNCINFN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYNCINFN_A::_1
    }
}
impl core::ops::Deref for SYNCINFN_R {
    type Target = crate::FieldReader<u8, SYNCINFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCINFn` writer - Input Synchronization Latched Flag\\nEach bit n controls the corresponding PWM channel n."]
pub struct SYNCINFN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCINFN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCINFN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Indicates no SYNC_IN event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCINFN_A::_0)
    }
    #[doc = "Indicates an SYNC_IN event has occurred, software can write 1 to clear this bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCINFN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "EADC Start of Conversion Flag\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCTRGFN_A {
    #[doc = "0: Indicates no EADC start of conversion trigger event has occurred"]
    _0 = 0,
    #[doc = "1: Indicates an EADC start of conversion trigger event has occurred, software can write 1 to clear this bit"]
    _1 = 1,
}
impl From<ADCTRGFN_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCTRGFN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCTRGFn` reader - EADC Start of Conversion Flag\\nEach bit n controls the corresponding PWM channel n."]
pub struct ADCTRGFN_R(crate::FieldReader<u8, ADCTRGFN_A>);
impl ADCTRGFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCTRGFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCTRGFN_A> {
        match self.bits {
            0 => Some(ADCTRGFN_A::_0),
            1 => Some(ADCTRGFN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCTRGFN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCTRGFN_A::_1
    }
}
impl core::ops::Deref for ADCTRGFN_R {
    type Target = crate::FieldReader<u8, ADCTRGFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCTRGFn` writer - EADC Start of Conversion Flag\\nEach bit n controls the corresponding PWM channel n."]
pub struct ADCTRGFN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTRGFN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTRGFN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Indicates no EADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCTRGFN_A::_0)
    }
    #[doc = "Indicates an EADC start of conversion trigger event has occurred, software can write 1 to clear this bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCTRGFN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "DAC Start of Conversion Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACTRGF_A {
    #[doc = "0: Indicates no DAC start of conversion trigger event has occurred"]
    _0 = 0,
    #[doc = "1: Indicates an DAC start of conversion trigger event has occurred, software can write 1 to clear this bit"]
    _1 = 1,
}
impl From<DACTRGF_A> for bool {
    #[inline(always)]
    fn from(variant: DACTRGF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACTRGF` reader - DAC Start of Conversion Flag"]
pub struct DACTRGF_R(crate::FieldReader<bool, DACTRGF_A>);
impl DACTRGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACTRGF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACTRGF_A {
        match self.bits {
            false => DACTRGF_A::_0,
            true => DACTRGF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACTRGF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACTRGF_A::_1
    }
}
impl core::ops::Deref for DACTRGF_R {
    type Target = crate::FieldReader<bool, DACTRGF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACTRGF` writer - DAC Start of Conversion Flag"]
pub struct DACTRGF_W<'a> {
    w: &'a mut W,
}
impl<'a> DACTRGF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACTRGF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Indicates no DAC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACTRGF_A::_0)
    }
    #[doc = "Indicates an DAC start of conversion trigger event has occurred, software can write 1 to clear this bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACTRGF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Time-base Counter Equal to 0xFFFF Latched Flag\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn cntmaxfn(&self) -> CNTMAXFN_R {
        CNTMAXFN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Input Synchronization Latched Flag\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn syncinfn(&self) -> SYNCINFN_R {
        SYNCINFN_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:21 - EADC Start of Conversion Flag\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn adctrgfn(&self) -> ADCTRGFN_R {
        ADCTRGFN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - DAC Start of Conversion Flag"]
    #[inline(always)]
    pub fn dactrgf(&self) -> DACTRGF_R {
        DACTRGF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Time-base Counter Equal to 0xFFFF Latched Flag\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn cntmaxfn(&mut self) -> CNTMAXFN_W {
        CNTMAXFN_W { w: self }
    }
    #[doc = "Bits 8:10 - Input Synchronization Latched Flag\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn syncinfn(&mut self) -> SYNCINFN_W {
        SYNCINFN_W { w: self }
    }
    #[doc = "Bits 16:21 - EADC Start of Conversion Flag\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn adctrgfn(&mut self) -> ADCTRGFN_W {
        ADCTRGFN_W { w: self }
    }
    #[doc = "Bit 24 - DAC Start of Conversion Flag"]
    #[inline(always)]
    pub fn dactrgf(&mut self) -> DACTRGF_W {
        DACTRGF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_status](index.html) module"]
pub struct PWM_STATUS_SPEC;
impl crate::RegisterSpec for PWM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_status::R](R) reader structure"]
impl crate::Readable for PWM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_status::W](W) writer structure"]
impl crate::Writable for PWM_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_STATUS to value 0"]
impl crate::Resettable for PWM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
