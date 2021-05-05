#[doc = "Register `I2C_WKCTL` reader"]
pub struct R(crate::R<I2C_WKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_WKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_WKCTL_SPEC>> for R {
    fn from(reader: crate::R<I2C_WKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_WKCTL` writer"]
pub struct W(crate::W<I2C_WKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_WKCTL_SPEC>;
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
impl core::convert::From<crate::W<I2C_WKCTL_SPEC>> for W {
    fn from(writer: crate::W<I2C_WKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C Wake-up Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKEN_A {
    #[doc = "0: I2C wake-up function Disabled"]
    _0 = 0,
    #[doc = "1: I2C wake-up function Enabled"]
    _1 = 1,
}
impl From<WKEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKEN` reader - I2C Wake-up Enable Bit"]
pub struct WKEN_R(crate::FieldReader<bool, WKEN_A>);
impl WKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKEN_A {
        match self.bits {
            false => WKEN_A::_0,
            true => WKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKEN_A::_1
    }
}
impl core::ops::Deref for WKEN_R {
    type Target = crate::FieldReader<bool, WKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKEN` writer - I2C Wake-up Enable Bit"]
pub struct WKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C wake-up function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKEN_A::_0)
    }
    #[doc = "I2C wake-up function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKEN_A::_1)
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
    #[doc = "Bit 0 - I2C Wake-up Enable Bit"]
    #[inline(always)]
    pub fn wken(&self) -> WKEN_R {
        WKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Wake-up Enable Bit"]
    #[inline(always)]
    pub fn wken(&mut self) -> WKEN_W {
        WKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Wake-up Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_wkctl](index.html) module"]
pub struct I2C_WKCTL_SPEC;
impl crate::RegisterSpec for I2C_WKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_wkctl::R](R) reader structure"]
impl crate::Readable for I2C_WKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_wkctl::W](W) writer structure"]
impl crate::Writable for I2C_WKCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_WKCTL to value 0"]
impl crate::Resettable for I2C_WKCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
