#[doc = "Register `SC_RXTOUT` reader"]
pub struct R(crate::R<SC_RXTOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_RXTOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_RXTOUT_SPEC>> for R {
    fn from(reader: crate::R<SC_RXTOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_RXTOUT` writer"]
pub struct W(crate::W<SC_RXTOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_RXTOUT_SPEC>;
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
impl core::convert::From<crate::W<SC_RXTOUT_SPEC>> for W {
    fn from(writer: crate::W<SC_RXTOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFTM` reader - SC Receiver FIFO Time-out (ETU Base)\\nNote1: The counter unit is ETU based and the interval of time-out is RFTM + 0.5.\\nNote2: Filling all 0 to this field indicates to disable this function."]
pub struct RFTM_R(crate::FieldReader<u16, u16>);
impl RFTM_R {
    pub(crate) fn new(bits: u16) -> Self {
        RFTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFTM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFTM` writer - SC Receiver FIFO Time-out (ETU Base)\\nNote1: The counter unit is ETU based and the interval of time-out is RFTM + 0.5.\\nNote2: Filling all 0 to this field indicates to disable this function."]
pub struct RFTM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFTM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - SC Receiver FIFO Time-out (ETU Base)\\nNote1: The counter unit is ETU based and the interval of time-out is RFTM + 0.5.\\nNote2: Filling all 0 to this field indicates to disable this function."]
    #[inline(always)]
    pub fn rftm(&self) -> RFTM_R {
        RFTM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - SC Receiver FIFO Time-out (ETU Base)\\nNote1: The counter unit is ETU based and the interval of time-out is RFTM + 0.5.\\nNote2: Filling all 0 to this field indicates to disable this function."]
    #[inline(always)]
    pub fn rftm(&mut self) -> RFTM_W {
        RFTM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Receive Buffer Time-out Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_rxtout](index.html) module"]
pub struct SC_RXTOUT_SPEC;
impl crate::RegisterSpec for SC_RXTOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_rxtout::R](R) reader structure"]
impl crate::Readable for SC_RXTOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_rxtout::W](W) writer structure"]
impl crate::Writable for SC_RXTOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_RXTOUT to value 0"]
impl crate::Resettable for SC_RXTOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
