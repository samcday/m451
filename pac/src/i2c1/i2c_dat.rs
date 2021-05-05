#[doc = "Register `I2C_DAT` reader"]
pub struct R(crate::R<I2C_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_DAT_SPEC>> for R {
    fn from(reader: crate::R<I2C_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_DAT` writer"]
pub struct W(crate::W<I2C_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_DAT_SPEC>;
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
impl core::convert::From<crate::W<I2C_DAT_SPEC>> for W {
    fn from(writer: crate::W<I2C_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - I2C Data \\nBit \\[7:0\\]
is located with the 8-bit transferred/received data of I2C serial port."]
pub struct DAT_R(crate::FieldReader<u8, u8>);
impl DAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT` writer - I2C Data \\nBit \\[7:0\\]
is located with the 8-bit transferred/received data of I2C serial port."]
pub struct DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Data \\nBit \\[7:0\\]
is located with the 8-bit transferred/received data of I2C serial port."]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Data \\nBit \\[7:0\\]
is located with the 8-bit transferred/received data of I2C serial port."]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W {
        DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_dat](index.html) module"]
pub struct I2C_DAT_SPEC;
impl crate::RegisterSpec for I2C_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_dat::R](R) reader structure"]
impl crate::Readable for I2C_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_dat::W](W) writer structure"]
impl crate::Writable for I2C_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_DAT to value 0"]
impl crate::Resettable for I2C_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
