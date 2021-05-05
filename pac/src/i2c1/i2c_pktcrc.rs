#[doc = "Register `I2C_PKTCRC` reader"]
pub struct R(crate::R<I2C_PKTCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_PKTCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_PKTCRC_SPEC>> for R {
    fn from(reader: crate::R<I2C_PKTCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PECCRC` reader - Packet Error Checking Byte Value"]
pub struct PECCRC_R(crate::FieldReader<u8, u8>);
impl PECCRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PECCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PECCRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Packet Error Checking Byte Value"]
    #[inline(always)]
    pub fn peccrc(&self) -> PECCRC_R {
        PECCRC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C Packet Error Checking Byte Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_pktcrc](index.html) module"]
pub struct I2C_PKTCRC_SPEC;
impl crate::RegisterSpec for I2C_PKTCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_pktcrc::R](R) reader structure"]
impl crate::Readable for I2C_PKTCRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_PKTCRC to value 0"]
impl crate::Resettable for I2C_PKTCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
