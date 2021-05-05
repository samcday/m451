#[doc = "Register `MXPLD0` reader"]
pub struct R(crate::R<MXPLD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MXPLD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MXPLD0_SPEC>> for R {
    fn from(reader: crate::R<MXPLD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MXPLD0` writer"]
pub struct W(crate::W<MXPLD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MXPLD0_SPEC>;
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
impl core::convert::From<crate::W<MXPLD0_SPEC>> for W {
    fn from(writer: crate::W<MXPLD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MXPLD` reader - Maximal Payload\\nDefine the data length which is transmitted to host (IN token) or the actual data length which is received from the host (OUT token). It also used to indicate that the endpoint is ready to be transmitted out IN token or received in OUT token.\\n(1) When the register is written by CPU, \\nFor IN token, the value of MXPLD is used to define the data length to be transmitted and indicate the data buffer is ready.\\nFor OUT token, it means that the controller is ready to receive data from the host and the value of MXPLD is the maximal data length comes from host.\\n(2) When the register is read by CPU,\\nFor IN token, the value of MXPLD is indicated by the data length be transmitted to host\\nFor OUT token, the value of MXPLD is indicated the actual data length received from host.\\nNote: Once MXPLD is written, the data packets will be transmitted/received immediately after IN/OUT token arrived."]
pub struct MXPLD_R(crate::FieldReader<u16, u16>);
impl MXPLD_R {
    pub(crate) fn new(bits: u16) -> Self {
        MXPLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MXPLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MXPLD` writer - Maximal Payload\\nDefine the data length which is transmitted to host (IN token) or the actual data length which is received from the host (OUT token). It also used to indicate that the endpoint is ready to be transmitted out IN token or received in OUT token.\\n(1) When the register is written by CPU, \\nFor IN token, the value of MXPLD is used to define the data length to be transmitted and indicate the data buffer is ready.\\nFor OUT token, it means that the controller is ready to receive data from the host and the value of MXPLD is the maximal data length comes from host.\\n(2) When the register is read by CPU,\\nFor IN token, the value of MXPLD is indicated by the data length be transmitted to host\\nFor OUT token, the value of MXPLD is indicated the actual data length received from host.\\nNote: Once MXPLD is written, the data packets will be transmitted/received immediately after IN/OUT token arrived."]
pub struct MXPLD_W<'a> {
    w: &'a mut W,
}
impl<'a> MXPLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Maximal Payload\\nDefine the data length which is transmitted to host (IN token) or the actual data length which is received from the host (OUT token). It also used to indicate that the endpoint is ready to be transmitted out IN token or received in OUT token.\\n(1) When the register is written by CPU, \\nFor IN token, the value of MXPLD is used to define the data length to be transmitted and indicate the data buffer is ready.\\nFor OUT token, it means that the controller is ready to receive data from the host and the value of MXPLD is the maximal data length comes from host.\\n(2) When the register is read by CPU,\\nFor IN token, the value of MXPLD is indicated by the data length be transmitted to host\\nFor OUT token, the value of MXPLD is indicated the actual data length received from host.\\nNote: Once MXPLD is written, the data packets will be transmitted/received immediately after IN/OUT token arrived."]
    #[inline(always)]
    pub fn mxpld(&self) -> MXPLD_R {
        MXPLD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Maximal Payload\\nDefine the data length which is transmitted to host (IN token) or the actual data length which is received from the host (OUT token). It also used to indicate that the endpoint is ready to be transmitted out IN token or received in OUT token.\\n(1) When the register is written by CPU, \\nFor IN token, the value of MXPLD is used to define the data length to be transmitted and indicate the data buffer is ready.\\nFor OUT token, it means that the controller is ready to receive data from the host and the value of MXPLD is the maximal data length comes from host.\\n(2) When the register is read by CPU,\\nFor IN token, the value of MXPLD is indicated by the data length be transmitted to host\\nFor OUT token, the value of MXPLD is indicated the actual data length received from host.\\nNote: Once MXPLD is written, the data packets will be transmitted/received immediately after IN/OUT token arrived."]
    #[inline(always)]
    pub fn mxpld(&mut self) -> MXPLD_W {
        MXPLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint 0 Maximal Payload Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mxpld0](index.html) module"]
pub struct MXPLD0_SPEC;
impl crate::RegisterSpec for MXPLD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mxpld0::R](R) reader structure"]
impl crate::Readable for MXPLD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mxpld0::W](W) writer structure"]
impl crate::Writable for MXPLD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MXPLD0 to value 0"]
impl crate::Resettable for MXPLD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
