#[doc = "Register `I2C_CLKTOUT` reader"]
pub struct R(crate::R<I2C_CLKTOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CLKTOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_CLKTOUT_SPEC>> for R {
    fn from(reader: crate::R<I2C_CLKTOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CLKTOUT` writer"]
pub struct W(crate::W<I2C_CLKTOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CLKTOUT_SPEC>;
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
impl core::convert::From<crate::W<I2C_CLKTOUT_SPEC>> for W {
    fn from(writer: crate::W<I2C_CLKTOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKTO` reader - Bus Clock Low Timer\\nThe field is used to configure the cumulative clock extension time-out.\\nNote: If the user wants to revise the value of CLKLTOUT, the TORSTEN bit shall be set to 1 and clear to 0 first in the BUSEN is set."]
pub struct CLKTO_R(crate::FieldReader<u8, u8>);
impl CLKTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKTO` writer - Bus Clock Low Timer\\nThe field is used to configure the cumulative clock extension time-out.\\nNote: If the user wants to revise the value of CLKLTOUT, the TORSTEN bit shall be set to 1 and clear to 0 first in the BUSEN is set."]
pub struct CLKTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bus Clock Low Timer\\nThe field is used to configure the cumulative clock extension time-out.\\nNote: If the user wants to revise the value of CLKLTOUT, the TORSTEN bit shall be set to 1 and clear to 0 first in the BUSEN is set."]
    #[inline(always)]
    pub fn clkto(&self) -> CLKTO_R {
        CLKTO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bus Clock Low Timer\\nThe field is used to configure the cumulative clock extension time-out.\\nNote: If the user wants to revise the value of CLKLTOUT, the TORSTEN bit shall be set to 1 and clear to 0 first in the BUSEN is set."]
    #[inline(always)]
    pub fn clkto(&mut self) -> CLKTO_W {
        CLKTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Management Clock Low Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_clktout](index.html) module"]
pub struct I2C_CLKTOUT_SPEC;
impl crate::RegisterSpec for I2C_CLKTOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_clktout::R](R) reader structure"]
impl crate::Readable for I2C_CLKTOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_clktout::W](W) writer structure"]
impl crate::Writable for I2C_CLKTOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CLKTOUT to value 0x05"]
impl crate::Resettable for I2C_CLKTOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
