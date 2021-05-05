#[doc = "Register `WWDT_RLDCNT` writer"]
pub struct W(crate::W<WWDT_RLDCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDT_RLDCNT_SPEC>;
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
impl core::convert::From<crate::W<WWDT_RLDCNT_SPEC>> for W {
    fn from(writer: crate::W<WWDT_RLDCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLDCNT` writer - WWDT Reload Counter Register\\nWriting 0x00005AA5 to this register will reload the WWDT counter value to 0x3F.\\nNote: User can only write WWDT_RLDCNT register to reload WWDT counter value when current WWDT counter value between 0 and CMPDAT (WWDT_CTL\\[21:16\\]). If user writes WWDT_RLDCNT when current WWDT counter value is larger than CMPDAT , WWDT reset signal will generate immediately."]
pub struct RLDCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RLDCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - WWDT Reload Counter Register\\nWriting 0x00005AA5 to this register will reload the WWDT counter value to 0x3F.\\nNote: User can only write WWDT_RLDCNT register to reload WWDT counter value when current WWDT counter value between 0 and CMPDAT (WWDT_CTL\\[21:16\\]). If user writes WWDT_RLDCNT when current WWDT counter value is larger than CMPDAT , WWDT reset signal will generate immediately."]
    #[inline(always)]
    pub fn rldcnt(&mut self) -> RLDCNT_W {
        RLDCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WWDT Reload Counter Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdt_rldcnt](index.html) module"]
pub struct WWDT_RLDCNT_SPEC;
impl crate::RegisterSpec for WWDT_RLDCNT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wwdt_rldcnt::W](W) writer structure"]
impl crate::Writable for WWDT_RLDCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WWDT_RLDCNT to value 0"]
impl crate::Resettable for WWDT_RLDCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
