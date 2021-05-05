#[doc = "Register `FMC_ISPTRG` reader"]
pub struct R(crate::R<FMC_ISPTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_ISPTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_ISPTRG_SPEC>> for R {
    fn from(reader: crate::R<FMC_ISPTRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_ISPTRG` writer"]
pub struct W(crate::W<FMC_ISPTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_ISPTRG_SPEC>;
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
impl core::convert::From<crate::W<FMC_ISPTRG_SPEC>> for W {
    fn from(writer: crate::W<FMC_ISPTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ISP Start Trigger (Write Protect)\\nWrite 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISPGO_A {
    #[doc = "0: ISP operation is finished"]
    _0 = 0,
    #[doc = "1: ISP is progressed"]
    _1 = 1,
}
impl From<ISPGO_A> for bool {
    #[inline(always)]
    fn from(variant: ISPGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISPGO` reader - ISP Start Trigger (Write Protect)\\nWrite 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct ISPGO_R(crate::FieldReader<bool, ISPGO_A>);
impl ISPGO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISPGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISPGO_A {
        match self.bits {
            false => ISPGO_A::_0,
            true => ISPGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISPGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISPGO_A::_1
    }
}
impl core::ops::Deref for ISPGO_R {
    type Target = crate::FieldReader<bool, ISPGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPGO` writer - ISP Start Trigger (Write Protect)\\nWrite 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct ISPGO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISPGO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ISP operation is finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISPGO_A::_0)
    }
    #[doc = "ISP is progressed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISPGO_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ISP Start Trigger (Write Protect)\\nWrite 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ispgo(&self) -> ISPGO_R {
        ISPGO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISP Start Trigger (Write Protect)\\nWrite 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ispgo(&mut self) -> ISPGO_W {
        ISPGO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_isptrg](index.html) module"]
pub struct FMC_ISPTRG_SPEC;
impl crate::RegisterSpec for FMC_ISPTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_isptrg::R](R) reader structure"]
impl crate::Readable for FMC_ISPTRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_isptrg::W](W) writer structure"]
impl crate::Writable for FMC_ISPTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_ISPTRG to value 0"]
impl crate::Resettable for FMC_ISPTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
