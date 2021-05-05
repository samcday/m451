#[doc = "Register `WDT_ALTCTL` reader"]
pub struct R(crate::R<WDT_ALTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_ALTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDT_ALTCTL_SPEC>> for R {
    fn from(reader: crate::R<WDT_ALTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_ALTCTL` writer"]
pub struct W(crate::W<WDT_ALTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_ALTCTL_SPEC>;
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
impl core::convert::From<crate::W<WDT_ALTCTL_SPEC>> for W {
    fn from(writer: crate::W<WDT_ALTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT Reset Delay Selection (Write Protect)\\nWhen WDT time-out happened, user has a time named WDT Reset Delay Period to clear WDT counter by setting RSTCNT (WDT_CTL\\[0\\]) to prevent WDT time-out reset happened. User can select a suitable setting of RSTDSEL for different WDT Reset Delay Period.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This register will be reset to 0 if WDT time-out reset happened.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTDSEL_A {
    #[doc = "0: WDT Reset Delay Period is 1026 * WDT_CLK"]
    _0 = 0,
    #[doc = "1: WDT Reset Delay Period is 130 * WDT_CLK"]
    _1 = 1,
    #[doc = "2: WDT Reset Delay Period is 18 * WDT_CLK"]
    _2 = 2,
    #[doc = "3: WDT Reset Delay Period is 3 * WDT_CLK"]
    _3 = 3,
}
impl From<RSTDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTDSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RSTDSEL` reader - WDT Reset Delay Selection (Write Protect)\\nWhen WDT time-out happened, user has a time named WDT Reset Delay Period to clear WDT counter by setting RSTCNT (WDT_CTL\\[0\\]) to prevent WDT time-out reset happened. User can select a suitable setting of RSTDSEL for different WDT Reset Delay Period.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This register will be reset to 0 if WDT time-out reset happened."]
pub struct RSTDSEL_R(crate::FieldReader<u8, RSTDSEL_A>);
impl RSTDSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSTDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTDSEL_A {
        match self.bits {
            0 => RSTDSEL_A::_0,
            1 => RSTDSEL_A::_1,
            2 => RSTDSEL_A::_2,
            3 => RSTDSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RSTDSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RSTDSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == RSTDSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == RSTDSEL_A::_3
    }
}
impl core::ops::Deref for RSTDSEL_R {
    type Target = crate::FieldReader<u8, RSTDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTDSEL` writer - WDT Reset Delay Selection (Write Protect)\\nWhen WDT time-out happened, user has a time named WDT Reset Delay Period to clear WDT counter by setting RSTCNT (WDT_CTL\\[0\\]) to prevent WDT time-out reset happened. User can select a suitable setting of RSTDSEL for different WDT Reset Delay Period.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This register will be reset to 0 if WDT time-out reset happened."]
pub struct RSTDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTDSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "WDT Reset Delay Period is 1026 * WDT_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTDSEL_A::_0)
    }
    #[doc = "WDT Reset Delay Period is 130 * WDT_CLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTDSEL_A::_1)
    }
    #[doc = "WDT Reset Delay Period is 18 * WDT_CLK"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RSTDSEL_A::_2)
    }
    #[doc = "WDT Reset Delay Period is 3 * WDT_CLK"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(RSTDSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - WDT Reset Delay Selection (Write Protect)\\nWhen WDT time-out happened, user has a time named WDT Reset Delay Period to clear WDT counter by setting RSTCNT (WDT_CTL\\[0\\]) to prevent WDT time-out reset happened. User can select a suitable setting of RSTDSEL for different WDT Reset Delay Period.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This register will be reset to 0 if WDT time-out reset happened."]
    #[inline(always)]
    pub fn rstdsel(&self) -> RSTDSEL_R {
        RSTDSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - WDT Reset Delay Selection (Write Protect)\\nWhen WDT time-out happened, user has a time named WDT Reset Delay Period to clear WDT counter by setting RSTCNT (WDT_CTL\\[0\\]) to prevent WDT time-out reset happened. User can select a suitable setting of RSTDSEL for different WDT Reset Delay Period.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This register will be reset to 0 if WDT time-out reset happened."]
    #[inline(always)]
    pub fn rstdsel(&mut self) -> RSTDSEL_W {
        RSTDSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Alternative Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_altctl](index.html) module"]
pub struct WDT_ALTCTL_SPEC;
impl crate::RegisterSpec for WDT_ALTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_altctl::R](R) reader structure"]
impl crate::Readable for WDT_ALTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_altctl::W](W) writer structure"]
impl crate::Writable for WDT_ALTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_ALTCTL to value 0"]
impl crate::Resettable for WDT_ALTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
