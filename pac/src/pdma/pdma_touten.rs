#[doc = "Register `PDMA_TOUTEN` reader"]
pub struct R(crate::R<PDMA_TOUTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TOUTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_TOUTEN_SPEC>> for R {
    fn from(reader: crate::R<PDMA_TOUTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_TOUTEN` writer"]
pub struct W(crate::W<PDMA_TOUTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_TOUTEN_SPEC>;
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
impl core::convert::From<crate::W<PDMA_TOUTEN_SPEC>> for W {
    fn from(writer: crate::W<PDMA_TOUTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Time-out Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOUTENN_A {
    #[doc = "0: PDMA Channel n time-out function Disable"]
    _0 = 0,
    #[doc = "1: PDMA Channel n time-out function Enable"]
    _1 = 1,
}
impl From<TOUTENN_A> for u8 {
    #[inline(always)]
    fn from(variant: TOUTENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOUTENn` reader - PDMA Time-out Enable Bits"]
pub struct TOUTENN_R(crate::FieldReader<u8, TOUTENN_A>);
impl TOUTENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOUTENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOUTENN_A> {
        match self.bits {
            0 => Some(TOUTENN_A::_0),
            1 => Some(TOUTENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTENN_A::_1
    }
}
impl core::ops::Deref for TOUTENN_R {
    type Target = crate::FieldReader<u8, TOUTENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTENn` writer - PDMA Time-out Enable Bits"]
pub struct TOUTENN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDMA Channel n time-out function Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTENN_A::_0)
    }
    #[doc = "PDMA Channel n time-out function Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PDMA Time-out Enable Bits"]
    #[inline(always)]
    pub fn toutenn(&self) -> TOUTENN_R {
        TOUTENN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDMA Time-out Enable Bits"]
    #[inline(always)]
    pub fn toutenn(&mut self) -> TOUTENN_W {
        TOUTENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Time-out Enable Register (M45xD/M45xC Only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_touten](index.html) module"]
pub struct PDMA_TOUTEN_SPEC;
impl crate::RegisterSpec for PDMA_TOUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_touten::R](R) reader structure"]
impl crate::Readable for PDMA_TOUTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_touten::W](W) writer structure"]
impl crate::Writable for PDMA_TOUTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_TOUTEN to value 0"]
impl crate::Resettable for PDMA_TOUTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
