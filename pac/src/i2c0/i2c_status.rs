#[doc = "Register `I2C_STATUS` reader"]
pub struct R(crate::R<I2C_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_STATUS_SPEC>> for R {
    fn from(reader: crate::R<I2C_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - I2C Status\\n2. If the BUSEN and PECEN are enabled, the status of PECERR, I2C_BUSSTS\\[3\\], is used to substitute for I2C_STATUS to check the ACK status in the last frame when the byte count done interrupt has active and the PEC frame has been transformed."]
pub struct STATUS_R(crate::FieldReader<u8, u8>);
impl STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Status\\n2. If the BUSEN and PECEN are enabled, the status of PECERR, I2C_BUSSTS\\[3\\], is used to substitute for I2C_STATUS to check the ACK status in the last frame when the byte count done interrupt has active and the PEC frame has been transformed."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_status](index.html) module"]
pub struct I2C_STATUS_SPEC;
impl crate::RegisterSpec for I2C_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_status::R](R) reader structure"]
impl crate::Readable for I2C_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_STATUS to value 0xf8"]
impl crate::Resettable for I2C_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf8
    }
}
