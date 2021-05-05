#[doc = "Register `SYS_PORCTL` reader"]
pub struct R(crate::R<SYS_PORCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_PORCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_PORCTL_SPEC>> for R {
    fn from(reader: crate::R<SYS_PORCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_PORCTL` writer"]
pub struct W(crate::W<SYS_PORCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_PORCTL_SPEC>;
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
impl core::convert::From<crate::W<SYS_PORCTL_SPEC>> for W {
    fn from(writer: crate::W<SYS_PORCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POROFF` reader - Power-on Reset Enable Bit (Write Protect)\\nWhen powered on, the POR circuit generates a reset signal to reset the whole chip function, but noise on the power may cause the POR active again. User can disable internal POR circuit to avoid unpredictable noise to cause chip reset by writing 0x5AA5 to this field.\\nThe POR function will be active again when this field is set to another value or chip is reset by other reset source, including:\\nnRESET, Watchdog, LVR reset, BOD reset, ICE reset command and the software-chip reset function.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct POROFF_R(crate::FieldReader<u16, u16>);
impl POROFF_R {
    pub(crate) fn new(bits: u16) -> Self {
        POROFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POROFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POROFF` writer - Power-on Reset Enable Bit (Write Protect)\\nWhen powered on, the POR circuit generates a reset signal to reset the whole chip function, but noise on the power may cause the POR active again. User can disable internal POR circuit to avoid unpredictable noise to cause chip reset by writing 0x5AA5 to this field.\\nThe POR function will be active again when this field is set to another value or chip is reset by other reset source, including:\\nnRESET, Watchdog, LVR reset, BOD reset, ICE reset command and the software-chip reset function.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct POROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> POROFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Power-on Reset Enable Bit (Write Protect)\\nWhen powered on, the POR circuit generates a reset signal to reset the whole chip function, but noise on the power may cause the POR active again. User can disable internal POR circuit to avoid unpredictable noise to cause chip reset by writing 0x5AA5 to this field.\\nThe POR function will be active again when this field is set to another value or chip is reset by other reset source, including:\\nnRESET, Watchdog, LVR reset, BOD reset, ICE reset command and the software-chip reset function.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn poroff(&self) -> POROFF_R {
        POROFF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Power-on Reset Enable Bit (Write Protect)\\nWhen powered on, the POR circuit generates a reset signal to reset the whole chip function, but noise on the power may cause the POR active again. User can disable internal POR circuit to avoid unpredictable noise to cause chip reset by writing 0x5AA5 to this field.\\nThe POR function will be active again when this field is set to another value or chip is reset by other reset source, including:\\nnRESET, Watchdog, LVR reset, BOD reset, ICE reset command and the software-chip reset function.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn poroff(&mut self) -> POROFF_W {
        POROFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power-On-reset Controller Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_porctl](index.html) module"]
pub struct SYS_PORCTL_SPEC;
impl crate::RegisterSpec for SYS_PORCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_porctl::R](R) reader structure"]
impl crate::Readable for SYS_PORCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_porctl::W](W) writer structure"]
impl crate::Writable for SYS_PORCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_PORCTL to value 0"]
impl crate::Resettable for SYS_PORCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
