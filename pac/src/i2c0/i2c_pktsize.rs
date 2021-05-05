#[doc = "Register `I2C_PKTSIZE` reader"]
pub struct R(crate::R<I2C_PKTSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_PKTSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_PKTSIZE_SPEC>> for R {
    fn from(reader: crate::R<I2C_PKTSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_PKTSIZE` writer"]
pub struct W(crate::W<I2C_PKTSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_PKTSIZE_SPEC>;
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
impl core::convert::From<crate::W<I2C_PKTSIZE_SPEC>> for W {
    fn from(writer: crate::W<I2C_PKTSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLDSIZE` reader - Transfer Byte Number\\nThe transmission or receive byte number in one transaction when the PECEN is set. The maximum transaction or receive byte is 255 Bytes. \\nNotice: The byte number counting includes address, command code, and data frame."]
pub struct PLDSIZE_R(crate::FieldReader<u8, u8>);
impl PLDSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLDSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLDSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLDSIZE` writer - Transfer Byte Number\\nThe transmission or receive byte number in one transaction when the PECEN is set. The maximum transaction or receive byte is 255 Bytes. \\nNotice: The byte number counting includes address, command code, and data frame."]
pub struct PLDSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLDSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transfer Byte Number\\nThe transmission or receive byte number in one transaction when the PECEN is set. The maximum transaction or receive byte is 255 Bytes. \\nNotice: The byte number counting includes address, command code, and data frame."]
    #[inline(always)]
    pub fn pldsize(&self) -> PLDSIZE_R {
        PLDSIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transfer Byte Number\\nThe transmission or receive byte number in one transaction when the PECEN is set. The maximum transaction or receive byte is 255 Bytes. \\nNotice: The byte number counting includes address, command code, and data frame."]
    #[inline(always)]
    pub fn pldsize(&mut self) -> PLDSIZE_W {
        PLDSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Packet Error Checking Byte Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_pktsize](index.html) module"]
pub struct I2C_PKTSIZE_SPEC;
impl crate::RegisterSpec for I2C_PKTSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_pktsize::R](R) reader structure"]
impl crate::Readable for I2C_PKTSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_pktsize::W](W) writer structure"]
impl crate::Writable for I2C_PKTSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_PKTSIZE to value 0"]
impl crate::Resettable for I2C_PKTSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
