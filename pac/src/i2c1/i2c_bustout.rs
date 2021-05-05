#[doc = "Register `I2C_BUSTOUT` reader"]
pub struct R(crate::R<I2C_BUSTOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_BUSTOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_BUSTOUT_SPEC>> for R {
    fn from(reader: crate::R<I2C_BUSTOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_BUSTOUT` writer"]
pub struct W(crate::W<I2C_BUSTOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_BUSTOUT_SPEC>;
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
impl core::convert::From<crate::W<I2C_BUSTOUT_SPEC>> for W {
    fn from(writer: crate::W<I2C_BUSTOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSTO` reader - Bus Management Time-out Value\\nIndicate the bus time-out value in bus is IDLE or SCLK low.\\nNote: If the user wants to revise the value of BUSTOUT, the TORSTEN (I2C_BUSTCTL\\[4\\]) bit shall be set to 1 and clear to 0 first in the BUSEN(I2C_BUSCTL\\[7\\]) is set."]
pub struct BUSTO_R(crate::FieldReader<u8, u8>);
impl BUSTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        BUSTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSTO` writer - Bus Management Time-out Value\\nIndicate the bus time-out value in bus is IDLE or SCLK low.\\nNote: If the user wants to revise the value of BUSTOUT, the TORSTEN (I2C_BUSTCTL\\[4\\]) bit shall be set to 1 and clear to 0 first in the BUSEN(I2C_BUSCTL\\[7\\]) is set."]
pub struct BUSTO_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bus Management Time-out Value\\nIndicate the bus time-out value in bus is IDLE or SCLK low.\\nNote: If the user wants to revise the value of BUSTOUT, the TORSTEN (I2C_BUSTCTL\\[4\\]) bit shall be set to 1 and clear to 0 first in the BUSEN(I2C_BUSCTL\\[7\\]) is set."]
    #[inline(always)]
    pub fn busto(&self) -> BUSTO_R {
        BUSTO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bus Management Time-out Value\\nIndicate the bus time-out value in bus is IDLE or SCLK low.\\nNote: If the user wants to revise the value of BUSTOUT, the TORSTEN (I2C_BUSTCTL\\[4\\]) bit shall be set to 1 and clear to 0 first in the BUSEN(I2C_BUSCTL\\[7\\]) is set."]
    #[inline(always)]
    pub fn busto(&mut self) -> BUSTO_W {
        BUSTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Management Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_bustout](index.html) module"]
pub struct I2C_BUSTOUT_SPEC;
impl crate::RegisterSpec for I2C_BUSTOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_bustout::R](R) reader structure"]
impl crate::Readable for I2C_BUSTOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_bustout::W](W) writer structure"]
impl crate::Writable for I2C_BUSTOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_BUSTOUT to value 0x05"]
impl crate::Resettable for I2C_BUSTOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
