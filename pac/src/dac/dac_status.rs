#[doc = "Register `DAC_STATUS` reader"]
pub struct R(crate::R<DAC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DAC_STATUS_SPEC>> for R {
    fn from(reader: crate::R<DAC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_STATUS` writer"]
pub struct W(crate::W<DAC_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_STATUS_SPEC>;
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
impl core::convert::From<crate::W<DAC_STATUS_SPEC>> for W {
    fn from(writer: crate::W<DAC_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC Conversion Complete Finish Flag\\nThis bit set to 1 when conversion time counter counts to SETTLET. It is cleared to 0 when DAC starts a new conversion. User writes 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINISH_A {
    #[doc = "0: DAC is in conversion state"]
    _0 = 0,
    #[doc = "1: DAC conversion finish"]
    _1 = 1,
}
impl From<FINISH_A> for bool {
    #[inline(always)]
    fn from(variant: FINISH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FINISH` reader - DAC Conversion Complete Finish Flag\\nThis bit set to 1 when conversion time counter counts to SETTLET. It is cleared to 0 when DAC starts a new conversion. User writes 1 to clear this bit to 0."]
pub struct FINISH_R(crate::FieldReader<bool, FINISH_A>);
impl FINISH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FINISH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINISH_A {
        match self.bits {
            false => FINISH_A::_0,
            true => FINISH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FINISH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FINISH_A::_1
    }
}
impl core::ops::Deref for FINISH_R {
    type Target = crate::FieldReader<bool, FINISH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINISH` writer - DAC Conversion Complete Finish Flag\\nThis bit set to 1 when conversion time counter counts to SETTLET. It is cleared to 0 when DAC starts a new conversion. User writes 1 to clear this bit to 0."]
pub struct FINISH_W<'a> {
    w: &'a mut W,
}
impl<'a> FINISH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINISH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC is in conversion state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FINISH_A::_0)
    }
    #[doc = "DAC conversion finish"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FINISH_A::_1)
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
#[doc = "DMA Under Run Interrupt Flag\\nUser writes 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR_A {
    #[doc = "0: No DMA under-run error condition occurred"]
    _0 = 0,
    #[doc = "1: DMA under-run error condition occurred"]
    _1 = 1,
}
impl From<DMAUDR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDR` reader - DMA Under Run Interrupt Flag\\nUser writes 1 to clear this bit."]
pub struct DMAUDR_R(crate::FieldReader<bool, DMAUDR_A>);
impl DMAUDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR_A {
        match self.bits {
            false => DMAUDR_A::_0,
            true => DMAUDR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMAUDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMAUDR_A::_1
    }
}
impl core::ops::Deref for DMAUDR_R {
    type Target = crate::FieldReader<bool, DMAUDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDR` writer - DMA Under Run Interrupt Flag\\nUser writes 1 to clear this bit."]
pub struct DMAUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA under-run error condition occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAUDR_A::_0)
    }
    #[doc = "DMA under-run error condition occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAUDR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "DAC Busy Flag (Read Only)\\nThis is read only bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: DAC is ready for next conversion"]
    _0 = 0,
    #[doc = "1: DAC is busy in conversion"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - DAC Busy Flag (Read Only)\\nThis is read only bit."]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSY_A::_1
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DAC Conversion Complete Finish Flag\\nThis bit set to 1 when conversion time counter counts to SETTLET. It is cleared to 0 when DAC starts a new conversion. User writes 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn finish(&self) -> FINISH_R {
        FINISH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Under Run Interrupt Flag\\nUser writes 1 to clear this bit."]
    #[inline(always)]
    pub fn dmaudr(&self) -> DMAUDR_R {
        DMAUDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAC Busy Flag (Read Only)\\nThis is read only bit."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Conversion Complete Finish Flag\\nThis bit set to 1 when conversion time counter counts to SETTLET. It is cleared to 0 when DAC starts a new conversion. User writes 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn finish(&mut self) -> FINISH_W {
        FINISH_W { w: self }
    }
    #[doc = "Bit 1 - DMA Under Run Interrupt Flag\\nUser writes 1 to clear this bit."]
    #[inline(always)]
    pub fn dmaudr(&mut self) -> DMAUDR_W {
        DMAUDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_status](index.html) module"]
pub struct DAC_STATUS_SPEC;
impl crate::RegisterSpec for DAC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_status::R](R) reader structure"]
impl crate::Readable for DAC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_status::W](W) writer structure"]
impl crate::Writable for DAC_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_STATUS to value 0"]
impl crate::Resettable for DAC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
