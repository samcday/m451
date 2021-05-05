#[doc = "Register `ACMP_STATUS` reader"]
pub struct R(crate::R<ACMP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACMP_STATUS_SPEC>> for R {
    fn from(reader: crate::R<ACMP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMP_STATUS` writer"]
pub struct W(crate::W<ACMP_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMP_STATUS_SPEC>;
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
impl core::convert::From<crate::W<ACMP_STATUS_SPEC>> for W {
    fn from(writer: crate::W<ACMP_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMPIF0` reader - Comparator 0 Interrupt Flag\\nThis bit is set by hardware when the edge condition defined by INTPOL (ACMP_CTL0\\[9:8\\]) is detected on comparator 0 output. This will generate an interrupt if ACMPIE (ACMP_CTL0\\[1\\]) is set to 1.\\nNote: Write 1 to clear this bit to 0."]
pub struct ACMPIF0_R(crate::FieldReader<bool, bool>);
impl ACMPIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMPIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACMPIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMPIF0` writer - Comparator 0 Interrupt Flag\\nThis bit is set by hardware when the edge condition defined by INTPOL (ACMP_CTL0\\[9:8\\]) is detected on comparator 0 output. This will generate an interrupt if ACMPIE (ACMP_CTL0\\[1\\]) is set to 1.\\nNote: Write 1 to clear this bit to 0."]
pub struct ACMPIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMPIF0_W<'a> {
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
#[doc = "Field `ACMPIF1` reader - Comparator 1 Interrupt Flag\\nThis bit is set by hardware when the edge condition defined by INTPOL (ACMP_CTL1\\[9:8\\]) is detected on comparator 1 output. This will cause an interrupt if ACMPIE (ACMP_CTL1\\[1\\]) is set to 1.\\nNote: Write 1 to clear this bit to 0."]
pub struct ACMPIF1_R(crate::FieldReader<bool, bool>);
impl ACMPIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMPIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACMPIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMPIF1` writer - Comparator 1 Interrupt Flag\\nThis bit is set by hardware when the edge condition defined by INTPOL (ACMP_CTL1\\[9:8\\]) is detected on comparator 1 output. This will cause an interrupt if ACMPIE (ACMP_CTL1\\[1\\]) is set to 1.\\nNote: Write 1 to clear this bit to 0."]
pub struct ACMPIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMPIF1_W<'a> {
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
#[doc = "Field `ACMPO0` reader - Comparator 0 Output\\nSynchronized to the PCLK to allow reading by software. Cleared when the comparator 0 is disabled, i.e. ACMPEN (ACMP_CTL0\\[0\\]) is cleared to 0."]
pub struct ACMPO0_R(crate::FieldReader<bool, bool>);
impl ACMPO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMPO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACMPO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMPO0` writer - Comparator 0 Output\\nSynchronized to the PCLK to allow reading by software. Cleared when the comparator 0 is disabled, i.e. ACMPEN (ACMP_CTL0\\[0\\]) is cleared to 0."]
pub struct ACMPO0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMPO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ACMPO1` reader - Comparator 1 Output\\nSynchronized to the PCLK to allow reading by software. Cleared when the comparator 1 is disabled, i.e. ACMPEN (ACMP_CTL1\\[0\\]) is cleared to 0."]
pub struct ACMPO1_R(crate::FieldReader<bool, bool>);
impl ACMPO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMPO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACMPO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMPO1` writer - Comparator 1 Output\\nSynchronized to the PCLK to allow reading by software. Cleared when the comparator 1 is disabled, i.e. ACMPEN (ACMP_CTL1\\[0\\]) is cleared to 0."]
pub struct ACMPO1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMPO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Comparator 0 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP0 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKIF0_A {
    #[doc = "0: No power-down wake-up occurred"]
    _0 = 0,
    #[doc = "1: Power-down wake-up occurred"]
    _1 = 1,
}
impl From<WKIF0_A> for bool {
    #[inline(always)]
    fn from(variant: WKIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKIF0` reader - Comparator 0 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP0 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0."]
pub struct WKIF0_R(crate::FieldReader<bool, WKIF0_A>);
impl WKIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKIF0_A {
        match self.bits {
            false => WKIF0_A::_0,
            true => WKIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKIF0_A::_1
    }
}
impl core::ops::Deref for WKIF0_R {
    type Target = crate::FieldReader<bool, WKIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKIF0` writer - Comparator 0 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP0 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0."]
pub struct WKIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> WKIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No power-down wake-up occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKIF0_A::_0)
    }
    #[doc = "Power-down wake-up occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKIF0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Comparator 1 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP1 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKIF1_A {
    #[doc = "0: No power-down wake-up occurred"]
    _0 = 0,
    #[doc = "1: Power-down wake-up occurred"]
    _1 = 1,
}
impl From<WKIF1_A> for bool {
    #[inline(always)]
    fn from(variant: WKIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKIF1` reader - Comparator 1 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP1 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0."]
pub struct WKIF1_R(crate::FieldReader<bool, WKIF1_A>);
impl WKIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKIF1_A {
        match self.bits {
            false => WKIF1_A::_0,
            true => WKIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKIF1_A::_1
    }
}
impl core::ops::Deref for WKIF1_R {
    type Target = crate::FieldReader<bool, WKIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKIF1` writer - Comparator 1 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP1 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0."]
pub struct WKIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No power-down wake-up occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKIF1_A::_0)
    }
    #[doc = "Power-down wake-up occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKIF1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt Flag\\nThis bit is set by hardware when the edge condition defined by INTPOL (ACMP_CTL0\\[9:8\\]) is detected on comparator 0 output. This will generate an interrupt if ACMPIE (ACMP_CTL0\\[1\\]) is set to 1.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn acmpif0(&self) -> ACMPIF0_R {
        ACMPIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Flag\\nThis bit is set by hardware when the edge condition defined by INTPOL (ACMP_CTL1\\[9:8\\]) is detected on comparator 1 output. This will cause an interrupt if ACMPIE (ACMP_CTL1\\[1\\]) is set to 1.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn acmpif1(&self) -> ACMPIF1_R {
        ACMPIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator 0 Output\\nSynchronized to the PCLK to allow reading by software. Cleared when the comparator 0 is disabled, i.e. ACMPEN (ACMP_CTL0\\[0\\]) is cleared to 0."]
    #[inline(always)]
    pub fn acmpo0(&self) -> ACMPO0_R {
        ACMPO0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator 1 Output\\nSynchronized to the PCLK to allow reading by software. Cleared when the comparator 1 is disabled, i.e. ACMPEN (ACMP_CTL1\\[0\\]) is cleared to 0."]
    #[inline(always)]
    pub fn acmpo1(&self) -> ACMPO1_R {
        ACMPO1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comparator 0 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP0 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn wkif0(&self) -> WKIF0_R {
        WKIF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator 1 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP1 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn wkif1(&self) -> WKIF1_R {
        WKIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt Flag\\nThis bit is set by hardware when the edge condition defined by INTPOL (ACMP_CTL0\\[9:8\\]) is detected on comparator 0 output. This will generate an interrupt if ACMPIE (ACMP_CTL0\\[1\\]) is set to 1.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn acmpif0(&mut self) -> ACMPIF0_W {
        ACMPIF0_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Flag\\nThis bit is set by hardware when the edge condition defined by INTPOL (ACMP_CTL1\\[9:8\\]) is detected on comparator 1 output. This will cause an interrupt if ACMPIE (ACMP_CTL1\\[1\\]) is set to 1.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn acmpif1(&mut self) -> ACMPIF1_W {
        ACMPIF1_W { w: self }
    }
    #[doc = "Bit 4 - Comparator 0 Output\\nSynchronized to the PCLK to allow reading by software. Cleared when the comparator 0 is disabled, i.e. ACMPEN (ACMP_CTL0\\[0\\]) is cleared to 0."]
    #[inline(always)]
    pub fn acmpo0(&mut self) -> ACMPO0_W {
        ACMPO0_W { w: self }
    }
    #[doc = "Bit 5 - Comparator 1 Output\\nSynchronized to the PCLK to allow reading by software. Cleared when the comparator 1 is disabled, i.e. ACMPEN (ACMP_CTL1\\[0\\]) is cleared to 0."]
    #[inline(always)]
    pub fn acmpo1(&mut self) -> ACMPO1_W {
        ACMPO1_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 0 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP0 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn wkif0(&mut self) -> WKIF0_W {
        WKIF0_W { w: self }
    }
    #[doc = "Bit 9 - Comparator 1 Power-down Wake-up Interrupt Flag\\nThis bit will be set to 1 when ACMP1 wake-up interrupt event occurs.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn wkif1(&mut self) -> WKIF1_W {
        WKIF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmp_status](index.html) module"]
pub struct ACMP_STATUS_SPEC;
impl crate::RegisterSpec for ACMP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmp_status::R](R) reader structure"]
impl crate::Readable for ACMP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmp_status::W](W) writer structure"]
impl crate::Writable for ACMP_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACMP_STATUS to value 0"]
impl crate::Resettable for ACMP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
