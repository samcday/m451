#[doc = "Register `SYS_IPRST2` reader"]
pub struct R(crate::R<SYS_IPRST2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_IPRST2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_IPRST2_SPEC>> for R {
    fn from(reader: crate::R<SYS_IPRST2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_IPRST2` writer"]
pub struct W(crate::W<SYS_IPRST2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_IPRST2_SPEC>;
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
impl core::convert::From<crate::W<SYS_IPRST2_SPEC>> for W {
    fn from(writer: crate::W<SYS_IPRST2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SC0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC0RST_A {
    #[doc = "0: SC0 controller normal operation"]
    _0 = 0,
    #[doc = "1: SC0 controller reset"]
    _1 = 1,
}
impl From<SC0RST_A> for bool {
    #[inline(always)]
    fn from(variant: SC0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC0RST` reader - SC0 Controller Reset"]
pub struct SC0RST_R(crate::FieldReader<bool, SC0RST_A>);
impl SC0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC0RST_A {
        match self.bits {
            false => SC0RST_A::_0,
            true => SC0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SC0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SC0RST_A::_1
    }
}
impl core::ops::Deref for SC0RST_R {
    type Target = crate::FieldReader<bool, SC0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC0RST` writer - SC0 Controller Reset"]
pub struct SC0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SC0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC0RST_A::_0)
    }
    #[doc = "SC0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC0RST_A::_1)
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
#[doc = "DAC Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRST_A {
    #[doc = "0: DAC controller normal operation"]
    _0 = 0,
    #[doc = "1: DAC controller reset"]
    _1 = 1,
}
impl From<DACRST_A> for bool {
    #[inline(always)]
    fn from(variant: DACRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRST` reader - DAC Controller Reset"]
pub struct DACRST_R(crate::FieldReader<bool, DACRST_A>);
impl DACRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACRST_A {
        match self.bits {
            false => DACRST_A::_0,
            true => DACRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACRST_A::_1
    }
}
impl core::ops::Deref for DACRST_R {
    type Target = crate::FieldReader<bool, DACRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACRST` writer - DAC Controller Reset"]
pub struct DACRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DACRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACRST_A::_0)
    }
    #[doc = "DAC controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "PWM0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM0RST_A {
    #[doc = "0: PWM0 controller normal operation"]
    _0 = 0,
    #[doc = "1: PWM0 controller reset"]
    _1 = 1,
}
impl From<PWM0RST_A> for bool {
    #[inline(always)]
    fn from(variant: PWM0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM0RST` reader - PWM0 Controller Reset"]
pub struct PWM0RST_R(crate::FieldReader<bool, PWM0RST_A>);
impl PWM0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM0RST_A {
        match self.bits {
            false => PWM0RST_A::_0,
            true => PWM0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWM0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWM0RST_A::_1
    }
}
impl core::ops::Deref for PWM0RST_R {
    type Target = crate::FieldReader<bool, PWM0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM0RST` writer - PWM0 Controller Reset"]
pub struct PWM0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWM0RST_A::_0)
    }
    #[doc = "PWM0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWM0RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "PWM1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM1RST_A {
    #[doc = "0: PWM1 controller normal operation"]
    _0 = 0,
    #[doc = "1: PWM1 controller reset"]
    _1 = 1,
}
impl From<PWM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: PWM1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM1RST` reader - PWM1 Controller Reset"]
pub struct PWM1RST_R(crate::FieldReader<bool, PWM1RST_A>);
impl PWM1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM1RST_A {
        match self.bits {
            false => PWM1RST_A::_0,
            true => PWM1RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWM1RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWM1RST_A::_1
    }
}
impl core::ops::Deref for PWM1RST_R {
    type Target = crate::FieldReader<bool, PWM1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM1RST` writer - PWM1 Controller Reset"]
pub struct PWM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWM1RST_A::_0)
    }
    #[doc = "PWM1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWM1RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SC0 Controller Reset"]
    #[inline(always)]
    pub fn sc0rst(&self) -> SC0RST_R {
        SC0RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 12 - DAC Controller Reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PWM0 Controller Reset"]
    #[inline(always)]
    pub fn pwm0rst(&self) -> PWM0RST_R {
        PWM0RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PWM1 Controller Reset"]
    #[inline(always)]
    pub fn pwm1rst(&self) -> PWM1RST_R {
        PWM1RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SC0 Controller Reset"]
    #[inline(always)]
    pub fn sc0rst(&mut self) -> SC0RST_W {
        SC0RST_W { w: self }
    }
    #[doc = "Bit 12 - DAC Controller Reset"]
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W {
        DACRST_W { w: self }
    }
    #[doc = "Bit 16 - PWM0 Controller Reset"]
    #[inline(always)]
    pub fn pwm0rst(&mut self) -> PWM0RST_W {
        PWM0RST_W { w: self }
    }
    #[doc = "Bit 17 - PWM1 Controller Reset"]
    #[inline(always)]
    pub fn pwm1rst(&mut self) -> PWM1RST_W {
        PWM1RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_iprst2](index.html) module"]
pub struct SYS_IPRST2_SPEC;
impl crate::RegisterSpec for SYS_IPRST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_iprst2::R](R) reader structure"]
impl crate::Readable for SYS_IPRST2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_iprst2::W](W) writer structure"]
impl crate::Writable for SYS_IPRST2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_IPRST2 to value 0"]
impl crate::Resettable for SYS_IPRST2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
