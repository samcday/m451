#[doc = "Register `I2C_WKSTS` reader"]
pub struct R(crate::R<I2C_WKSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_WKSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_WKSTS_SPEC>> for R {
    fn from(reader: crate::R<I2C_WKSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_WKSTS` writer"]
pub struct W(crate::W<I2C_WKSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_WKSTS_SPEC>;
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
impl core::convert::From<crate::W<I2C_WKSTS_SPEC>> for W {
    fn from(writer: crate::W<I2C_WKSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKIF` reader - I2C Wake-up Flag\\nWhen chip is woken up from Power-down mode by I2C, this bit is set to 1. Software can write 1 to clear this bit."]
pub struct WKIF_R(crate::FieldReader<bool, bool>);
impl WKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKIF` writer - I2C Wake-up Flag\\nWhen chip is woken up from Power-down mode by I2C, this bit is set to 1. Software can write 1 to clear this bit."]
pub struct WKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKIF_W<'a> {
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
    #[doc = "Bit 0 - I2C Wake-up Flag\\nWhen chip is woken up from Power-down mode by I2C, this bit is set to 1. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn wkif(&self) -> WKIF_R {
        WKIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Wake-up Flag\\nWhen chip is woken up from Power-down mode by I2C, this bit is set to 1. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn wkif(&mut self) -> WKIF_W {
        WKIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Wake-up Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_wksts](index.html) module"]
pub struct I2C_WKSTS_SPEC;
impl crate::RegisterSpec for I2C_WKSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_wksts::R](R) reader structure"]
impl crate::Readable for I2C_WKSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_wksts::W](W) writer structure"]
impl crate::Writable for I2C_WKSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_WKSTS to value 0"]
impl crate::Resettable for I2C_WKSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
